//! # Implementing std::error::Error
//!
//! `std::error::Error` requires Display + Debug and optionally provides `source()`.

use std::error::Error;
use std::fmt;

/// Low-level parse error
#[derive(Debug)]
pub struct ParseError {
    pub input: String,
    pub reason: String,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "failed to parse '{}': {}", self.input, self.reason)
    }
}

impl Error for ParseError {} // source() defaults to None

/// Higher-level validation error that wraps a cause
#[derive(Debug)]
pub struct ValidationError {
    pub field: String,
    pub source: Box<dyn Error + Send + Sync>,
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

/// Parse a string as an age (u8)
pub fn parse_age(s: &str) -> Result<u8, ParseError> {
    s.parse::<u8>().map_err(|e| ParseError {
        input: s.to_string(),
        reason: e.to_string(),
    })
}

/// Validate user age with context
pub fn validate_user_age(s: &str) -> Result<u8, ValidationError> {
    parse_age(s).map_err(|e| ValidationError {
        field: "age".to_string(),
        source: Box::new(e),
    })
}

/// Print full error chain
pub fn print_error_chain(e: &dyn Error) -> String {
    let mut result = format!("Error: {}", e);
    let mut cause = e.source();
    while let Some(c) = cause {
        result.push_str(&format!("\n  Caused by: {}", c));
        cause = c.source();
    }
    result
}

/// Collect heterogeneous errors
pub fn collect_errors() -> Vec<Box<dyn Error>> {
    vec![Box::new(ParseError {
        input: "x".to_string(),
        reason: "not a number".to_string(),
    })]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_error_display() {
        let e = ParseError {
            input: "abc".to_string(),
            reason: "invalid".to_string(),
        };
        let msg = format!("{}", e);
        assert!(msg.contains("abc"));
    }

    #[test]
    fn test_parse_error_is_error_trait() {
        let e: Box<dyn Error> = Box::new(ParseError {
            input: "x".to_string(),
            reason: "bad".to_string(),
        });
        assert!(e.source().is_none());
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

    #[test]
    fn test_error_chain_string() {
        let result = validate_user_age("bad");
        if let Err(e) = result {
            let chain = print_error_chain(&e);
            assert!(chain.contains("validation failed"));
            assert!(chain.contains("Caused by"));
        }
    }

    #[test]
    fn test_collect_heterogeneous() {
        let errors = collect_errors();
        assert_eq!(errors.len(), 1);
    }
}
