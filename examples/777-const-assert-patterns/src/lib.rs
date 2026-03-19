#![allow(clippy::all)]
//! # Const Assert Patterns
//!
//! Compile-time assertions and constraints.

/// Static assertion that a condition holds at compile time
#[macro_export]
macro_rules! const_assert {
    ($cond:expr) => {
        const _: () = assert!($cond);
    };
    ($cond:expr, $msg:literal) => {
        const _: () = assert!($cond, $msg);
    };
}

/// Assert two values are equal at compile time
#[macro_export]
macro_rules! const_assert_eq {
    ($a:expr, $b:expr) => {
        const _: () = assert!($a == $b);
    };
}

/// Assert type size at compile time
#[macro_export]
macro_rules! const_assert_size {
    ($t:ty, $size:expr) => {
        const _: () = assert!(std::mem::size_of::<$t>() == $size);
    };
}

// Example usage of const assertions
const_assert!(1 + 1 == 2);
const_assert_eq!(2 * 3, 6);

/// Validate configuration at compile time
pub const MIN_BUFFER_SIZE: usize = 64;
pub const MAX_BUFFER_SIZE: usize = 4096;
pub const BUFFER_SIZE: usize = 256;

const_assert!(BUFFER_SIZE >= MIN_BUFFER_SIZE);
const_assert!(BUFFER_SIZE <= MAX_BUFFER_SIZE);
const_assert!(BUFFER_SIZE.is_power_of_two());

/// Type with size constraint
#[repr(C)]
pub struct Header {
    pub magic: [u8; 4],
    pub version: u32,
    pub flags: u32,
    pub reserved: u32,
}

const_assert_size!(Header, 16);

/// Ensure alignment
const_assert!(std::mem::align_of::<Header>() == 4);

/// Validate enum discriminant size
pub enum Status {
    Ok = 0,
    Error = 1,
    Pending = 2,
}

const_assert!(std::mem::size_of::<Status>() == 1);

/// Compile-time computed constant with validation
pub const fn validated_percentage(p: u8) -> u8 {
    assert!(p <= 100, "percentage must be <= 100");
    p
}

pub const DEFAULT_PERCENTAGE: u8 = validated_percentage(75);

/// Compile-time bounds checking
pub const fn safe_index<const N: usize>(idx: usize) -> usize {
    assert!(idx < N, "index out of bounds");
    idx
}

// This would fail to compile:
// const BAD_INDEX: usize = safe_index::<5>(10);

pub const GOOD_INDEX: usize = safe_index::<5>(3);

/// Non-zero type at compile time
pub const fn non_zero(n: u64) -> u64 {
    assert!(n != 0, "value must be non-zero");
    n
}

pub const DIVISOR: u64 = non_zero(42);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_buffer_size_valid() {
        assert!(BUFFER_SIZE >= MIN_BUFFER_SIZE);
        assert!(BUFFER_SIZE <= MAX_BUFFER_SIZE);
    }

    #[test]
    fn test_header_size() {
        assert_eq!(std::mem::size_of::<Header>(), 16);
    }

    #[test]
    fn test_default_percentage() {
        assert_eq!(DEFAULT_PERCENTAGE, 75);
    }

    #[test]
    fn test_good_index() {
        assert_eq!(GOOD_INDEX, 3);
    }

    #[test]
    fn test_divisor() {
        assert_eq!(DIVISOR, 42);
    }

    // Compile-time tests
    const_assert!(MIN_BUFFER_SIZE < MAX_BUFFER_SIZE);
    const_assert!(DIVISOR > 0);
}
