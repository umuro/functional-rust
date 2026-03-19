#![allow(clippy::all)]
// 056: Result as Monad
// Chain fallible operations with and_then and ?

use std::num::ParseIntError;

#[derive(Debug, PartialEq)]
enum CalcError {
    Parse(String),
    DivByZero,
}

fn parse_int(s: &str) -> Result<i32, CalcError> {
    s.parse::<i32>()
        .map_err(|e| CalcError::Parse(e.to_string()))
}

fn safe_div(a: i32, b: i32) -> Result<i32, CalcError> {
    if b == 0 {
        Err(CalcError::DivByZero)
    } else {
        Ok(a / b)
    }
}

// Approach 1: Using and_then (monadic bind)
fn compute_bind(s1: &str, s2: &str) -> Result<i32, CalcError> {
    parse_int(s1).and_then(|a| parse_int(s2).and_then(|b| safe_div(a, b)))
}

// Approach 2: Using ? operator (syntactic sugar for bind)
fn compute_question(s1: &str, s2: &str) -> Result<i32, CalcError> {
    let a = parse_int(s1)?;
    let b = parse_int(s2)?;
    safe_div(a, b)
}

// Approach 3: Chained pipeline
fn pipeline(s: &str) -> Result<i32, CalcError> {
    parse_int(s)
        .and_then(|n| safe_div(n, 2))
        .map(|n| n + 1)
        .map(|n| n * 2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_int() {
        assert_eq!(parse_int("42"), Ok(42));
        assert!(parse_int("abc").is_err());
    }

    #[test]
    fn test_compute_bind() {
        assert_eq!(compute_bind("10", "3"), Ok(3));
        assert_eq!(compute_bind("10", "0"), Err(CalcError::DivByZero));
        assert!(compute_bind("abc", "3").is_err());
    }

    #[test]
    fn test_compute_question() {
        assert_eq!(compute_question("10", "3"), Ok(3));
        assert_eq!(compute_question("10", "0"), Err(CalcError::DivByZero));
    }

    #[test]
    fn test_pipeline() {
        assert_eq!(pipeline("10"), Ok(12));
        assert!(pipeline("abc").is_err());
    }
}
