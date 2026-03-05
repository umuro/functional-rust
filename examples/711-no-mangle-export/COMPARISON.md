# OCaml vs Rust: `#[no_mangle]` Exporting Functions to C

## Side-by-Side Code

### OCaml
```ocaml
(* OCaml uses Callback.register to make functions callable from C.
   C code calls caml_named_value("rust_add") to get a closure handle,
   then invokes it with caml_callback2(..., Val_int(a), Val_int(b)).
   The GC, boxing, and tagging remain hidden from the C caller. *)

let () =
  Callback.register "rust_add" (fun a b -> (a : int) + b);
  Callback.register "rust_fib" (fun n ->
    let rec fib k = if k <= 1 then k else fib (k-1) + fib (k-2) in
    fib (n : int)
  )
```

### Rust (idiomatic export)
```rust
use std::os::raw::c_int;

/// Add two integers — symbol emitted as `rust_add` with C ABI.
#[no_mangle]
pub extern "C" fn rust_add(a: c_int, b: c_int) -> c_int {
    a + b
}

/// Fibonacci, iterative. Returns -1 for negative input (C-style error code).
#[no_mangle]
pub extern "C" fn rust_fib(n: c_int) -> c_int {
    if n < 0 { return -1; }
    if n <= 1 { return n; }
    let (mut a, mut b) = (0i32, 1i32);
    for _ in 2..=n {
        let c = a.wrapping_add(b);
        a = b; b = c;
    }
    b
}
```

### Rust (static string export)
```rust
use std::os::raw::c_char;

/// Return a pointer to a 'static, null-terminated version string.
#[no_mangle]
pub extern "C" fn rust_version() -> *const c_char {
    b"1.0.0\0".as_ptr().cast()
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Export mechanism | `Callback.register "name" fn` | `#[no_mangle] pub extern "C" fn name(...)` |
| Integer type | `int` (tagged boxed OCaml int) | `c_int` = `i32` on most platforms |
| C string | Not natively supported; use `Bytes` + stubs | `*const c_char` pointing to a `b"...\0"` literal |
| Error signalling | `option` / `result` (OCaml-only) | Return code convention (`-1`, `NULL`) |
| Symbol visibility | Named via runtime registry | Compile-time symbol in the object file |

## Key Insights

1. **Name mangling vs stability.** Rust mangles every symbol by default (path + hash) so that generic instantiations and crate versions can coexist. `#[no_mangle]` suppresses this, making the emitted symbol byte-for-byte what the source says — `rust_add` → `rust_add`. OCaml never mangles in the same sense; instead it uses a runtime name registry (`Callback.register`) that C must navigate through the OCaml runtime API.

2. **ABI contract.** `pub extern "C"` switches from Rust's default (unspecified) calling convention to the platform C ABI: arguments on the stack or in registers in the C-defined order, return in the C-defined register. Without `extern "C"`, the binary interface is undefined and C cannot reliably call the function even if the symbol is visible.

3. **No Rust types across the boundary.** OCaml's `Callback` mechanism still passes OCaml-boxed values (`Val_int`, `caml_callback2`); the C caller must understand the OCaml representation. Rust's FFI exports use only C primitives (`c_int`, `*const c_char`) — the C caller needs nothing Rust-specific at all. The discipline cost: no `Result`, no `String`, no panics — errors become return codes.

4. **Static strings without allocation.** `b"1.0.0\0".as_ptr().cast::<c_char>()` returns a pointer into the `.rodata` segment — zero heap cost, valid for the process lifetime. The C caller gets a `const char *` that needs no `free`. OCaml would require `Bytes` marshalling or a C stub with `caml_copy_string`.

5. **Testing exported functions from Rust.** `#[no_mangle] pub extern "C"` functions are still ordinary Rust functions at the type level; the Rust test runner calls them directly without going through the C ABI. This means you get full `cargo test` coverage on your exports with no extra test harness.

## When to Use Each Style

**Use `#[no_mangle] pub extern "C"` when:** building a shared library (`cdylib`) consumed by C, Python (ctypes/cffi), Node.js (ffi-napi), or any other language that can load native symbols. This is the standard "Rust as a library" pattern.

**Use OCaml `Callback.register` when:** embedding OCaml inside a C host application and needing OCaml closures callable from C; the C side must link against the OCaml runtime and use `caml_callback` helpers rather than calling the function directly.
