#![allow(clippy::all)]
//! format_args! and Formatting
//!
//! Efficient string formatting.

use std::fmt::Write;

/// Write formatted data to a buffer.
pub fn write_to_buffer(buf: &mut String, name: &str, value: i32) {
    write!(buf, "{}: {}", name, value).unwrap();
}

/// Format with padding.
pub fn format_padded(s: &str, width: usize) -> String {
    format!("{:>width$}", s, width = width)
}

/// Format number with thousands separator.
pub fn format_number(n: u64) -> String {
    let s = n.to_string();
    let mut result = String::new();
    for (i, c) in s.chars().rev().enumerate() {
        if i > 0 && i % 3 == 0 {
            result.push(',');
        }
        result.push(c);
    }
    result.chars().rev().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_write_to_buffer() {
        let mut buf = String::new();
        write_to_buffer(&mut buf, "count", 42);
        assert_eq!(buf, "count: 42");
    }

    #[test]
    fn test_format_padded() {
        assert_eq!(format_padded("hi", 5), "   hi");
    }

    #[test]
    fn test_format_number() {
        assert_eq!(format_number(1000), "1,000");
        assert_eq!(format_number(1000000), "1,000,000");
    }

    #[test]
    fn test_format_small_number() {
        assert_eq!(format_number(42), "42");
    }

    #[test]
    fn test_multiple_writes() {
        let mut buf = String::new();
        write_to_buffer(&mut buf, "a", 1);
        buf.push_str(", ");
        write_to_buffer(&mut buf, "b", 2);
        assert!(buf.contains("a: 1"));
        assert!(buf.contains("b: 2"));
    }
}
