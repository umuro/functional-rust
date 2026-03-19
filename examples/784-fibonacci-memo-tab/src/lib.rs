//! # Fibonacci: Memoization vs Tabulation
//!
//! Comparing top-down and bottom-up dynamic programming approaches.

use std::collections::HashMap;

// ═══════════════════════════════════════════════════════════════════════════════
// Naive Recursive (exponential time)
// ═══════════════════════════════════════════════════════════════════════════════

/// Naive recursive Fibonacci - O(2^n)
pub fn fib_naive(n: u64) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => fib_naive(n - 1) + fib_naive(n - 2),
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Memoization (Top-Down DP)
// ═══════════════════════════════════════════════════════════════════════════════

/// Memoized Fibonacci using HashMap
pub fn fib_memo(n: u64) -> u64 {
    fn helper(n: u64, cache: &mut HashMap<u64, u64>) -> u64 {
        if let Some(&result) = cache.get(&n) {
            return result;
        }
        let result = match n {
            0 => 0,
            1 => 1,
            _ => helper(n - 1, cache) + helper(n - 2, cache),
        };
        cache.insert(n, result);
        result
    }

    let mut cache = HashMap::new();
    helper(n, &mut cache)
}

/// Memoized Fibonacci using Vec (more efficient for dense range)
pub fn fib_memo_vec(n: usize) -> u64 {
    fn helper(n: usize, cache: &mut Vec<Option<u64>>) -> u64 {
        if let Some(result) = cache[n] {
            return result;
        }
        let result = match n {
            0 => 0,
            1 => 1,
            _ => helper(n - 1, cache) + helper(n - 2, cache),
        };
        cache[n] = Some(result);
        result
    }

    let mut cache = vec![None; n + 1];
    helper(n, &mut cache)
}

// ═══════════════════════════════════════════════════════════════════════════════
// Tabulation (Bottom-Up DP)
// ═══════════════════════════════════════════════════════════════════════════════

/// Tabulated Fibonacci - O(n) time, O(n) space
pub fn fib_tabulation(n: u64) -> u64 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }

    let mut table = vec![0u64; (n + 1) as usize];
    table[1] = 1;

    for i in 2..=n as usize {
        table[i] = table[i - 1] + table[i - 2];
    }

    table[n as usize]
}

/// Space-optimized tabulation - O(n) time, O(1) space
pub fn fib_optimized(n: u64) -> u64 {
    if n == 0 {
        return 0;
    }

    let mut a = 0u64;
    let mut b = 1u64;

    for _ in 1..n {
        let temp = a + b;
        a = b;
        b = temp;
    }

    b
}

// ═══════════════════════════════════════════════════════════════════════════════
// Matrix Exponentiation - O(log n)
// ═══════════════════════════════════════════════════════════════════════════════

/// Matrix multiplication for 2x2 matrices
fn matrix_mult(a: [[u64; 2]; 2], b: [[u64; 2]; 2]) -> [[u64; 2]; 2] {
    [
        [
            a[0][0] * b[0][0] + a[0][1] * b[1][0],
            a[0][0] * b[0][1] + a[0][1] * b[1][1],
        ],
        [
            a[1][0] * b[0][0] + a[1][1] * b[1][0],
            a[1][0] * b[0][1] + a[1][1] * b[1][1],
        ],
    ]
}

/// Matrix exponentiation Fibonacci - O(log n)
pub fn fib_matrix(n: u64) -> u64 {
    if n == 0 {
        return 0;
    }

    let mut result = [[1u64, 0], [0, 1]]; // Identity
    let mut base = [[1u64, 1], [1, 0]]; // Fibonacci matrix
    let mut exp = n - 1;

    while exp > 0 {
        if exp % 2 == 1 {
            result = matrix_mult(result, base);
        }
        base = matrix_mult(base, base);
        exp /= 2;
    }

    result[0][0]
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXPECTED: [u64; 21] = [
        0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610, 987, 1597, 2584, 4181, 6765,
    ];

    #[test]
    fn test_fib_naive() {
        for (i, &expected) in EXPECTED.iter().enumerate().take(15) {
            assert_eq!(fib_naive(i as u64), expected);
        }
    }

    #[test]
    fn test_fib_memo() {
        for (i, &expected) in EXPECTED.iter().enumerate() {
            assert_eq!(fib_memo(i as u64), expected);
        }
    }

    #[test]
    fn test_fib_memo_vec() {
        for (i, &expected) in EXPECTED.iter().enumerate() {
            assert_eq!(fib_memo_vec(i), expected);
        }
    }

    #[test]
    fn test_fib_tabulation() {
        for (i, &expected) in EXPECTED.iter().enumerate() {
            assert_eq!(fib_tabulation(i as u64), expected);
        }
    }

    #[test]
    fn test_fib_optimized() {
        for (i, &expected) in EXPECTED.iter().enumerate() {
            assert_eq!(fib_optimized(i as u64), expected);
        }
    }

    #[test]
    fn test_fib_matrix() {
        for (i, &expected) in EXPECTED.iter().enumerate() {
            assert_eq!(fib_matrix(i as u64), expected);
        }
    }

    #[test]
    fn test_large_fib() {
        // All methods should agree
        let n = 50;
        let expected = fib_optimized(n);
        assert_eq!(fib_memo(n), expected);
        assert_eq!(fib_tabulation(n), expected);
        assert_eq!(fib_matrix(n), expected);
    }
}
