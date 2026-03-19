//! # Result Pattern Matching (Ok/Err)
//!
//! Handle fallible operations with the Result type and ? operator.

use std::num::ParseIntError;

/// Custom error type for demonstration.
#[derive(Debug, Clone, PartialEq)]
pub enum MyError {
    Parse(String),
    Range(i32),
    DivZero,
}

impl std::fmt::Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            MyError::Parse(s) => write!(f, "parse error: {}", s),
            MyError::Range(n) => write!(f, "{} out of range", n),
            MyError::DivZero => write!(f, "division by zero"),
        }
    }
}

impl std::error::Error for MyError {}

/// Parse a string to i32, mapping the error.
pub fn parse(s: &str) -> Result<i32, MyError> {
    s.parse()
        .map_err(|e: ParseIntError| MyError::Parse(e.to_string()))
}

/// Validate that a number is in range [1, 100].
pub fn validate(n: i32) -> Result<i32, MyError> {
    if (1..=100).contains(&n) {
        Ok(n)
    } else {
        Err(MyError::Range(n))
    }
}

/// Safe division with error handling.
pub fn safe_div(a: i32, b: i32) -> Result<i32, MyError> {
    if b == 0 {
        Err(MyError::DivZero)
    } else {
        Ok(a / b)
    }
}

/// Process using ? operator for early return on error.
pub fn process(s: &str) -> Result<i32, MyError> {
    let n = parse(s)?;
    let v = validate(n)?;
    Ok(v * v)
}

/// Alternative using and_then combinators.
pub fn process_combinators(s: &str) -> Result<i32, MyError> {
    parse(s).and_then(validate).map(|v| v * v)
}

/// Alternative using explicit match.
pub fn process_match(s: &str) -> Result<i32, MyError> {
    match parse(s) {
        Ok(n) => match validate(n) {
            Ok(v) => Ok(v * v),
            Err(e) => Err(e),
        },
        Err(e) => Err(e),
    }
}

/// Convert Result to Option, discarding error info.
pub fn result_to_option<T, E>(r: Result<T, E>) -> Option<T> {
    r.ok()
}

/// Convert Option to Result with custom error.
pub fn option_to_result<T>(opt: Option<T>, err: &str) -> Result<T, String> {
    opt.ok_or_else(|| err.to_string())
}

/// Map error type.
pub fn map_error_example(s: &str) -> Result<i32, String> {
    parse(s).map_err(|e| format!("Failed: {}", e))
}

/// Collect Vec<Result<T, E>> into Result<Vec<T>, E>.
pub fn collect_results(strings: &[&str]) -> Result<Vec<i32>, MyError> {
    strings.iter().map(|s| parse(s)).collect()
}

/// Use unwrap_or_else for default on error.
pub fn parse_or_default(s: &str, default: i32) -> i32 {
    parse(s).unwrap_or(default)
}

/// Chain multiple fallible operations.
pub fn complex_chain(a: &str, b: &str) -> Result<i32, MyError> {
    let x = parse(a)?;
    let y = parse(b)?;
    let sum = x.checked_add(y).ok_or(MyError::Range(i32::MAX))?;
    validate(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_valid() {
        assert_eq!(parse("42"), Ok(42));
        assert_eq!(parse("-10"), Ok(-10));
    }

    #[test]
    fn test_parse_invalid() {
        assert!(parse("abc").is_err());
        assert!(parse("").is_err());
    }

    #[test]
    fn test_validate_in_range() {
        assert_eq!(validate(1), Ok(1));
        assert_eq!(validate(50), Ok(50));
        assert_eq!(validate(100), Ok(100));
    }

    #[test]
    fn test_validate_out_of_range() {
        assert_eq!(validate(0), Err(MyError::Range(0)));
        assert_eq!(validate(101), Err(MyError::Range(101)));
        assert_eq!(validate(-5), Err(MyError::Range(-5)));
    }

    #[test]
    fn test_safe_div() {
        assert_eq!(safe_div(10, 2), Ok(5));
        assert_eq!(safe_div(10, 0), Err(MyError::DivZero));
    }

    #[test]
    fn test_process_valid() {
        assert_eq!(process("42"), Ok(1764)); // 42 * 42
        assert_eq!(process("10"), Ok(100));
    }

    #[test]
    fn test_process_parse_error() {
        assert!(matches!(process("abc"), Err(MyError::Parse(_))));
    }

    #[test]
    fn test_process_range_error() {
        assert_eq!(process("0"), Err(MyError::Range(0)));
        assert_eq!(process("101"), Err(MyError::Range(101)));
    }

    #[test]
    fn test_process_approaches_equivalent() {
        let cases = ["42", "abc", "0", "100", "101"];
        for s in cases {
            assert_eq!(process(s), process_combinators(s));
            assert_eq!(process(s), process_match(s));
        }
    }

    #[test]
    fn test_result_to_option() {
        assert_eq!(result_to_option(Ok::<_, ()>(42)), Some(42));
        assert_eq!(result_to_option(Err::<i32, _>("error")), None);
    }

    #[test]
    fn test_option_to_result() {
        assert_eq!(option_to_result(Some(42), "missing"), Ok(42));
        assert_eq!(
            option_to_result(None::<i32>, "missing"),
            Err("missing".to_string())
        );
    }

    #[test]
    fn test_collect_results_all_ok() {
        let strings = vec!["1", "2", "3"];
        assert_eq!(collect_results(&strings), Ok(vec![1, 2, 3]));
    }

    #[test]
    fn test_collect_results_with_error() {
        let strings = vec!["1", "x", "3"];
        assert!(collect_results(&strings).is_err());
    }

    #[test]
    fn test_parse_or_default() {
        assert_eq!(parse_or_default("42", 0), 42);
        assert_eq!(parse_or_default("abc", 0), 0);
    }

    #[test]
    fn test_complex_chain() {
        assert_eq!(complex_chain("10", "20"), Ok(30));
        assert!(complex_chain("10", "abc").is_err());
        assert!(complex_chain("50", "60").is_err()); // 110 out of range
    }
}
