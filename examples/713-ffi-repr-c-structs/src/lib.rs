//! 713 — #[repr(C)] Structs for FFI Interop
//!
//! `#[repr(C)]` locks a Rust struct's memory layout to C ABI rules:
//! fields appear in declaration order, padding matches C, and `sizeof`
//! agrees across both languages — enabling zero-copy struct sharing.

use std::mem;

/// C equivalent: `typedef struct { double x; double y; } Point2D;`
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point2D {
    pub x: f64,
    pub y: f64,
}

/// C equivalent: `typedef struct { Point2D origin; double width; double height; } Rect;`
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rect {
    pub origin: Point2D,
    pub width: f64,
    pub height: f64,
}

/// C equivalent: `typedef struct { uint8_t r, g, b, a; } Color;`
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

/// C equivalent: `typedef struct { float real; float imag; } Complex32;`
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Complex32 {
    pub real: f32,
    pub imag: f32,
}

// ── Pure Rust geometry functions ──────────────────────────────────────────

pub fn rect_area(r: Rect) -> f64 {
    r.width * r.height
}

pub fn rect_perimeter(r: Rect) -> f64 {
    2.0 * (r.width + r.height)
}

pub fn point_distance(a: Point2D, b: Point2D) -> f64 {
    let dx = b.x - a.x;
    let dy = b.y - a.y;
    (dx * dx + dy * dy).sqrt()
}

pub fn color_luminance(c: Color) -> f64 {
    // ITU-R BT.601 luma coefficients
    0.299 * f64::from(c.r) + 0.587 * f64::from(c.g) + 0.114 * f64::from(c.b)
}

pub fn complex_magnitude(c: Complex32) -> f32 {
    (c.real * c.real + c.imag * c.imag).sqrt()
}

// ── Simulated FFI boundary: extern "C" exports ────────────────────────────
//
// In a real FFI scenario these would be called from C.
// `#[no_mangle]` + `extern "C"` ensure the symbol name and calling convention
// match what C code expects.

#[no_mangle]
pub extern "C" fn ffi_rect_area(r: Rect) -> f64 {
    rect_area(r)
}

#[no_mangle]
pub extern "C" fn ffi_rect_perimeter(r: Rect) -> f64 {
    rect_perimeter(r)
}

#[no_mangle]
pub extern "C" fn ffi_color_luminance(c: Color) -> f64 {
    color_luminance(c)
}

// ── Layout verification helpers ───────────────────────────────────────────

/// Returns `(size_of, align_of)` for Point2D — must match C's `sizeof`/`_Alignof`.
pub fn point2d_layout() -> (usize, usize) {
    (mem::size_of::<Point2D>(), mem::align_of::<Point2D>())
}

/// Returns `(size_of, align_of)` for Rect.
pub fn rect_layout() -> (usize, usize) {
    (mem::size_of::<Rect>(), mem::align_of::<Rect>())
}

/// Returns `(size_of, align_of)` for Color.
pub fn color_layout() -> (usize, usize) {
    (mem::size_of::<Color>(), mem::align_of::<Color>())
}

// ── Zero-copy byte-slice reinterpretation (unsafe, FFI-style) ─────────────

/// Serialize a `Point2D` to its raw C-compatible bytes.
///
/// # Safety
/// `Point2D` is `#[repr(C)]` and contains only `f64` fields, so reinterpreting
/// as bytes is well-defined. The caller must not hold the reference past the
/// lifetime of `p`.
pub fn point2d_as_bytes(p: &Point2D) -> &[u8] {
    // SAFETY: Point2D is #[repr(C)] with no padding between two f64 fields.
    // The slice lifetime is tied to `p`.
    unsafe {
        std::slice::from_raw_parts(
            (p as *const Point2D).cast::<u8>(),
            mem::size_of::<Point2D>(),
        )
    }
}

/// Deserialize a `Point2D` from a raw byte slice (simulates reading from C).
///
/// Returns `None` if the slice is not exactly `size_of::<Point2D>()` bytes.
///
/// # Safety
/// The bytes must originate from a valid `Point2D` written with C ABI layout.
pub fn point2d_from_bytes(bytes: &[u8]) -> Option<Point2D> {
    if bytes.len() != mem::size_of::<Point2D>() {
        return None;
    }
    // SAFETY: We verified the length. Point2D is #[repr(C)] with f64 fields
    // (no uninitialized bytes, no invalid bit patterns for f64).
    let p = unsafe { std::ptr::read_unaligned(bytes.as_ptr().cast::<Point2D>()) };
    Some(p)
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── Layout tests ──────────────────────────────────────────────────────

    #[test]
    fn point2d_size_matches_c() {
        // C: struct { double x; double y; } → 16 bytes, align 8
        let (size, align) = point2d_layout();
        assert_eq!(size, 16, "Point2D must be 16 bytes (two f64)");
        assert_eq!(align, 8, "Point2D must be 8-byte aligned");
    }

    #[test]
    fn rect_size_matches_c() {
        // C: struct { Point2D origin; double width; double height; } → 32 bytes
        let (size, align) = rect_layout();
        assert_eq!(size, 32, "Rect must be 32 bytes");
        assert_eq!(align, 8, "Rect must be 8-byte aligned");
    }

    #[test]
    fn color_size_matches_c() {
        // C: struct { uint8_t r, g, b, a; } → 4 bytes, align 1
        let (size, align) = color_layout();
        assert_eq!(size, 4, "Color must be 4 bytes");
        assert_eq!(align, 1, "Color must be 1-byte aligned");
    }

    #[test]
    fn complex32_size_matches_c() {
        // C: struct { float real; float imag; } → 8 bytes, align 4
        assert_eq!(mem::size_of::<Complex32>(), 8);
        assert_eq!(mem::align_of::<Complex32>(), 4);
    }

    // ── Geometry tests ────────────────────────────────────────────────────

    #[test]
    fn rect_area_computes_correctly() {
        let r = Rect {
            origin: Point2D { x: 0.0, y: 0.0 },
            width: 10.0,
            height: 5.0,
        };
        assert!((rect_area(r) - 50.0).abs() < f64::EPSILON);
    }

    #[test]
    fn rect_perimeter_computes_correctly() {
        let r = Rect {
            origin: Point2D { x: 1.0, y: 2.0 },
            width: 3.0,
            height: 4.0,
        };
        assert!((rect_perimeter(r) - 14.0).abs() < f64::EPSILON);
    }

    #[test]
    fn point_distance_unit() {
        let a = Point2D { x: 0.0, y: 0.0 };
        let b = Point2D { x: 3.0, y: 4.0 };
        assert!((point_distance(a, b) - 5.0).abs() < 1e-10);
    }

    #[test]
    fn point_distance_same_point_is_zero() {
        let p = Point2D { x: 7.0, y: -3.0 };
        assert_eq!(point_distance(p, p), 0.0);
    }

    #[test]
    fn color_luminance_white() {
        let white = Color {
            r: 255,
            g: 255,
            b: 255,
            a: 255,
        };
        // 0.299*255 + 0.587*255 + 0.114*255 = 255
        assert!((color_luminance(white) - 255.0).abs() < 0.01);
    }

    #[test]
    fn color_luminance_black() {
        let black = Color {
            r: 0,
            g: 0,
            b: 0,
            a: 255,
        };
        assert_eq!(color_luminance(black), 0.0);
    }

    #[test]
    fn complex_magnitude_pythagorean() {
        let c = Complex32 {
            real: 3.0,
            imag: 4.0,
        };
        assert!((complex_magnitude(c) - 5.0).abs() < 1e-6);
    }

    // ── FFI extern "C" surface (called through safe wrapper here) ─────────

    #[test]
    fn ffi_rect_area_matches_safe() {
        let r = Rect {
            origin: Point2D { x: 0.0, y: 0.0 },
            width: 7.0,
            height: 8.0,
        };
        assert_eq!(ffi_rect_area(r), rect_area(r));
    }

    #[test]
    fn ffi_color_luminance_matches_safe() {
        let c = Color {
            r: 128,
            g: 64,
            b: 32,
            a: 255,
        };
        assert!((ffi_color_luminance(c) - color_luminance(c)).abs() < f64::EPSILON);
    }

    // ── Round-trip byte serialisation ─────────────────────────────────────

    #[test]
    fn point2d_round_trips_through_bytes() {
        let original = Point2D { x: 1.5, y: -2.75 };
        let bytes = point2d_as_bytes(&original);
        let recovered = point2d_from_bytes(bytes).expect("round-trip failed");
        assert_eq!(original, recovered);
    }

    #[test]
    fn point2d_from_bytes_rejects_wrong_length() {
        let short = [0u8; 4];
        assert!(point2d_from_bytes(&short).is_none());
    }

    // ── Field offset verification (repr(C) contract) ──────────────────────

    #[test]
    fn point2d_field_offsets() {
        // With #[repr(C)] and two f64 fields, x is at offset 0, y at offset 8.
        let p = Point2D { x: 0.0, y: 0.0 };
        let base = &p as *const Point2D as usize;
        let x_offset = &p.x as *const f64 as usize - base;
        let y_offset = &p.y as *const f64 as usize - base;
        assert_eq!(x_offset, 0);
        assert_eq!(y_offset, 8);
    }

    #[test]
    fn color_field_offsets() {
        // u8 fields with repr(C): no padding between single-byte fields.
        let c = Color {
            r: 0,
            g: 0,
            b: 0,
            a: 0,
        };
        let base = &c as *const Color as usize;
        assert_eq!(&c.r as *const u8 as usize - base, 0);
        assert_eq!(&c.g as *const u8 as usize - base, 1);
        assert_eq!(&c.b as *const u8 as usize - base, 2);
        assert_eq!(&c.a as *const u8 as usize - base, 3);
    }
}
