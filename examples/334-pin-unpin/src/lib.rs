//! # Pin and Unpin
//!
//! `Pin<P>` prevents a value from moving in memory — required for
//! self-referential futures that async state machines create.

use std::marker::PhantomPinned;
use std::pin::Pin;

/// A self-referential struct that contains a pointer to its own field.
/// Moving this struct would invalidate the internal pointer.
pub struct SelfRef {
    data: String,
    ptr: *const u8,
    _pin: PhantomPinned, // Removes the auto-Unpin impl
}

impl SelfRef {
    /// Create a new pinned SelfRef. The pointer is set after pinning.
    pub fn new(s: &str) -> Pin<Box<Self>> {
        let mut boxed = Box::new(Self {
            data: s.to_string(),
            ptr: std::ptr::null(),
            _pin: PhantomPinned,
        });

        // Set the self-referential pointer
        boxed.ptr = boxed.data.as_ptr();

        // Safety: we never move the value out of the Box after this
        unsafe { Pin::new_unchecked(boxed) }
    }

    /// Get the data string.
    pub fn get_data(&self) -> &str {
        &self.data
    }

    /// Check if the internal pointer is still valid (points to data).
    pub fn ptr_valid(&self) -> bool {
        self.ptr == self.data.as_ptr()
    }
}

/// A normal struct that can be freely moved (implements Unpin).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Normal {
    pub x: i32,
}

impl Normal {
    pub fn new(x: i32) -> Self {
        Self { x }
    }
}

/// Demonstrates working with pinned values.
pub fn pin_demo() -> (String, bool) {
    let sr = SelfRef::new("hello");
    let data = sr.as_ref().get_data().to_string();
    let valid = sr.as_ref().ptr_valid();
    (data, valid)
}

/// Pin a normal value (Unpin types can be unpinned).
pub fn pin_unpin_demo() -> i32 {
    let mut n = Normal::new(42);
    let pinned = Pin::new(&mut n);

    // Because Normal: Unpin, we can get the inner value back
    let inner = Pin::into_inner(pinned);
    inner.x
}

/// Check if a type implements Unpin at compile time.
pub fn assert_unpin<T: Unpin>() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_self_ref_data() {
        let sr = SelfRef::new("hello");
        assert_eq!(sr.as_ref().get_data(), "hello");
    }

    #[test]
    fn test_self_ref_ptr_valid() {
        let sr = SelfRef::new("test");
        assert!(sr.as_ref().ptr_valid());
    }

    #[test]
    fn test_normal_is_unpin() {
        assert_unpin::<Normal>();
        assert_unpin::<i32>();
        assert_unpin::<String>();
        assert_unpin::<Vec<u8>>();
    }

    #[test]
    fn test_pin_into_inner_for_unpin() {
        let mut n = Normal::new(100);
        let p = Pin::new(&mut n);
        let inner = Pin::into_inner(p);
        assert_eq!(inner.x, 100);
    }

    #[test]
    fn test_pinned_value_access() {
        let mut v = 99i32;
        let pv = Pin::new(&mut v);
        assert_eq!(*pv, 99);
    }

    #[test]
    fn test_pin_demo() {
        let (data, valid) = pin_demo();
        assert_eq!(data, "hello");
        assert!(valid);
    }
}
