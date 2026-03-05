//! # std::io::Error patterns
//!
//! `std::io::Error` wraps OS errors with `ErrorKind` for portable classification.

use std::io::{self, ErrorKind};

/// Validate a port number
pub fn validate_port(port: u16) -> io::Result<u16> {
    if port == 0 {
        return Err(io::Error::new(ErrorKind::InvalidInput, "port cannot be zero"));
    }
    if port < 1024 {
        return Err(io::Error::new(
            ErrorKind::PermissionDenied,
            format!("port {} requires root", port),
        ));
    }
    Ok(port)
}

/// Check path validity
pub fn check_path(path: &str) -> io::Result<()> {
    if path.is_empty() {
        return Err(io::Error::new(ErrorKind::InvalidInput, "path cannot be empty"));
    }
    Ok(())
}

/// Classify io::Error by kind
pub fn classify_error(e: &io::Error) -> &'static str {
    match e.kind() {
        ErrorKind::NotFound => "not found",
        ErrorKind::PermissionDenied => "permission denied",
        ErrorKind::InvalidInput => "invalid input",
        ErrorKind::WouldBlock => "would block",
        _ => "other",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        assert_eq!(validate_port(8080).unwrap(), 8080);
    }

    #[test]
    fn test_check_path_empty() {
        let r = check_path("");
        assert_eq!(r.unwrap_err().kind(), ErrorKind::InvalidInput);
    }

    #[test]
    fn test_classify_error() {
        let e = io::Error::new(ErrorKind::NotFound, "file missing");
        assert_eq!(classify_error(&e), "not found");
    }
}
