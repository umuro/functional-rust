#![allow(clippy::all)]
// From<SourceError> for AppError enables automatic conversion at the ? call site.
#[derive(Debug, PartialEq)]
pub enum AppError {
    Parse(String),
    DivisionByZero,
}

impl From<std::num::ParseIntError> for AppError {
    fn from(e: std::num::ParseIntError) -> Self {
        AppError::Parse(e.to_string())
    }
}

fn safe_div(a: i32, b: i32) -> Result<i32, AppError> {
    if b == 0 {
        Err(AppError::DivisionByZero)
    } else {
        Ok(a / b)
    }
}

pub fn compute(s: &str, divisor: i32) -> Result<i32, AppError> {
    let n: i32 = s.parse()?; // ParseIntError auto-converted to AppError via From
    safe_div(n, divisor)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_ok() {
        assert_eq!(compute("10", 2), Ok(5));
    }

    #[test]
    fn test_parse_error_converted_via_from() {
        match compute("abc", 2) {
            Err(AppError::Parse(_)) => {}
            other => panic!("expected AppError::Parse, got {:?}", other),
        }
    }

    #[test]
    fn test_division_error_passes_through_unconverted() {
        assert_eq!(compute("10", 0), Err(AppError::DivisionByZero));
    }
}
