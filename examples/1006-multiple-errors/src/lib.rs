// 1006: Multiple Error Types
// Unifying multiple error types: Box<dyn Error> vs enum approach

use std::fmt;
use std::num::ParseIntError;

// Individual error types
#[derive(Debug)]
struct IoError(String);

impl fmt::Display for IoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "IO error: {}", self.0)
    }
}
impl std::error::Error for IoError {}

#[derive(Debug)]
struct NetError(String);

impl fmt::Display for NetError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "network error: {}", self.0)
    }
}
impl std::error::Error for NetError {}

// Approach 1: Box<dyn Error> — quick and flexible
fn do_io_boxed() -> Result<String, Box<dyn std::error::Error>> {
    Err(Box::new(IoError("file not found".into())))
}

fn do_parse_boxed(s: &str) -> Result<i64, Box<dyn std::error::Error>> {
    let n: i64 = s.parse()?; // ParseIntError auto-boxed
    Ok(n)
}

fn do_net_boxed() -> Result<String, Box<dyn std::error::Error>> {
    Err(Box::new(NetError("timeout".into())))
}

fn process_boxed() -> Result<i64, Box<dyn std::error::Error>> {
    let data = do_io_boxed().or_else(|_| Ok::<_, Box<dyn std::error::Error>>("42".into()))?;
    let parsed = do_parse_boxed(&data)?;
    let _response =
        do_net_boxed().or_else(|_| Ok::<String, Box<dyn std::error::Error>>("ok".into()))?;
    Ok(parsed)
}

// Approach 2: Typed enum — exhaustive matching
#[derive(Debug)]
enum AppError {
    Io(IoError),
    Parse(ParseIntError),
    Net(NetError),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Io(e) => write!(f, "{}", e),
            AppError::Parse(e) => write!(f, "parse: {}", e),
            AppError::Net(e) => write!(f, "{}", e),
        }
    }
}
impl std::error::Error for AppError {}

impl From<IoError> for AppError {
    fn from(e: IoError) -> Self {
        AppError::Io(e)
    }
}
impl From<ParseIntError> for AppError {
    fn from(e: ParseIntError) -> Self {
        AppError::Parse(e)
    }
}
impl From<NetError> for AppError {
    fn from(e: NetError) -> Self {
        AppError::Net(e)
    }
}

fn do_io_typed() -> Result<String, IoError> {
    Ok("42".into())
}

fn do_parse_typed(s: &str) -> Result<i64, ParseIntError> {
    s.parse()
}

fn process_typed() -> Result<i64, AppError> {
    let data = do_io_typed()?; // IoError -> AppError
    let parsed = do_parse_typed(&data)?; // ParseIntError -> AppError
    Ok(parsed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_boxed_error() {
        let result = process_boxed();
        assert_eq!(result.unwrap(), 42);
    }

    #[test]
    fn test_boxed_io_error() {
        let err = do_io_boxed().unwrap_err();
        assert!(err.to_string().contains("IO error"));
    }

    #[test]
    fn test_typed_success() {
        assert_eq!(process_typed().unwrap(), 42);
    }

    #[test]
    fn test_typed_pattern_match() {
        let err: AppError = IoError("test".into()).into();
        assert!(matches!(err, AppError::Io(_)));

        let err: AppError = "abc".parse::<i64>().unwrap_err().into();
        assert!(matches!(err, AppError::Parse(_)));
    }

    #[test]
    fn test_boxed_parse_error() {
        let result = do_parse_boxed("not_a_number");
        assert!(result.is_err());
    }

    #[test]
    fn test_display_format() {
        let err = AppError::Net(NetError("timeout".into()));
        assert_eq!(err.to_string(), "network error: timeout");
    }
}
