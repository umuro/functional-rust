//! 300. Chaining errors with source()
//!
//! `Error::source()` creates a linked list of causes — traverse to print full chain.

use std::error::Error;
use std::fmt;

#[derive(Debug)]
struct FileError { path: String }
impl fmt::Display for FileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "file '{}' not found", self.path)
    }
}
impl Error for FileError {}

#[derive(Debug)]
struct ConfigError { source: FileError }
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

#[derive(Debug)]
struct StartupError { source: ConfigError }
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

/// Walk the source() chain and print each error
fn print_error_chain(e: &dyn Error) {
    println!("Error: {}", e);
    let mut cause: Option<&dyn Error> = e.source();
    let mut depth = 1;
    while let Some(c) = cause {
        println!("{}Caused by: {}", "  ".repeat(depth), c);
        cause = c.source();
        depth += 1;
    }
}

/// Collect the full error chain into a Vec
fn error_chain(e: &dyn Error) -> Vec<String> {
    let mut chain = vec![e.to_string()];
    let mut cause = e.source();
    while let Some(c) = cause {
        chain.push(c.to_string());
        cause = c.source();
    }
    chain
}

fn main() {
    let err = StartupError {
        source: ConfigError {
            source: FileError { path: "config.toml".to_string() }
        }
    };

    print_error_chain(&err);
    println!();
    let chain = error_chain(&err);
    println!("Chain length: {}", chain.len());
    println!("Root cause: {}", chain.last().unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chain_length() {
        let err = StartupError {
            source: ConfigError {
                source: FileError { path: "x".to_string() }
            }
        };
        let chain = error_chain(&err);
        assert_eq!(chain.len(), 3);
    }

    #[test]
    fn test_root_cause() {
        let err = StartupError {
            source: ConfigError {
                source: FileError { path: "missing.toml".to_string() }
            }
        };
        let chain = error_chain(&err);
        assert!(chain.last().unwrap().contains("missing.toml"));
    }

    #[test]
    fn test_source_none_for_root() {
        let e = FileError { path: "x".to_string() };
        assert!(e.source().is_none());
    }
}
