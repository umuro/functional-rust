//! 312. Downcasting boxed errors
//!
//! `downcast_ref::<T>()` recovers the concrete type from `Box<dyn Error>`.

use std::error::Error;
use std::fmt;

#[derive(Debug, PartialEq)]
struct ParseError { input: String }
impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "parse error: '{}'", self.input)
    }
}
impl Error for ParseError {}

#[derive(Debug)]
struct NetworkError { code: u32, message: String }
impl fmt::Display for NetworkError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "network error {}: {}", self.code, self.message)
    }
}
impl Error for NetworkError {}

#[derive(Debug)]
struct IoError { path: String }
impl fmt::Display for IoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "IO error: '{}'", self.path)
    }
}
impl Error for IoError {}

fn handle_error(e: &dyn Error) {
    // Downcast to specific types
    if let Some(pe) = e.downcast_ref::<ParseError>() {
        println!("Handling parse error for input: '{}'", pe.input);
        return;
    }
    if let Some(ne) = e.downcast_ref::<NetworkError>() {
        if ne.code == 404 {
            println!("Not found: {}", ne.message);
        } else {
            println!("Network error {}: {}", ne.code, ne.message);
        }
        return;
    }
    // Generic fallback
    println!("Unknown error: {}", e);
}

fn make_errors() -> Vec<Box<dyn Error>> {
    vec![
        Box::new(ParseError { input: "abc".to_string() }),
        Box::new(NetworkError { code: 404, message: "not found".to_string() }),
        Box::new(IoError { path: "/etc/missing".to_string() }),
    ]
}

fn main() {
    for err in make_errors() {
        handle_error(err.as_ref());
    }

    // Downcast with Box::downcast::<T>()
    let boxed: Box<dyn Error> = Box::new(ParseError { input: "xyz".to_string() });
    match boxed.downcast::<ParseError>() {
        Ok(pe) => println!("Downcasted: {:?}", pe),
        Err(original) => println!("Wrong type: {}", original),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_downcast_ref_success() {
        let e: Box<dyn Error> = Box::new(ParseError { input: "x".to_string() });
        assert!(e.downcast_ref::<ParseError>().is_some());
        assert!(e.downcast_ref::<NetworkError>().is_none());
    }

    #[test]
    fn test_downcast_box() {
        let e: Box<dyn Error> = Box::new(ParseError { input: "abc".to_string() });
        let result = e.downcast::<ParseError>();
        assert!(result.is_ok());
    }

    #[test]
    fn test_downcast_wrong_type() {
        let e: Box<dyn Error + Send + Sync> =
            Box::new(IoError { path: "/x".to_string() });
        assert!(e.downcast_ref::<ParseError>().is_none());
    }
}
