#![allow(clippy::all)]
//! # thiserror-style derive macros
//!
//! Manually implementing what `#[derive(thiserror::Error)]` generates.

use std::error::Error;
use std::fmt;

/// Database error - what thiserror would generate for:
/// #[derive(thiserror::Error, Debug)]
/// pub enum DbError {
///     #[error("connection to '{host}' failed")]
///     ConnectionFailed { host: String },
///     #[error("query failed: {0}")]
///     QueryFailed(String),
/// }
#[derive(Debug)]
pub enum DbError {
    ConnectionFailed { host: String },
    QueryFailed(String),
}

impl fmt::Display for DbError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DbError::ConnectionFailed { host } => {
                write!(f, "connection to '{}' failed", host)
            }
            DbError::QueryFailed(sql) => write!(f, "query failed: {}", sql),
        }
    }
}

impl Error for DbError {}

/// Application error wrapping DbError
#[derive(Debug)]
pub enum AppError {
    Db(DbError),
    Auth(String),
    Config { key: String, reason: String },
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Db(e) => write!(f, "database error: {}", e),
            AppError::Auth(msg) => write!(f, "auth error: {}", msg),
            AppError::Config { key, reason } => {
                write!(f, "config error for '{}': {}", key, reason)
            }
        }
    }
}

impl Error for AppError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            AppError::Db(e) => Some(e),
            _ => None,
        }
    }
}

// From impl (what #[from] generates)
impl From<DbError> for AppError {
    fn from(e: DbError) -> Self {
        AppError::Db(e)
    }
}

/// Connect to database
pub fn connect(host: &str) -> Result<(), DbError> {
    if host == "bad-host" {
        Err(DbError::ConnectionFailed {
            host: host.to_string(),
        })
    } else {
        Ok(())
    }
}

/// Run application
pub fn run(host: &str) -> Result<(), AppError> {
    connect(host)?; // From<DbError> for AppError
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_db_display() {
        let e = DbError::ConnectionFailed {
            host: "localhost".to_string(),
        };
        assert!(format!("{}", e).contains("localhost"));
    }

    #[test]
    fn test_from_conversion() {
        let db_err = DbError::QueryFailed("SELECT *".to_string());
        let app_err: AppError = db_err.into();
        assert!(matches!(app_err, AppError::Db(_)));
    }

    #[test]
    fn test_source_chain() {
        let app_err = AppError::Db(DbError::QueryFailed("bad".to_string()));
        assert!(app_err.source().is_some());
    }

    #[test]
    fn test_run_ok() {
        assert!(run("good-host").is_ok());
    }

    #[test]
    fn test_run_err() {
        assert!(run("bad-host").is_err());
    }

    #[test]
    fn test_config_error() {
        let e = AppError::Config {
            key: "port".to_string(),
            reason: "missing".to_string(),
        };
        assert!(format!("{}", e).contains("port"));
    }
}
