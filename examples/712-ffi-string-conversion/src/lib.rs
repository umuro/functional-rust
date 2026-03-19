#![allow(clippy::all)]
//! 712 — String / CString / CStr Conversion for FFI
//!
//! Rust strings (`&str`, `String`) are UTF-8 and length-prefixed.
//! C strings (`char*`) are null-terminated and encoding-agnostic.
//! `CString` and `CStr` bridge these two worlds without leaking memory
//! or invoking undefined behaviour.
//!
//! Two directions:
//!   Rust → C: `CString::new(s)` — heap-allocated, null-terminated, owned.
//!   C → Rust: `CStr::from_ptr(ptr)` — borrows the C buffer, zero-copy.

use std::ffi::{CStr, CString, NulError};
use std::os::raw::c_char;

// ── Rust → C direction ────────────────────────────────────────────────────

/// Convert a Rust `&str` into a heap-allocated, null-terminated `CString`.
///
/// Returns `Err` if `s` contains an interior NUL byte, which would silently
/// truncate the string from C's perspective.
pub fn rust_to_cstring(s: &str) -> Result<CString, NulError> {
    CString::new(s)
}

/// Get the raw `*const c_char` pointer from a `CString` for passing to C.
///
/// The pointer is valid only as long as the `CString` is alive — store the
/// `CString` in a local variable for the duration of the FFI call.
///
/// # Safety
/// The returned pointer must not outlive `cs`.
pub fn cstring_as_ptr(cs: &CString) -> *const c_char {
    cs.as_ptr()
}

// ── C → Rust direction ────────────────────────────────────────────────────

/// Borrow a null-terminated C string as a `&CStr`.
///
/// # Safety
/// `ptr` must be non-null and point to a valid, null-terminated C string
/// for at least the lifetime of the returned `&CStr`.
pub unsafe fn ptr_to_cstr<'a>(ptr: *const c_char) -> &'a CStr {
    // SAFETY: caller guarantees ptr is non-null and null-terminated.
    CStr::from_ptr(ptr)
}

/// Convert a `&CStr` to a Rust `&str`, returning an error if the bytes are
/// not valid UTF-8.
pub fn cstr_to_str(cs: &CStr) -> Result<&str, std::str::Utf8Error> {
    cs.to_str()
}

/// Full round-trip: C pointer → owned `String`, validating UTF-8.
///
/// # Safety
/// `ptr` must be non-null and point to a valid, null-terminated C string.
pub unsafe fn ptr_to_string(ptr: *const c_char) -> Result<String, std::str::Utf8Error> {
    // SAFETY: propagated from caller guarantee.
    let cstr = CStr::from_ptr(ptr);
    cstr.to_str().map(str::to_owned)
}

// ── Simulated C functions (self-contained, no external linker needed) ─────

/// Simulated C: returns a static greeting string (null-terminated C literal).
///
/// The `c"..."` literal (Rust 1.77+) is placed in `.rodata`; `.as_ptr()` yields
/// a `*const c_char` valid for the process lifetime.
#[no_mangle]
pub extern "C" fn c_greeting() -> *const c_char {
    c"Hello from the C side!".as_ptr()
}

/// Simulated C: compute the length of a null-terminated string.
///
/// # Safety
/// `s` must be non-null and null-terminated.
#[no_mangle]
pub unsafe extern "C" fn c_strlen(s: *const c_char) -> usize {
    if s.is_null() {
        return 0;
    }
    // SAFETY: caller guarantees s is non-null and null-terminated.
    CStr::from_ptr(s).to_bytes().len()
}

// ── Safe wrapper over the simulated C functions ───────────────────────────

/// Retrieve the greeting from the simulated C library as an owned `String`.
pub fn get_greeting() -> String {
    let ptr = c_greeting();
    // SAFETY: c_greeting() returns a pointer to a 'static null-terminated
    // byte literal. It is non-null and valid for the process lifetime.
    unsafe { CStr::from_ptr(ptr).to_string_lossy().into_owned() }
}

/// Compute the byte length of a Rust string via the simulated C strlen.
pub fn string_len_via_c(s: &str) -> Result<usize, NulError> {
    let cs = CString::new(s)?;
    // SAFETY: cs is alive for the duration of this call; c_strlen only reads
    // until the null terminator.
    Ok(unsafe { c_strlen(cs.as_ptr()) })
}

// ── Tests ─────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // ── rust_to_cstring ────────────────────────────────────────────────────

    #[test]
    fn test_rust_to_cstring_happy_path() {
        let cs = rust_to_cstring("hello").unwrap();
        // CStr should compare equal to the original bytes.
        assert_eq!(cs.to_str().unwrap(), "hello");
    }

    #[test]
    fn test_rust_to_cstring_interior_nul_is_error() {
        // A NUL byte inside the string must produce an error, not silent truncation.
        assert!(rust_to_cstring("hel\0lo").is_err());
    }

    #[test]
    fn test_rust_to_cstring_empty_string() {
        let cs = rust_to_cstring("").unwrap();
        assert_eq!(cs.to_str().unwrap(), "");
        // Even an empty CString is null-terminated: length in bytes == 1 (the NUL).
        assert_eq!(cs.as_bytes_with_nul().len(), 1);
    }

    #[test]
    fn test_rust_to_cstring_unicode() {
        // UTF-8 content survives the round-trip as long as there's no interior NUL.
        let cs = rust_to_cstring("こんにちは").unwrap();
        assert_eq!(cs.to_str().unwrap(), "こんにちは");
    }

    // ── ptr_to_cstr / ptr_to_string ───────────────────────────────────────

    #[test]
    fn test_ptr_to_cstr_from_static_literal() {
        let ptr = b"static\0".as_ptr() as *const c_char;
        // SAFETY: ptr points to a NUL-terminated byte literal with 'static lifetime.
        let s = unsafe { ptr_to_cstr(ptr) };
        assert_eq!(s.to_str().unwrap(), "static");
    }

    #[test]
    fn test_ptr_to_string_round_trip() {
        let original = "round-trip";
        let cs = CString::new(original).unwrap();
        // SAFETY: cs is alive for the duration of this block.
        let recovered = unsafe { ptr_to_string(cs.as_ptr()) }.unwrap();
        assert_eq!(recovered, original);
    }

    // ── cstr_to_str UTF-8 validation ─────────────────────────────────────

    #[test]
    fn test_cstr_to_str_invalid_utf8_returns_error() {
        // 0xFF is not valid UTF-8.
        let bytes = b"\xff\0";
        // SAFETY: bytes is null-terminated.
        let cs = unsafe { CStr::from_bytes_with_nul_unchecked(bytes) };
        assert!(cstr_to_str(cs).is_err());
    }

    // ── simulated C functions ─────────────────────────────────────────────

    #[test]
    fn test_c_greeting_returns_valid_string() {
        let greeting = get_greeting();
        assert_eq!(greeting, "Hello from the C side!");
    }

    #[test]
    fn test_c_strlen_empty() {
        assert_eq!(string_len_via_c("").unwrap(), 0);
    }

    #[test]
    fn test_c_strlen_ascii() {
        assert_eq!(string_len_via_c("hello").unwrap(), 5);
    }

    #[test]
    fn test_c_strlen_null_pointer_returns_zero() {
        // Direct call with null — safe wrapper is not involved here.
        // SAFETY: c_strlen explicitly checks for null before dereferencing.
        assert_eq!(unsafe { c_strlen(std::ptr::null()) }, 0);
    }

    // ── cstring_as_ptr lifetime discipline ────────────────────────────────

    #[test]
    fn test_cstring_as_ptr_is_null_terminated() {
        let cs = CString::new("test").unwrap();
        let ptr = cstring_as_ptr(&cs);
        // SAFETY: cs is alive; ptr is null-terminated by CString invariant.
        let back = unsafe { CStr::from_ptr(ptr) };
        assert_eq!(back.to_str().unwrap(), "test");
    }
}
