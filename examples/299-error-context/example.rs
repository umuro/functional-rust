//! 299. Adding context to errors
//!
//! Context wrapping adds layers of "where/why" around errors via the `source()` chain.

use std::error::Error;
use std::fmt;

/// Generic context wrapper
#[derive(Debug)]
pub struct Context<E> {
    message: String,
    source: E,
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
trait WithContext<T, E> {
    fn with_context<F: FnOnce() -> String>(self, f: F) -> Result<T, Context<E>>;
    fn context(self, msg: &str) -> Result<T, Context<E>>;
}

impl<T, E: Error> WithContext<T, E> for Result<T, E> {
    fn with_context<F: FnOnce() -> String>(self, f: F) -> Result<T, Context<E>> {
        self.map_err(|e| Context { message: f(), source: e })
    }
    fn context(self, msg: &str) -> Result<T, Context<E>> {
        self.with_context(|| msg.to_string())
    }
}

#[derive(Debug)]
struct IoError(String);
impl fmt::Display for IoError { fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{}", self.0) } }
impl Error for IoError {}

fn read_file(path: &str) -> Result<String, IoError> {
    if path.ends_with(".missing") { Err(IoError(format!("{}: not found", path))) }
    else { Ok(format!("contents of {}", path)) }
}

fn load_config(path: &str) -> Result<String, Context<IoError>> {
    read_file(path).context(&format!("loading config from '{}'", path))
}

fn print_chain(e: &dyn Error) {
    println!("  Error: {}", e);
    let mut cause = e.source();
    while let Some(c) = cause {
        println!("  Caused by: {}", c);
        cause = c.source();
    }
}

fn main() {
    match load_config("app.missing") {
        Ok(c) => println!("Config: {}", c),
        Err(ref e) => print_chain(e),
    }
    match load_config("app.toml") {
        Ok(c) => println!("Config: {}", c),
        Err(ref e) => print_chain(e),
    }
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
        assert!(format!("{}", e).contains("loading config"));
    }

    #[test]
    fn test_context_message() {
        let result: Result<(), IoError> = Err(IoError("fail".to_string()));
        let ctx = result.context("doing something");
        assert!(ctx.is_err());
        assert!(format!("{}", ctx.unwrap_err()).contains("doing something"));
    }
}
