// 763. JSON-Like Format Built From Scratch
// Recursive AST + hand-written serializer and parser

use std::collections::HashMap;
use std::fmt;

// ── Value type ─────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub enum Json {
    Null,
    Bool(bool),
    Number(f64),
    Str(String),
    Array(Vec<Json>),
    Object(Vec<(String, Json)>),
}

// ── Serializer ─────────────────────────────────────────────────────────────────

impl fmt::Display for Json {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Json::Null        => write!(f, "null"),
            Json::Bool(b)     => write!(f, "{b}"),
            Json::Number(n)   => {
                if *n == n.floor() && n.abs() < 1e15 {
                    write!(f, "{}", *n as i64)
                } else {
                    write!(f, "{n}")
                }
            }
            Json::Str(s)      => {
                write!(f, "\"")?;
                for c in s.chars() {
                    match c {
                        '"'  => write!(f, "\\\"")?,
                        '\\' => write!(f, "\\\\")?,
                        '\n' => write!(f, "\\n")?,
                        '\t' => write!(f, "\\t")?,
                        c    => write!(f, "{c}")?,
                    }
                }
                write!(f, "\"")
            }
            Json::Array(arr)  => {
                write!(f, "[")?;
                for (i, v) in arr.iter().enumerate() {
                    if i > 0 { write!(f, ", ")?; }
                    write!(f, "{v}")?;
                }
                write!(f, "]")
            }
            Json::Object(obj) => {
                write!(f, "{{")?;
                for (i, (k, v)) in obj.iter().enumerate() {
                    if i > 0 { write!(f, ", ")?; }
                    write!(f, "\"{k}\": {v}")?;
                }
                write!(f, "}}")
            }
        }
    }
}

// ── Parser ─────────────────────────────────────────────────────────────────────

#[derive(Debug)]
pub struct ParseError(String);

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{}", self.0) }
}

struct Parser<'a> {
    s: &'a [u8],
    pos: usize,
}

impl<'a> Parser<'a> {
    fn new(s: &'a str) -> Self { Parser { s: s.as_bytes(), pos: 0 } }

    fn peek(&self) -> Option<u8> { self.s.get(self.pos).copied() }

    fn next(&mut self) -> Option<u8> {
        let b = self.s.get(self.pos).copied()?;
        self.pos += 1;
        Some(b)
    }

    fn skip_ws(&mut self) {
        while matches!(self.peek(), Some(b' ' | b'\t' | b'\n' | b'\r')) {
            self.pos += 1;
        }
    }

    fn expect(&mut self, b: u8) -> Result<(), ParseError> {
        match self.next() {
            Some(c) if c == b => Ok(()),
            c => Err(ParseError(format!("expected '{}' got {:?}", b as char, c.map(|x| x as char)))),
        }
    }

    fn parse_string(&mut self) -> Result<String, ParseError> {
        self.expect(b'"')?;
        let mut out = String::new();
        loop {
            match self.next().ok_or_else(|| ParseError("unterminated string".into()))? {
                b'"' => return Ok(out),
                b'\\' => {
                    match self.next().ok_or_else(|| ParseError("escape at EOF".into()))? {
                        b'n' => out.push('\n'),
                        b't' => out.push('\t'),
                        b'"' => out.push('"'),
                        b'\\' => out.push('\\'),
                        c => out.push(c as char),
                    }
                }
                c => out.push(c as char),
            }
        }
    }

    fn parse_value(&mut self) -> Result<Json, ParseError> {
        self.skip_ws();
        match self.peek().ok_or_else(|| ParseError("unexpected EOF".into()))? {
            b'"' => Ok(Json::Str(self.parse_string()?)),
            b't' => { self.pos += 4; Ok(Json::Bool(true)) }
            b'f' => { self.pos += 5; Ok(Json::Bool(false)) }
            b'n' => { self.pos += 4; Ok(Json::Null) }
            b'[' => {
                self.pos += 1;
                self.skip_ws();
                if self.peek() == Some(b']') { self.pos += 1; return Ok(Json::Array(vec![])); }
                let mut arr = vec![];
                loop {
                    arr.push(self.parse_value()?);
                    self.skip_ws();
                    match self.peek() {
                        Some(b',') => { self.pos += 1; }
                        Some(b']') => { self.pos += 1; break; }
                        _ => return Err(ParseError("expected ',' or ']'".into())),
                    }
                }
                Ok(Json::Array(arr))
            }
            b'{' => {
                self.pos += 1;
                self.skip_ws();
                if self.peek() == Some(b'}') { self.pos += 1; return Ok(Json::Object(vec![])); }
                let mut obj = vec![];
                loop {
                    self.skip_ws();
                    let k = self.parse_string()?;
                    self.skip_ws();
                    self.expect(b':')?;
                    let v = self.parse_value()?;
                    obj.push((k, v));
                    self.skip_ws();
                    match self.peek() {
                        Some(b',') => { self.pos += 1; }
                        Some(b'}') => { self.pos += 1; break; }
                        _ => return Err(ParseError("expected ',' or '}'".into())),
                    }
                }
                Ok(Json::Object(obj))
            }
            c if c == b'-' || c.is_ascii_digit() => {
                let start = self.pos;
                while matches!(self.peek(), Some(b'0'..=b'9' | b'.' | b'-' | b'e' | b'E' | b'+')) {
                    self.pos += 1;
                }
                let tok = std::str::from_utf8(&self.s[start..self.pos]).unwrap();
                tok.parse::<f64>().map(Json::Number)
                    .map_err(|e| ParseError(format!("bad number {tok}: {e}")))
            }
            c => Err(ParseError(format!("unexpected char '{}'", c as char))),
        }
    }
}

pub fn parse(s: &str) -> Result<Json, ParseError> {
    Parser::new(s).parse_value()
}

fn main() {
    let v = Json::Object(vec![
        ("name".into(), Json::Str("Alice".into())),
        ("age".into(), Json::Number(30.0)),
        ("scores".into(), Json::Array(vec![Json::Number(95.0), Json::Number(87.0)])),
        ("active".into(), Json::Bool(true)),
        ("address".into(), Json::Null),
    ]);

    let s = v.to_string();
    println!("Serialized:\n{s}\n");

    let v2 = parse(&s).expect("parse failed");
    println!("Re-serialized:\n{v2}");
    println!("\nEqual: {}", v == v2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trip_primitives() {
        for s in ["null", "true", "false", "42", "\"hello\""] {
            let v = parse(s).expect(s);
            assert_eq!(v.to_string(), s, "round-trip {s}");
        }
    }

    #[test]
    fn round_trip_array() {
        let s = "[1, 2, 3]";
        let v = parse(s).unwrap();
        assert!(matches!(v, Json::Array(_)));
        assert_eq!(v.to_string(), "[1, 2, 3]");
    }

    #[test]
    fn nested_object() {
        let s = r#"{"a": {"b": 1}}"#;
        let v = parse(s).unwrap();
        assert!(matches!(v, Json::Object(_)));
    }

    #[test]
    fn escaped_string() {
        let s = r#""hel\"lo""#;
        let v = parse(s).unwrap();
        assert_eq!(v, Json::Str("hel\"lo".into()));
    }
}
