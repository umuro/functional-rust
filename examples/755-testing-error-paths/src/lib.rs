//! # Testing Error Paths
//!
//! Testing error cases and unwrap discipline.

/// Parse errors with detailed information
#[derive(Debug, PartialEq, Clone)]
pub enum ParseError {
    Empty,
    TooLong { len: usize, max: usize },
    InvalidChar { ch: char, pos: usize },
    OutOfRange { value: i64, min: i64, max: i64 },
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::Empty => write!(f, "input is empty"),
            ParseError::TooLong { len, max } => write!(f, "too long: {} > {}", len, max),
            ParseError::InvalidChar { ch, pos } => write!(f, "invalid char {:?} at {}", ch, pos),
            ParseError::OutOfRange { value, min, max } => {
                write!(f, "{} out of range [{}, {}]", value, min, max)
            }
        }
    }
}

/// Parse a string to a positive u32
pub fn parse_positive(s: &str) -> Result<u32, ParseError> {
    if s.is_empty() {
        return Err(ParseError::Empty);
    }
    if s.len() > 10 {
        return Err(ParseError::TooLong {
            len: s.len(),
            max: 10,
        });
    }
    for (pos, ch) in s.char_indices() {
        if !ch.is_ascii_digit() {
            return Err(ParseError::InvalidChar { ch, pos });
        }
    }
    let n: u64 = s.parse().unwrap();
    if n == 0 || n > u32::MAX as u64 {
        return Err(ParseError::OutOfRange {
            value: n as i64,
            min: 1,
            max: u32::MAX as i64,
        });
    }
    Ok(n as u32)
}

/// Safe division
pub fn divide(a: i64, b: i64) -> Result<i64, &'static str> {
    if b == 0 {
        Err("cannot divide by zero")
    } else {
        Ok(a / b)
    }
}

/// Get the first element of a slice
pub fn head<T: Clone>(v: &[T]) -> Result<T, &'static str> {
    v.first().cloned().ok_or("empty slice")
}

/// Get the last element of a slice
pub fn tail<T: Clone>(v: &[T]) -> Result<T, &'static str> {
    v.last().cloned().ok_or("empty slice")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_positive_valid() {
        assert_eq!(parse_positive("123"), Ok(123));
        assert_eq!(parse_positive("1"), Ok(1));
        assert_eq!(parse_positive("4294967295"), Ok(u32::MAX));
    }

    #[test]
    fn test_parse_positive_empty() {
        assert_eq!(parse_positive(""), Err(ParseError::Empty));
    }

    #[test]
    fn test_parse_positive_too_long() {
        let result = parse_positive("12345678901");
        assert_eq!(result, Err(ParseError::TooLong { len: 11, max: 10 }));
    }

    #[test]
    fn test_parse_positive_invalid_char() {
        assert_eq!(
            parse_positive("12x4"),
            Err(ParseError::InvalidChar { ch: 'x', pos: 2 })
        );
    }

    #[test]
    fn test_parse_positive_zero() {
        let result = parse_positive("0");
        assert!(matches!(result, Err(ParseError::OutOfRange { .. })));
    }

    #[test]
    fn test_divide_success() {
        assert_eq!(divide(10, 2), Ok(5));
        assert_eq!(divide(-10, 2), Ok(-5));
    }

    #[test]
    fn test_divide_by_zero() {
        assert_eq!(divide(10, 0), Err("cannot divide by zero"));
    }

    #[test]
    fn test_head_success() {
        assert_eq!(head(&[1, 2, 3]), Ok(1));
    }

    #[test]
    fn test_head_empty() {
        assert_eq!(head::<i32>(&[]), Err("empty slice"));
    }

    #[test]
    fn test_tail_success() {
        assert_eq!(tail(&[1, 2, 3]), Ok(3));
    }

    #[test]
    fn test_error_display() {
        let err = ParseError::InvalidChar { ch: 'x', pos: 5 };
        assert_eq!(format!("{}", err), "invalid char 'x' at 5");
    }
}
