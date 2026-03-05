/// 744: Unit Test Organisation — modules, helpers, AAA pattern

// ── Code under test ───────────────────────────────────────────────────────────

pub fn clamp(lo: i32, hi: i32, x: i32) -> i32 {
    x.max(lo).min(hi)
}

pub fn divide_checked(a: i64, b: i64) -> Option<i64> {
    if b == 0 { None } else { Some(a / b) }
}

pub fn is_palindrome(s: &str) -> bool {
    let bytes = s.as_bytes();
    let n = bytes.len();
    (0..n / 2).all(|i| bytes[i] == bytes[n - 1 - i])
}

pub fn fizzbuzz(n: u32) -> String {
    match (n % 3, n % 5) {
        (0, 0) => "FizzBuzz".into(),
        (0, _) => "Fizz".into(),
        (_, 0) => "Buzz".into(),
        _      => n.to_string(),
    }
}


// ── Test helpers ──────────────────────────────────────────────────────────────

#[cfg(test)]
mod helpers {
    /// Assert that two f64 values are equal within epsilon.
    pub fn assert_approx_eq(a: f64, b: f64, eps: f64) {
        assert!((a - b).abs() < eps,
            "assert_approx_eq failed: |{} - {}| = {} >= {}",
            a, b, (a - b).abs(), eps);
    }

    /// Assert that a slice is sorted ascending.
    pub fn assert_sorted<T: Ord + std::fmt::Debug>(v: &[T]) {
        for w in v.windows(2) {
            assert!(w[0] <= w[1], "not sorted: {:?}", v);
        }
    }
}

// ── Unit tests ────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use helpers::*;

    // ── clamp ─────────────────────────────────────────────────────────────────

    #[test]
    fn test_clamp_when_below_lo_returns_lo() {
        // Arrange
        let (lo, hi, x) = (0, 10, -5);
        // Act
        let result = clamp(lo, hi, x);
        // Assert
        assert_eq!(result, 0);
    }

    #[test]
    fn test_clamp_when_within_range_returns_x() {
        assert_eq!(clamp(0, 10, 5), 5);
    }

    #[test]
    fn test_clamp_when_above_hi_returns_hi() {
        assert_eq!(clamp(0, 10, 15), 10);
    }

    #[test]
    fn test_clamp_at_boundaries() {
        assert_eq!(clamp(0, 10, 0), 0);
        assert_eq!(clamp(0, 10, 10), 10);
    }

    // ── divide_checked ────────────────────────────────────────────────────────

    #[test]
    fn test_divide_checked_non_zero_returns_some() {
        assert_eq!(divide_checked(10, 3), Some(3));
    }

    #[test]
    fn test_divide_checked_by_zero_returns_none() {
        assert_eq!(divide_checked(42, 0), None);
    }

    #[test]
    fn test_divide_checked_negative_dividend() {
        assert_eq!(divide_checked(-10, 2), Some(-5));
    }

    // ── is_palindrome ─────────────────────────────────────────────────────────

    #[test]
    fn test_palindrome_empty_is_palindrome() {
        assert!(is_palindrome(""));
    }

    #[test]
    fn test_palindrome_single_char_is_palindrome() {
        assert!(is_palindrome("a"));
    }

    #[test]
    fn test_palindrome_racecar_is_palindrome() {
        assert!(is_palindrome("racecar"));
    }

    #[test]
    fn test_palindrome_hello_is_not_palindrome() {
        assert!(!is_palindrome("hello"));
    }

    // ── fizzbuzz ──────────────────────────────────────────────────────────────

    #[test]
    fn test_fizzbuzz_divisible_by_both_returns_fizzbuzz() {
        assert_eq!(fizzbuzz(15), "FizzBuzz");
    }

    #[test]
    fn test_fizzbuzz_divisible_by_3_returns_fizz() {
        assert_eq!(fizzbuzz(9), "Fizz");
    }

    #[test]
    fn test_fizzbuzz_divisible_by_5_returns_buzz() {
        assert_eq!(fizzbuzz(10), "Buzz");
    }

    #[test]
    fn test_fizzbuzz_other_returns_number() {
        assert_eq!(fizzbuzz(7), "7");
    }

    // ── helpers ───────────────────────────────────────────────────────────────

    #[test]
    fn test_assert_approx_eq_passes() {
        assert_approx_eq(0.1 + 0.2, 0.3, 1e-10);
    }

    #[test]
    fn test_assert_sorted_passes() {
        assert_sorted(&[1, 2, 3, 4, 5]);
        assert_sorted(&[1u8]);
        assert_sorted::<i32>(&[]);
    }

    // ── should_panic example ──────────────────────────────────────────────────

    #[test]
    #[should_panic]
    fn test_integer_division_by_zero_panics() {
        let zero = std::hint::black_box(0u32);
        let _ = 5u32 / zero;
    }
}
