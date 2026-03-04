# 710: Calling C Functions with `extern "C"`

**Difficulty:** 4  **Level:** Expert

Declare C function signatures, call them safely, and wrap the unsafe boundary in a clean Rust API.

## The Problem This Solves

The C ABI is the universal language of system interfaces. Every OS, database driver, cryptographic library, and hardware SDK exposes a C API. Rust must be able to call these libraries — not by reimplementing them, but by linking against the existing binary and calling through the C ABI at runtime.

`extern "C"` is the mechanism. You declare the function signatures Rust needs to know about, and the linker resolves the actual addresses at link time. The call itself happens at runtime. Because Rust's safety model cannot look inside a C function body — C has no borrow checker — every `extern "C"` call is unconditionally `unsafe`. You are responsible for: passing valid pointers, respecting ownership of any memory the C function returns, and matching the C function's actual type signature exactly.

unsafe is a tool, not a crutch — use only when safe Rust genuinely can't express the pattern.

## The Intuition

Think of `extern "C"` as a phone book entry: you declare the name and calling convention so Rust can generate the correct machine code, but the function itself lives in a different binary. Rust will place the arguments in the right registers and stack slots (per the C ABI), jump to the address, and trust that the C code does what the declaration says.

If your declaration has the wrong types — say `i32` where C expects `unsigned long` on a 64-bit platform — the resulting call is undefined behaviour with no compiler error. Matching types precisely is the core discipline of FFI.

## How It Works in Rust

```rust
use std::os::raw::c_int;

// ── Simulated C library (Rust with C ABI) ───────────────────────────────
#[no_mangle]
pub extern "C" fn c_add(a: c_int, b: c_int) -> c_int { a + b }

#[no_mangle]
pub extern "C" fn c_clamp(n: c_int, lo: c_int, hi: c_int) -> c_int {
    if n < lo { lo } else if n > hi { hi } else { n }
}

// ── Safe wrappers — validate before crossing the FFI boundary ───────────
pub fn safe_add(a: i32, b: i32) -> i32 {
    unsafe {
        // SAFETY: c_add is pure integer addition; no preconditions beyond
        // valid i32 values, which Rust's type system guarantees.
        c_add(a, b)
    }
}

pub fn safe_clamp(n: i32, lo: i32, hi: i32) -> Result<i32, String> {
    if lo > hi {
        return Err(format!("lo ({lo}) > hi ({hi})"));
    }
    Ok(unsafe {
        // SAFETY: lo <= hi (checked above); all i32 values are valid inputs.
        c_clamp(n, lo, hi)
    })
}
```

Use `std::os::raw` types (`c_int`, `c_char`, `c_void`, etc.) rather than Rust primitives when declaring `extern "C"` signatures — their sizes match the C ABI on every target platform.

## What This Unlocks

- **System programming** — call `libc`, OS APIs, and hardware drivers that expose C interfaces (memory allocation, socket operations, GPU compute).
- **Cryptography and performance libraries** — OpenSSL, libsodium, BLAS, and similar libraries expose C APIs that Rust doesn't need to reinvent.
- **Gradual migration** — call existing C/C++ codebases from new Rust code while incrementally replacing modules.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| C FFI declaration | `external my_fn : int -> int = "my_fn"` | `extern "C" { fn my_fn(x: c_int) -> c_int; }` |
| Safety of C calls | Unchecked by default | Always `unsafe` — must write `unsafe { }` |
| Type mapping | OCaml `int` ≠ C `int` (tag bit) | `c_int` from `std::os::raw` — exact C ABI types |
| Linking | `(-cclib -lm)` in dune | `#[link(name = "m")]` or build.rs `println!("cargo:rustc-link-lib=m")` |
| Safe wrapper idiom | Typically not enforced | Strong convention: `unsafe fn` → `pub fn` wrapper |
