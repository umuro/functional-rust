#![allow(dead_code)]
#![allow(clippy::all)]
// 1016: Error Context
// Add context/backtrace to errors manually using wrapper structs

use std::fmt;

// Approach 1: Context wrapper struct
#[derive(Debug)]
struct ErrorWithContext {
    message: String,
    context: Vec<String>,
}

impl ErrorWithContext {
    fn new(message: impl Into<String>) -> Self {
        ErrorWithContext {
            message: message.into(),
            context: Vec::new(),
        }
    }

    fn with_context(mut self, ctx: impl Into<String>) -> Self {
        self.context.push(ctx.into());
        self
    }
}

impl fmt::Display for ErrorWithContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.context.is_empty() {
            write!(f, "{}", self.message)
        } else {
            let chain: Vec<&str> = self.context.iter().rev().map(|s| s.as_str()).collect();
            write!(f, "{}: {}", chain.join(" -> "), self.message)
        }
    }
}

impl std::error::Error for ErrorWithContext {}

// Extension trait for adding context to any Result
trait Context<T> {
    fn context(self, ctx: impl Into<String>) -> Result<T, ErrorWithContext>;
    fn with_context(self, f: impl FnOnce() -> String) -> Result<T, ErrorWithContext>;
}

impl<T> Context<T> for Result<T, ErrorWithContext> {
    fn context(self, ctx: impl Into<String>) -> Result<T, ErrorWithContext> {
        self.map_err(|e| e.with_context(ctx))
    }
    fn with_context(self, f: impl FnOnce() -> String) -> Result<T, ErrorWithContext> {
        self.map_err(|e| e.with_context(f()))
    }
}

// Low-level functions
fn read_file(path: &str) -> Result<String, ErrorWithContext> {
    if path == "/missing" {
        Err(ErrorWithContext::new("file not found"))
    } else {
        Ok("42".into())
    }
}

fn parse_config(content: &str) -> Result<i64, ErrorWithContext> {
    content
        .parse::<i64>()
        .map_err(|e| ErrorWithContext::new(format!("invalid number: {}", e)))
}

// Approach 2: Chain contexts through layers
fn load_setting(path: &str) -> Result<i64, ErrorWithContext> {
    let content = read_file(path).context("reading config")?;
    let value = parse_config(&content).context("parsing config")?;
    Ok(value)
}

fn init_system(path: &str) -> Result<i64, ErrorWithContext> {
    load_setting(path).context("system init")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_context() {
        let err = ErrorWithContext::new("oops");
        assert_eq!(err.to_string(), "oops");
    }

    #[test]
    fn test_single_context() {
        let err = ErrorWithContext::new("oops").with_context("loading");
        assert_eq!(err.to_string(), "loading: oops");
    }

    #[test]
    fn test_nested_context() {
        let result = init_system("/missing");
        let err = result.unwrap_err();
        assert_eq!(err.context.len(), 2);
        assert!(err.to_string().contains("system init"));
        assert!(err.to_string().contains("reading config"));
        assert!(err.to_string().contains("file not found"));
    }

    #[test]
    fn test_success_passes_through() {
        assert_eq!(init_system("/ok").unwrap(), 42);
    }

    #[test]
    fn test_context_trait() {
        let result: Result<i64, ErrorWithContext> = Err(ErrorWithContext::new("base"));
        let result = result.context("layer1");
        let result = result.map_err(|e| e.with_context("layer2"));
        let err = result.unwrap_err();
        assert_eq!(err.context.len(), 2);
    }

    #[test]
    fn test_lazy_context() {
        let result: Result<i64, ErrorWithContext> = Err(ErrorWithContext::new("base"));
        let result = result.with_context(|| format!("dynamic context {}", 42));
        let err = result.unwrap_err();
        assert!(err.to_string().contains("dynamic context 42"));
    }
}
