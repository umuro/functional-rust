//! # Adding Context to Errors
//!
//! Context wrapping adds layers of "where/why" around errors via the `source()` chain.

use std::error::Error;
use std::fmt;

/// Generic context wrapper
#[derive(Debug)]
pub struct Context<E> {
    pub message: String,
    pub source: E,
}

impl<E: fmt::Display> fmt::Display for Context<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl<E: Error + 'static> Error for Context<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.source)
    }
}

/// Extension trait to add context to any Result
pub trait WithContext<T, E> {
    fn with_context<F: FnOnce() -> String>(self, f: F) -> Result<T, Context<E>>;
    fn context(self, msg: &str) -> Result<T, Context<E>>;
}

impl<T, E: Error> WithContext<T, E> for Result<T, E> {
    fn with_context<F: FnOnce() -> String>(self, f: F) -> Result<T, Context<E>> {
        self.map_err(|e| Context {
            message: f(),
            source: e,
        })
    }

    fn context(self, msg: &str) -> Result<T, Context<E>> {
        self.with_context(|| msg.to_string())
    }
}

/// Simple IO error for demonstration
#[derive(Debug)]
pub struct IoError(pub String);

impl fmt::Display for IoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for IoError {}

/// Simulate reading a file
pub fn read_file(path: &str) -> Result<String, IoError> {
    if path.ends_with(".missing") {
        Err(IoError(format!("{}: not found", path)))
    } else {
        Ok(format!("contents of {}", path))
    }
}

/// Load config with context
pub fn load_config(path: &str) -> Result<String, Context<IoError>> {
    read_file(path).context(&format!("loading config from '{}'", path))
}

/// Print full error chain
pub fn format_error_chain(e: &dyn Error) -> String {
    let mut result = format!("Error: {}", e);
    let mut cause = e.source();
    while let Some(c) = cause {
        result.push_str(&format!("\n  Caused by: {}", c));
        cause = c.source();
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_context_ok() {
        let result = read_file("test.toml").context("reading test file");
        assert!(result.is_ok());
    }

    #[test]
    fn test_context_preserves_source() {
        let result = load_config("x.missing");
        assert!(result.is_err());
        let e = result.unwrap_err();
        assert!(e.source().is_some());
    }

    #[test]
    fn test_context_message() {
        let result: Result<(), IoError> = Err(IoError("fail".to_string()));
        let ctx = result.context("doing something");
        let e = ctx.unwrap_err();
        assert!(format!("{}", e).contains("doing something"));
    }

    #[test]
    fn test_with_context_lazy() {
        let mut called = false;
        let result: Result<i32, IoError> = Ok(42);
        let _ = result.with_context(|| {
            called = true;
            "should not be called".to_string()
        });
        assert!(!called); // closure not called on Ok
    }

    #[test]
    fn test_error_chain_format() {
        let result = load_config("app.missing");
        let e = result.unwrap_err();
        let chain = format_error_chain(&e);
        assert!(chain.contains("loading config"));
        assert!(chain.contains("Caused by"));
    }
}
