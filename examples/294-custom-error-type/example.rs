//! 294. Defining custom Error types
//!
//! Custom error enums document failure modes in the type system.

use std::fmt;

/// All the ways parsing a bounded integer can fail
#[derive(Debug, PartialEq)]
enum ParseError {
    InvalidNumber(String),
    OutOfRange { value: i64, min: i64, max: i64 },
    EmptyInput,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::InvalidNumber(s) =>
                write!(f, "invalid number: '{}'", s),
            ParseError::OutOfRange { value, min, max } =>
                write!(f, "value {} out of range [{}, {}]", value, min, max),
            ParseError::EmptyInput =>
                write!(f, "empty input"),
        }
    }
}

fn parse_bounded(s: &str, min: i64, max: i64) -> Result<i64, ParseError> {
    if s.is_empty() {
        return Err(ParseError::EmptyInput);
    }
    let n: i64 = s.parse()
        .map_err(|_| ParseError::InvalidNumber(s.to_string()))?;
    if n < min || n > max {
        return Err(ParseError::OutOfRange { value: n, min, max });
    }
    Ok(n)
}

fn main() {
    let test_cases = [
        ("42", 0i64, 100i64),
        ("abc", 0, 100),
        ("", 0, 100),
        ("200", 0, 100),
        ("-5", -10, 10),
    ];

    for (s, min, max) in &test_cases {
        match parse_bounded(s, *min, *max) {
            Ok(n) => println!("Ok({})", n),
            Err(e) => println!("Error: {}", e),
        }
    }

    // Pattern matching on error variants
    match parse_bounded("999", 0, 100) {
        Err(ParseError::OutOfRange { value, min, max }) =>
            println!("{} is not in [{}, {}]", value, min, max),
        _ => {}
    }
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
    fn test_out_of_range() {
        assert!(matches!(
            parse_bounded("200", 0, 100),
            Err(ParseError::OutOfRange { .. })
        ));
    }

    #[test]
    fn test_empty() {
        assert_eq!(parse_bounded("", 0, 100), Err(ParseError::EmptyInput));
    }
}
