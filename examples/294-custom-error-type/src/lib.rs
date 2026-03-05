//! # Custom Error Types
//!
//! Custom error enums document failure modes in the type system.

use std::fmt;

/// All the ways parsing a bounded integer can fail
#[derive(Debug, PartialEq)]
pub enum ParseError {
    InvalidNumber(String),
    OutOfRange { value: i64, min: i64, max: i64 },
    EmptyInput,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::InvalidNumber(s) => write!(f, "invalid number: '{}'", s),
            ParseError::OutOfRange { value, min, max } => {
                write!(f, "value {} out of range [{}, {}]", value, min, max)
            }
            ParseError::EmptyInput => write!(f, "empty input"),
        }
    }
}

/// Parse a string into a bounded integer
pub fn parse_bounded(s: &str, min: i64, max: i64) -> Result<i64, ParseError> {
    if s.is_empty() {
        return Err(ParseError::EmptyInput);
    }
    let n: i64 = s
        .parse()
        .map_err(|_| ParseError::InvalidNumber(s.to_string()))?;
    if n < min || n > max {
        return Err(ParseError::OutOfRange { value: n, min, max });
    }
    Ok(n)
}

/// Parse a percentage (0-100)
pub fn parse_percentage(s: &str) -> Result<i64, ParseError> {
    parse_bounded(s, 0, 100)
}

/// Parse a port number (1-65535)
pub fn parse_port(s: &str) -> Result<u16, ParseError> {
    parse_bounded(s, 1, 65535).map(|n| n as u16)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid() {
        assert_eq!(parse_bounded("42", 0, 100), Ok(42));
    }

    #[test]
    fn test_invalid_number() {
        assert!(matches!(
            parse_bounded("abc", 0, 100),
            Err(ParseError::InvalidNumber(_))
        ));
    }

    #[test]
    fn test_out_of_range_high() {
        assert!(matches!(
            parse_bounded("200", 0, 100),
            Err(ParseError::OutOfRange { value: 200, .. })
        ));
    }

    #[test]
    fn test_out_of_range_low() {
        assert!(matches!(
            parse_bounded("-5", 0, 100),
            Err(ParseError::OutOfRange { value: -5, .. })
        ));
    }

    #[test]
    fn test_empty() {
        assert_eq!(parse_bounded("", 0, 100), Err(ParseError::EmptyInput));
    }

    #[test]
    fn test_percentage_valid() {
        assert_eq!(parse_percentage("50"), Ok(50));
    }

    #[test]
    fn test_percentage_invalid() {
        assert!(matches!(parse_percentage("150"), Err(ParseError::OutOfRange { .. })));
    }

    #[test]
    fn test_port_valid() {
        assert_eq!(parse_port("8080"), Ok(8080));
    }

    #[test]
    fn test_error_display() {
        let err = ParseError::OutOfRange {
            value: 200,
            min: 0,
            max: 100,
        };
        assert_eq!(format!("{}", err), "value 200 out of range [0, 100]");
    }
}
