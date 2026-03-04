# 492: OsStr and OsString

**Difficulty:** 1  **Level:** Intermediate

Platform-native strings that may not be UTF-8 — the correct types for file paths and environment variables.

## The Problem This Solves

File paths and environment variables are not UTF-8 strings on all operating systems. On Windows, paths are UTF-16 and can contain sequences that aren't valid Unicode. On Unix, paths are arbitrary byte sequences — only `/` and null are special. A file named with arbitrary bytes is legal.

If you read a file path into a `String`, Rust will panic or return an error if the path contains non-UTF-8 bytes. That's correct behavior — but it means your program can't handle all valid paths on the OS it's running on.

`OsStr` and `OsString` are Rust's bridge types: they hold the OS-native representation losslessly. You pass them to filesystem APIs. When you need to display or manipulate them as text, you explicitly convert — and decide what to do if they're not valid UTF-8.

## The Intuition

A post box that accepts any parcel the postal service can deliver, not just parcels you've opened and verified. `OsString` holds whatever the OS hands you. `String` is what you get after you've unwrapped it and confirmed the contents are text. You convert at the boundary, deliberately.

## How It Works in Rust

1. **Receiving from the OS** — `std::env` and `std::fs` return `OsStr`/`OsString`:
   ```rust
   use std::ffi::{OsStr, OsString};

   let path: &OsStr = std::path::Path::new("/tmp/data").as_os_str();
   let var: OsString = std::env::var_os("PATH").unwrap();
   ```
2. **Convert to `&str` (may fail)**:
   ```rust
   if let Some(s) = path.to_str() {
       println!("utf-8 path: {}", s);
   }
   ```
3. **Lossy conversion** — always succeeds, replaces bad bytes with `U+FFFD`:
   ```rust
   let display = var.to_string_lossy(); // Cow<str>
   ```
4. **Building paths** — use `Path` and `PathBuf` which wrap `OsStr`/`OsString`:
   ```rust
   let mut p = std::path::PathBuf::from("/home/user");
   p.push("documents");
   p.set_extension("txt");
   ```
5. **Passing to C APIs** — convert to `CStr`/`CString` for FFI:
   ```rust
   use std::ffi::CString;
   let c = CString::new(path.to_str().unwrap()).unwrap();
   ```

## What This Unlocks

- **Portability** — code that handles `OsStr` correctly works on all platforms, including unusual file names.
- **Correct CLI tools** — argument parsers, file walkers, and env-var readers that don't silently drop valid inputs.
- **FFI safety** — understanding the `OsStr` → `CStr` pipeline is essential for calling C filesystem APIs.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Native string | `string` (bytes, no encoding) | `OsStr` (platform-native) |
| Filesystem paths | `string` | `Path` / `PathBuf` (wraps `OsStr`) |
| UTF-8 check | Manual | `OsStr::to_str()` → `Option<&str>` |
| Lossy display | Manual | `to_string_lossy()` → `Cow<str>` |
