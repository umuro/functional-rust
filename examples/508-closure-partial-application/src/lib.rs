#![allow(clippy::all)]
//! Partial Application with Closures
//!
//! Fix some arguments of a function, producing a specialized version.

/// Add two numbers.
pub fn add(x: i32, y: i32) -> i32 {
    x + y
}

/// Clamp value into [lo, hi] range.
pub fn clamp(lo: i32, hi: i32, x: i32) -> i32 {
    x.max(lo).min(hi)
}

/// Check if x is in [lo, hi] inclusive.
pub fn between(lo: i32, hi: i32, x: i32) -> bool {
    x >= lo && x <= hi
}

/// Generic partial application: fix first argument of a 2-arg function.
pub fn partial<A: Copy, B, C, F>(f: F, a: A) -> impl Fn(B) -> C
where
    F: Fn(A, B) -> C,
{
    move |b| f(a, b)
}

/// Partial with two fixed args (fix first two of a 3-arg function).
pub fn partial2<A: Copy, B: Copy, C, D, F>(f: F, a: A, b: B) -> impl Fn(C) -> D
where
    F: Fn(A, B, C) -> D,
{
    move |c| f(a, b, c)
}

/// Manual closure-based partial application approach.
pub fn create_adder(n: i32) -> impl Fn(i32) -> i32 {
    move |x| add(n, x)
}

/// Create a range checker with fixed bounds.
pub fn create_range_checker(lo: i32, hi: i32) -> impl Fn(i32) -> bool {
    move |x| between(lo, hi, x)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_manual_partial_add() {
        let add5 = |y: i32| add(5, y);
        assert_eq!(add5(10), 15);
        assert_eq!(add5(0), 5);
        assert_eq!(add5(-3), 2);
    }

    #[test]
    fn test_manual_partial_clamp() {
        let clamp_0_100 = |x| clamp(0, 100, x);
        assert_eq!(clamp_0_100(50), 50);
        assert_eq!(clamp_0_100(150), 100);
        assert_eq!(clamp_0_100(-10), 0);
    }

    #[test]
    fn test_generic_partial() {
        let mul_by_7 = partial(|x: i32, y: i32| x * y, 7);
        assert_eq!(mul_by_7(6), 42);
        assert_eq!(mul_by_7(0), 0);
    }

    #[test]
    fn test_partial2() {
        let check = partial2(between, 5, 10);
        assert!(check(7));
        assert!(check(5));
        assert!(check(10));
        assert!(!check(4));
        assert!(!check(11));
    }

    #[test]
    fn test_partial_string() {
        let prefix_checker = partial(|p: &str, s: &str| s.starts_with(p), "rust");
        assert!(prefix_checker("rustacean"));
        assert!(!prefix_checker("python"));
    }

    #[test]
    fn test_create_adder() {
        let add10 = create_adder(10);
        assert_eq!(add10(5), 15);
        assert_eq!(add10(-10), 0);
    }

    #[test]
    fn test_create_range_checker() {
        let in_teens = create_range_checker(13, 19);
        assert!(in_teens(15));
        assert!(in_teens(13));
        assert!(in_teens(19));
        assert!(!in_teens(12));
        assert!(!in_teens(20));
    }

    #[test]
    fn test_pipeline_with_partial() {
        let add5 = |x: &i32| add(5, *x);
        let double = |x: i32| x * 2;
        let in_teens = |x: &i32| between(13, 19, *x);

        let result: Vec<i32> = [1, 2, 3, 4, 5]
            .iter()
            .map(add5)
            .map(double)
            .filter(in_teens)
            .collect();

        assert_eq!(result, vec![14, 16, 18]);
    }
}
