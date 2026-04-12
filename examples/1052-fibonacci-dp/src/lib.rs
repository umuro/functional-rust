#![allow(dead_code)]
#![allow(clippy::all)]
// 1052: Fibonacci Bottom-Up DP with O(1) Space

// Approach 1: Vec-based bottom-up DP
fn fib_vec(n: usize) -> u64 {
    if n <= 1 {
        return n as u64;
    }
    let mut dp = vec![0u64; n + 1];
    dp[1] = 1;
    for i in 2..=n {
        dp[i] = dp[i - 1] + dp[i - 2];
    }
    dp[n]
}

// Approach 2: O(1) space — two variables
fn fib_const(n: usize) -> u64 {
    if n <= 1 {
        return n as u64;
    }
    let (mut a, mut b) = (0u64, 1u64);
    for _ in 2..=n {
        let t = a + b;
        a = b;
        b = t;
    }
    b
}

// Approach 3: Iterator/fold approach
fn fib_fold(n: usize) -> u64 {
    if n <= 1 {
        return n as u64;
    }
    let (_, b) = (2..=n).fold((0u64, 1u64), |(a, b), _| (b, a + b));
    b
}

#[cfg(test)]
mod tests {
    use super::*;

    const CASES: &[(usize, u64)] = &[
        (0, 0),
        (1, 1),
        (2, 1),
        (5, 5),
        (10, 55),
        (20, 6765),
        (30, 832040),
    ];

    #[test]
    fn test_fib_vec() {
        for &(n, expected) in CASES {
            assert_eq!(fib_vec(n), expected, "fib_vec({n})");
        }
    }

    #[test]
    fn test_fib_const() {
        for &(n, expected) in CASES {
            assert_eq!(fib_const(n), expected, "fib_const({n})");
        }
    }

    #[test]
    fn test_fib_fold() {
        for &(n, expected) in CASES {
            assert_eq!(fib_fold(n), expected, "fib_fold({n})");
        }
    }
}
