// 1017: Typed Error Hierarchy
// Enum with variants for each subsystem

use std::fmt;

// Subsystem error types
#[derive(Debug, PartialEq)]
enum DbError {
    ConnectionFailed,
    QueryFailed(String),
    NotFound(String),
}

impl fmt::Display for DbError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DbError::ConnectionFailed => write!(f, "database connection failed"),
            DbError::QueryFailed(q) => write!(f, "query failed: {}", q),
            DbError::NotFound(id) => write!(f, "not found: {}", id),
        }
    }
}
impl std::error::Error for DbError {}

#[derive(Debug, PartialEq)]
enum AuthError {
    InvalidToken,
    Expired,
    Forbidden(String),
}

impl fmt::Display for AuthError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AuthError::InvalidToken => write!(f, "invalid token"),
            AuthError::Expired => write!(f, "token expired"),
            AuthError::Forbidden(r) => write!(f, "forbidden: {}", r),
        }
    }
}
impl std::error::Error for AuthError {}

#[derive(Debug, PartialEq)]
enum ApiError {
    BadRequest(String),
    RateLimit,
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ApiError::BadRequest(msg) => write!(f, "bad request: {}", msg),
            ApiError::RateLimit => write!(f, "rate limited"),
        }
    }
}
impl std::error::Error for ApiError {}

// Top-level error unifies all subsystems
#[derive(Debug)]
enum AppError {
    Db(DbError),
    Auth(AuthError),
    Api(ApiError),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Db(e) => write!(f, "[DB] {}", e),
            AppError::Auth(e) => write!(f, "[Auth] {}", e),
            AppError::Api(e) => write!(f, "[API] {}", e),
        }
    }
}
impl std::error::Error for AppError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            AppError::Db(e) => Some(e),
            AppError::Auth(e) => Some(e),
            AppError::Api(e) => Some(e),
        }
    }
}

impl From<DbError> for AppError {
    fn from(e: DbError) -> Self { AppError::Db(e) }
}
impl From<AuthError> for AppError {
    fn from(e: AuthError) -> Self { AppError::Auth(e) }
}
impl From<ApiError> for AppError {
    fn from(e: ApiError) -> Self { AppError::Api(e) }
}

// Subsystem functions
fn db_find_user(id: &str) -> Result<String, DbError> {
    if id == "missing" {
        Err(DbError::NotFound(id.into()))
    } else {
        Ok(format!("user_{}", id))
    }
}

fn auth_check(token: &str) -> Result<(), AuthError> {
    if token.is_empty() {
        Err(AuthError::InvalidToken)
    } else if token == "expired" {
        Err(AuthError::Expired)
    } else {
        Ok(())
    }
}

// App layer — ? auto-converts via From
fn get_user(token: &str, user_id: &str) -> Result<String, AppError> {
    auth_check(token)?;     // AuthError -> AppError
    let user = db_find_user(user_id)?; // DbError -> AppError
    Ok(user)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(get_user("valid", "123").unwrap(), "user_123");
    }

    #[test]
    fn test_auth_error() {
        let err = get_user("", "123").unwrap_err();
        assert!(matches!(err, AppError::Auth(AuthError::InvalidToken)));
    }

    #[test]
    fn test_expired_token() {
        let err = get_user("expired", "123").unwrap_err();
        assert!(matches!(err, AppError::Auth(AuthError::Expired)));
    }

    #[test]
    fn test_db_error() {
        let err = get_user("valid", "missing").unwrap_err();
        assert!(matches!(err, AppError::Db(DbError::NotFound(_))));
    }

    #[test]
    fn test_display_format() {
        let err = AppError::Db(DbError::QueryFailed("SELECT *".into()));
        assert_eq!(err.to_string(), "[DB] query failed: SELECT *");

        let err = AppError::Auth(AuthError::Expired);
        assert_eq!(err.to_string(), "[Auth] token expired");
    }

    #[test]
    fn test_error_source() {
        use std::error::Error;
        let err = AppError::Db(DbError::ConnectionFailed);
        let source = err.source().unwrap();
        assert_eq!(source.to_string(), "database connection failed");
    }

    #[test]
    fn test_pattern_matching_exhaustive() {
        // The compiler ensures all subsystems are handled
        fn handle(err: AppError) -> &'static str {
            match err {
                AppError::Db(_) => "database issue",
                AppError::Auth(_) => "auth issue",
                AppError::Api(_) => "api issue",
            }
        }
        assert_eq!(handle(AppError::Api(ApiError::RateLimit)), "api issue");
    }
}
