// 1011: The ? (Try) Operator
// Deep dive: early return, From conversion, desugaring

use std::fmt;
use std::num::ParseIntError;

#[derive(Debug, PartialEq)]
enum AppError {
    NotFound,
    ParseFailed(String),
    TooLarge(i64),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::NotFound => write!(f, "not found"),
            AppError::ParseFailed(s) => write!(f, "parse failed: {}", s),
            AppError::TooLarge(n) => write!(f, "too large: {}", n),
        }
    }
}
impl std::error::Error for AppError {}

impl From<ParseIntError> for AppError {
    fn from(e: ParseIntError) -> Self {
        AppError::ParseFailed(e.to_string())
    }
}

fn read_data(key: &str) -> Result<String, AppError> {
    if key == "missing" {
        Err(AppError::NotFound)
    } else {
        Ok("42".into())
    }
}

fn parse_data(s: &str) -> Result<i64, ParseIntError> {
    s.parse::<i64>()
}

fn validate(n: i64) -> Result<i64, AppError> {
    if n > 100 {
        Err(AppError::TooLarge(n))
    } else {
        Ok(n)
    }
}

// Approach 1: The ? operator — what it looks like
fn process_try(key: &str) -> Result<i64, AppError> {
    let s = read_data(key)?; // early return if Err
    let n = parse_data(&s)?; // ParseIntError -> AppError via From
    let v = validate(n)?; // early return if Err
    Ok(v)
}

// Approach 2: What ? desugars to (approximately)
fn process_desugared(key: &str) -> Result<i64, AppError> {
    let s = match read_data(key) {
        Ok(v) => v,
        Err(e) => return Err(From::from(e)),
    };
    let n = match parse_data(&s) {
        Ok(v) => v,
        Err(e) => return Err(From::from(e)), // From<ParseIntError>
    };
    let v = match validate(n) {
        Ok(v) => v,
        Err(e) => return Err(From::from(e)),
    };
    Ok(v)
}

// Approach 3: ? in expression position
fn process_inline(key: &str) -> Result<i64, AppError> {
    validate(parse_data(&read_data(key)?)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_try_success() {
        assert_eq!(process_try("ok"), Ok(42));
    }

    #[test]
    fn test_try_not_found() {
        assert_eq!(process_try("missing"), Err(AppError::NotFound));
    }

    #[test]
    fn test_desugared_matches_try() {
        assert_eq!(process_try("ok"), process_desugared("ok"));
        assert_eq!(process_try("missing"), process_desugared("missing"));
    }

    #[test]
    fn test_inline_matches_try() {
        assert_eq!(process_try("ok"), process_inline("ok"));
        assert_eq!(process_try("missing"), process_inline("missing"));
    }

    #[test]
    fn test_from_conversion() {
        // ? calls From::from on the error
        let parse_err = "abc".parse::<i64>().unwrap_err();
        let app_err: AppError = parse_err.into();
        assert!(matches!(app_err, AppError::ParseFailed(_)));
    }

    #[test]
    fn test_try_in_closure() {
        // ? works in closures that return Result
        let process = |key: &str| -> Result<i64, AppError> {
            let s = read_data(key)?;
            Ok(s.parse::<i64>()?)
        };
        assert_eq!(process("ok"), Ok(42));
    }

    #[test]
    fn test_too_large() {
        // If we had data "200", validate would fail
        fn process_large() -> Result<i64, AppError> {
            let n: i64 = "200"
                .parse()
                .map_err(|e: ParseIntError| AppError::ParseFailed(e.to_string()))?;
            validate(n)
        }
        assert_eq!(process_large(), Err(AppError::TooLarge(200)));
    }
}
