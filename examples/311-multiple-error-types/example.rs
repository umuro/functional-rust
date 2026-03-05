//! 311. Handling multiple error types
//!
//! Unify multiple error types with an enum + `impl From` for each variant.

use std::fmt;
use std::num::ParseIntError;

// ---- Source error types ----
#[derive(Debug)] struct IoError(String);
#[derive(Debug)] struct DbError(String);

impl fmt::Display for IoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "IO: {}", self.0) }
}
impl fmt::Display for DbError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "DB: {}", self.0) }
}
impl std::error::Error for IoError {}
impl std::error::Error for DbError {}

// ---- Unified error enum ----
#[derive(Debug)]
enum AppError {
    Io(IoError),
    Db(DbError),
    Parse(ParseIntError),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Io(e) => write!(f, "IO error: {}", e),
            AppError::Db(e) => write!(f, "DB error: {}", e),
            AppError::Parse(e) => write!(f, "parse error: {}", e),
        }
    }
}

// From impls enable ? operator
impl From<IoError> for AppError {
    fn from(e: IoError) -> Self { AppError::Io(e) }
}
impl From<DbError> for AppError {
    fn from(e: DbError) -> Self { AppError::Db(e) }
}
impl From<ParseIntError> for AppError {
    fn from(e: ParseIntError) -> Self { AppError::Parse(e) }
}

// ---- Functions returning different error types ----
fn read_file(path: &str) -> Result<String, IoError> {
    if path == "missing" { Err(IoError(format!("{}: not found", path))) }
    else { Ok("42".to_string()) }
}

fn query_db(n: i32) -> Result<Vec<i32>, DbError> {
    if n < 0 { Err(DbError("negative input not allowed".to_string())) }
    else { Ok(vec![n, n*2, n*3]) }
}

// ---- Pipeline using ? with automatic conversion ----
fn pipeline(path: &str) -> Result<Vec<i32>, AppError> {
    let content = read_file(path)?;  // IoError -> AppError via From
    let n: i32 = content.trim().parse()?;  // ParseIntError -> AppError via From
    let rows = query_db(n)?;  // DbError -> AppError via From
    Ok(rows)
}

fn main() {
    println!("{:?}", pipeline("data.txt")); // Ok
    println!("{:?}", pipeline("missing"));  // Err(Io(...))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pipeline_ok() {
        assert!(pipeline("data.txt").is_ok());
    }

    #[test]
    fn test_pipeline_io_err() {
        assert!(matches!(pipeline("missing"), Err(AppError::Io(_))));
    }

    #[test]
    fn test_from_io_error() {
        let e: AppError = IoError("test".to_string()).into();
        assert!(matches!(e, AppError::Io(_)));
    }
}
