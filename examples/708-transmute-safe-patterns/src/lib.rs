#![allow(clippy::all)]
//! # Transmute Safe Patterns

use std::mem;

/// Safe transmute for same-size types
pub fn bytes_to_u32(bytes: [u8; 4]) -> u32 {
    u32::from_ne_bytes(bytes)
}

/// Transmute slice of bytes to slice of u32 (requires alignment)
pub fn transmute_slice_safe(bytes: &[u8]) -> Option<&[u32]> {
    if bytes.len() % 4 != 0 {
        return None;
    }
    if bytes.as_ptr() as usize % mem::align_of::<u32>() != 0 {
        return None;
    }
    Some(unsafe { std::slice::from_raw_parts(bytes.as_ptr() as *const u32, bytes.len() / 4) })
}

/// Zero-copy view (when types have same layout)
#[repr(C)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

pub fn floats_to_point(arr: [f32; 2]) -> Point {
    Point {
        x: arr[0],
        y: arr[1],
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_bytes() {
        assert_eq!(
            bytes_to_u32([1, 0, 0, 0]),
            1u32.to_ne_bytes()
                .iter()
                .fold(0u32, |a, &b| a * 256 + b as u32)
                .swap_bytes()
        );
    }
}
