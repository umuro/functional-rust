#![allow(clippy::all)]
//! # Option Pattern Matching (Some/None)
//!
//! Handle optional values safely with pattern matching and combinators.

/// Safe division that returns None for division by zero.
pub fn safe_div(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        None
    } else {
        Some(a / b)
    }
}

/// Safe square root that returns None for negative numbers.
pub fn safe_sqrt(x: f64) -> Option<f64> {
    if x < 0.0 {
        None
    } else {
        Some(x.sqrt())
    }
}

/// Combine operations using Option combinators.
pub fn compute(a: i32, b: i32) -> Option<f64> {
    safe_div(a, b)
        .map(|q| q as f64)
        .and_then(safe_sqrt)
        .map(|r| r * 2.0)
}

/// Alternative using if-let chains (more imperative style).
pub fn compute_if_let(a: i32, b: i32) -> Option<f64> {
    if let Some(q) = safe_div(a, b) {
        if let Some(r) = safe_sqrt(q as f64) {
            return Some(r * 2.0);
        }
    }
    None
}

/// Alternative using match (explicit pattern matching).
pub fn compute_match(a: i32, b: i32) -> Option<f64> {
    match safe_div(a, b) {
        Some(q) => match safe_sqrt(q as f64) {
            Some(r) => Some(r * 2.0),
            None => None,
        },
        None => None,
    }
}

/// Filter and transform optional values in a collection.
pub fn uppercase_names(names: &[Option<&str>]) -> Vec<String> {
    names
        .iter()
        .filter_map(|o| o.map(str::to_uppercase))
        .collect()
}

/// Demonstrate unwrap variants.
pub fn unwrap_demos(opt: Option<i32>) -> (i32, i32, i32) {
    let with_default = opt.unwrap_or(0);
    let with_else = opt.unwrap_or_else(|| 42);
    let with_type_default = opt.unwrap_or_default();
    (with_default, with_else, with_type_default)
}

/// Parse and validate using and_then.
pub fn parse_positive(s: &str) -> Option<i32> {
    s.parse::<i32>().ok().filter(|&n| n > 0)
}

/// Alternative using and_then explicitly.
pub fn parse_positive_and_then(s: &str) -> Option<i32> {
    s.parse::<i32>()
        .ok()
        .and_then(|n| if n > 0 { Some(n) } else { None })
}

/// Flatten nested Options.
pub fn flatten_option(nested: Option<Option<i32>>) -> Option<i32> {
    nested.flatten()
}

/// Zip two Options together.
pub fn zip_options<T, U>(a: Option<T>, b: Option<U>) -> Option<(T, U)> {
    a.zip(b)
}

/// Get the first Some from two Options.
pub fn first_some<T>(a: Option<T>, b: Option<T>) -> Option<T> {
    a.or(b)
}

/// Alternative using or_else for lazy evaluation.
pub fn first_some_lazy<T>(a: Option<T>, b: impl FnOnce() -> Option<T>) -> Option<T> {
    a.or_else(b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safe_div() {
        assert_eq!(safe_div(10, 2), Some(5));
        assert_eq!(safe_div(10, 0), None);
        assert_eq!(safe_div(7, 3), Some(2));
    }

    #[test]
    fn test_safe_sqrt() {
        assert_eq!(safe_sqrt(4.0), Some(2.0));
        assert_eq!(safe_sqrt(0.0), Some(0.0));
        assert_eq!(safe_sqrt(-1.0), None);
    }

    #[test]
    fn test_compute() {
        let result = compute(10, 2);
        assert!(result.is_some());
        assert!((result.unwrap() - 4.472).abs() < 0.01); // sqrt(5) * 2
    }

    #[test]
    fn test_compute_div_zero() {
        assert_eq!(compute(10, 0), None);
    }

    #[test]
    fn test_compute_negative_sqrt() {
        assert_eq!(compute(-4, 2), None); // sqrt of -2
    }

    #[test]
    fn test_compute_approaches_equivalent() {
        let cases = [(10, 2), (10, 0), (-4, 2), (16, 4)];
        for (a, b) in cases {
            assert_eq!(compute(a, b), compute_if_let(a, b));
            assert_eq!(compute(a, b), compute_match(a, b));
        }
    }

    #[test]
    fn test_uppercase_names() {
        let names: Vec<Option<&str>> = vec![Some("alice"), None, Some("bob")];
        assert_eq!(uppercase_names(&names), vec!["ALICE", "BOB"]);
    }

    #[test]
    fn test_unwrap_demos() {
        assert_eq!(unwrap_demos(Some(10)), (10, 10, 10));
        assert_eq!(unwrap_demos(None), (0, 42, 0));
    }

    #[test]
    fn test_parse_positive() {
        assert_eq!(parse_positive("42"), Some(42));
        assert_eq!(parse_positive("0"), None);
        assert_eq!(parse_positive("-5"), None);
        assert_eq!(parse_positive("abc"), None);
    }

    #[test]
    fn test_parse_positive_approaches_equivalent() {
        let cases = ["42", "0", "-5", "abc", "100"];
        for s in cases {
            assert_eq!(parse_positive(s), parse_positive_and_then(s));
        }
    }

    #[test]
    fn test_flatten() {
        assert_eq!(flatten_option(Some(Some(42))), Some(42));
        assert_eq!(flatten_option(Some(None)), None);
        assert_eq!(flatten_option(None), None);
    }

    #[test]
    fn test_zip() {
        assert_eq!(zip_options(Some(1), Some("hello")), Some((1, "hello")));
        assert_eq!(zip_options(Some(1), None::<&str>), None);
        assert_eq!(zip_options(None::<i32>, Some("hello")), None);
    }

    #[test]
    fn test_first_some() {
        assert_eq!(first_some(Some(1), Some(2)), Some(1));
        assert_eq!(first_some(None, Some(2)), Some(2));
        assert_eq!(first_some(Some(1), None), Some(1));
        assert_eq!(first_some(None::<i32>, None), None);
    }
}
