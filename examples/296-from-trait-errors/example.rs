//! 296. From trait for error conversion
//!
//! `impl From<E> for MyErr` enables automatic error conversion via `?`.

use std::fmt;
use std::num::ParseIntError;

#[derive(Debug)]
enum AppError {
    Parse(ParseIntError),
    Io(std::io::Error),
    Logic(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Parse(e) => write!(f, "parse error: {}", e),
            AppError::Io(e)    => write!(f, "IO error: {}", e),
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

impl From<std::io::Error> for AppError {
    fn from(e: std::io::Error) -> Self {
        AppError::Io(e)
    }
}

// Functions that return different error types
fn parse_number(s: &str) -> Result<i32, ParseIntError> {
    s.trim().parse()
}

fn validate_positive(n: i32) -> Result<i32, AppError> {
    if n > 0 { Ok(n) }
    else { Err(AppError::Logic(format!("{} is not positive", n))) }
}

// ? converts ParseIntError -> AppError automatically via From
fn process(input: &str) -> Result<i32, AppError> {
    let n = parse_number(input)?; // From<ParseIntError> called
    let validated = validate_positive(n)?;
    Ok(validated * 2)
}

fn main() {
    println!("{:?}", process("21"));    // Ok(42)
    println!("{:?}", process("abc"));   // Err(Parse(...))
    println!("{:?}", process("-5"));    // Err(Logic(...))

    // Manual From conversion
    let parse_err: ParseIntError = "x".parse::<i32>().unwrap_err();
    let app_err: AppError = AppError::from(parse_err);
    println!("Converted: {:?}", app_err);
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
}
