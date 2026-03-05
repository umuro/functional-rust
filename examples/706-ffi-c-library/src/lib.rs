//! # FFI C Library

// Example of calling C library functions
// extern "C" { fn strlen(s: *const i8) -> usize; }

/// Safe wrapper around C-style string length
pub fn safe_strlen(s: &str) -> usize { s.len() }

/// Demonstrates extern block concept
pub mod ffi_example {
    use std::ffi::CString;
    
    pub fn c_string_example(s: &str) -> Option<CString> {
        CString::new(s).ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_strlen() { assert_eq!(safe_strlen("hello"), 5); }
    #[test]
    fn test_cstring() {
        let cs = ffi_example::c_string_example("test").unwrap();
        assert_eq!(cs.to_bytes(), b"test");
    }
}
