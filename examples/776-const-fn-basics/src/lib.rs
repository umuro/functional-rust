#![allow(clippy::all)]
//! # Const Fn Basics
//!
//! Functions that can be evaluated at compile time.

/// Factorial computed at compile time
pub const fn factorial(n: u64) -> u64 {
    if n <= 1 {
        1
    } else {
        n * factorial(n - 1)
    }
}

/// GCD at compile time
pub const fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

/// Power at compile time
pub const fn pow(base: u64, exp: u32) -> u64 {
    if exp == 0 {
        1
    } else if exp.is_multiple_of(2) {
        let half = pow(base, exp / 2);
        half * half
    } else {
        base * pow(base, exp - 1)
    }
}

/// Is prime check at compile time
pub const fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 {
        return true;
    }
    if n.is_multiple_of(2) {
        return false;
    }
    let mut i = 3;
    while i * i <= n {
        if n.is_multiple_of(i) {
            return false;
        }
        i += 2;
    }
    true
}

/// String length at compile time
pub const fn const_str_len(s: &str) -> usize {
    s.len()
}

/// Check if byte is ASCII digit
pub const fn is_ascii_digit(b: u8) -> bool {
    b >= b'0' && b <= b'9'
}

/// Count digits in number
pub const fn digit_count(mut n: u64) -> usize {
    if n == 0 {
        return 1;
    }
    let mut count = 0;
    while n > 0 {
        count += 1;
        n /= 10;
    }
    count
}

/// Sum of digits
pub const fn digit_sum(mut n: u64) -> u64 {
    let mut sum = 0;
    while n > 0 {
        sum += n % 10;
        n /= 10;
    }
    sum
}

/// Reverse a number
pub const fn reverse_number(mut n: u64) -> u64 {
    let mut reversed = 0;
    while n > 0 {
        reversed = reversed * 10 + n % 10;
        n /= 10;
    }
    reversed
}

// Compile-time constants using const fn
pub const FACTORIAL_10: u64 = factorial(10);
pub const GCD_48_18: u64 = gcd(48, 18);
pub const TWO_TO_10: u64 = pow(2, 10);
pub const IS_17_PRIME: bool = is_prime(17);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factorial() {
        assert_eq!(factorial(0), 1);
        assert_eq!(factorial(1), 1);
        assert_eq!(factorial(5), 120);
        assert_eq!(FACTORIAL_10, 3628800);
    }

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(48, 18), 6);
        assert_eq!(gcd(17, 13), 1);
        assert_eq!(GCD_48_18, 6);
    }

    #[test]
    fn test_pow() {
        assert_eq!(pow(2, 0), 1);
        assert_eq!(pow(2, 10), 1024);
        assert_eq!(pow(3, 4), 81);
        assert_eq!(TWO_TO_10, 1024);
    }

    #[test]
    fn test_is_prime() {
        assert!(!is_prime(0));
        assert!(!is_prime(1));
        assert!(is_prime(2));
        assert!(is_prime(17));
        assert!(!is_prime(18));
        assert!(IS_17_PRIME);
    }

    #[test]
    fn test_digit_count() {
        assert_eq!(digit_count(0), 1);
        assert_eq!(digit_count(5), 1);
        assert_eq!(digit_count(123), 3);
        assert_eq!(digit_count(1000), 4);
    }

    #[test]
    fn test_digit_sum() {
        assert_eq!(digit_sum(123), 6);
        assert_eq!(digit_sum(999), 27);
    }

    #[test]
    fn test_reverse_number() {
        assert_eq!(reverse_number(123), 321);
        assert_eq!(reverse_number(100), 1);
    }

    // Compile-time test via const
    const _: () = {
        assert!(factorial(5) == 120);
        assert!(gcd(12, 8) == 4);
    };
}
