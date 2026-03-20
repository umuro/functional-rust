#![allow(clippy::all)]
//! # Error Handling in Tests
//!
//! Tests can return Result, use ? operator, and #[should_panic].

#[derive(Debug, PartialEq)]
pub enum MathError {
    DivisionByZero,
    NegativeInput(i64),
}

impl std::fmt::Display for MathError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DivisionByZero => write!(f, "division by zero"),
            Self::NegativeInput(n) => write!(f, "negative input: {n}"),
        }
    }
}

pub fn safe_div(a: i64, b: i64) -> Result<i64, MathError> {
    if b == 0 {
        Err(MathError::DivisionByZero)
    } else {
        Ok(a / b)
    }
}

pub fn safe_sqrt(x: i64) -> Result<u64, MathError> {
    if x < 0 {
        Err(MathError::NegativeInput(x))
    } else {
        Ok((x as f64).sqrt() as u64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test returning Result with ?
    #[test]
    fn test_div_ok() -> Result<(), MathError> {
        assert_eq!(safe_div(10, 2)?, 5);
        Ok(())
    }

    // Traditional assertion
    #[test]
    fn test_div_zero() {
        assert_eq!(safe_div(5, 0), Err(MathError::DivisionByZero));
    }

    // Test returning Result
    #[test]
    fn test_sqrt_ok() -> Result<(), MathError> {
        assert_eq!(safe_sqrt(16)?, 4);
        Ok(())
    }

    // Match on error variant
    #[test]
    fn test_sqrt_neg() {
        assert_eq!(safe_sqrt(-9).unwrap_err(), MathError::NegativeInput(-9));
    }

    // Test that something panics
    #[test]
    #[should_panic]
    fn test_panics_on_unwrap() {
        safe_div(1, 0).unwrap();
    }

    // Test panic message
    #[test]
    #[should_panic(expected = "division by zero")]
    fn test_panic_message() {
        safe_div(1, 0).expect("division by zero");
    }
}
