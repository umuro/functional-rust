//! 298. anyhow-style boxed errors
//!
//! `Box<dyn Error + Send + Sync>` is a universal error container — the anyhow pattern.

use std::error::Error;
use std::fmt;

// Type alias for ergonomics (what anyhow::Result is essentially)
type AnyResult<T> = Result<T, Box<dyn Error + Send + Sync>>;

// A simple context wrapper
#[derive(Debug)]
struct WithContext {
    context: String,
    source: Box<dyn Error + Send + Sync>,
}

impl fmt::Display for WithContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.context)
    }
}

impl Error for WithContext {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(self.source.as_ref())
    }
}

// Extension trait for adding context (like anyhow's .context())
trait ResultExt<T> {
    fn context(self, msg: &str) -> AnyResult<T>;
}

impl<T, E: Error + Send + Sync + 'static> ResultExt<T> for Result<T, E> {
    fn context(self, msg: &str) -> AnyResult<T> {
        self.map_err(|e| Box::new(WithContext {
            context: msg.to_string(),
            source: Box::new(e),
        }) as Box<dyn Error + Send + Sync>)
    }
}

fn parse_port(s: &str) -> AnyResult<u16> {
    let n: u16 = s.parse()?; // ? boxes any error
    if n == 0 { return Err("port cannot be zero".into()); } // .into() on &str!
    Ok(n)
}

fn load_config(port_str: &str, host: &str) -> AnyResult<String> {
    let port = parse_port(port_str).context("invalid port number")?;
    if host.is_empty() { return Err("empty hostname".into()); }
    Ok(format!("{}:{}", host, port))
}

fn main() {
    match load_config("8080", "localhost") {
        Ok(addr) => println!("Config: {}", addr),
        Err(e) => println!("Error: {}", e),
    }

    match load_config("abc", "localhost") {
        Ok(addr) => println!("Config: {}", addr),
        Err(e) => {
            println!("Error: {}", e);
            if let Some(src) = e.source() {
                println!("  Caused by: {}", src);
            }
        }
    }

    match load_config("8080", "") {
        Ok(addr) => println!("Config: {}", addr),
        Err(e) => println!("Error: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_config_ok() {
        assert!(load_config("8080", "localhost").is_ok());
    }

    #[test]
    fn test_parse_port_bad() {
        assert!(parse_port("abc").is_err());
    }

    #[test]
    fn test_empty_host() {
        assert!(load_config("8080", "").is_err());
    }

    #[test]
    fn test_port_zero() {
        assert!(parse_port("0").is_err());
    }
}
