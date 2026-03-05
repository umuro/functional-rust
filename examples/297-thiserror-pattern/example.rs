//! 297. thiserror-style derive macros
//!
//! Manually implementing what `#[derive(thiserror::Error)]` generates.

use std::fmt;
use std::error::Error;

// --- What thiserror generates for you ---

// #[derive(thiserror::Error, Debug)]
// pub enum DbError {
//     #[error("connection to '{host}' failed")]
//     ConnectionFailed { host: String },
//     #[error("query failed: {0}")]
//     QueryFailed(String),
// }

// Here's the manual equivalent:

#[derive(Debug)]
pub enum DbError {
    ConnectionFailed { host: String },
    QueryFailed(String),
}

impl fmt::Display for DbError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DbError::ConnectionFailed { host } =>
                write!(f, "connection to '{}' failed", host),
            DbError::QueryFailed(sql) =>
                write!(f, "query failed: {}", sql),
        }
    }
}

impl Error for DbError {}

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
            AppError::Config { key, reason } =>
                write!(f, "config error for '{}': {}", key, reason),
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

// From impls (what #[from] generates)
impl From<DbError> for AppError {
    fn from(e: DbError) -> Self { AppError::Db(e) }
}

fn connect(host: &str) -> Result<(), DbError> {
    if host == "bad-host" {
        Err(DbError::ConnectionFailed { host: host.to_string() })
    } else {
        Ok(())
    }
}

fn run(host: &str) -> Result<(), AppError> {
    connect(host)?; // From<DbError> for AppError
    Ok(())
}

fn main() {
    let errors: Vec<AppError> = vec![
        AppError::Db(DbError::ConnectionFailed { host: "localhost".to_string() }),
        AppError::Auth("invalid token".to_string()),
        AppError::Config { key: "port".to_string(), reason: "missing".to_string() },
    ];
    for e in &errors {
        println!("{}", e);
    }

    match run("bad-host") {
        Ok(()) => println!("Connected!"),
        Err(ref e) => {
            println!("Failed: {}", e);
            if let Some(src) = e.source() {
                println!("  Source: {}", src);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_db_display() {
        let e = DbError::ConnectionFailed { host: "localhost".to_string() };
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
}
