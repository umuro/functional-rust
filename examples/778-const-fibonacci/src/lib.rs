//! # Const Fibonacci
//!
//! Computing Fibonacci numbers at compile time.

/// Naive recursive Fibonacci (works for small n in const)
pub const fn fib_recursive(n: u64) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => fib_recursive(n - 1) + fib_recursive(n - 2),
    }
}

/// Iterative Fibonacci (efficient for const)
pub const fn fib_iterative(n: u64) -> u64 {
    if n == 0 {
        return 0;
    }
    let mut a = 0u64;
    let mut b = 1u64;
    let mut i = 1;
    while i < n {
        let temp = a + b;
        a = b;
        b = temp;
        i += 1;
    }
    b
}

/// Matrix exponentiation Fibonacci (O(log n) at compile time)
pub const fn fib_matrix(n: u64) -> u64 {
    if n == 0 {
        return 0;
    }

    // Matrix [[1,1],[1,0]]^n
    let (mut a, mut b, mut c, mut d) = (1u64, 1u64, 1u64, 0u64);
    let (mut ra, mut rb, mut rc, mut rd) = (1u64, 0u64, 0u64, 1u64); // Identity

    let mut exp = n - 1;
    while exp > 0 {
        if exp % 2 == 1 {
            let new_ra = ra * a + rb * c;
            let new_rb = ra * b + rb * d;
            let new_rc = rc * a + rd * c;
            let new_rd = rc * b + rd * d;
            ra = new_ra;
            rb = new_rb;
            rc = new_rc;
            rd = new_rd;
        }
        let new_a = a * a + b * c;
        let new_b = a * b + b * d;
        let new_c = c * a + d * c;
        let new_d = c * b + d * d;
        a = new_a;
        b = new_b;
        c = new_c;
        d = new_d;
        exp /= 2;
    }

    ra
}

/// Check if n is a Fibonacci number
pub const fn is_fibonacci(n: u64) -> bool {
    if n == 0 {
        return true;
    }
    let mut a = 0u64;
    let mut b = 1u64;
    while b < n {
        let temp = a + b;
        a = b;
        b = temp;
    }
    b == n
}

/// Generate Fibonacci array at compile time
pub const fn fib_array<const N: usize>() -> [u64; N] {
    let mut arr = [0u64; N];
    if N > 0 {
        arr[0] = 0;
    }
    if N > 1 {
        arr[1] = 1;
    }
    let mut i = 2;
    while i < N {
        arr[i] = arr[i - 1] + arr[i - 2];
        i += 1;
    }
    arr
}

// Compile-time constants
pub const FIB_10: u64 = fib_iterative(10);
pub const FIB_20: u64 = fib_iterative(20);
pub const FIB_FIRST_20: [u64; 20] = fib_array();

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fib_recursive() {
        assert_eq!(fib_recursive(0), 0);
        assert_eq!(fib_recursive(1), 1);
        assert_eq!(fib_recursive(10), 55);
    }

    #[test]
    fn test_fib_iterative() {
        assert_eq!(fib_iterative(0), 0);
        assert_eq!(fib_iterative(1), 1);
        assert_eq!(fib_iterative(10), 55);
        assert_eq!(fib_iterative(20), 6765);
    }

    #[test]
    fn test_fib_matrix() {
        assert_eq!(fib_matrix(0), 0);
        assert_eq!(fib_matrix(1), 1);
        assert_eq!(fib_matrix(10), 55);
        assert_eq!(fib_matrix(20), 6765);
    }

    #[test]
    fn test_is_fibonacci() {
        assert!(is_fibonacci(0));
        assert!(is_fibonacci(1));
        assert!(is_fibonacci(55));
        assert!(is_fibonacci(6765));
        assert!(!is_fibonacci(4));
        assert!(!is_fibonacci(100));
    }

    #[test]
    fn test_fib_array() {
        assert_eq!(FIB_FIRST_20[10], 55);
        assert_eq!(FIB_FIRST_20[19], 4181);
    }

    #[test]
    fn test_compile_time_values() {
        assert_eq!(FIB_10, 55);
        assert_eq!(FIB_20, 6765);
    }

    // Compile-time verification
    const _: () = assert!(fib_iterative(10) == 55);
    const _: () = assert!(fib_matrix(10) == fib_iterative(10));
}
