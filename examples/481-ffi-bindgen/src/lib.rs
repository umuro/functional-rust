//! # FFI Bindgen — Automatic Bindings
//!
//! Using bindgen to generate Rust bindings from C headers.

// In a real project, bindgen would generate code like this from C headers.
// This demonstrates the patterns bindgen produces.

// Example of what bindgen generates for a C header:
// ```c
// typedef struct Point { double x; double y; } Point;
// double distance(const Point* a, const Point* b);
// ```

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

// Bindgen generates extern blocks
extern "C" {
    // This would link to C code
    // pub fn distance(a: *const Point, b: *const Point) -> f64;
}

// Safe wrapper around bindgen-generated code
pub fn safe_distance(a: &Point, b: &Point) -> f64 {
    let dx = a.x - b.x;
    let dy = a.y - b.y;
    (dx * dx + dy * dy).sqrt()
}

// Opaque type pattern (bindgen generates these for incomplete types)
#[repr(C)]
pub struct OpaqueType {
    _data: [u8; 0],
    _marker: std::marker::PhantomData<(*mut u8, std::marker::PhantomPinned)>,
}

// Function pointer types
pub type Callback = Option<unsafe extern "C" fn(i32) -> i32>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point() {
        let a = Point { x: 0.0, y: 0.0 };
        let b = Point { x: 3.0, y: 4.0 };
        assert!((safe_distance(&a, &b) - 5.0).abs() < 1e-10);
    }
}
