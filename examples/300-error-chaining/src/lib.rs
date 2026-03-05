//! # Chaining Errors with source()
//!
//! `Error::source()` creates a linked list of causes — traverse to print full chain.

use std::error::Error;
use std::fmt;

/// File error - root cause
#[derive(Debug)]
pub struct FileError {
    pub path: String,
}

impl fmt::Display for FileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "file '{}' not found", self.path)
    }
}

impl Error for FileError {}

/// Config error - wraps FileError
#[derive(Debug)]
pub struct ConfigError {
    pub source: FileError,
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "failed to read configuration")
    }
}

impl Error for ConfigError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.source)
    }
}

/// Startup error - wraps ConfigError
#[derive(Debug)]
pub struct StartupError {
    pub source: ConfigError,
}

impl fmt::Display for StartupError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "application startup failed")
    }
}

impl Error for StartupError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.source)
    }
}

/// Walk the source() chain and format each error
pub fn format_error_chain(e: &dyn Error) -> String {
    let mut result = format!("Error: {}", e);
    let mut cause = e.source();
    let mut depth = 1;
    while let Some(c) = cause {
        result.push_str(&format!("\n{}Caused by: {}", "  ".repeat(depth), c));
        cause = c.source();
        depth += 1;
    }
    result
}

/// Collect the full error chain into a Vec
pub fn error_chain(e: &dyn Error) -> Vec<String> {
    let mut chain = vec![e.to_string()];
    let mut cause = e.source();
    while let Some(c) = cause {
        chain.push(c.to_string());
        cause = c.source();
    }
    chain
}

/// Get the root cause
pub fn root_cause(e: &dyn Error) -> &dyn Error {
    let mut current = e;
    while let Some(source) = current.source() {
        current = source;
    }
    current
}

/// Create a test error chain
pub fn make_test_chain(path: &str) -> StartupError {
    StartupError {
        source: ConfigError {
            source: FileError {
                path: path.to_string(),
            },
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chain_length() {
        let err = make_test_chain("config.toml");
        let chain = error_chain(&err);
        assert_eq!(chain.len(), 3);
    }

    #[test]
    fn test_root_cause_message() {
        let err = make_test_chain("missing.toml");
        let chain = error_chain(&err);
        assert!(chain.last().unwrap().contains("missing.toml"));
    }

    #[test]
    fn test_source_none_for_root() {
        let e = FileError {
            path: "x".to_string(),
        };
        assert!(e.source().is_none());
    }

    #[test]
    fn test_format_chain() {
        let err = make_test_chain("app.conf");
        let formatted = format_error_chain(&err);
        assert!(formatted.contains("startup failed"));
        assert!(formatted.contains("configuration"));
        assert!(formatted.contains("app.conf"));
    }

    #[test]
    fn test_root_cause() {
        let err = make_test_chain("test.cfg");
        let root = root_cause(&err);
        assert!(root.to_string().contains("test.cfg"));
    }

    #[test]
    fn test_display_messages() {
        let err = make_test_chain("data.json");
        assert_eq!(format!("{}", err), "application startup failed");
        assert_eq!(format!("{}", err.source), "failed to read configuration");
        assert_eq!(format!("{}", err.source.source), "file 'data.json' not found");
    }
}
