#![allow(clippy::result_unit_err)]
#![allow(clippy::manual_is_multiple_of)]
#![allow(unused_variables)]
#![allow(clippy::match_like_matches)]
#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::manual_range_contains)]
#![allow(clippy::explicit_iter_loop)]
#![allow(clippy::needless_lifetimes)]
#![allow(clippy::char_lit_as_u8)]
#![allow(clippy::while_let_loop)]
#![allow(clippy::manual_strip)]
#![allow(clippy::useless_vec)]
#![allow(clippy::needless_borrow)]
#![allow(clippy::redundant_closure)]
#![allow(unused_imports)]
#![allow(dead_code)]
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
