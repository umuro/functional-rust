#![allow(clippy::all)]
//! # When to Panic vs Return Result
//!
//! Result for recoverable errors, panic for programming bugs.

/// Library function: user provides invalid input -> use Result
pub fn parse_age(s: &str) -> Result<u8, String> {
    let n: i32 = s.parse().map_err(|_| format!("'{}' is not a number", s))?;
    if n < 0 || n > 150 {
        return Err(format!("age {} is out of range [0, 150]", n));
    }
    Ok(n as u8)
}

/// Internal: programmer error -> can panic
pub fn get_element<T>(arr: &[T], index: usize) -> &T {
    &arr[index] // panics if out of bounds
}

/// Invariant that must always hold
pub fn divide(a: i32, b: i32) -> i32 {
    assert!(b != 0, "divide: b must not be zero");
    a / b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_age_valid() {
        assert_eq!(parse_age("25"), Ok(25));
        assert_eq!(parse_age("0"), Ok(0));
        assert_eq!(parse_age("150"), Ok(150));
    }

    #[test]
    fn test_parse_age_invalid() {
        assert!(parse_age("abc").is_err());
        assert!(parse_age("200").is_err());
        assert!(parse_age("-1").is_err());
    }

    #[test]
    fn test_get_element() {
        let arr = [10i32, 20, 30];
        assert_eq!(*get_element(&arr, 1), 20);
    }

    #[test]
    #[should_panic]
    fn test_get_element_panics() {
        let arr = [1i32];
        get_element(&arr, 99);
    }

    #[test]
    fn test_divide() {
        assert_eq!(divide(10, 2), 5);
    }

    #[test]
    #[should_panic]
    fn test_divide_by_zero_panics() {
        divide(10, 0);
    }
}
