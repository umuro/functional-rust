#![allow(clippy::all)]
//! # Option Combinators
//!
//! Work with optional values using `.map()`, `.filter()`, `.and_then()`, and `.unwrap_or()`.

/// Safe square root - returns None for negative inputs
pub fn safe_sqrt(x: f64) -> Option<f64> {
    if x >= 0.0 {
        Some(x.sqrt())
    } else {
        None
    }
}

/// Parse and compute square root
pub fn parse_and_sqrt(s: &str) -> Option<f64> {
    s.parse::<f64>().ok().and_then(safe_sqrt)
}

/// Map an optional value
pub fn double_option(opt: Option<i32>) -> Option<i32> {
    opt.map(|x| x * 2)
}

/// Filter by predicate
pub fn filter_even(opt: Option<i32>) -> Option<i32> {
    opt.filter(|&x| x % 2 == 0)
}

/// Get with default
pub fn get_or_default(opt: Option<i32>, default: i32) -> i32 {
    opt.unwrap_or(default)
}

/// Get with lazy default
pub fn get_or_compute<F>(opt: Option<i32>, f: F) -> i32
where
    F: FnOnce() -> i32,
{
    opt.unwrap_or_else(f)
}

/// Chain operations
pub fn chain_operations(s: &str) -> Option<i32> {
    s.parse::<i32>().ok().filter(|&x| x > 0).map(|x| x * x)
}

/// Use or for fallback Option
pub fn first_valid(a: Option<i32>, b: Option<i32>) -> Option<i32> {
    a.or(b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_some() {
        assert_eq!(Some(5i32).map(|x| x * 2), Some(10));
    }

    #[test]
    fn test_map_none() {
        assert_eq!(None::<i32>.map(|x| x * 2), None);
    }

    #[test]
    fn test_filter_pass() {
        assert_eq!(filter_even(Some(4)), Some(4));
    }

    #[test]
    fn test_filter_fail() {
        assert_eq!(filter_even(Some(3)), None);
    }

    #[test]
    fn test_and_then_chain() {
        let result = parse_and_sqrt("4.0");
        assert!((result.unwrap() - 2.0).abs() < 1e-10);
    }

    #[test]
    fn test_and_then_none() {
        assert_eq!(parse_and_sqrt("invalid"), None);
    }

    #[test]
    fn test_and_then_negative() {
        assert_eq!(parse_and_sqrt("-4.0"), None);
    }

    #[test]
    fn test_or_default() {
        assert_eq!(get_or_default(None, 42), 42);
        assert_eq!(get_or_default(Some(10), 42), 10);
    }

    #[test]
    fn test_or() {
        assert_eq!(first_valid(None, Some(5)), Some(5));
        assert_eq!(first_valid(Some(3), Some(5)), Some(3));
    }

    #[test]
    fn test_chain_operations() {
        assert_eq!(chain_operations("5"), Some(25));
        assert_eq!(chain_operations("-5"), None);
        assert_eq!(chain_operations("abc"), None);
    }
}
