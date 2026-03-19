#![allow(clippy::all)]
// 1018: Error Downcast
// Downcasting Box<dyn Error> to concrete type

use std::error::Error;
use std::fmt;

#[derive(Debug)]
struct DatabaseError(String);

impl fmt::Display for DatabaseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "database error: {}", self.0)
    }
}
impl Error for DatabaseError {}

#[derive(Debug)]
struct AuthError(String);

impl fmt::Display for AuthError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "auth error: {}", self.0)
    }
}
impl Error for AuthError {}

#[derive(Debug)]
struct NetworkError(String);

impl fmt::Display for NetworkError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "network error: {}", self.0)
    }
}
impl Error for NetworkError {}

// Functions returning type-erased errors
fn might_fail_db() -> Result<(), Box<dyn Error>> {
    Err(Box::new(DatabaseError("timeout".into())))
}

fn might_fail_auth() -> Result<(), Box<dyn Error>> {
    Err(Box::new(AuthError("expired token".into())))
}

// Approach 1: downcast_ref — borrow the concrete type
fn classify_error(err: &(dyn Error + 'static)) -> &'static str {
    if err.downcast_ref::<DatabaseError>().is_some() {
        "database"
    } else if err.downcast_ref::<AuthError>().is_some() {
        "auth"
    } else if err.downcast_ref::<NetworkError>().is_some() {
        "network"
    } else {
        "unknown"
    }
}

// Approach 2: downcast — take ownership of concrete type
fn handle_error(err: Box<dyn Error>) -> String {
    if let Ok(db_err) = err.downcast::<DatabaseError>() {
        format!("Handling DB: {}", db_err.0)
    } else {
        "unhandled error".into()
    }
}

// Approach 3: Type ID check
fn is_database_error(err: &(dyn Error + 'static)) -> bool {
    err.downcast_ref::<DatabaseError>().is_some()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_downcast_ref_db() {
        let err: Box<dyn Error> = Box::new(DatabaseError("test".into()));
        assert_eq!(classify_error(err.as_ref()), "database");

        let concrete = err.downcast_ref::<DatabaseError>().unwrap();
        assert_eq!(concrete.0, "test");
    }

    #[test]
    fn test_downcast_ref_auth() {
        let err: Box<dyn Error> = Box::new(AuthError("bad".into()));
        assert_eq!(classify_error(err.as_ref()), "auth");
    }

    #[test]
    fn test_downcast_ref_unknown() {
        let err: Box<dyn Error> = Box::new(std::io::Error::new(std::io::ErrorKind::Other, "misc"));
        assert_eq!(classify_error(err.as_ref()), "unknown");
    }

    #[test]
    fn test_downcast_owned() {
        let err: Box<dyn Error> = Box::new(DatabaseError("owned".into()));
        let result = handle_error(err);
        assert_eq!(result, "Handling DB: owned");
    }

    #[test]
    fn test_downcast_owned_wrong_type() {
        let err: Box<dyn Error> = Box::new(AuthError("nope".into()));
        let result = handle_error(err);
        assert_eq!(result, "unhandled error");
    }

    #[test]
    fn test_is_check() {
        let err: Box<dyn Error> = Box::new(DatabaseError("x".into()));
        assert!(is_database_error(err.as_ref()));

        let err: Box<dyn Error> = Box::new(AuthError("x".into()));
        assert!(!is_database_error(err.as_ref()));
    }

    #[test]
    fn test_from_result() {
        let result = might_fail_db();
        let err = result.unwrap_err();
        assert!(err.downcast_ref::<DatabaseError>().is_some());
    }
}
