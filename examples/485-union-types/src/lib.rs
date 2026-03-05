//! # Union Types — C-style Unions
//!
//! Rust unions for FFI compatibility and low-level optimization.

/// C-compatible union
#[repr(C)]
pub union Number {
    pub int_val: i64,
    pub float_val: f64,
    pub bytes: [u8; 8],
}

impl Number {
    pub fn from_int(v: i64) -> Self {
        Number { int_val: v }
    }

    pub fn from_float(v: f64) -> Self {
        Number { float_val: v }
    }

    /// Access requires unsafe - we don't know which variant is active
    pub unsafe fn as_int(&self) -> i64 {
        self.int_val
    }

    pub unsafe fn as_float(&self) -> f64 {
        self.float_val
    }

    pub unsafe fn as_bytes(&self) -> [u8; 8] {
        self.bytes
    }
}

/// Tagged union pattern
#[repr(C)]
pub struct TaggedNumber {
    tag: NumberTag,
    value: Number,
}

#[repr(u8)]
#[derive(Clone, Copy, PartialEq)]
pub enum NumberTag {
    Int = 0,
    Float = 1,
}

impl TaggedNumber {
    pub fn new_int(v: i64) -> Self {
        Self {
            tag: NumberTag::Int,
            value: Number::from_int(v),
        }
    }

    pub fn new_float(v: f64) -> Self {
        Self {
            tag: NumberTag::Float,
            value: Number::from_float(v),
        }
    }

    pub fn get_int(&self) -> Option<i64> {
        if self.tag == NumberTag::Int {
            Some(unsafe { self.value.as_int() })
        } else {
            None
        }
    }

    pub fn get_float(&self) -> Option<f64> {
        if self.tag == NumberTag::Float {
            Some(unsafe { self.value.as_float() })
        } else {
            None
        }
    }
}

/// ManuallyDrop in unions
pub union MaybeUninit<T: Copy> {
    uninit: (),
    value: T,
}

impl<T: Copy> MaybeUninit<T> {
    pub const fn uninit() -> Self {
        MaybeUninit { uninit: () }
    }

    pub fn write(&mut self, val: T) {
        self.value = val;
    }

    pub unsafe fn assume_init(&self) -> T {
        self.value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_union() {
        let n = Number::from_int(42);
        unsafe {
            assert_eq!(n.as_int(), 42);
        }

        let f = Number::from_float(3.14);
        unsafe {
            assert!((f.as_float() - 3.14).abs() < 1e-10);
        }
    }

    #[test]
    fn test_view_as_bytes() {
        let n = Number::from_int(0x0102030405060708i64);
        unsafe {
            let bytes = n.as_bytes();
            // Little endian
            assert_eq!(bytes[0], 0x08);
            assert_eq!(bytes[7], 0x01);
        }
    }

    #[test]
    fn test_tagged_union() {
        let i = TaggedNumber::new_int(42);
        assert_eq!(i.get_int(), Some(42));
        assert_eq!(i.get_float(), None);

        let f = TaggedNumber::new_float(2.5);
        assert_eq!(f.get_float(), Some(2.5));
        assert_eq!(f.get_int(), None);
    }

    #[test]
    fn test_maybe_uninit() {
        let mut m: MaybeUninit<i32> = MaybeUninit::uninit();
        m.write(42);
        unsafe {
            assert_eq!(m.assume_init(), 42);
        }
    }
}
