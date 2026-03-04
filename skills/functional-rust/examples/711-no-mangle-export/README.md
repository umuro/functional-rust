# 711: #[no_mangle] Exporting Rust Functions to C

**Difficulty:** 4  **Level:** Expert

Emit stable C-ABI symbols from Rust with `#[no_mangle] pub extern "C" fn` — the entry point for Rust-as-a-library.

## The Problem This Solves

Rust normally mangles every function name to encode its full path, generic parameters, and crate version. `add` in crate `math` compiled at version 1.2.3 becomes something like `_ZN4math3add17hd3b3f2c1e4a5b6c7E`. That's great for Rust-to-Rust linking but completely opaque to C, Python, Node.js, or any other language consuming a shared library.

`#[no_mangle]` tells the compiler to emit the symbol exactly as written in source — `rust_add` stays `rust_add`. Combined with `pub extern "C"` on the function signature, you get a stable, C-ABI-compatible symbol that any C program can call with a normal `#include` and link.

The ABI boundary is a contract: no panics, no Rust types in the signature, no `Result`, no `String`, no `Vec`. If any of these leak across the boundary, the C caller has no way to interpret them. Panics across FFI are undefined behaviour. The discipline is: convert everything to C types (`c_int`, `*const c_char`, `*mut T`) at the boundary, and convert errors to return codes, not `Result`.

## The Intuition

Think of `#[no_mangle] pub extern "C" fn` as building a door between your Rust code and the outside world. The door has a specific frame (the C ABI) that everything must fit through: plain integers, raw pointers, and void. Rust's rich types live inside the house. The door exports a simplified view that C understands. `cbindgen` can auto-generate the corresponding C header from your Rust function signatures.

## How It Works in Rust

```rust
use std::os::raw::c_int;

/// Exported as symbol `rust_add` — callable from C as:
///   extern int rust_add(int a, int b);
#[no_mangle]
pub extern "C" fn rust_add(a: c_int, b: c_int) -> c_int {
    a + b
}

/// For string output, use *mut c_char + length, never Rust String.
#[no_mangle]
pub extern "C" fn rust_version(buf: *mut u8, len: usize) -> c_int {
    let version = b"1.0.0";
    if buf.is_null() || len < version.len() { return -1; }
    unsafe {
        // SAFETY: buf is non-null and len is large enough.
        std::ptr::copy_nonoverlapping(version.as_ptr(), buf, version.len());
    }
    version.len() as c_int
}
```

In `Cargo.toml`, set `crate-type = ["cdylib"]` for a shared library or `["staticlib"]` for a static archive. Run `cbindgen --crate my_lib --output my_lib.h` to generate the C header.

Key rule: **never panic across the FFI boundary**. Wrap any panic-prone code in `std::panic::catch_unwind` and convert to a return code.

## What This Unlocks

- **Rust as a drop-in C library**: Any language with C FFI (Python via `ctypes`, Node via `ffi-napi`, Ruby via `fiddle`) can call your Rust code without a special binding layer.
- **Incremental Rust adoption**: Rewrite hot C functions in Rust one at a time, keeping the rest of the C codebase unchanged.
- **`cbindgen` integration**: Annotate Rust types and functions; auto-generate `.h` headers for C consumers — the source of truth stays in Rust.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Export to C | `Callback.register` / `-output-obj` | `#[no_mangle] pub extern "C" fn` |
| Name mangling | Module prefix added | `#[no_mangle]` disables it |
| ABI | OCaml calling convention | `extern "C"` = System V / cdecl |
| Header generation | Manual | `cbindgen` auto-generates `.h` |
| Crate type | `ocamlopt -output-obj` | `cdylib` or `staticlib` |
| Panic across boundary | Undefined (setjmp/longjmp) | UB — must use `catch_unwind` |
