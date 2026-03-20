#![allow(clippy::all)]
//! 711 — `#[no_mangle]` Exporting Rust Functions to C
//!
//! `#[no_mangle] pub extern "C" fn` is the declaration that turns a Rust
//! function into a stable, C-ABI symbol any foreign language can call.
//!
//! Without `#[no_mangle]`, the Rust compiler encodes the full module path,
//! generic parameters, and a hash into the symbol name (name mangling), making
//! it impossible for C to find the function by a known name.
//!
//! The ABI contract at the boundary: no Rust-only types (`String`, `Vec`,
//! `Result`), no panics, only C-compatible scalars and raw pointers.

use std::os::raw::{c_char, c_int};

// ── Exported symbols ──────────────────────────────────────────────────────────

/// Add two C integers. Exported as the bare symbol `rust_add`.
///
/// The `extern "C"` qualifier switches from Rust's default calling convention
/// to the platform C ABI so that C callers can push/pop arguments as expected.
#[no_mangle]
pub extern "C" fn rust_add(a: c_int, b: c_int) -> c_int {
    a + b
}

/// Compute the nth Fibonacci number. Returns -1 for negative input.
///
/// Uses an iterative accumulator — a direct loop is cleaner than an iterator
/// chain here because we need two mutable bindings updated in lockstep.
#[no_mangle]
pub extern "C" fn rust_fib(n: c_int) -> c_int {
    if n < 0 {
        return -1;
    }
    if n <= 1 {
        return n;
    }
    let (mut a, mut b) = (0i32, 1i32);
    for _ in 2..=n {
        let c = a.wrapping_add(b);
        a = b;
        b = c;
    }
    b
}

/// Absolute value of a C integer.
#[no_mangle]
pub extern "C" fn rust_abs(n: c_int) -> c_int {
    n.abs()
}

/// Return a pointer to a static, null-terminated version string.
///
/// SAFETY for the C caller: the pointer is valid for the lifetime of the
/// process (it points to a `'static` byte literal), is null-terminated, and
/// must not be mutated or freed. These invariants are documented in the
/// accompanying C header.
#[no_mangle]
pub extern "C" fn rust_version() -> *const c_char {
    // `c"1.0.0"` is a C string literal (Rust 1.77+): null-terminated, stored
    // in `.rodata`, zero allocation. `.as_ptr()` yields `*const c_char`.
    c"1.0.0".as_ptr()
}

/// Clamp `value` to the inclusive range `[lo, hi]`.
/// Returns `lo` when `value < lo`, `hi` when `value > hi`.
/// If `lo > hi` the behaviour is unspecified (mirrors C's convention of
/// leaving invalid ranges to the caller).
#[no_mangle]
pub extern "C" fn rust_clamp(value: c_int, lo: c_int, hi: c_int) -> c_int {
    value.clamp(lo, hi)
}

// ── Tests ─────────────────────────────────────────────────────────────────────
// We call the `#[no_mangle]` functions directly from Rust — the C ABI is
// transparent to the Rust test runner, which links and calls them normally.

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CStr;

    #[test]
    fn test_add_basic() {
        assert_eq!(rust_add(3, 4), 7);
        assert_eq!(rust_add(0, 0), 0);
        assert_eq!(rust_add(-5, 5), 0);
        assert_eq!(rust_add(-3, -4), -7);
    }

    #[test]
    fn test_fib_sequence() {
        // F(0)=0, F(1)=1, F(2)=1, F(3)=2, F(4)=3, F(5)=5, F(10)=55
        assert_eq!(rust_fib(0), 0);
        assert_eq!(rust_fib(1), 1);
        assert_eq!(rust_fib(2), 1);
        assert_eq!(rust_fib(5), 5);
        assert_eq!(rust_fib(10), 55);
    }

    #[test]
    fn test_fib_negative_returns_sentinel() {
        assert_eq!(rust_fib(-1), -1);
        assert_eq!(rust_fib(-100), -1);
    }

    #[test]
    fn test_abs() {
        assert_eq!(rust_abs(0), 0);
        assert_eq!(rust_abs(42), 42);
        assert_eq!(rust_abs(-42), 42);
        assert_eq!(rust_abs(i32::MAX), i32::MAX);
    }

    #[test]
    fn test_version_is_valid_c_string() {
        let ptr = rust_version();
        assert!(!ptr.is_null());
        // SAFETY: rust_version() returns a pointer to a 'static null-terminated
        // byte literal. It is valid, non-null, and properly terminated.
        let s = unsafe { CStr::from_ptr(ptr) };
        assert_eq!(s.to_str().unwrap(), "1.0.0");
    }

    #[test]
    fn test_clamp_within_and_out_of_range() {
        assert_eq!(rust_clamp(5, 0, 10), 5);
        assert_eq!(rust_clamp(-1, 0, 10), 0);
        assert_eq!(rust_clamp(15, 0, 10), 10);
        assert_eq!(rust_clamp(0, 0, 0), 0);
    }
}
