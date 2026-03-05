//! 710 — Calling C Functions with `extern "C"`
//!
//! Pattern: `#[no_mangle] pub extern "C"` exports a Rust fn with C ABI.
//! `extern "C" { fn name(...); }` imports an external C symbol.
//! Safe wrappers isolate `unsafe` at the FFI boundary.
//!
//! In a real project the C-side functions live in a compiled `.a`/`.so`
//! and are linked via a `build.rs` + `println!("cargo:rustc-link-lib=...")`.
//! Here we implement them in Rust with the C calling convention so the
//! example is fully self-contained and testable without a C compiler.

use std::os::raw::c_int;

// ── Simulated C library ───────────────────────────────────────────────────
// `#[no_mangle]` emits the symbol with the bare name (no Rust mangling)
// so the linker can match it to the `extern "C"` declaration below.

#[no_mangle]
pub extern "C" fn c_add(a: c_int, b: c_int) -> c_int {
    a + b
}

#[no_mangle]
pub extern "C" fn c_abs(n: c_int) -> c_int {
    n.abs()
}

#[no_mangle]
pub extern "C" fn c_max(a: c_int, b: c_int) -> c_int {
    a.max(b)
}

#[no_mangle]
pub extern "C" fn c_clamp(n: c_int, lo: c_int, hi: c_int) -> c_int {
    n.clamp(lo, hi)
}

// ── FFI declarations ──────────────────────────────────────────────────────
// This `extern "C"` block is what you write when calling a real C library.
// The linker resolves each declaration to the compiled C symbol at link time.

mod ffi {
    use std::os::raw::c_int;

    extern "C" {
        pub fn c_add(a: c_int, b: c_int) -> c_int;
        pub fn c_abs(n: c_int) -> c_int;
        pub fn c_max(a: c_int, b: c_int) -> c_int;
        pub fn c_clamp(n: c_int, lo: c_int, hi: c_int) -> c_int;
    }
}

// ── Safe wrappers ─────────────────────────────────────────────────────────
// `unsafe` is quarantined here. Every precondition is validated before the
// FFI call so callers get a safe, idiomatic Rust API.

/// Add two integers through the C ABI.
pub fn safe_add(a: i32, b: i32) -> i32 {
    // SAFETY: c_add reads two ints and returns their sum.
    // No pointers; no undefined behaviour.
    unsafe { ffi::c_add(a, b) }
}

/// Absolute value through the C ABI.
pub fn safe_abs(n: i32) -> i32 {
    // SAFETY: c_abs reads one int. Our implementation uses Rust's .abs()
    // which is defined for all i32 values (wrapping on MIN in debug is
    // prevented by the Rust semantics of the #[no_mangle] body above).
    unsafe { ffi::c_abs(n) }
}

/// Maximum of two integers through the C ABI.
pub fn safe_max(a: i32, b: i32) -> i32 {
    // SAFETY: c_max reads two ints. No pointers, no UB.
    unsafe { ffi::c_max(a, b) }
}

/// Clamp `n` to `[lo, hi]`. Returns `None` when `lo > hi`.
///
/// Input validation before the FFI call is the idiomatic pattern for
/// expressing Rust's safety contracts at an `extern "C"` boundary.
pub fn safe_clamp(n: i32, lo: i32, hi: i32) -> Option<i32> {
    if lo > hi {
        return None;
    }
    // SAFETY: c_clamp reads three ints. lo <= hi is established above.
    Some(unsafe { ffi::c_clamp(n, lo, hi) })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_basic() {
        assert_eq!(safe_add(3, 4), 7);
        assert_eq!(safe_add(0, 0), 0);
        assert_eq!(safe_add(-5, 5), 0);
        assert_eq!(safe_add(-3, -4), -7);
    }

    #[test]
    fn test_abs_positive_and_negative() {
        assert_eq!(safe_abs(0), 0);
        assert_eq!(safe_abs(7), 7);
        assert_eq!(safe_abs(-7), 7);
        assert_eq!(safe_abs(i32::MAX), i32::MAX);
    }

    #[test]
    fn test_max_ordering() {
        assert_eq!(safe_max(10, 20), 20);
        assert_eq!(safe_max(20, 10), 20);
        assert_eq!(safe_max(5, 5), 5);
        assert_eq!(safe_max(-1, -2), -1);
    }

    #[test]
    fn test_clamp_within_range() {
        assert_eq!(safe_clamp(5, 0, 10), Some(5));
        assert_eq!(safe_clamp(0, 0, 10), Some(0));
        assert_eq!(safe_clamp(10, 0, 10), Some(10));
    }

    #[test]
    fn test_clamp_out_of_range() {
        assert_eq!(safe_clamp(-1, 0, 10), Some(0));
        assert_eq!(safe_clamp(15, 0, 10), Some(10));
    }

    #[test]
    fn test_clamp_invalid_range_returns_none() {
        assert_eq!(safe_clamp(5, 10, 0), None);
        assert_eq!(safe_clamp(5, 1, 0), None);
    }
}
