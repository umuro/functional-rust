//! # Result Combinators
//!
//! Transform, chain, and recover from errors using `.map()`, `.and_then()`, and `.or_else()`.

/// Parse a string into an integer with a custom error message
pub fn parse_int(s: &str) -> Result<i32, String> {
    s.parse::<i32>().map_err(|e| format!("parse error: {}", e))
}

/// Divide two numbers, returning an error for division by zero
pub fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err("division by zero".to_string())
    } else {
        Ok(a / b)
    }
}

/// Chain parse and divide operations
pub fn parse_and_divide(s: &str, divisor: i32) -> Result<i32, String> {
    parse_int(s).and_then(|n| divide(n, divisor))
}

/// Map on Ok value
pub fn double_result(r: Result<i32, String>) -> Result<i32, String> {
    r.map(|x| x * 2)
}

/// Recover from error with a default
pub fn with_default(r: Result<i32, String>, default: i32) -> Result<i32, String> {
    r.or_else(|_| Ok(default))
}

/// Add context to error messages
pub fn with_context(r: Result<i32, String>, context: &str) -> Result<i32, String> {
    r.map_err(|e| format!("{}: {}", context, e))
}

/// Full pipeline example
pub fn full_pipeline(s: &str) -> Result<i32, String> {
    parse_int(s)
        .and_then(|n| divide(n, 4))
        .map(|n| n + 1)
        .map_err(|e| format!("Pipeline failed: {}", e))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_int_ok() {
        assert_eq!(parse_int("42"), Ok(42));
    }

    #[test]
    fn test_parse_int_err() {
        assert!(parse_int("abc").is_err());
    }

    #[test]
    fn test_divide_ok() {
        assert_eq!(divide(10, 2), Ok(5));
    }

    #[test]
    fn test_divide_by_zero() {
        assert_eq!(divide(10, 0), Err("division by zero".to_string()));
    }

    #[test]
    fn test_map_ok() {
        let r: Result<i32, String> = Ok(5);
        assert_eq!(r.map(|x| x * 2), Ok(10));
    }

    #[test]
    fn test_and_then_chain() {
        let r = parse_and_divide("10", 2);
        assert_eq!(r, Ok(5));
    }

    #[test]
    fn test_and_then_short_circuit() {
        let r = parse_and_divide("abc", 2);
        assert!(r.is_err());
    }

    #[test]
    fn test_or_else_recovery() {
        let r: Result<i32, String> = Err("bad".to_string());
        let recovered = with_default(r, 42);
        assert_eq!(recovered, Ok(42));
    }

    #[test]
    fn test_map_err() {
        let r: Result<i32, String> = Err("bad".to_string());
        let mapped = with_context(r, "Error");
        assert_eq!(mapped, Err("Error: bad".to_string()));
    }

    #[test]
    fn test_full_pipeline() {
        assert_eq!(full_pipeline("20"), Ok(6)); // 20/4 + 1 = 6
    }
}
