//! # Parse Error Handling
//!
//! Custom FromStr implementation with detailed error types.

use std::fmt;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum ParsePositiveError {
    Empty,
    InvalidNumber(String),
    NotPositive(i64),
}

impl fmt::Display for ParsePositiveError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => write!(f, "empty input"),
            Self::InvalidNumber(s) => write!(f, "not a number: {s}"),
            Self::NotPositive(n) => write!(f, "{n} is not positive"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct PositiveInt(pub u64);

impl FromStr for PositiveInt {
    type Err = ParsePositiveError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(ParsePositiveError::Empty);
        }
        let n: i64 = s
            .parse()
            .map_err(|_| ParsePositiveError::InvalidNumber(s.to_string()))?;
        if n <= 0 {
            return Err(ParsePositiveError::NotPositive(n));
        }
        Ok(PositiveInt(n as u64))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid() {
        assert_eq!("42".parse::<PositiveInt>().unwrap().0, 42);
    }

    #[test]
    fn test_rejects_zero() {
        assert_eq!(
            "0".parse::<PositiveInt>().unwrap_err(),
            ParsePositiveError::NotPositive(0)
        );
    }

    #[test]
    fn test_rejects_empty() {
        assert_eq!(
            "".parse::<PositiveInt>().unwrap_err(),
            ParsePositiveError::Empty
        );
    }

    #[test]
    fn test_rejects_text() {
        assert!(matches!(
            "abc".parse::<PositiveInt>().unwrap_err(),
            ParsePositiveError::InvalidNumber(_)
        ));
    }

    #[test]
    fn test_negative() {
        assert_eq!(
            "-5".parse::<PositiveInt>().unwrap_err(),
            ParsePositiveError::NotPositive(-5)
        );
    }
}
