//! Test Helper Macros
//!
//! Macros that simplify testing.

/// Assert approximately equal for floats.
#[macro_export]
macro_rules! assert_approx {
    ($left:expr, $right:expr) => {
        assert_approx!($left, $right, 1e-6)
    };
    ($left:expr, $right:expr, $epsilon:expr) => {
        let left = $left;
        let right = $right;
        let diff = (left - right).abs();
        assert!(
            diff < $epsilon,
            "assertion failed: `{} ≈ {}` (diff: {})",
            left, right, diff
        );
    };
}

/// Assert that expression panics.
#[macro_export]
macro_rules! assert_panics {
    ($body:expr) => {{
        let result = std::panic::catch_unwind(|| $body);
        assert!(result.is_err(), "Expected panic but none occurred");
    }};
}

/// Test multiple inputs.
#[macro_export]
macro_rules! test_cases {
    ($func:expr, $($input:expr => $expected:expr),+ $(,)?) => {
        $(assert_eq!($func($input), $expected);)+
    };
}

pub fn double(x: i32) -> i32 { x * 2 }
pub fn square(x: i32) -> i32 { x * x }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_approx_equal() {
        assert_approx!(1.0, 1.0000001);
    }

    #[test]
    #[should_panic]
    fn test_approx_not_equal() {
        assert_approx!(1.0, 2.0);
    }

    #[test]
    fn test_cases_double() {
        test_cases!(double,
            0 => 0,
            1 => 2,
            5 => 10
        );
    }

    #[test]
    fn test_cases_square() {
        test_cases!(square,
            0 => 0,
            3 => 9,
            4 => 16
        );
    }

    #[test]
    fn test_panics_macro() {
        assert_panics!(panic!("test"));
    }
}
