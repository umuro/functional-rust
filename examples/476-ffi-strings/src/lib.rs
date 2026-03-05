//! # FFI Strings — String Handling Across Languages
//!
//! Converting between Rust strings and C strings.

use std::ffi::{CStr, CString, OsStr, OsString};
use std::os::raw::c_char;

/// CString: Owned null-terminated string for passing to C
pub fn create_c_string(s: &str) -> Result<CString, std::ffi::NulError> {
    CString::new(s)
}

/// Get raw pointer from CString
pub fn c_string_to_ptr(s: &CString) -> *const c_char {
    s.as_ptr()
}

/// CStr: Borrowed null-terminated string from C
///
/// # Safety
/// ptr must be a valid null-terminated string
pub unsafe fn borrow_c_string(ptr: *const c_char) -> Option<&'static CStr> {
    if ptr.is_null() {
        None
    } else {
        Some(CStr::from_ptr(ptr))
    }
}

/// Convert CStr to Rust &str
pub fn cstr_to_str(cstr: &CStr) -> Result<&str, std::str::Utf8Error> {
    cstr.to_str()
}

/// Convert CStr to Rust String (lossy)
pub fn cstr_to_string_lossy(cstr: &CStr) -> String {
    cstr.to_string_lossy().into_owned()
}

/// Safe FFI string wrapper
pub struct FfiString {
    inner: CString,
}

impl FfiString {
    pub fn new(s: &str) -> Result<Self, std::ffi::NulError> {
        Ok(Self {
            inner: CString::new(s)?,
        })
    }

    pub fn as_ptr(&self) -> *const c_char {
        self.inner.as_ptr()
    }

    pub fn to_str(&self) -> &str {
        self.inner.to_str().unwrap()
    }
}

/// String buffer for receiving C strings
pub struct StringBuffer {
    buffer: Vec<c_char>,
}

impl StringBuffer {
    pub fn new(capacity: usize) -> Self {
        Self {
            buffer: vec![0; capacity],
        }
    }

    pub fn as_mut_ptr(&mut self) -> *mut c_char {
        self.buffer.as_mut_ptr()
    }

    pub fn capacity(&self) -> usize {
        self.buffer.len()
    }

    /// Extract the string written by C code
    pub fn to_string(&self) -> Option<String> {
        // Find null terminator
        let null_pos = self.buffer.iter().position(|&c| c == 0)?;

        // Convert bytes
        let bytes: Vec<u8> = self.buffer[..null_pos]
            .iter()
            .map(|&c| c as u8)
            .collect();

        String::from_utf8(bytes).ok()
    }
}

/// Wide string handling (UTF-16)
#[cfg(windows)]
pub mod wide {
    pub fn to_wide(s: &str) -> Vec<u16> {
        s.encode_utf16().chain(std::iter::once(0)).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_c_string() {
        let s = create_c_string("hello").unwrap();
        assert_eq!(s.to_str().unwrap(), "hello");
    }

    #[test]
    fn test_c_string_null_rejected() {
        let result = create_c_string("hel\0lo");
        assert!(result.is_err());
    }

    #[test]
    fn test_cstr_roundtrip() {
        let original = "hello world";
        let c_string = CString::new(original).unwrap();
        let c_str = c_string.as_c_str();
        let back = cstr_to_str(c_str).unwrap();
        assert_eq!(original, back);
    }

    #[test]
    fn test_lossy_conversion() {
        let c_string = CString::new("hello").unwrap();
        let result = cstr_to_string_lossy(&c_string);
        assert_eq!(result, "hello");
    }

    #[test]
    fn test_ffi_string() {
        let ffi = FfiString::new("test").unwrap();
        assert!(!ffi.as_ptr().is_null());
        assert_eq!(ffi.to_str(), "test");
    }

    #[test]
    fn test_string_buffer() {
        let mut buf = StringBuffer::new(32);

        // Simulate C writing to buffer
        unsafe {
            let ptr = buf.as_mut_ptr() as *mut u8;
            *ptr = b'H';
            *ptr.add(1) = b'i';
            *ptr.add(2) = 0; // null terminator
        }

        assert_eq!(buf.to_string(), Some("Hi".to_string()));
    }

    #[test]
    fn test_cstring_bytes() {
        let s = CString::new("abc").unwrap();
        assert_eq!(s.as_bytes(), b"abc");
        assert_eq!(s.as_bytes_with_nul(), b"abc\0");
    }

    #[test]
    fn test_cstr_from_bytes() {
        let bytes = b"hello\0";
        let cstr = CStr::from_bytes_with_nul(bytes).unwrap();
        assert_eq!(cstr.to_str().unwrap(), "hello");
    }
}
