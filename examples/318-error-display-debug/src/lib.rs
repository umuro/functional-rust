//! # Error Display vs Debug
//!
//! Display is for users, Debug is for developers.

use std::fmt;

#[derive(Debug)]
pub enum DbError {
    ConnectionFailed(String),
    QueryTimeout(f64),
    NotFound(String),
}

impl fmt::Display for DbError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ConnectionFailed(h) => write!(f, "Cannot connect to {h}"),
            Self::QueryTimeout(s) => write!(f, "Query timed out after {s:.1}s"),
            Self::NotFound(k) => write!(f, "Record not found: {k}"),
        }
    }
}

impl std::error::Error for DbError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_human_readable() {
        let e = DbError::ConnectionFailed("localhost".into());
        assert_eq!(e.to_string(), "Cannot connect to localhost");
    }

    #[test]
    fn test_debug_has_variant() {
        let e = DbError::NotFound("x".into());
        let debug = format!("{:?}", e);
        assert!(debug.contains("NotFound"));
    }

    #[test]
    fn test_implements_error() {
        let e: Box<dyn std::error::Error> = Box::new(DbError::QueryTimeout(5.0));
        assert!(e.to_string().contains("5.0"));
    }

    #[test]
    fn test_timeout_format() {
        let e = DbError::QueryTimeout(30.567);
        assert!(e.to_string().contains("30.6")); // formatted to 1 decimal
    }
}
