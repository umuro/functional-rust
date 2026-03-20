📖 **[View on hightechmind.io →](https://hightechmind.io/rust/493-cstring-ffi)**

---

# CString and CStr for FFI
**Difficulty:** ⭐  
**Category:** Functional Programming  


`CString` and `CStr` are Rust's null-terminated string types for C interop — `CString` owns a null-terminated byte buffer for passing to C functions, and `CStr` borrows one for reading C output.

## Problem Statement

C strings are null-terminated byte arrays: the string ends at the first `\0`. Rust strings (`str`) can contain `\0` bytes and have an explicit length. Passing a Rust string directly to a C function expecting `char *` would either crash (no null terminator) or silently truncate at interior nulls. `CString::new(s)` validates that `s` contains no interior nulls and appends a terminating `\0`, producing a value safe to pass to any `extern "C"` function via `.as_ptr()`.

## Learning Outcomes

- Create a `CString` from a `&str` with `CString::new(s)` returning `Result`
- Understand that interior null bytes cause `CString::new` to fail
- Retrieve the raw pointer with `.as_ptr()` for FFI calls
- Inspect the bytes including the null terminator with `.as_bytes_with_nul()`
- Convert a `CStr` back to `&str` with `.to_str()` for reading C function output

## Rust Application

`CString::new` validates the input:

```rust
CString::new("hello").is_ok()      // true
CString::new("hel\0lo").is_err()   // true — interior null
```

Round-trip through `CStr`:

```rust
let c = CString::new("hi").unwrap();
assert_eq!(c.to_str().unwrap(), "hi");
assert_eq!(c.as_bytes_with_nul().last(), Some(&0u8));  // null terminator
```

In FFI usage:

```rust
extern "C" { fn strlen(s: *const i8) -> usize; }
let c = CString::new("hello").unwrap();
let len = unsafe { strlen(c.as_ptr()) };
```

## OCaml Approach

OCaml's C FFI uses `string` directly — the C bindings layer handles the null-termination:

```ocaml
external c_strlen : string -> int = "caml_string_length"

(* ocaml-ctypes uses Ctypes.string for null-terminated C strings *)
let strlen s = Ctypes.(coerce string (ptr char) s |> C.Functions.strlen)
```

The `ctypes` library provides `Ctypes.CArray`, `Ctypes.string`, and `Ctypes.ocaml_string` to manage the boundary between OCaml and C strings. OCaml strings can contain NUL bytes — passing them to C functions expecting null-terminated strings would truncate silently.

## Key Differences

1. **Null validation**: Rust's `CString::new` explicitly validates and rejects interior nulls; OCaml's `ctypes` library silently truncates at the first NUL when coercing to C strings.
2. **Ownership**: `CString` owns the null-terminated buffer; `CStr` borrows one. OCaml's GC manages string lifetime automatically but the C caller must not hold the pointer after the GC moves the string.
3. **Type separation**: Rust has `String`/`OsString`/`CString` as three distinct types with compile-time checked conversions; OCaml uses `string` everywhere with runtime checks in FFI layers.
4. **Safety**: Rust's `CString::as_ptr()` is only valid while the `CString` is alive; dropping it earlier is a use-after-free. OCaml's GC-managed strings can move, requiring pinning for long-lived C pointers.

## Exercises

1. **Safe `strlen` wrapper**: Write `fn safe_strlen(s: &str) -> Result<usize, NulError>` that creates a `CString` and calls a hypothetical `extern "C" fn strlen(*const i8) -> usize`.
2. **Read C output**: Given a `*const i8` returned by a C function, wrap it in `unsafe { CStr::from_ptr(ptr) }` and convert to a `String` with `.to_string_lossy()`.
3. **Null in the middle**: Write a test that passes `b"hel\x00lo"` to `CString::from_vec_unchecked` (unsafe) and observe that `.to_str()` returns only `"hel"` — demonstrating the truncation hazard.
