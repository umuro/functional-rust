#![allow(clippy::all)]
//! Attribute Macro Patterns
//!
//! What attribute macros can do.

/// Example: what #[log_calls] might add
pub fn logged_function(x: i32) -> i32 {
    // Generated: println!("Entering logged_function");
    let result = x + 1;
    // Generated: println!("Exiting logged_function");
    result
}

/// Example: what #[test_case] might expand to
pub fn factorial(n: u64) -> u64 {
    if n <= 1 {
        1
    } else {
        n * factorial(n - 1)
    }
}

/// Simulating #[timed] attribute
pub fn timed_operation() -> std::time::Duration {
    let start = std::time::Instant::now();
    std::thread::sleep(std::time::Duration::from_millis(1));
    start.elapsed()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_logged_function() {
        assert_eq!(logged_function(5), 6);
    }

    #[test]
    fn test_factorial() {
        assert_eq!(factorial(5), 120);
        assert_eq!(factorial(0), 1);
    }

    #[test]
    fn test_timed() {
        let d = timed_operation();
        assert!(d.as_millis() >= 1);
    }

    #[test]
    fn test_factorial_10() {
        assert_eq!(factorial(10), 3628800);
    }

    #[test]
    fn test_factorial_1() {
        assert_eq!(factorial(1), 1);
    }
}
