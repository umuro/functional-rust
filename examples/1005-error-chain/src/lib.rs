// 1005: Error Chaining
// Chain errors with context using map_err

use std::fmt;

#[derive(Debug)]
enum IoError {
    NotFound,
    Corrupted(String),
}

impl fmt::Display for IoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IoError::NotFound => write!(f, "not found"),
            IoError::Corrupted(s) => write!(f, "corrupted: {}", s),
        }
    }
}

#[derive(Debug)]
struct AppError {
    context: String,
    source: IoError,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.context, self.source)
    }
}

impl std::error::Error for AppError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None // IoError doesn't impl Error in this example for simplicity
    }
}

// Low-level function returning raw error
fn read_file(path: &str) -> Result<String, IoError> {
    match path {
        "/missing" => Err(IoError::NotFound),
        "/bad" => Err(IoError::Corrupted("invalid utf-8".into())),
        _ => Ok("data".into()),
    }
}

// Approach 1: map_err to add context
fn load_config(path: &str) -> Result<String, AppError> {
    read_file(path).map_err(|e| AppError {
        context: format!("loading {}", path),
        source: e,
    })
}

// Approach 2: Generic context extension trait
trait WithContext<T> {
    fn with_context(self, ctx: impl FnOnce() -> String) -> Result<T, AppError>;
}

impl<T> WithContext<T> for Result<T, IoError> {
    fn with_context(self, ctx: impl FnOnce() -> String) -> Result<T, AppError> {
        self.map_err(|e| AppError {
            context: ctx(),
            source: e,
        })
    }
}

fn load_config_ext(path: &str) -> Result<String, AppError> {
    read_file(path).with_context(|| format!("loading {}", path))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(load_config("/ok").unwrap(), "data");
    }

    #[test]
    fn test_map_err_context() {
        let err = load_config("/missing").unwrap_err();
        assert_eq!(err.context, "loading /missing");
        assert!(matches!(err.source, IoError::NotFound));
    }

    #[test]
    fn test_corrupted_context() {
        let err = load_config("/bad").unwrap_err();
        assert!(err.to_string().contains("corrupted"));
        assert!(err.to_string().contains("loading /bad"));
    }

    #[test]
    fn test_extension_trait() {
        assert_eq!(load_config_ext("/ok").unwrap(), "data");
        let err = load_config_ext("/missing").unwrap_err();
        assert_eq!(err.context, "loading /missing");
    }

    #[test]
    fn test_display_format() {
        let err = load_config("/missing").unwrap_err();
        assert_eq!(err.to_string(), "loading /missing: not found");
    }
}
