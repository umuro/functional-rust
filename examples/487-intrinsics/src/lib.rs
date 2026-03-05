//! # Intrinsics — Compiler Built-ins
//!
//! Using SIMD and other compiler intrinsics.

// Stable SIMD through std::simd (nightly) or std::arch

/// Portable SIMD-like operations
pub fn sum_f32_slice(data: &[f32]) -> f32 {
    data.iter().sum()
}

/// Manual unrolling pattern
pub fn sum_unrolled(data: &[f32]) -> f32 {
    let mut sum = 0.0;
    let chunks = data.chunks_exact(4);
    let remainder = chunks.remainder();

    for chunk in chunks {
        sum += chunk[0] + chunk[1] + chunk[2] + chunk[3];
    }

    for &val in remainder {
        sum += val;
    }

    sum
}

/// Leading/trailing zeros (intrinsic-backed)
pub fn leading_zeros(n: u32) -> u32 {
    n.leading_zeros()
}

pub fn trailing_zeros(n: u32) -> u32 {
    n.trailing_zeros()
}

pub fn count_ones(n: u32) -> u32 {
    n.count_ones()
}

/// Rotate operations
pub fn rotate_left(n: u32, bits: u32) -> u32 {
    n.rotate_left(bits)
}

pub fn rotate_right(n: u32, bits: u32) -> u32 {
    n.rotate_right(bits)
}

/// Byte swap
pub fn swap_bytes(n: u32) -> u32 {
    n.swap_bytes()
}

/// Saturating arithmetic
pub fn saturating_add(a: u8, b: u8) -> u8 {
    a.saturating_add(b)
}

pub fn saturating_sub(a: u8, b: u8) -> u8 {
    a.saturating_sub(b)
}

/// Checked arithmetic
pub fn checked_add(a: i32, b: i32) -> Option<i32> {
    a.checked_add(b)
}

pub fn checked_mul(a: i32, b: i32) -> Option<i32> {
    a.checked_mul(b)
}

/// Wrapping arithmetic
pub fn wrapping_add(a: u8, b: u8) -> u8 {
    a.wrapping_add(b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum() {
        let data = [1.0f32, 2.0, 3.0, 4.0, 5.0];
        assert!((sum_f32_slice(&data) - 15.0).abs() < 1e-6);
        assert!((sum_unrolled(&data) - 15.0).abs() < 1e-6);
    }

    #[test]
    fn test_bit_operations() {
        assert_eq!(leading_zeros(0b00010000), 27);
        assert_eq!(trailing_zeros(0b00010000), 4);
        assert_eq!(count_ones(0b10101010), 4);
    }

    #[test]
    fn test_rotate() {
        assert_eq!(rotate_left(0b0001, 2), 0b0100);
        assert_eq!(rotate_right(0b0100, 2), 0b0001);
    }

    #[test]
    fn test_swap_bytes() {
        assert_eq!(swap_bytes(0x12345678), 0x78563412);
    }

    #[test]
    fn test_saturating() {
        assert_eq!(saturating_add(250, 10), 255);
        assert_eq!(saturating_sub(10, 20), 0);
    }

    #[test]
    fn test_checked() {
        assert_eq!(checked_add(i32::MAX, 1), None);
        assert_eq!(checked_add(10, 20), Some(30));
    }

    #[test]
    fn test_wrapping() {
        assert_eq!(wrapping_add(250, 10), 4); // Wraps around
    }
}
