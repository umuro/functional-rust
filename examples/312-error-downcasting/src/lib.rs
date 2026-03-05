//! # Downcasting Boxed Errors
//!
//! `downcast_ref::<T>()` recovers the concrete type from `Box<dyn Error>`.

use std::error::Error;
use std::fmt;

#[derive(Debug, PartialEq)]
pub struct ParseError { pub input: String }

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "parse error: '{}'", self.input)
    }
}
impl Error for ParseError {}

#[derive(Debug)]
pub struct NetworkError { pub code: u32, pub message: String }

impl fmt::Display for NetworkError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "network error {}: {}", self.code, self.message)
    }
}
impl Error for NetworkError {}

/// Handle error by downcasting to specific types
pub fn handle_error(e: &(dyn Error + 'static)) -> String {
    if let Some(pe) = e.downcast_ref::<ParseError>() {
        return format!("Parse error for: {}", pe.input);
    }
    if let Some(ne) = e.downcast_ref::<NetworkError>() {
        return format!("Network {}: {}", ne.code, ne.message);
    }
    format!("Unknown: {}", e)
}

/// Create heterogeneous error collection
pub fn make_errors() -> Vec<Box<dyn Error>> {
    vec![
        Box::new(ParseError { input: "abc".to_string() }),
        Box::new(NetworkError { code: 404, message: "not found".to_string() }),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_downcast_ref_success() {
        let e: Box<dyn Error> = Box::new(ParseError { input: "x".to_string() });
        assert!(e.downcast_ref::<ParseError>().is_some());
        assert!(e.downcast_ref::<NetworkError>().is_none());
    }

    #[test]
    fn test_handle_parse_error() {
        let e: Box<dyn Error> = Box::new(ParseError { input: "test".to_string() });
        let result = handle_error(e.as_ref());
        assert!(result.contains("Parse error"));
    }

    #[test]
    fn test_handle_network_error() {
        let e: Box<dyn Error> = Box::new(NetworkError { code: 500, message: "fail".to_string() });
        let result = handle_error(e.as_ref());
        assert!(result.contains("Network 500"));
    }

    #[test]
    fn test_downcast_box() {
        let e: Box<dyn Error> = Box::new(ParseError { input: "abc".to_string() });
        let result = e.downcast::<ParseError>();
        assert!(result.is_ok());
    }
}
