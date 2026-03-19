//! # Functional Rust Grand Tour
//! Summary of functional programming patterns in Rust.

// Re-exports and summaries of patterns covered

/// Option: Maybe monad
pub fn option_demo() -> Option<i32> {
    Some(42).map(|x| x * 2)
}

/// Result: Error monad
pub fn result_demo() -> Result<i32, &'static str> {
    Ok(10).and_then(|x| Ok(x + 1))
}

/// Iterator: List operations
pub fn iter_demo() -> i32 {
    (1..=10).filter(|x| x % 2 == 0).sum()
}

/// Pattern matching
pub fn match_demo(x: Option<i32>) -> &'static str {
    match x {
        Some(n) if n > 0 => "positive",
        Some(_) => "other",
        None => "none",
    }
}

/// Higher-order functions
pub fn hof_demo<F: Fn(i32) -> i32>(f: F) -> i32 {
    f(10)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_all() {
        assert_eq!(option_demo(), Some(84));
        assert_eq!(result_demo(), Ok(11));
        assert_eq!(iter_demo(), 30);
        assert_eq!(match_demo(Some(5)), "positive");
        assert_eq!(hof_demo(|x| x * 2), 20);
    }
}
