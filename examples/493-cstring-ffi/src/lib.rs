#![allow(clippy::all)]
// 493. CString and CStr for FFI

use std::ffi::CString;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() {
        assert!(CString::new("hello").is_ok());
    }
    #[test]
    fn test_interior_null() {
        assert!(CString::new("hel\0lo").is_err());
    }
    #[test]
    fn test_roundtrip() {
        let c = CString::new("hi").unwrap();
        assert_eq!(c.to_str().unwrap(), "hi");
    }
    #[test]
    fn test_null_bytes() {
        let c = CString::new("hi").unwrap();
        let b = c.as_bytes_with_nul();
        assert_eq!(b.last(), Some(&0u8));
    }
}
