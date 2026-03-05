// 1024: File Operation Errors
// std::io::Error kinds and handling

use std::fs;
use std::io::{self, Write};
use std::path::Path;

// Approach 1: Basic file operations with io::Error
fn read_file(path: &str) -> Result<String, io::Error> {
    fs::read_to_string(path)
}

fn write_file(path: &str, content: &str) -> Result<(), io::Error> {
    fs::write(path, content)
}

// Approach 2: Classifying io::Error by kind
fn classify_io_error(err: &io::Error) -> &'static str {
    match err.kind() {
        io::ErrorKind::NotFound => "file not found",
        io::ErrorKind::PermissionDenied => "permission denied",
        io::ErrorKind::AlreadyExists => "already exists",
        io::ErrorKind::InvalidInput => "invalid input",
        io::ErrorKind::TimedOut => "timed out",
        io::ErrorKind::Interrupted => "interrupted",
        io::ErrorKind::WouldBlock => "would block",
        _ => "other IO error",
    }
}

// Approach 3: Converting io::Error to app-specific error
#[derive(Debug)]
enum FileError {
    NotFound(String),
    PermissionDenied(String),
    Other(String),
}

impl std::fmt::Display for FileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FileError::NotFound(p) => write!(f, "file not found: {}", p),
            FileError::PermissionDenied(p) => write!(f, "permission denied: {}", p),
            FileError::Other(msg) => write!(f, "file error: {}", msg),
        }
    }
}

fn read_file_typed(path: &str) -> Result<String, FileError> {
    fs::read_to_string(path).map_err(|e| match e.kind() {
        io::ErrorKind::NotFound => FileError::NotFound(path.into()),
        io::ErrorKind::PermissionDenied => FileError::PermissionDenied(path.into()),
        _ => FileError::Other(e.to_string()),
    })
}

// Safe file operation with existence check
fn read_if_exists(path: &str) -> Result<Option<String>, io::Error> {
    if Path::new(path).exists() {
        fs::read_to_string(path).map(Some)
    } else {
        Ok(None)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_nonexistent() {
        let err = read_file("/nonexistent_file_12345").unwrap_err();
        assert_eq!(err.kind(), io::ErrorKind::NotFound);
    }

    #[test]
    fn test_classify_not_found() {
        let err = io::Error::new(io::ErrorKind::NotFound, "test");
        assert_eq!(classify_io_error(&err), "file not found");
    }

    #[test]
    fn test_classify_permission() {
        let err = io::Error::new(io::ErrorKind::PermissionDenied, "test");
        assert_eq!(classify_io_error(&err), "permission denied");
    }

    #[test]
    fn test_write_read_roundtrip() {
        let tmp = "/tmp/rust_test_1024.txt";
        write_file(tmp, "hello rust").unwrap();
        let content = read_file(tmp).unwrap();
        assert_eq!(content, "hello rust");
        fs::remove_file(tmp).unwrap();
    }

    #[test]
    fn test_typed_error() {
        let err = read_file_typed("/nonexistent_12345").unwrap_err();
        assert!(matches!(err, FileError::NotFound(_)));
        assert!(err.to_string().contains("not found"));
    }

    #[test]
    fn test_read_if_exists() {
        let result = read_if_exists("/nonexistent_12345").unwrap();
        assert!(result.is_none());

        let tmp = "/tmp/rust_test_1024b.txt";
        fs::write(tmp, "exists").unwrap();
        let result = read_if_exists(tmp).unwrap();
        assert_eq!(result, Some("exists".to_string()));
        fs::remove_file(tmp).unwrap();
    }

    #[test]
    fn test_io_error_display() {
        let err = io::Error::new(io::ErrorKind::NotFound, "missing.txt");
        assert_eq!(err.to_string(), "missing.txt");
    }

    #[test]
    fn test_error_kind_matching() {
        // io::ErrorKind is an enum — exhaustive matching available
        let err = fs::read_to_string("/no_such_file_xyz").unwrap_err();
        match err.kind() {
            io::ErrorKind::NotFound => {} // expected
            other => panic!("unexpected error kind: {:?}", other),
        }
    }
}
