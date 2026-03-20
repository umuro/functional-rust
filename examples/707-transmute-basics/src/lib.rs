#![allow(clippy::all)]
//! 707 — std::mem::transmute: Reinterpreting Bytes
//!
//! Safe-alternative-first approach: the safe APIs are always listed first.
//! Transmute is used only where no stable safe alternative exists.
//!
//! Clippy correctly warns when a safe API covers the same operation (e.g.
//! `f32::to_bits` instead of `transmute::<f32, u32>`). The examples here
//! therefore focus on cases where transmute is genuinely necessary:
//!   - Array-of-T → array-of-U (no safe std API for whole-array reinterpretation)
//!   - `#[repr(C)]` struct ↔ byte array (FFI / packed-pixel patterns)

use std::mem;

// ── Safe primitives — the recommended ways ───────────────────────────────

/// IEEE-754 bit pattern of an f32 — prefer this over transmute.
pub fn f32_bits(f: f32) -> u32 {
    f.to_bits()
}
pub fn f32_from_bits(bits: u32) -> f32 {
    f32::from_bits(bits)
}

/// UTF-8 byte view of a &str — prefer this over transmute.
pub fn str_bytes(s: &str) -> &[u8] {
    s.as_bytes()
}

// ── Case 1: [f32; 4] → [u32; 4] ──────────────────────────────────────────
//
// No std API reinterprets a whole array in one call.
// The safe iterator version is shown alongside for comparison.

/// Safe version — element-by-element, no unsafe needed.
pub fn f32x4_to_bits_safe(arr: [f32; 4]) -> [u32; 4] {
    arr.map(f32::to_bits)
}

/// Transmute version — one instruction when optimised, same result.
///
/// # Safety
/// `[f32; 4]` and `[u32; 4]` have identical size (16 bytes) and alignment (4).
/// Every u32 bit pattern is valid, so no validity invariant can be broken.
pub fn f32x4_to_bits_transmute(arr: [f32; 4]) -> [u32; 4] {
    // SAFETY: [f32; 4] and [u32; 4] have the same size (4 × 4 = 16 bytes)
    // and the same alignment (4). Every bit-pattern of [u32; 4] is valid.
    unsafe { mem::transmute::<[f32; 4], [u32; 4]>(arr) }
}

/// Round-trip: [u32; 4] → [f32; 4].
///
/// # Safety
/// Every bit pattern of `[u32; 4]` corresponds to some f32 value (incl. NaN/Inf).
pub fn u32x4_to_f32x4(arr: [u32; 4]) -> [f32; 4] {
    // SAFETY: [u32; 4] and [f32; 4] have the same size and alignment.
    // Every u32 bit pattern maps to a valid (if non-finite) f32.
    unsafe { mem::transmute::<[u32; 4], [f32; 4]>(arr) }
}

// ── Case 2: #[repr(C)] struct ↔ [u8; N] ──────────────────────────────────
//
// Common in FFI / packed pixel formats. `#[repr(C)]` guarantees layout.

/// A 32-bit RGBA colour — layout-stable for FFI / pixel-buffer operations.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rgba {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

/// Safe version using field access — verbose but zero-unsafe.
pub fn rgba_to_bytes_safe(c: Rgba) -> [u8; 4] {
    [c.r, c.g, c.b, c.a]
}

/// Transmute version — `#[repr(C)]` makes this sound.
///
/// # Safety
/// `Rgba` is `#[repr(C)]` with four `u8` fields → size 4, align 1.
/// `[u8; 4]` has size 4, align 1, and every bit pattern is a valid `u8`.
pub fn rgba_to_bytes_transmute(c: Rgba) -> [u8; 4] {
    // SAFETY: Rgba is #[repr(C)] — layout is [r, g, b, a] with no padding.
    // [u8; 4] has no validity invariants, so every bit pattern is sound.
    unsafe { mem::transmute::<Rgba, [u8; 4]>(c) }
}

/// Reconstruct an Rgba from four bytes.
///
/// # Safety
/// `Rgba` is `#[repr(C)]` with four `u8` fields. Every `u8` is a valid
/// field value, so every `[u8; 4]` produces a valid `Rgba`.
pub fn bytes_to_rgba(b: [u8; 4]) -> Rgba {
    // SAFETY: Rgba is #[repr(C)] with size 4 and align 1.
    // Every byte sequence maps to a valid Rgba — all u8 values are legal.
    unsafe { mem::transmute::<[u8; 4], Rgba>(b) }
}

// ── Case 3: generic bytes-of view (raw pointer, not transmute) ────────────
//
// Transmute cannot express "borrow the bytes of an arbitrary T" because the
// output lifetime is not encoded in the types.  The idiomatic approach uses
// `std::slice::from_raw_parts` instead.

/// Return a byte view of any `Copy + Sized` value, tied to its lifetime.
///
/// # Safety
/// The returned slice borrows from `val` and must not outlive it.
/// `T` must have no padding bytes if the caller cares about the byte values
/// (padding is uninitialised and reading it is undefined behaviour).
pub fn bytes_of<T: Copy>(val: &T) -> &[u8] {
    // SAFETY: val is a valid reference, so the pointer is non-null and aligned.
    // size_of::<T>() bytes starting at that pointer are part of the value.
    // The lifetime of the returned slice is tied to val's lifetime.
    unsafe { std::slice::from_raw_parts(val as *const T as *const u8, mem::size_of::<T>()) }
}

// ─────────────────────────────────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    // ── Safe primitive round-trips (baseline) ─────────────────────────────

    #[test]
    fn test_f32_bits_round_trip() {
        for f in [0.0_f32, 1.0, -1.0, f32::MAX, std::f32::consts::PI] {
            assert_eq!(f32_from_bits(f32_bits(f)).to_bits(), f.to_bits());
        }
    }

    #[test]
    fn test_str_bytes_is_as_bytes() {
        let s = "hello 🦀";
        assert_eq!(str_bytes(s), s.as_bytes());
    }

    // ── [f32; 4] ↔ [u32; 4] ──────────────────────────────────────────────

    #[test]
    fn test_f32x4_transmute_matches_safe() {
        let arr = [0.0_f32, 1.0, -1.0, std::f32::consts::PI];
        assert_eq!(
            f32x4_to_bits_transmute(arr),
            f32x4_to_bits_safe(arr),
            "transmute and map must produce identical bits"
        );
    }

    #[test]
    fn test_f32x4_round_trip() {
        let original = [1.0_f32, 2.0, 3.0, 4.0];
        let bits = f32x4_to_bits_transmute(original);
        let recovered = u32x4_to_f32x4(bits);
        // Compare bit patterns to handle NaN correctly.
        for (a, b) in original.iter().zip(recovered.iter()) {
            assert_eq!(a.to_bits(), b.to_bits());
        }
    }

    #[test]
    fn test_f32x4_known_bit_patterns() {
        // 1.0f32 = 0x3F800000, 0.0f32 = 0x00000000
        let arr = [1.0_f32, 0.0, 1.0, 0.0];
        let bits = f32x4_to_bits_transmute(arr);
        assert_eq!(bits[0], 0x3F80_0000);
        assert_eq!(bits[1], 0x0000_0000);
    }

    // ── Rgba ↔ [u8; 4] ───────────────────────────────────────────────────

    #[test]
    fn test_rgba_transmute_matches_safe() {
        let c = Rgba {
            r: 0xDE,
            g: 0xAD,
            b: 0xBE,
            a: 0xEF,
        };
        assert_eq!(rgba_to_bytes_transmute(c), rgba_to_bytes_safe(c));
    }

    #[test]
    fn test_rgba_round_trip() {
        let original = Rgba {
            r: 255,
            g: 128,
            b: 0,
            a: 64,
        };
        let bytes = rgba_to_bytes_transmute(original);
        let recovered = bytes_to_rgba(bytes);
        assert_eq!(original, recovered);
    }

    #[test]
    fn test_rgba_byte_order() {
        let c = Rgba {
            r: 0x12,
            g: 0x34,
            b: 0x56,
            a: 0x78,
        };
        let bytes = rgba_to_bytes_transmute(c);
        // #[repr(C)] guarantees field order: r, g, b, a
        assert_eq!(bytes, [0x12, 0x34, 0x56, 0x78]);
    }

    #[test]
    fn test_rgba_zero() {
        let c = Rgba {
            r: 0,
            g: 0,
            b: 0,
            a: 0,
        };
        assert_eq!(rgba_to_bytes_transmute(c), [0, 0, 0, 0]);
        assert_eq!(bytes_to_rgba([0; 4]), c);
    }

    // ── bytes_of generic view ─────────────────────────────────────────────

    #[test]
    fn test_bytes_of_u32_length() {
        let n: u32 = 42;
        assert_eq!(bytes_of(&n).len(), 4);
    }

    #[test]
    fn test_bytes_of_rgba_matches_transmute() {
        let c = Rgba {
            r: 1,
            g: 2,
            b: 3,
            a: 4,
        };
        assert_eq!(bytes_of(&c), rgba_to_bytes_transmute(c));
    }
}
