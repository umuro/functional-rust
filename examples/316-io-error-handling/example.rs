//! 316. std::io::Error patterns
//!
//! `std::io::Error` wraps OS errors with `ErrorKind` for portable classification.

use std::io::{self, ErrorKind};
use std::fs;

/// Simulate file reading with various error conditions
fn read_config(path: &str) -> io::Result<String> {
    if path.is_empty() {
        return Err(io::Error::new(ErrorKind::InvalidInput, "path cannot be empty"));
    }
    fs::read_to_string(path)
}

/// Classify and handle io::Error by kind
fn handle_io_error(e: &io::Error) {
    match e.kind() {
        ErrorKind::NotFound =>
            println!("  File not found"),
        ErrorKind::PermissionDenied =>
            println!("  Permission denied"),
        ErrorKind::InvalidInput =>
            println!("  Invalid input: {}", e),
        ErrorKind::WouldBlock =>
            println!("  Would block (try again)"),
        other =>
            println!("  Other error: {:?} — {}", other, e),
    }
}

/// Custom io::Error creation
fn validate_port(port: u16) -> io::Result<u16> {
    if port == 0 {
        return Err(io::Error::new(ErrorKind::InvalidInput, "port cannot be zero"));
    }
    if port < 1024 {
        return Err(io::Error::new(
            ErrorKind::PermissionDenied,
            format!("port {} requires root privileges", port),
        ));
    }
    Ok(port)
}

fn main() {
    // Reading a file that doesn't exist
    match read_config("/nonexistent/config.toml") {
        Ok(content) => println!("Config: {}", content),
        Err(ref e) => {
            print!("Error: ");
            handle_io_error(e);
            println!("  OS error code: {:?}", e.raw_os_error());
        }
    }

    // Reading with empty path
    match read_config("") {
        Ok(_) => {}
        Err(ref e) => { print!("Empty path: "); handle_io_error(e); }
    }

    // Custom error creation
    println!("Port 0: {:?}", validate_port(0));
    println!("Port 80: {:?}", validate_port(80));
    println!("Port 8080: {:?}", validate_port(8080));

    // Creating io::Error from OS error code
    let not_found = io::Error::from_raw_os_error(2); // ENOENT on Unix
    println!("From OS code 2: {:?}", not_found.kind());

    // io::Error::from(ErrorKind)
    let would_block = io::Error::from(ErrorKind::WouldBlock);
    println!("WouldBlock: {}", would_block);

    // Wrapping an io::Error with context
    match read_config("/nonexistent") {
        Err(e) => {
            println!("Original kind: {:?}", e.kind());
            // Wrap in a new error with context
            let wrapped = io::Error::new(e.kind(), format!("loading app config: {}", e));
            println!("Wrapped: {}", wrapped);
        }
        Ok(_) => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::ErrorKind;

    #[test]
    fn test_validate_port_zero() {
        let r = validate_port(0);
        assert!(r.is_err());
        assert_eq!(r.unwrap_err().kind(), ErrorKind::InvalidInput);
    }

    #[test]
    fn test_validate_port_privileged() {
        let r = validate_port(80);
        assert_eq!(r.unwrap_err().kind(), ErrorKind::PermissionDenied);
    }

    #[test]
    fn test_validate_port_ok() {
        assert_eq!(validate_port(8080), Ok(8080));
    }

    #[test]
    fn test_read_nonexistent() {
        let r = read_config("/nonexistent/file.txt");
        assert!(r.is_err());
        assert_eq!(r.unwrap_err().kind(), ErrorKind::NotFound);
    }

    #[test]
    fn test_empty_path_error() {
        let r = read_config("");
        assert_eq!(r.unwrap_err().kind(), ErrorKind::InvalidInput);
    }
}
