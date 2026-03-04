# 715: FFI Error Codes — Converting C Errors to Rust Result

**Difficulty:** 4  **Level:** Expert

Wrap C integer error codes in a typed `Result<T, E>` at the FFI boundary — never let raw error integers bleed into Rust application code.

## The Problem This Solves

C libraries signal errors through integer return codes: `0` for success, negative values for specific errors, or positive `errno` values. This convention is pervasive — POSIX, OpenSSL, SQLite, libusb, and virtually every C library you'll ever bind. The codes are meaningful only if you know the library; raw integers in Rust code are opaque and unverifiable.

The idiomatic Rust pattern is to convert at the boundary. You write a thin `unsafe` wrapper that calls the C function, checks the return code, and maps it to `Result<T, MyError>`. From that point on, the rest of your Rust code uses `?` propagation, pattern matching, and typed error handling — no integer comparisons, no magic number `if rc == -22`.

The `#[repr(i32)]` enum technique lets you define error codes as named variants with the exact numeric values the C library uses. This makes the mapping zero-cost (it's a cast, not a table lookup) and makes the code self-documenting — `PosixError::InvalidArg` is vastly clearer than `-22`.

## The Intuition

Think of the FFI boundary as a translation booth. C speaks "integers with magic meanings." Rust speaks "typed Results with pattern matching." Your wrapper is the translator. It sits at the boundary, takes the integer C hands it, and returns either `Ok(value)` or `Err(TypedError::SpecificProblem)`. Every caller on the Rust side gets clean types; the translation happens exactly once.

The rule: one `unsafe` block per C call, immediately wrapped in the error conversion. The moment you're in safe Rust, you have a `Result`.

## How It Works in Rust

```rust
use std::os::raw::c_int;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PosixError {
    NotFound     = -2,   // ENOENT
    AccessDenied = -13,  // EACCES
    AlreadyExists = -17, // EEXIST
    InvalidArg   = -22,  // EINVAL
    Unknown      = i32::MIN,
}

impl PosixError {
    pub fn from_raw(code: c_int) -> Self {
        match code {
            -2  => Self::NotFound,
            -13 => Self::AccessDenied,
            -17 => Self::AlreadyExists,
            -22 => Self::InvalidArg,
            _   => Self::Unknown,
        }
    }
}

/// Wraps a C call that returns 0 on success, negative on error.
fn check(rc: c_int) -> Result<(), PosixError> {
    if rc == 0 { Ok(()) } else { Err(PosixError::from_raw(rc)) }
}

/// Safe Rust API hiding the C error-code convention.
pub fn safe_open(path: &str) -> Result<i32, PosixError> {
    let rc = unsafe { c_open(path.as_ptr(), path.len()) };
    check(rc).map(|_| rc)
}
```

For `errno`-style errors (the C function sets the global `errno` on failure), use `std::io::Error::last_os_error()` and map it to your error type.

## What This Unlocks

- **Idiomatic error propagation**: Use `?` throughout your Rust code — no special-case integer handling for C library errors.
- **Exhaustive error handling**: `match err { PosixError::NotFound => ..., PosixError::AccessDenied => ..., _ => ... }` — the compiler enforces you handle every case.
- **One unsafe block**: All C interaction is contained in the thin wrapper. Every caller above it is safe Rust with typed errors.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| C error handling | `Unix.Unix_error` exception | `Result<T, E>` returned |
| errno capture | `Unix.errno` | `std::io::Error::last_os_error()` |
| Error codes | `Unix.error` variants | Custom `#[repr(i32)]` enum |
| Safe wrapper boundary | Module + exception | `fn safe_foo() -> Result<T, MyError>` |
| Zero = success | Checked by Unix module | Checked and converted in wrapper |
| Propagation | `try`/`with` | `?` operator |
