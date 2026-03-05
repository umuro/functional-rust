// 1004: Error Conversion
// From trait for automatic error conversion with ? operator

use std::fmt;
use std::num::ParseIntError;

// Sub-error types
#[derive(Debug)]
enum IoError {
    FileNotFound(String),
    PermissionDenied(String),
}

impl fmt::Display for IoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IoError::FileNotFound(p) => write!(f, "file not found: {}", p),
            IoError::PermissionDenied(p) => write!(f, "permission denied: {}", p),
        }
    }
}

impl std::error::Error for IoError {}

// Unified app error with From impls
#[derive(Debug)]
enum AppError {
    Io(IoError),
    Parse(ParseIntError),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Io(e) => write!(f, "IO: {}", e),
            AppError::Parse(e) => write!(f, "Parse: {}", e),
        }
    }
}

impl std::error::Error for AppError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            AppError::Io(e) => Some(e),
            AppError::Parse(e) => Some(e),
        }
    }
}

// From impls enable automatic conversion with ?
impl From<IoError> for AppError {
    fn from(e: IoError) -> Self {
        AppError::Io(e)
    }
}

impl From<ParseIntError> for AppError {
    fn from(e: ParseIntError) -> Self {
        AppError::Parse(e)
    }
}

// Functions that return sub-errors
fn read_config(path: &str) -> Result<String, IoError> {
    if path == "/missing" {
        Err(IoError::FileNotFound(path.to_string()))
    } else {
        Ok("42".to_string())
    }
}

// The ? operator automatically calls From to convert errors
fn load_config(path: &str) -> Result<i64, AppError> {
    let content = read_config(path)?;  // IoError -> AppError via From
    let value: i64 = content.parse()?; // ParseIntError -> AppError via From
    Ok(value)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_successful_load() {
        assert_eq!(load_config("/ok").unwrap(), 42);
    }

    #[test]
    fn test_io_error_conversion() {
        let result = load_config("/missing");
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err, AppError::Io(IoError::FileNotFound(_))));
        assert!(err.to_string().contains("file not found"));
    }

    #[test]
    fn test_from_io_error() {
        let io_err = IoError::FileNotFound("test".into());
        let app_err: AppError = io_err.into();
        assert!(matches!(app_err, AppError::Io(_)));
    }

    #[test]
    fn test_from_parse_error() {
        let parse_err: ParseIntError = "abc".parse::<i64>().unwrap_err();
        let app_err: AppError = parse_err.into();
        assert!(matches!(app_err, AppError::Parse(_)));
    }

    #[test]
    fn test_error_source_chain() {
        use std::error::Error;
        let result = load_config("/missing");
        let err = result.unwrap_err();
        // source() returns the inner error
        assert!(err.source().is_some());
    }

    #[test]
    fn test_question_mark_converts() {
        fn inner() -> Result<i64, AppError> {
            let _s = read_config("/ok")?; // auto-converts IoError
            Ok(42)
        }
        assert_eq!(inner().unwrap(), 42);
    }
}
