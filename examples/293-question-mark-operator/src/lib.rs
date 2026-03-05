//! # The ? Operator
//!
//! `?` desugars to match + return Err(e.into()), enabling clean error propagation.

use std::fmt;
use std::num::ParseIntError;

#[derive(Debug, PartialEq)]
pub enum AppError {
    Parse(String),
    DivByZero,
    NegativeInput,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Parse(e) => write!(f, "parse error: {}", e),
            AppError::DivByZero => write!(f, "division by zero"),
            AppError::NegativeInput => write!(f, "negative input"),
        }
    }
}

impl From<ParseIntError> for AppError {
    fn from(e: ParseIntError) -> Self {
        AppError::Parse(e.to_string())
    }
}

/// Parse a string as a positive integer
pub fn parse_positive(s: &str) -> Result<u32, AppError> {
    let n: i32 = s.parse()?; // ? auto-converts ParseIntError via From
    if n < 0 {
        Err(AppError::NegativeInput)
    } else {
        Ok(n as u32)
    }
}

/// Safe division returning error on division by zero
pub fn safe_div(a: u32, b: u32) -> Result<u32, AppError> {
    if b == 0 {
        Err(AppError::DivByZero)
    } else {
        Ok(a / b)
    }
}

/// Compute using chain of ? operators
pub fn compute(a_str: &str, b_str: &str) -> Result<u32, AppError> {
    let a = parse_positive(a_str)?;
    let b = parse_positive(b_str)?;
    let result = safe_div(a, b)?;
    Ok(result * 2)
}

/// ? on Option - returns None early
pub fn find_double(v: &[i32], target: i32) -> Option<i32> {
    let idx = v.iter().position(|&x| x == target)?;
    let val = v.get(idx)?;
    Some(val * 2)
}

/// Chain multiple optional operations
pub fn parse_and_lookup(s: &str, map: &std::collections::HashMap<i32, &str>) -> Option<String> {
    let n = s.parse::<i32>().ok()?;
    let value = map.get(&n)?;
    Some(value.to_uppercase())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_compute_success() {
        assert_eq!(compute("20", "4"), Ok(10));
    }

    #[test]
    fn test_compute_parse_error() {
        assert!(matches!(compute("abc", "4"), Err(AppError::Parse(_))));
    }

    #[test]
    fn test_compute_div_zero() {
        assert!(matches!(compute("20", "0"), Err(AppError::DivByZero)));
    }

    #[test]
    fn test_compute_negative() {
        assert!(matches!(compute("-5", "2"), Err(AppError::NegativeInput)));
    }

    #[test]
    fn test_question_mark_option_found() {
        let v = [1i32, 2, 3];
        assert_eq!(find_double(&v, 2), Some(4));
    }

    #[test]
    fn test_question_mark_option_not_found() {
        let v = [1i32, 2, 3];
        assert_eq!(find_double(&v, 9), None);
    }

    #[test]
    fn test_parse_and_lookup() {
        let mut map = HashMap::new();
        map.insert(1, "hello");
        map.insert(2, "world");
        assert_eq!(parse_and_lookup("1", &map), Some("HELLO".to_string()));
        assert_eq!(parse_and_lookup("99", &map), None);
        assert_eq!(parse_and_lookup("abc", &map), None);
    }
}
