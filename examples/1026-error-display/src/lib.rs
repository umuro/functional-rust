#![allow(clippy::all)]
// 1026: Custom Display for Nested Errors with Source Chain
// Walking the Error::source() chain for human-readable output

use std::error::Error;
use std::fmt;

// Inner error (root cause)
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
impl Error for IoError {}

// Middle error (wraps inner)
#[derive(Debug)]
struct ConfigError {
    operation: String,
    source: IoError,
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} failed", self.operation)
    }
}

impl Error for ConfigError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.source)
    }
}

// Outer error (wraps middle)
#[derive(Debug)]
struct AppError {
    module_name: String,
    source: ConfigError,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}] error", self.module_name)
    }
}

impl Error for AppError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.source)
    }
}

// Approach 1: Walk the source chain
fn display_error_chain(err: &dyn Error) -> String {
    let mut chain = Vec::new();
    let mut current: Option<&dyn Error> = Some(err);
    let mut depth = 0;

    while let Some(e) = current {
        let prefix = if depth == 0 { "Error" } else { "Caused by" };
        let indent = "  ".repeat(depth);
        chain.push(format!("{}{}: {}", indent, prefix, e));
        current = e.source();
        depth += 1;
    }

    chain.join("\n")
}

// Approach 2: Collect all error messages into a vec
fn error_sources(err: &dyn Error) -> Vec<String> {
    let mut sources = vec![err.to_string()];
    let mut current = err.source();
    while let Some(e) = current {
        sources.push(e.to_string());
        current = e.source();
    }
    sources
}

// Approach 3: Single-line display with arrows
fn display_inline(err: &dyn Error) -> String {
    error_sources(err).join(" -> ")
}

fn make_error() -> AppError {
    AppError {
        module_name: "config".into(),
        source: ConfigError {
            operation: "reading settings".into(),
            source: IoError::FileNotFound("/etc/app.conf".into()),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_chain() {
        let err = make_error();
        let chain = display_error_chain(&err);
        assert!(chain.contains("Error:"));
        assert!(chain.contains("Caused by:"));
        assert!(chain.contains("file not found"));
        let lines: Vec<&str> = chain.lines().collect();
        assert_eq!(lines.len(), 3); // outer, middle, inner
    }

    #[test]
    fn test_error_sources() {
        let err = make_error();
        let sources = error_sources(&err);
        assert_eq!(sources.len(), 3);
        assert_eq!(sources[0], "[config] error");
        assert_eq!(sources[1], "reading settings failed");
        assert!(sources[2].contains("file not found"));
    }

    #[test]
    fn test_inline_display() {
        let err = make_error();
        let inline = display_inline(&err);
        assert!(inline.contains(" -> "));
        assert!(inline.starts_with("[config] error"));
    }

    #[test]
    fn test_source_chain() {
        let err = make_error();
        // Level 0: AppError
        assert_eq!(err.to_string(), "[config] error");
        // Level 1: ConfigError
        let src1 = err.source().unwrap();
        assert_eq!(src1.to_string(), "reading settings failed");
        // Level 2: IoError
        let src2 = src1.source().unwrap();
        assert!(src2.to_string().contains("file not found"));
        // Level 3: None
        assert!(src2.source().is_none());
    }

    #[test]
    fn test_single_error_chain() {
        let err = IoError::FileNotFound("test.txt".into());
        let chain = display_error_chain(&err);
        assert_eq!(chain, "Error: file not found: test.txt");
        assert_eq!(error_sources(&err).len(), 1);
    }

    #[test]
    fn test_display_vs_debug() {
        let err = make_error();
        // Display: human-readable
        assert_eq!(format!("{}", err), "[config] error");
        // Debug: programmer-readable with structure
        let debug = format!("{:?}", err);
        assert!(debug.contains("AppError"));
        assert!(debug.contains("ConfigError"));
    }
}
