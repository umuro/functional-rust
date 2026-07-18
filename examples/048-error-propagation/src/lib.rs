#![allow(clippy::all)]
// The ? operator propagates errors from a Result-returning function without explicit match.
#[derive(Debug, PartialEq)]
pub enum AppError {
    ParseError(String),
    DivisionByZero,
}

fn parse_int(s: &str) -> Result<i32, AppError> {
    s.parse::<i32>().map_err(|_| AppError::ParseError(s.to_string()))
}

fn safe_div(a: i32, b: i32) -> Result<i32, AppError> {
    if b == 0 {
        Err(AppError::DivisionByZero)
    } else {
        Ok(a / b)
    }
}

pub fn compute(s: &str, divisor: i32) -> Result<i32, AppError> {
    let n = parse_int(s)?;
    let d = safe_div(n, divisor)?;
    Ok(d * 2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_ok() {
        assert_eq!(compute("10", 2), Ok(10));
    }

    #[test]
    fn test_compute_parse_error_propagates() {
        assert_eq!(compute("abc", 2), Err(AppError::ParseError("abc".to_string())));
    }

    #[test]
    fn test_compute_division_error_propagates() {
        assert_eq!(compute("10", 0), Err(AppError::DivisionByZero));
    }
}
