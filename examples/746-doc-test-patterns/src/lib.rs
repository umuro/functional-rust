#![allow(clippy::all)]
//! # Documentation Tests
//!
//! Code examples in `///` doc comments are compiled and executed as tests.
//! Documentation can never go out of date.

/// Clamps `x` to the inclusive range `[lo, hi]`.
///
/// # Examples
///
/// ```
/// use example_746_doc_test_patterns::clamp;
/// assert_eq!(clamp(0, 10, -5), 0);
/// assert_eq!(clamp(0, 10, 5), 5);
/// assert_eq!(clamp(0, 10, 15), 10);
/// // Boundaries are inclusive
/// assert_eq!(clamp(0, 10, 0), 0);
/// assert_eq!(clamp(0, 10, 10), 10);
/// ```
pub fn clamp(lo: i32, hi: i32, x: i32) -> i32 {
    x.max(lo).min(hi)
}

/// Repeats `s` exactly `n` times.
///
/// # Examples
///
/// ```
/// use example_746_doc_test_patterns::repeat;
/// assert_eq!(repeat("ab", 3), "ababab");
/// assert_eq!(repeat("x", 0), "");
/// assert_eq!(repeat("", 5), "");
/// ```
pub fn repeat(s: &str, n: usize) -> String {
    s.repeat(n)
}

/// Splits `s` on the first occurrence of `delim`.
///
/// Returns `None` if `delim` is not found.
///
/// # Examples
///
/// ```
/// use example_746_doc_test_patterns::split_once_char;
/// assert_eq!(split_once_char("key:value", ':'), Some(("key", "value")));
/// assert_eq!(split_once_char("no-delim", ':'), None);
/// assert_eq!(split_once_char("a:b:c", ':'), Some(("a", "b:c")));
/// ```
pub fn split_once_char(s: &str, delim: char) -> Option<(&str, &str)> {
    s.split_once(delim)
}

/// Safe division that returns `Err` if divisor is zero.
///
/// # Errors
///
/// Returns `Err("division by zero")` when `b == 0`.
///
/// # Examples
///
/// ```
/// use example_746_doc_test_patterns::safe_div;
/// assert_eq!(safe_div(10, 2), Ok(5));
/// assert_eq!(safe_div(10, 0), Err("division by zero"));
/// assert_eq!(safe_div(-9, 3), Ok(-3));
/// ```
pub fn safe_div(a: i64, b: i64) -> Result<i64, &'static str> {
    if b == 0 {
        Err("division by zero")
    } else {
        Ok(a / b)
    }
}

/// Computes factorial of n.
///
/// # Panics
///
/// Panics if `n` is zero (this implementation treats 0! as undefined).
///
/// # Examples
///
/// ```
/// use example_746_doc_test_patterns::factorial;
/// assert_eq!(factorial(1), 1);
/// assert_eq!(factorial(5), 120);
/// ```
///
/// Attempting to compute factorial(0) will panic:
///
/// ```should_panic
/// example_746_doc_test_patterns::factorial(0);
/// ```
pub fn factorial(n: u64) -> u64 {
    if n == 0 {
        panic!("factorial(0) is undefined in this implementation")
    }
    (1..=n).product()
}

/// Alternative factorial that handles 0 correctly.
///
/// # Examples
///
/// ```
/// use example_746_doc_test_patterns::factorial_safe;
/// assert_eq!(factorial_safe(0), 1);
/// assert_eq!(factorial_safe(1), 1);
/// assert_eq!(factorial_safe(5), 120);
/// assert_eq!(factorial_safe(10), 3628800);
/// ```
pub fn factorial_safe(n: u64) -> u64 {
    if n == 0 {
        1
    } else {
        (1..=n).product()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clamp_edge_cases() {
        assert_eq!(clamp(i32::MIN, i32::MAX, 0), 0);
        assert_eq!(clamp(5, 5, 100), 5); // lo == hi
    }

    #[test]
    fn test_repeat_unicode() {
        assert_eq!(repeat("🦀", 3), "🦀🦀🦀");
    }

    #[test]
    fn test_safe_div_negative() {
        assert_eq!(safe_div(-10, -2), Ok(5));
    }

    #[test]
    fn test_split_empty_string() {
        assert_eq!(split_once_char("", ':'), None);
    }

    #[test]
    fn test_factorial_safe_large() {
        assert_eq!(factorial_safe(12), 479001600);
    }
}
