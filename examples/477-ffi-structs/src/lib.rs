//! # FFI Structs — C-Compatible Data Structures
//!
//! Using #[repr(C)] for predictable memory layout.

use std::ffi::c_char;

/// C-compatible struct with repr(C)
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

/// Struct with padding (repr(C) makes padding predictable)
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Padded {
    pub a: u8,   // 1 byte
    // 7 bytes padding
    pub b: u64,  // 8 bytes
    pub c: u8,   // 1 byte
    // 7 bytes padding
}

/// Packed struct (no padding, may be unaligned)
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct Packed {
    pub a: u8,
    pub b: u64,
    pub c: u8,
}

/// Struct with explicit alignment
#[repr(C, align(16))]
#[derive(Debug, Clone, Copy)]
pub struct Aligned16 {
    pub value: u64,
}

/// C-compatible enum
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Color {
    Red = 0,
    Green = 1,
    Blue = 2,
}

/// C-compatible tagged union pattern
#[repr(C)]
pub struct TaggedUnion {
    pub tag: u32,
    pub value: Value,
}

#[repr(C)]
pub union Value {
    pub int_val: i64,
    pub float_val: f64,
    pub ptr_val: *mut std::ffi::c_void,
}

impl TaggedUnion {
    pub const TAG_INT: u32 = 0;
    pub const TAG_FLOAT: u32 = 1;
    pub const TAG_PTR: u32 = 2;

    pub fn new_int(v: i64) -> Self {
        Self {
            tag: Self::TAG_INT,
            value: Value { int_val: v },
        }
    }

    pub fn new_float(v: f64) -> Self {
        Self {
            tag: Self::TAG_FLOAT,
            value: Value { float_val: v },
        }
    }

    pub fn get_int(&self) -> Option<i64> {
        if self.tag == Self::TAG_INT {
            Some(unsafe { self.value.int_val })
        } else {
            None
        }
    }

    pub fn get_float(&self) -> Option<f64> {
        if self.tag == Self::TAG_FLOAT {
            Some(unsafe { self.value.float_val })
        } else {
            None
        }
    }
}

/// Struct with fixed-size array
#[repr(C)]
pub struct FixedBuffer {
    pub data: [u8; 256],
    pub len: usize,
}

impl FixedBuffer {
    pub fn new() -> Self {
        Self {
            data: [0; 256],
            len: 0,
        }
    }

    pub fn from_bytes(bytes: &[u8]) -> Self {
        let mut buf = Self::new();
        let copy_len = bytes.len().min(256);
        buf.data[..copy_len].copy_from_slice(&bytes[..copy_len]);
        buf.len = copy_len;
        buf
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.data[..self.len]
    }
}

impl Default for FixedBuffer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_size() {
        assert_eq!(std::mem::size_of::<Point>(), 16);
    }

    #[test]
    fn test_padded_vs_packed() {
        // Padded has padding
        assert_eq!(std::mem::size_of::<Padded>(), 24);

        // Packed has no padding
        assert_eq!(std::mem::size_of::<Packed>(), 10);
    }

    #[test]
    fn test_aligned() {
        assert_eq!(std::mem::align_of::<Aligned16>(), 16);
    }

    #[test]
    fn test_enum_values() {
        assert_eq!(Color::Red as i32, 0);
        assert_eq!(Color::Green as i32, 1);
        assert_eq!(Color::Blue as i32, 2);
    }

    #[test]
    fn test_tagged_union() {
        let int_val = TaggedUnion::new_int(42);
        assert_eq!(int_val.get_int(), Some(42));
        assert_eq!(int_val.get_float(), None);

        let float_val = TaggedUnion::new_float(3.14);
        assert_eq!(float_val.get_float(), Some(3.14));
        assert_eq!(float_val.get_int(), None);
    }

    #[test]
    fn test_fixed_buffer() {
        let buf = FixedBuffer::from_bytes(b"hello");
        assert_eq!(buf.len, 5);
        assert_eq!(buf.as_slice(), b"hello");
    }

    #[test]
    fn test_struct_layout() {
        // Fields are at predictable offsets with repr(C)
        let p = Point { x: 1.0, y: 2.0 };
        let ptr = &p as *const Point as *const u8;

        unsafe {
            let x_ptr = ptr as *const f64;
            let y_ptr = ptr.add(8) as *const f64;
            assert_eq!(*x_ptr, 1.0);
            assert_eq!(*y_ptr, 2.0);
        }
    }
}
