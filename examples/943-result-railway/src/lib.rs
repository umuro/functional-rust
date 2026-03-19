/// Result Type — Railway-Oriented Error Handling
///
/// Using Result with combinators (and_then/map) for chaining fallible
/// operations. Errors short-circuit the pipeline automatically.
/// Rust's `?` operator makes this even more ergonomic than OCaml's `>>=`.

/// Parse a string to i32.
pub fn parse_int(s: &str) -> Result<i32, String> {
    s.parse::<i32>()
        .map_err(|_| format!("not an integer: {:?}", s))
}

/// Validate that a number is positive.
pub fn positive(x: i32) -> Result<i32, String> {
    if x > 0 {
        Ok(x)
    } else {
        Err(format!("{} is not positive", x))
    }
}

/// Safe square root of a positive integer.
pub fn sqrt_safe(x: i32) -> Result<f64, String> {
    positive(x).map(|n| (n as f64).sqrt())
}

/// Pipeline using `and_then` (equivalent to OCaml's `>>=` bind).
pub fn process_bind(s: &str) -> Result<f64, String> {
    parse_int(s).and_then(positive).and_then(sqrt_safe)
}

/// Pipeline using the `?` operator — idiomatic Rust.
pub fn process(s: &str) -> Result<f64, String> {
    let n = parse_int(s)?;
    let n = positive(n)?;
    let result = sqrt_safe(n)?;
    Ok(result)
}

/// Map over the Ok value without changing error type.
pub fn process_doubled(s: &str) -> Result<f64, String> {
    process(s).map(|v| v * 2.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_input() {
        let r = process("16").unwrap();
        assert!((r - 4.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_valid_25() {
        let r = process("25").unwrap();
        assert!((r - 5.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_negative() {
        assert!(process("-4").is_err());
        assert_eq!(process("-4").unwrap_err(), "-4 is not positive");
    }

    #[test]
    fn test_not_integer() {
        assert!(process("hello").is_err());
    }

    #[test]
    fn test_zero() {
        assert!(process("0").is_err());
    }

    #[test]
    fn test_bind_matches_question_mark() {
        for s in &["16", "25", "-4", "hello", "0"] {
            assert_eq!(process(s), process_bind(s));
        }
    }

    #[test]
    fn test_map() {
        let r = process_doubled("16").unwrap();
        assert!((r - 8.0).abs() < f64::EPSILON);
    }
}
