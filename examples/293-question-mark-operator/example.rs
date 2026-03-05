//! 293. ? operator and early returns
//!
//! `?` desugars to match + return Err(e.into()), enabling clean error propagation.

use std::num::ParseIntError;
use std::fmt;

#[derive(Debug)]
enum AppError {
    Parse(ParseIntError),
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
        AppError::Parse(e)
    }
}

fn parse_positive(s: &str) -> Result<u32, AppError> {
    let n: i32 = s.parse()?; // ? auto-converts ParseIntError via From
    if n < 0 {
        Err(AppError::NegativeInput)
    } else {
        Ok(n as u32)
    }
}

fn safe_div(a: u32, b: u32) -> Result<u32, AppError> {
    if b == 0 { Err(AppError::DivByZero) } else { Ok(a / b) }
}

// Chain of ? operators
fn compute(a_str: &str, b_str: &str) -> Result<u32, AppError> {
    let a = parse_positive(a_str)?;
    let b = parse_positive(b_str)?;
    let result = safe_div(a, b)?;
    Ok(result * 2)
}

// ? on Option
fn find_double(v: &[i32], target: i32) -> Option<i32> {
    let idx = v.iter().position(|&x| x == target)?;
    let val = v.get(idx)?;
    Some(val * 2)
}

fn main() {
    println!("{:?}", compute("20", "4"));   // Ok(10)
    println!("{:?}", compute("abc", "4"));  // Err(Parse(...))
    println!("{:?}", compute("20", "0"));   // Err(DivByZero)
    println!("{:?}", compute("-5", "2"));   // Err(NegativeInput)

    let v = [1, 2, 3, 4, 5];
    println!("{:?}", find_double(&v, 3));   // Some(6)
    println!("{:?}", find_double(&v, 99));  // None
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn test_question_mark_option() {
        let v = [1i32, 2, 3];
        assert_eq!(find_double(&v, 2), Some(4));
        assert_eq!(find_double(&v, 9), None);
    }
}
