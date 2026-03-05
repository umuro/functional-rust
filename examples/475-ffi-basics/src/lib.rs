//! # FFI Basics — Foreign Function Interface
//!
//! Basics of calling C code from Rust and vice versa.

use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int};

// Note: In a real FFI scenario, you'd link to external C libraries
// Here we demonstrate the patterns without actual linking

/// Convert Rust string to C string
pub fn rust_to_c_string(s: &str) -> Result<CString, std::ffi::NulError> {
    CString::new(s)
}

/// Convert C string to Rust string (unsafe - trusts the pointer)
///
/// # Safety
/// - `ptr` must be a valid, null-terminated C string
/// - The memory must remain valid for the lifetime of the returned str
pub unsafe fn c_to_rust_string<'a>(ptr: *const c_char) -> &'a str {
    CStr::from_ptr(ptr).to_str().unwrap_or("")
}

/// Safe wrapper for C string conversion
pub fn c_to_rust_string_safe(ptr: *const c_char) -> Option<String> {
    if ptr.is_null() {
        return None;
    }

    unsafe {
        CStr::from_ptr(ptr)
            .to_str()
            .ok()
            .map(|s| s.to_string())
    }
}

/// FFI-safe type aliases
pub type Size = usize;
pub type CInt = c_int;

/// Example of declaring external C functions (not linked in tests)
#[cfg(not(test))]
extern "C" {
    fn strlen(s: *const c_char) -> Size;
    fn abs(n: c_int) -> c_int;
}

/// Safe wrapper for external functions
#[cfg(not(test))]
pub fn safe_strlen(s: &str) -> Option<Size> {
    let c_str = CString::new(s).ok()?;
    Some(unsafe { strlen(c_str.as_ptr()) })
}

/// Passing arrays to C
pub fn array_to_c<T>(slice: &[T]) -> (*const T, usize) {
    (slice.as_ptr(), slice.len())
}

/// Passing mutable arrays to C
pub fn array_to_c_mut<T>(slice: &mut [T]) -> (*mut T, usize) {
    (slice.as_mut_ptr(), slice.len())
}

/// Buffer allocation pattern for FFI
pub struct FfiBuffer {
    data: Vec<u8>,
}

impl FfiBuffer {
    pub fn new(size: usize) -> Self {
        Self {
            data: vec![0; size],
        }
    }

    pub fn as_ptr(&self) -> *const u8 {
        self.data.as_ptr()
    }

    pub fn as_mut_ptr(&mut self) -> *mut u8 {
        self.data.as_mut_ptr()
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.data
    }
}

/// Example callback type for FFI
pub type FfiCallback = extern "C" fn(c_int, *mut std::ffi::c_void) -> c_int;

/// Context for callbacks
pub struct CallbackContext {
    pub value: i32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rust_to_c_string() {
        let c_str = rust_to_c_string("hello").unwrap();
        assert_eq!(c_str.as_bytes(), b"hello");
    }

    #[test]
    fn test_rust_to_c_string_with_null() {
        let result = rust_to_c_string("hel\0lo");
        assert!(result.is_err());
    }

    #[test]
    fn test_c_to_rust_string_safe() {
        let c_str = CString::new("hello").unwrap();
        let result = c_to_rust_string_safe(c_str.as_ptr());
        assert_eq!(result, Some("hello".to_string()));
    }

    #[test]
    fn test_c_to_rust_string_null() {
        let result = c_to_rust_string_safe(std::ptr::null());
        assert_eq!(result, None);
    }

    #[test]
    fn test_array_to_c() {
        let arr = [1, 2, 3, 4, 5];
        let (ptr, len) = array_to_c(&arr);

        assert!(!ptr.is_null());
        assert_eq!(len, 5);

        unsafe {
            assert_eq!(*ptr, 1);
            assert_eq!(*ptr.add(4), 5);
        }
    }

    #[test]
    fn test_ffi_buffer() {
        let mut buf = FfiBuffer::new(10);
        assert_eq!(buf.len(), 10);

        // Simulate C function writing to buffer
        let ptr = buf.as_mut_ptr();
        unsafe {
            *ptr = b'H';
            *ptr.add(1) = b'i';
        }

        assert_eq!(&buf.as_slice()[..2], b"Hi");
    }

    #[test]
    fn test_cstring_bytes() {
        let c_str = CString::new("hello").unwrap();

        // as_bytes doesn't include null
        assert_eq!(c_str.as_bytes(), b"hello");

        // as_bytes_with_nul includes null
        assert_eq!(c_str.as_bytes_with_nul(), b"hello\0");
    }
}
