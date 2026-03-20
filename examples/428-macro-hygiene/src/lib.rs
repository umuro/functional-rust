#![allow(clippy::all)]
//! Macro Hygiene
//!
//! How macros avoid name collisions.

/// Macros create fresh identifiers by default.
/// This prevents accidental shadowing.

#[macro_export]
macro_rules! hygienic_example {
    ($val:expr) => {{
        let result = $val; // 'result' is hygienic
        result * 2
    }};
}

/// Demonstrate that macro vars don't leak.
pub fn test_hygiene() -> i32 {
    let result = 10; // Outer 'result'
    let doubled = hygienic_example!(5); // Inner 'result' is separate
    result + doubled // 10 + 10 = 20
}

/// Non-hygienic when you want shared names.
#[macro_export]
macro_rules! with_counter {
    (|$c:ident| $body:block) => {{
        let mut $c = 0;
        $body
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hygienic_example() {
        let result = 100; // This 'result' is separate
        assert_eq!(hygienic_example!(5), 10);
        assert_eq!(result, 100); // Unchanged
    }

    #[test]
    fn test_hygiene_function() {
        assert_eq!(test_hygiene(), 20);
    }

    #[test]
    fn test_nested_hygiene() {
        let x = hygienic_example!(hygienic_example!(3));
        assert_eq!(x, 12); // ((3 * 2) * 2)
    }

    #[test]
    fn test_with_counter() {
        let v = with_counter!(|counter| {
            counter += 1;
            counter += 1;
            counter
        });
        assert_eq!(v, 2);
    }

    #[test]
    fn test_multiple_calls() {
        let a = hygienic_example!(1);
        let b = hygienic_example!(2);
        assert_eq!(a + b, 6);
    }
}
