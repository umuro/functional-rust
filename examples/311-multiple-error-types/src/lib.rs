//! # Handling Multiple Error Types
//!
//! Unify multiple error types with an enum + `impl From` for each variant.

use std::fmt;
use std::num::ParseIntError;

#[derive(Debug)]
pub struct IoError(pub String);
#[derive(Debug)]
pub struct DbError(pub String);

impl fmt::Display for IoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "IO: {}", self.0) }
}
impl fmt::Display for DbError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "DB: {}", self.0) }
}
impl std::error::Error for IoError {}
impl std::error::Error for DbError {}

#[derive(Debug)]
pub enum AppError {
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

impl From<IoError> for AppError {
    fn from(e: IoError) -> Self { AppError::Io(e) }
}
impl From<DbError> for AppError {
    fn from(e: DbError) -> Self { AppError::Db(e) }
}
impl From<ParseIntError> for AppError {
    fn from(e: ParseIntError) -> Self { AppError::Parse(e) }
}

pub fn read_file(path: &str) -> Result<String, IoError> {
    if path == "missing" { Err(IoError(format!("{}: not found", path))) }
    else { Ok("42".to_string()) }
}

pub fn query_db(n: i32) -> Result<Vec<i32>, DbError> {
    if n < 0 { Err(DbError("negative input".to_string())) }
    else { Ok(vec![n, n*2, n*3]) }
}

pub fn pipeline(path: &str) -> Result<Vec<i32>, AppError> {
    let content = read_file(path)?;
    let n: i32 = content.trim().parse()?;
    let rows = query_db(n)?;
    Ok(rows)
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

    #[test]
    fn test_from_db_error() {
        let e: AppError = DbError("test".to_string()).into();
        assert!(matches!(e, AppError::Db(_)));
    }
}
