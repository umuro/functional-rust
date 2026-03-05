//! # From Trait for Error Conversion
//!
//! `impl From<E> for MyErr` enables automatic error conversion via `?`.

use std::fmt;
use std::num::ParseIntError;

#[derive(Debug)]
pub enum AppError {
    Parse(ParseIntError),
    Logic(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Parse(e) => write!(f, "parse error: {}", e),
            AppError::Logic(s) => write!(f, "logic error: {}", s),
        }
    }
}

// These From impls make `?` work seamlessly
impl From<ParseIntError> for AppError {
    fn from(e: ParseIntError) -> Self {
        AppError::Parse(e)
    }
}

/// Parse a number - returns ParseIntError
pub fn parse_number(s: &str) -> Result<i32, ParseIntError> {
    s.trim().parse()
}

/// Validate that a number is positive
pub fn validate_positive(n: i32) -> Result<i32, AppError> {
    if n > 0 {
        Ok(n)
    } else {
        Err(AppError::Logic(format!("{} is not positive", n)))
    }
}

/// Process input - ? converts ParseIntError -> AppError automatically via From
pub fn process(input: &str) -> Result<i32, AppError> {
    let n = parse_number(input)?; // From<ParseIntError> called
    let validated = validate_positive(n)?;
    Ok(validated * 2)
}

/// Alternative: explicit conversion with .into()
pub fn process_explicit(input: &str) -> Result<i32, AppError> {
    let n = parse_number(input).map_err(|e| e.into())?;
    validate_positive(n).map(|v| v * 2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_ok() {
        assert_eq!(process("21"), Ok(42));
    }

    #[test]
    fn test_process_parse_err() {
        assert!(matches!(process("abc"), Err(AppError::Parse(_))));
    }

    #[test]
    fn test_process_logic_err() {
        assert!(matches!(process("-1"), Err(AppError::Logic(_))));
    }

    #[test]
    fn test_from_conversion() {
        let e: ParseIntError = "x".parse::<i32>().unwrap_err();
        let app: AppError = e.into(); // via From
        assert!(matches!(app, AppError::Parse(_)));
    }

    #[test]
    fn test_process_whitespace() {
        assert_eq!(process("  42  "), Ok(84));
    }

    #[test]
    fn test_process_explicit_same() {
        assert_eq!(process_explicit("21"), Ok(42));
    }
}
