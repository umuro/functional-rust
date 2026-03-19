// 1013: Panic vs Result
// When to panic vs return Result: unwrap, expect, assertions

// Approach 1: panic! / unwrap / expect — for bugs and invariants
fn divide_or_panic(a: i64, b: i64) -> i64 {
    if b == 0 {
        panic!("division by zero: programming error");
    }
    a / b
}

fn first_element(slice: &[i64]) -> i64 {
    // unwrap: panics with generic message
    // expect: panics with custom message — preferred
    slice.first().copied().expect("slice must not be empty")
}

// Approach 2: Result — for expected/recoverable failures
fn divide(a: i64, b: i64) -> Result<i64, String> {
    if b == 0 {
        Err("division by zero".into())
    } else {
        Ok(a / b)
    }
}

fn parse_positive(s: &str) -> Result<i64, String> {
    let n: i64 = s.parse().map_err(|_| format!("not a number: {}", s))?;
    if n <= 0 {
        Err(format!("not positive: {}", n))
    } else {
        Ok(n)
    }
}

// Approach 3: debug_assert for development-only checks
fn process_data(data: &[i64]) -> i64 {
    debug_assert!(!data.is_empty(), "data must not be empty");
    assert!(data.len() <= 1000, "data too large"); // always checked
    data.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_divide_success() {
        assert_eq!(divide_or_panic(10, 2), 5);
    }

    #[test]
    #[should_panic(expected = "division by zero")]
    fn test_divide_panic() {
        divide_or_panic(10, 0);
    }

    #[test]
    fn test_first_element() {
        assert_eq!(first_element(&[1, 2, 3]), 1);
    }

    #[test]
    #[should_panic(expected = "must not be empty")]
    fn test_first_element_panic() {
        first_element(&[]);
    }

    #[test]
    fn test_result_divide() {
        assert_eq!(divide(10, 2), Ok(5));
        assert_eq!(divide(10, 0), Err("division by zero".into()));
    }

    #[test]
    fn test_parse_positive() {
        assert_eq!(parse_positive("42"), Ok(42));
        assert!(parse_positive("-5").unwrap_err().contains("not positive"));
        assert!(parse_positive("abc").unwrap_err().contains("not a number"));
    }

    #[test]
    fn test_process_data() {
        assert_eq!(process_data(&[1, 2, 3]), 6);
    }

    #[test]
    fn test_unwrap_vs_expect() {
        // unwrap: generic panic message
        let val: Option<i64> = Some(42);
        assert_eq!(val.unwrap(), 42);

        // expect: custom panic message — better for debugging
        assert_eq!(val.expect("should have a value"), 42);
    }

    #[test]
    fn test_guidelines() {
        // Use panic/unwrap/expect when:
        // - Logic error / invariant violation (bug in your code)
        // - Prototype/example code
        // - Tests

        // Use Result when:
        // - Input validation
        // - File/network operations
        // - Parsing user data
        // - Any expected failure the caller should handle
        assert!(true); // documenting the distinction
    }
}
