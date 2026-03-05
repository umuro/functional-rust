// 724. Zero-copy parsing with byte slices
//
// Returns &str / &[u8] slices into the input buffer — no allocation.
// Lifetimes tie parsed references to the original input.

use std::str;

// ── Error type ────────────────────────────────────────────────────────────────

#[derive(Debug, PartialEq)]
pub enum ParseError {
    UnexpectedEof,
    InvalidUtf8,
    MissingDelimiter(u8),
    InvalidFormat(&'static str),
}

// ── Low-level byte-slice combinators ─────────────────────────────────────────

/// Take `n` bytes from the front of `buf`, returning (taken, rest).
pub fn take(buf: &[u8], n: usize) -> Result<(&[u8], &[u8]), ParseError> {
    if buf.len() < n {
        Err(ParseError::UnexpectedEof)
    } else {
        Ok((&buf[..n], &buf[n..]))
    }
}

/// Consume bytes until `delimiter` (exclusive), returning (before, after_delim).
pub fn take_until(buf: &[u8], delimiter: u8) -> Result<(&[u8], &[u8]), ParseError> {
    buf.iter()
        .position(|&b| b == delimiter)
        .map(|pos| (&buf[..pos], &buf[pos + 1..]))
        .ok_or(ParseError::MissingDelimiter(delimiter))
}

/// Interpret a byte slice as UTF-8 `&str` — zero-copy, zero allocation.
pub fn as_str(buf: &[u8]) -> Result<&str, ParseError> {
    str::from_utf8(buf).map_err(|_| ParseError::InvalidUtf8)
}

/// Skip leading ASCII whitespace, returning the trimmed slice.
pub fn skip_whitespace(buf: &[u8]) -> &[u8] {
    let pos = buf
        .iter()
        .position(|b| !b.is_ascii_whitespace())
        .unwrap_or(buf.len());
    &buf[pos..]
}

// ── Span — index-pair view into a shared buffer ───────────────────────────────

/// A lightweight window into a byte buffer: start index + length.
/// Mirrors the OCaml `span` record, but without copying.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub start: usize,
    pub len: usize,
}

impl Span {
    pub fn new(start: usize, len: usize) -> Self {
        Self { start, len }
    }

    /// Resolve the span against the original buffer — still zero-copy.
    pub fn slice<'a>(&self, buf: &'a [u8]) -> &'a [u8] {
        &buf[self.start..self.start + self.len]
    }

    pub fn as_str<'a>(&self, buf: &'a [u8]) -> Result<&'a str, ParseError> {
        as_str(self.slice(buf))
    }
}

/// Split `buf` at the first `sep` byte, returning two `Span`s (no allocation).
pub fn span_split_at(buf: &[u8], start: usize, len: usize, sep: u8) -> Option<(Span, Span)> {
    let slice = &buf[start..start + len];
    slice.iter().position(|&b| b == sep).map(|pos| {
        let left = Span::new(start, pos);
        // +1 to skip the separator itself
        let right = Span::new(start + pos + 1, len - pos - 1);
        (left, right)
    })
}

// ── HTTP request-line parser ──────────────────────────────────────────────────

/// Parsed HTTP request line.  All fields borrow from the input buffer.
#[derive(Debug, PartialEq)]
pub struct RequestLine<'a> {
    pub method: &'a str,
    pub path: &'a str,
    pub version: &'a str,
}

/// Parse `"METHOD /path HTTP/1.x\r\n"` without allocating.
///
/// Every `&str` in the returned struct points directly into `buf`.
pub fn parse_request_line(buf: &[u8]) -> Result<RequestLine<'_>, ParseError> {
    // Consume up to first space → method
    let (method_bytes, rest) = take_until(buf, b' ')?;
    let method = as_str(method_bytes)?;

    let rest = skip_whitespace(rest);

    // Consume up to second space → path
    let (path_bytes, rest) = take_until(rest, b' ')?;
    let path = as_str(path_bytes)?;

    let rest = skip_whitespace(rest);

    // Consume up to \r\n or end of slice → version
    let version_bytes = rest
        .iter()
        .position(|&b| b == b'\r' || b == b'\n')
        .map(|pos| &rest[..pos])
        .unwrap_or(rest);
    let version = as_str(version_bytes)?;

    Ok(RequestLine {
        method,
        path,
        version,
    })
}

// ── CSV field iterator — yields &str slices, zero-copy ───────────────────────

/// Iterator over comma-separated fields in a single CSV row.
/// Yields `&str` slices borrowed from the original input.
pub struct CsvFields<'a> {
    remaining: &'a [u8],
    done: bool,
}

impl<'a> CsvFields<'a> {
    pub fn new(row: &'a [u8]) -> Self {
        Self {
            remaining: row,
            done: false,
        }
    }
}

impl<'a> Iterator for CsvFields<'a> {
    type Item = Result<&'a str, ParseError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }
        match self.remaining.iter().position(|&b| b == b',') {
            Some(pos) => {
                let field = &self.remaining[..pos];
                self.remaining = &self.remaining[pos + 1..];
                Some(as_str(field))
            }
            None => {
                // Last field — consume everything
                self.done = true;
                if self.remaining.is_empty() {
                    None
                } else {
                    let field = self.remaining;
                    self.remaining = &[];
                    Some(as_str(field))
                }
            }
        }
    }
}

/// Collect all CSV fields from a row into a `Vec<&str>`, zero-copy.
pub fn parse_csv_row(row: &[u8]) -> Result<Vec<&str>, ParseError> {
    CsvFields::new(row).collect()
}

// ── Key=Value line parser ─────────────────────────────────────────────────────

/// A single `key=value` pair, both halves borrowing from the input.
#[derive(Debug, PartialEq)]
pub struct KeyValue<'a> {
    pub key: &'a str,
    pub value: &'a str,
}

pub fn parse_key_value(line: &[u8]) -> Result<KeyValue<'_>, ParseError> {
    let (key_bytes, value_bytes) = take_until(line, b'=')?;
    Ok(KeyValue {
        key: as_str(key_bytes)?,
        value: as_str(value_bytes)?,
    })
}

// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // ── take ─────────────────────────────────────────────────────────────────

    #[test]
    fn take_splits_correctly() {
        let buf = b"Hello, world!";
        let (head, tail) = take(buf, 5).unwrap();
        assert_eq!(head, b"Hello");
        assert_eq!(tail, b", world!");
    }

    #[test]
    fn take_eof_returns_error() {
        let buf = b"Hi";
        assert_eq!(take(buf, 10), Err(ParseError::UnexpectedEof));
    }

    #[test]
    fn take_zero_returns_empty_head() {
        let buf = b"abc";
        let (head, tail) = take(buf, 0).unwrap();
        assert_eq!(head, b"");
        assert_eq!(tail, b"abc");
    }

    // ── take_until ───────────────────────────────────────────────────────────

    #[test]
    fn take_until_finds_delimiter() {
        let buf = b"key=value";
        let (before, after) = take_until(buf, b'=').unwrap();
        assert_eq!(before, b"key");
        assert_eq!(after, b"value");
    }

    #[test]
    fn take_until_missing_delimiter_errors() {
        let buf = b"nodot";
        assert_eq!(
            take_until(buf, b'.'),
            Err(ParseError::MissingDelimiter(b'.'))
        );
    }

    // ── Span ─────────────────────────────────────────────────────────────────

    #[test]
    fn span_slice_is_zero_copy() {
        let buf = b"Hello, world!";
        let span = Span::new(7, 5);
        assert_eq!(span.slice(buf), b"world");
        assert_eq!(span.as_str(buf).unwrap(), "world");
    }

    #[test]
    fn span_split_at_produces_two_windows() {
        let buf = b"left:right";
        let (l, r) = span_split_at(buf, 0, buf.len(), b':').unwrap();
        assert_eq!(l.as_str(buf).unwrap(), "left");
        assert_eq!(r.as_str(buf).unwrap(), "right");
    }

    #[test]
    fn span_split_at_missing_sep_returns_none() {
        let buf = b"nodot";
        assert!(span_split_at(buf, 0, buf.len(), b'.').is_none());
    }

    // ── HTTP request-line ─────────────────────────────────────────────────────

    #[test]
    fn parse_request_line_get() {
        let input = b"GET /index.html HTTP/1.1\r\n";
        let req = parse_request_line(input).unwrap();
        assert_eq!(req.method, "GET");
        assert_eq!(req.path, "/index.html");
        assert_eq!(req.version, "HTTP/1.1");
    }

    #[test]
    fn parse_request_line_post_no_crlf() {
        let input = b"POST /api/data HTTP/2";
        let req = parse_request_line(input).unwrap();
        assert_eq!(req.method, "POST");
        assert_eq!(req.path, "/api/data");
        assert_eq!(req.version, "HTTP/2");
    }

    #[test]
    fn parse_request_line_missing_path_errors() {
        let input = b"GET";
        assert!(parse_request_line(input).is_err());
    }

    // ── CSV row ───────────────────────────────────────────────────────────────

    #[test]
    fn parse_csv_row_three_fields() {
        let row = b"alice,30,engineer";
        let fields = parse_csv_row(row).unwrap();
        assert_eq!(fields, vec!["alice", "30", "engineer"]);
    }

    #[test]
    fn parse_csv_row_single_field() {
        let row = b"only";
        let fields = parse_csv_row(row).unwrap();
        assert_eq!(fields, vec!["only"]);
    }

    #[test]
    fn parse_csv_row_empty_fields() {
        let row = b"a,,c";
        let fields = parse_csv_row(row).unwrap();
        assert_eq!(fields, vec!["a", "", "c"]);
    }

    // ── key=value ─────────────────────────────────────────────────────────────

    #[test]
    fn parse_key_value_basic() {
        let line = b"host=localhost";
        let kv = parse_key_value(line).unwrap();
        assert_eq!(kv.key, "host");
        assert_eq!(kv.value, "localhost");
    }

    #[test]
    fn parse_key_value_value_with_equals() {
        // Only splits on the FIRST '='
        let line = b"url=http://x?a=1";
        let kv = parse_key_value(line).unwrap();
        assert_eq!(kv.key, "url");
        assert_eq!(kv.value, "http://x?a=1");
    }

    #[test]
    fn parse_key_value_missing_equals_errors() {
        let line = b"noequals";
        assert!(parse_key_value(line).is_err());
    }

    // ── Lifetime safety (compile-time) ────────────────────────────────────────

    #[test]
    fn parsed_fields_borrow_from_input() {
        let input = b"name=Ferris";
        let kv = parse_key_value(input).unwrap();
        // Both &str slices point into `input` — no heap allocation occurred.
        assert!(std::ptr::eq(kv.key.as_bytes().as_ptr(), input.as_ptr()));
        assert!(std::ptr::eq(kv.value.as_bytes().as_ptr(), unsafe {
            input.as_ptr().add(5)
        }));
    }
}
