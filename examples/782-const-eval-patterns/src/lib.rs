#![allow(clippy::all)]
//! # Const Eval Patterns
//!
//! Patterns for compile-time computation.

/// Compile-time string hash (FNV-1a)
pub const fn const_hash(s: &str) -> u64 {
    let bytes = s.as_bytes();
    let mut hash: u64 = 0xcbf29ce484222325;
    let mut i = 0;
    while i < bytes.len() {
        hash ^= bytes[i] as u64;
        hash = hash.wrapping_mul(0x100000001b3);
        i += 1;
    }
    hash
}

/// Compile-time max of two values
pub const fn const_max(a: usize, b: usize) -> usize {
    if a > b {
        a
    } else {
        b
    }
}

/// Compile-time min
pub const fn const_min(a: usize, b: usize) -> usize {
    if a < b {
        a
    } else {
        b
    }
}

/// Compile-time clamp
pub const fn const_clamp(val: i64, min: i64, max: i64) -> i64 {
    if val < min {
        min
    } else if val > max {
        max
    } else {
        val
    }
}

/// Log2 at compile time
pub const fn const_log2(n: usize) -> usize {
    if n == 0 {
        0
    } else {
        (usize::BITS - n.leading_zeros() - 1) as usize
    }
}

/// Next power of two at compile time
pub const fn const_next_pow2(n: usize) -> usize {
    if n == 0 {
        1
    } else {
        1 << (usize::BITS - (n - 1).leading_zeros())
    }
}

/// Count digits at compile time
pub const fn const_digit_count(mut n: u64) -> usize {
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

/// Reverse bits at compile time
pub const fn const_reverse_bits(mut n: u32) -> u32 {
    let mut result = 0u32;
    let mut i = 0;
    while i < 32 {
        result = (result << 1) | (n & 1);
        n >>= 1;
        i += 1;
    }
    result
}

/// Population count at compile time
pub const fn const_popcount(mut n: u64) -> u32 {
    let mut count = 0;
    while n != 0 {
        count += 1;
        n &= n - 1;
    }
    count
}

/// Ceiling division at compile time
pub const fn const_ceil_div(a: usize, b: usize) -> usize {
    a.div_ceil(b)
}

/// Align up to multiple at compile time
pub const fn const_align_up(n: usize, align: usize) -> usize {
    (n + align - 1) & !(align - 1)
}

// Example compile-time computed values
pub const HASH_HELLO: u64 = const_hash("hello");
pub const LOG2_256: usize = const_log2(256);
pub const NEXT_POW2_100: usize = const_next_pow2(100);
pub const DIGITS_IN_MILLION: usize = const_digit_count(1_000_000);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_const_hash() {
        // Different strings produce different hashes
        assert_ne!(const_hash("hello"), const_hash("world"));
        // Same string produces same hash
        assert_eq!(const_hash("test"), const_hash("test"));
    }

    #[test]
    fn test_const_max_min() {
        assert_eq!(const_max(3, 7), 7);
        assert_eq!(const_min(3, 7), 3);
    }

    #[test]
    fn test_const_clamp() {
        assert_eq!(const_clamp(5, 0, 10), 5);
        assert_eq!(const_clamp(-5, 0, 10), 0);
        assert_eq!(const_clamp(15, 0, 10), 10);
    }

    #[test]
    fn test_const_log2() {
        assert_eq!(const_log2(1), 0);
        assert_eq!(const_log2(8), 3);
        assert_eq!(const_log2(256), 8);
        assert_eq!(LOG2_256, 8);
    }

    #[test]
    fn test_const_next_pow2() {
        assert_eq!(const_next_pow2(1), 1);
        assert_eq!(const_next_pow2(5), 8);
        assert_eq!(const_next_pow2(100), 128);
        assert_eq!(NEXT_POW2_100, 128);
    }

    #[test]
    fn test_const_digit_count() {
        assert_eq!(const_digit_count(0), 1);
        assert_eq!(const_digit_count(123), 3);
        assert_eq!(DIGITS_IN_MILLION, 7);
    }

    #[test]
    fn test_const_popcount() {
        assert_eq!(const_popcount(0), 0);
        assert_eq!(const_popcount(0b1111), 4);
        assert_eq!(const_popcount(0b10101010), 4);
    }

    #[test]
    fn test_const_align_up() {
        assert_eq!(const_align_up(10, 8), 16);
        assert_eq!(const_align_up(16, 8), 16);
        assert_eq!(const_align_up(0, 8), 0);
    }

    // Compile-time tests
    const _: () = assert!(const_log2(256) == 8);
    const _: () = assert!(const_next_pow2(100) == 128);
}
