// 769. Streaming Parser for Large Inputs
// Iterator-based: one record at a time, zero excess allocation

use std::io::{BufRead, Cursor};

// ── Record type ────────────────────────────────────────────────────────────────

#[derive(Debug, PartialEq)]
pub struct Record {
    pub id: u64,
    pub name: String,
    pub value: f64,
}

// ── Parse error ────────────────────────────────────────────────────────────────

#[derive(Debug)]
pub struct ParseError {
    pub line: usize,
    pub message: String,
}

// ── Streaming iterator ─────────────────────────────────────────────────────────

pub struct RecordStream<R: BufRead> {
    reader: R,
    line_buf: String,
    line_num: usize,
    skip_errors: bool,
}

impl<R: BufRead> RecordStream<R> {
    pub fn new(reader: R) -> Self {
        Self { reader, line_buf: String::new(), line_num: 0, skip_errors: false }
    }

    pub fn skip_errors(mut self) -> Self {
        self.skip_errors = true;
        self
    }

    fn parse_line(s: &str) -> Option<Record> {
        let parts: Vec<&str> = s.splitn(3, ',').collect();
        if parts.len() < 3 { return None; }
        let id   = parts[0].trim().parse().ok()?;
        let name = parts[1].trim().to_string();
        let value = parts[2].trim().parse().ok()?;
        Some(Record { id, name, value })
    }
}

impl<R: BufRead> Iterator for RecordStream<R> {
    type Item = Result<Record, ParseError>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            self.line_buf.clear();
            match self.reader.read_line(&mut self.line_buf) {
                Ok(0) => return None,   // EOF
                Ok(_) => {}
                Err(e) => return Some(Err(ParseError {
                    line: self.line_num,
                    message: e.to_string(),
                })),
            }
            self.line_num += 1;
            let trimmed = self.line_buf.trim();
            if trimmed.is_empty() { continue; }     // skip blank lines

            match Self::parse_line(trimmed) {
                Some(rec) => return Some(Ok(rec)),
                None if self.skip_errors => continue,  // bad line, skip
                None => return Some(Err(ParseError {
                    line: self.line_num,
                    message: format!("could not parse: '{trimmed}'"),
                })),
            }
        }
    }
}

// ── Convenience constructor from &str ──────────────────────────────────────────

pub fn stream_str(s: &str) -> RecordStream<Cursor<&[u8]>> {
    RecordStream::new(Cursor::new(s.as_bytes()))
}

// ── Aggregation: process without collecting ────────────────────────────────────

pub fn sum_values(stream: impl Iterator<Item = Result<Record, ParseError>>) -> f64 {
    stream.filter_map(|r| r.ok()).map(|r| r.value).sum()
}

fn main() {
    let data = "1, Alice, 95.5\n2, Bob, 87.0\n3, Carol, 100.0\nbad line\n4, Dave, 72.3\n";

    println!("=== With error reporting ===");
    for item in stream_str(data) {
        match item {
            Ok(r)  => println!("  OK  : {r:?}"),
            Err(e) => println!("  ERR line {}: {}", e.line, e.message),
        }
    }

    println!("\n=== Skip errors ===");
    let valid: Vec<Record> = stream_str(data)
        .skip_errors()
        .filter_map(|r| r.ok())
        .collect();
    println!("Valid records: {}", valid.len());

    println!("\n=== Sum without collecting ===");
    let total = sum_values(stream_str(data).skip_errors());
    println!("Sum of values: {total:.1}");

    println!("\n=== Chained iterator ops ===");
    let top = stream_str(data)
        .skip_errors()
        .filter_map(|r| r.ok())
        .filter(|r| r.value >= 90.0)
        .count();
    println!("Records with value >= 90: {top}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = "1,Alice,95.5\n2,Bob,87.0\n3,Carol,100.0\n";

    #[test]
    fn parses_all_valid() {
        let records: Vec<_> = stream_str(DATA).filter_map(|r| r.ok()).collect();
        assert_eq!(records.len(), 3);
        assert_eq!(records[0].name, "Alice");
    }

    #[test]
    fn reports_bad_line() {
        let data = "1,A,1.0\nbad\n2,B,2.0\n";
        let results: Vec<_> = stream_str(data).collect();
        assert!(results[1].is_err());
    }

    #[test]
    fn skip_errors_skips_bad() {
        let data = "1,A,1.0\nbad\n2,B,2.0\n";
        let records: Vec<_> = stream_str(data).skip_errors().filter_map(|r| r.ok()).collect();
        assert_eq!(records.len(), 2);
    }

    #[test]
    fn sum_works() {
        let total = sum_values(stream_str(DATA).skip_errors());
        assert!((total - 282.5).abs() < 0.001);
    }
}
