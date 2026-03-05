//! 295. Implementing std::error::Error
//!
//! `std::error::Error` requires Display + Debug and optionally provides `source()`.

use std::error::Error;
use std::fmt;

/// Low-level parse error
#[derive(Debug)]
struct ParseError {
    input: String,
    reason: String,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "failed to parse '{}': {}", self.input, self.reason)
    }
}

impl Error for ParseError {} // source() defaults to None

/// Higher-level validation error that wraps a cause
#[derive(Debug)]
struct ValidationError {
    field: String,
    source: Box<dyn Error + Send + Sync>,
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "validation failed for field '{}'", self.field)
    }
}

impl Error for ValidationError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(self.source.as_ref())
    }
}

fn parse_age(s: &str) -> Result<u8, ParseError> {
    s.parse::<u8>().map_err(|e| ParseError {
        input: s.to_string(),
        reason: e.to_string(),
    })
}

fn validate_user_age(s: &str) -> Result<u8, ValidationError> {
    parse_age(s).map_err(|e| ValidationError {
        field: "age".to_string(),
        source: Box::new(e),
    })
}

fn print_error_chain(e: &dyn Error) {
    println!("Error: {}", e);
    let mut cause = e.source();
    while let Some(c) = cause {
        println!("  Caused by: {}", c);
        cause = c.source();
    }
}

fn main() {
    match validate_user_age("abc") {
        Ok(age) => println!("Age: {}", age),
        Err(ref e) => print_error_chain(e),
    }

    match validate_user_age("25") {
        Ok(age) => println!("Valid age: {}", age),
        Err(ref e) => print_error_chain(e),
    }

    // Box<dyn Error> for dynamic dispatch
    let errs: Vec<Box<dyn Error>> = vec![
        Box::new(ParseError { input: "x".to_string(), reason: "not a number".to_string() }),
    ];
    for e in &errs {
        println!("Dynamic: {}", e);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_error_display() {
        let e = ParseError { input: "abc".to_string(), reason: "invalid".to_string() };
        let msg = format!("{}", e);
        assert!(msg.contains("abc"));
    }

    #[test]
    fn test_validation_error_source() {
        let result = validate_user_age("abc");
        assert!(result.is_err());
        let e = result.unwrap_err();
        assert!(e.source().is_some());
    }

    #[test]
    fn test_valid_age() {
        assert_eq!(validate_user_age("25"), Ok(25));
    }
}
