# 712: String/CString/CStr Conversion for FFI

**Difficulty:** 4  **Level:** Expert

Bridge Rust's UTF-8 strings and C's null-terminated strings without leaking memory or causing undefined behaviour.

## The Problem This Solves

Rust strings (`&str`, `String`) are UTF-8, length-prefixed, and not null-terminated. C strings (`char*`) are null-terminated, encoding-agnostic, and have no length field. These two representations are incompatible — you cannot pass a Rust `String` directly to a C function that expects `const char*`, nor can you read a `char*` as a Rust `&str` without checking for null termination and valid UTF-8.

The mismatch is a common source of bugs at FFI boundaries: passing a non-null-terminated buffer causes C to read past the end (buffer overread), forgetting a CString in a local variable causes a dangling pointer when the temporary is dropped, and failing to validate UTF-8 when reading back a C string causes silent string corruption. Rust's `CString` and `CStr` types encode these safety constraints in the type system.

unsafe is a tool, not a crutch — use only when safe Rust genuinely can't express the pattern.

## The Intuition

There are two directions at the FFI boundary:

**Rust → C** (`CString`): Allocate a Rust-owned, heap-allocated buffer that *is* null-terminated. `CString::new(s)` returns an error if `s` contains an interior null byte (which would silently truncate the C string). Call `.as_ptr()` to get the raw pointer for the C function. The pointer is valid as long as the `CString` is alive — never inline the `CString` as a temporary or the pointer dangles immediately.

**C → Rust** (`CStr`): Borrow a null-terminated C string as a Rust type that tracks the null terminator. `CStr::from_ptr(ptr)` requires `unsafe` because you must guarantee the pointer is non-null, null-terminated, and valid for the borrow's duration. From there, `.to_str()` validates UTF-8 and gives you a `&str`.

## How It Works in Rust

```rust
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

// ── Rust → C: create a null-terminated CString ──────────────────────────
pub fn measure(s: &str) -> Result<usize, std::ffi::NulError> {
    // Returns Err if s contains '\0' (would truncate the C string)
    let cstring = CString::new(s)?;

    // IMPORTANT: bind to a variable — do NOT use as an inline temporary!
    // CString::new("hello").as_ptr()  ← DANGLING POINTER (dropped immediately)
    let len = unsafe {
        // SAFETY: cstring is non-null, null-terminated, and lives for this call.
        c_strlen(cstring.as_ptr())
    };
    Ok(len)
}

// ── C → Rust: borrow a C string as CStr, then convert to &str ───────────
pub fn rust_greeting() -> String {
    let ptr = unsafe {
        // SAFETY: c_greeting() returns a 'static null-terminated C string.
        c_greeting()
    };
    unsafe {
        // SAFETY: ptr is non-null, null-terminated, valid UTF-8 (we control
        // the C side). CStr borrows — no allocation.
        CStr::from_ptr(ptr)
            .to_str()
            .expect("valid UTF-8")
            .to_owned()  // ← clone into an owned String to escape the 'static lifetime
    }
}
```

The lifetime of `CStr` is tied to the pointer's validity — do not store `CStr` longer than the C memory is valid. For C strings that Rust must own, use `CString`; for borrowing C strings temporarily, use `CStr`.

## What This Unlocks

- **Wrapping C APIs** — database drivers, TLS libraries, and OS path APIs all deal in null-terminated strings. These types let you wrap them without copying unless necessary.
- **Embedding scripting languages** — Lua, Python, and similar C-based runtimes pass strings as `char*`; `CString`/`CStr` is the bridge.
- **Plugin systems** — load a shared library, receive a `const char*` plugin name, and convert it safely to a Rust `&str` for logging and routing.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| String representation | Length-prefixed byte sequence | `&str` = UTF-8 + length; `String` = owned UTF-8 |
| C string type | `char *` via `Ctypes.string` | `CString` (owned), `CStr` (borrowed) |
| Null terminator | Added by `Ctypes.ocaml_string_start` | Must use `CString::new(s)` — returns `Err` on interior null |
| Dangling pointer risk | GC prevents (usually) | Manual: bind `CString` to a variable before `.as_ptr()` |
| UTF-8 validation | Not checked | `CStr::to_str()` returns `Err` on invalid UTF-8 |
