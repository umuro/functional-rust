#![allow(dead_code)]
#![allow(clippy::all)]
// 1021: Error Propagation Depth
// 5-level error propagation with ?

use std::fmt;

#[derive(Debug, PartialEq)]
enum AppError {
    ConfigMissing(String),
    ParseFailed(String),
    ValidationFailed(String),
    ServiceUnavailable(String),
    Timeout,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::ConfigMissing(s) => write!(f, "config missing: {}", s),
            AppError::ParseFailed(s) => write!(f, "parse failed: {}", s),
            AppError::ValidationFailed(s) => write!(f, "validation: {}", s),
            AppError::ServiceUnavailable(s) => write!(f, "service unavailable: {}", s),
            AppError::Timeout => write!(f, "timeout"),
        }
    }
}
impl std::error::Error for AppError {}

// Level 1: Config layer
fn read_config(key: &str) -> Result<String, AppError> {
    if key == "missing" {
        Err(AppError::ConfigMissing(key.into()))
    } else {
        Ok("8080".into())
    }
}

// Level 2: Parse layer
fn parse_port(s: &str) -> Result<u16, AppError> {
    s.parse::<u16>()
        .map_err(|_| AppError::ParseFailed(s.into()))
}

// Level 3: Validation layer
fn validate_port(port: u16) -> Result<u16, AppError> {
    if port == 0 {
        Err(AppError::ValidationFailed(format!("port {} invalid", port)))
    } else {
        Ok(port)
    }
}

// Level 4: Connection layer
fn connect(_host: &str, port: u16) -> Result<String, AppError> {
    if port == 9999 {
        Err(AppError::ServiceUnavailable("connection refused".into()))
    } else {
        Ok(format!("connected:{}", port))
    }
}

// Level 5: Application layer — chains all with ?
fn start_service(key: &str, host: &str) -> Result<String, AppError> {
    let raw = read_config(key)?; // Level 1
    let port = parse_port(&raw)?; // Level 2
    let valid = validate_port(port)?; // Level 3
    let conn = connect(host, valid)?; // Level 4
    Ok(conn) // Level 5 success
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_full_success() {
        assert_eq!(
            start_service("port", "localhost"),
            Ok("connected:8080".into())
        );
    }

    #[test]
    fn test_level1_config_error() {
        let err = start_service("missing", "localhost").unwrap_err();
        assert!(matches!(err, AppError::ConfigMissing(_)));
    }

    #[test]
    fn test_level2_parse_error() {
        // parse_port directly
        let err = parse_port("abc").unwrap_err();
        assert!(matches!(err, AppError::ParseFailed(_)));
    }

    #[test]
    fn test_level3_validation_error() {
        let err = validate_port(0).unwrap_err();
        assert!(matches!(err, AppError::ValidationFailed(_)));
    }

    #[test]
    fn test_level4_connection_error() {
        let err = connect("host", 9999).unwrap_err();
        assert!(matches!(err, AppError::ServiceUnavailable(_)));
    }

    #[test]
    fn test_error_display() {
        let err = AppError::ConfigMissing("db_url".into());
        assert_eq!(err.to_string(), "config missing: db_url");

        let err = AppError::Timeout;
        assert_eq!(err.to_string(), "timeout");
    }

    #[test]
    fn test_question_mark_propagates_correctly() {
        // Each ? passes the error through unchanged
        fn layer_test() -> Result<(), AppError> {
            let _ = read_config("missing")?;
            Ok(())
        }
        assert!(matches!(layer_test(), Err(AppError::ConfigMissing(_))));
    }

    #[test]
    fn test_all_layers_independent() {
        assert!(read_config("ok").is_ok());
        assert!(parse_port("8080").is_ok());
        assert!(validate_port(80).is_ok());
        assert!(connect("localhost", 80).is_ok());
    }
}
