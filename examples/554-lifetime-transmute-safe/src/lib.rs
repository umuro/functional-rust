#![allow(clippy::all)]
//! Safe Transmute Patterns
//!
//! Converting between types with same representation.

/// Safe byte slice to str (with validation).
pub fn bytes_to_str(bytes: &[u8]) -> Result<&str, std::str::Utf8Error> {
    std::str::from_utf8(bytes)
}

/// Safe conversion via From/Into.
pub fn convert_safe() {
    let n: u32 = 42;
    let bytes = n.to_le_bytes();
    let back = u32::from_le_bytes(bytes);
    assert_eq!(n, back);
}

/// Transmute alternative: use bytemuck-style.
pub fn as_bytes<T: Copy>(val: &T) -> &[u8] {
    unsafe { std::slice::from_raw_parts(val as *const T as *const u8, std::mem::size_of::<T>()) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bytes_to_str() {
        let bytes = b"hello";
        assert_eq!(bytes_to_str(bytes).unwrap(), "hello");
    }

    #[test]
    fn test_bytes_to_str_invalid() {
        let bytes = &[0xFF, 0xFE];
        assert!(bytes_to_str(bytes).is_err());
    }

    #[test]
    fn test_convert_safe() {
        convert_safe(); // just verify it runs
    }

    #[test]
    fn test_as_bytes() {
        let n: u32 = 0x12345678;
        let bytes = as_bytes(&n);
        assert_eq!(bytes.len(), 4);
    }
}
