// 1023: Safe Integer Parsing
// str::parse::<i64>() and handling ParseIntError

use std::num::ParseIntError;

// Approach 1: Basic parse with Result
fn parse_int(s: &str) -> Result<i64, ParseIntError> {
    s.parse::<i64>()
}

// Approach 2: Parse with custom error message
fn parse_int_msg(s: &str) -> Result<i64, String> {
    s.parse::<i64>()
        .map_err(|e| format!("cannot parse '{}' as integer: {}", s, e))
}

// Approach 3: Parse with validation
fn parse_positive(s: &str) -> Result<i64, String> {
    let n: i64 = s.parse().map_err(|_| format!("not a number: {}", s))?;
    if n < 0 {
        Err(format!("negative: {}", n))
    } else {
        Ok(n)
    }
}

fn parse_in_range(s: &str, min: i64, max: i64) -> Result<i64, String> {
    let n: i64 = s.parse().map_err(|_| format!("not a number: {}", s))?;
    if n < min {
        Err(format!("{} < min({})", n, min))
    } else if n > max {
        Err(format!("{} > max({})", n, max))
    } else {
        Ok(n)
    }
}

// Parse with default (Option-based)
fn parse_or_default(s: &str, default: i64) -> i64 {
    s.parse::<i64>().unwrap_or(default)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_parse() {
        assert_eq!(parse_int("42"), Ok(42));
        assert_eq!(parse_int("-17"), Ok(-17));
        assert_eq!(parse_int("0"), Ok(0));
    }

    #[test]
    fn test_parse_errors() {
        assert!(parse_int("abc").is_err());
        assert!(parse_int("").is_err());
        assert!(parse_int("12.5").is_err()); // no floats
        assert!(parse_int("99999999999999999999").is_err()); // overflow
    }

    #[test]
    fn test_parse_with_message() {
        let err = parse_int_msg("abc").unwrap_err();
        assert!(err.contains("cannot parse"));
        assert!(err.contains("abc"));
    }

    #[test]
    fn test_parse_positive() {
        assert_eq!(parse_positive("42"), Ok(42));
        assert_eq!(parse_positive("0"), Ok(0));
        assert!(parse_positive("-5").unwrap_err().contains("negative"));
        assert!(parse_positive("xyz").unwrap_err().contains("not a number"));
    }

    #[test]
    fn test_parse_in_range() {
        assert_eq!(parse_in_range("50", 1, 100), Ok(50));
        assert_eq!(parse_in_range("1", 1, 100), Ok(1));
        assert_eq!(parse_in_range("100", 1, 100), Ok(100));
        assert!(parse_in_range("0", 1, 100).is_err());
        assert!(parse_in_range("101", 1, 100).is_err());
        assert!(parse_in_range("abc", 1, 100).is_err());
    }

    #[test]
    fn test_parse_or_default() {
        assert_eq!(parse_or_default("42", 0), 42);
        assert_eq!(parse_or_default("abc", 0), 0);
        assert_eq!(parse_or_default("", -1), -1);
    }

    #[test]
    fn test_parse_int_error_kind() {
        // ParseIntError has useful information
        let err = "abc".parse::<i64>().unwrap_err();
        assert_eq!(err.to_string(), "invalid digit found in string");

        let err = "".parse::<i64>().unwrap_err();
        assert_eq!(err.to_string(), "cannot parse integer from empty string");
    }

    #[test]
    fn test_whitespace_handling() {
        // Rust's parse does NOT trim whitespace
        assert!(parse_int(" 42").is_err());
        assert!(parse_int("42 ").is_err());
        // Trim first if needed
        assert_eq!(" 42 ".trim().parse::<i64>(), Ok(42));
    }
}
