# 493: CString and CStr for FFI

**Difficulty:** 1  **Level:** Intermediate

Bridge Rust strings to C — the null-terminated string types you need for foreign function interfaces.

## The Problem This Solves

C strings are null-terminated: the string `"hello"` is stored as 6 bytes — `h`, `e`, `l`, `l`, `o`, `\0`. The `\0` marks the end. C functions expect a `*const char` pointing to this null-terminated sequence. Rust's `String` and `&str` have no null terminator — they track length separately. You can't pass a `&str` directly to a C function.

More dangerously: if a string contains an interior null byte (`"hel\0lo"`), a C function would think it's only 3 characters long, silently truncating. Rust's `CString::new()` validates this — it returns an error if the string contains any null bytes. You catch the bug at the boundary, not as a silent data corruption.

`CString` (owned) and `CStr` (borrowed) are the safe wrappers for this. `CString` adds the null terminator and validates no interior nulls. `CStr` is the borrowed view used to receive C strings coming in. The pattern: use `CString` when sending to C, use `CStr` when receiving from C.

## The Intuition

Think of the relationship:

| Rust | C FFI |
|------|-------|
| `String` | owns `char*` data, Rust-managed |
| `&str` | borrowed `char*`, no null terminator |
| `CString` | owns `char*` data, null-terminated, validated |
| `CStr` | borrowed `*const char`, null-terminated |

`CString::new("hello")` returns `Result<CString, NulError>` — the error case catches interior nulls. Once you have a `CString`, call `.as_ptr()` to get the `*const c_char` to pass to C.

`CStr::from_ptr(ptr)` is `unsafe` because Rust has no way to verify that the C pointer is valid or that the string is properly terminated. You're asserting: "I know this pointer is safe." Inside the `unsafe` block, you extract the string and convert it to a Rust `&str` via `.to_str()` (validates UTF-8) or `.to_string_lossy()` (replaces invalid UTF-8).

## How It Works in Rust

```rust
use std::ffi::{CString, CStr, c_char};

// CString — owned null-terminated string
let cs = CString::new("hello").expect("no interior nulls");
//                              ^^ returns Err if string contains '\0'

// Get raw pointer for FFI
let ptr: *const c_char = cs.as_ptr();
// Pass `ptr` to a C function — it's valid as long as `cs` is alive

// Interior null → explicit error (not silent truncation)
match CString::new("hel\0lo") {
    Ok(_)  => unreachable!(),
    Err(e) => println!("null at byte {}", e.nul_position()), // 3
}

// CStr — borrowed view of a C string
// From known-good bytes (include the \0!)
let bytes: &[u8] = b"hello\0";
let cstr = CStr::from_bytes_with_nul(bytes).expect("valid");

// Convert CStr to Rust types
cstr.to_str()            // Result<&str, Utf8Error> — validates UTF-8
cstr.to_string_lossy()   // Cow<str> — replaces invalid UTF-8

// Round-trip: Rust String → CString → CStr → &str
let original = "round trip";
let cstring = CString::new(original).unwrap();
let cstr_ref: &CStr = cstring.as_c_str();
let back: &str = cstr_ref.to_str().unwrap();
assert_eq!(original, back);

// Receiving a C string pointer (unsafe — you're asserting pointer validity)
unsafe {
    let received = CStr::from_ptr(ptr);
    println!("{}", received.to_string_lossy());
}

// Check that CString includes the null terminator
let bytes_with_null = cstring.as_bytes_with_nul();
assert_eq!(bytes_with_null.last(), Some(&0u8));
```

## What This Unlocks

- **Calling C libraries** — pass Rust strings to any C API that expects `const char*`.
- **Receiving C strings** — safely convert C function outputs back to Rust `&str` or `String`.
- **Preventing FFI bugs** — `CString::new()` validates your string before it reaches C, catching interior nulls at the Rust boundary.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Null-terminated string | Manual `Bytes` with appended `'\000'` | `CString::new("...")` — validates + adds `\0` |
| Borrowed C string | Manual `Bytes` slice | `CStr` — borrowed, length from `\0` terminator |
| Interior null check | `String.exists ((=) '\000')` — manual | `CString::new()` → `Err(NulError)` |
| C pointer | `Bytes` pointer via `Bigarray`/`Ctypes` | `.as_ptr()` → `*const c_char` |
| Receive from C | `Ctypes.CArray.to_list` | `unsafe { CStr::from_ptr(ptr) }` |
| To Rust string | `Bytes.to_string` (no validation) | `.to_str()` (validates UTF-8) / `.to_string_lossy()` |
