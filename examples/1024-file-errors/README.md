📖 **[View on hightechmind.io →](https://hightechmind.io/rust/1024-file-errors)**

---

# 1024-file-errors — File Operation Errors
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  


## Problem Statement

File I/O is one of the most common sources of recoverable errors in production software. Files may not exist, permissions may be wrong, disks may be full, or paths may be invalid. Every language with a type system faces the question of how to represent these errors at the type level.

Rust's `std::io::Error` unifies all I/O errors into a single type with an `ErrorKind` discriminant for runtime classification. This enables generic I/O code while still allowing precise error handling where needed.

## Learning Outcomes

- Use `std::fs` functions and handle `io::Error` return types
- Classify errors by `io::ErrorKind` (NotFound, PermissionDenied, etc.)
- Convert `io::Error` to application-specific error types
- Understand how `io::Error` carries an OS error code alongside its kind
- Chain file operations with `?` in a larger pipeline

## Rust Application

`src/lib.rs` demonstrates the full cycle: `read_file` and `write_file` use `fs::read_to_string` and `fs::write`, both returning `Result<_, io::Error>`. `classify_io_error` uses `err.kind()` to return a human-readable category. A custom `FileError` enum converts `io::Error` into application-level variants (`NotFound`, `PermissionDenied`, `Other`), separating the application's error model from the OS-level error model.

`io::Error::other` (stable since 1.74) creates custom `io::Error` values from any error type, useful when building I/O abstractions.

## OCaml Approach

OCaml's `Unix` module raises exceptions for file errors:

```ocaml
let read_file path =
  try
    let ic = open_in path in
    let content = In_channel.input_all ic in
    close_in ic;
    Ok content
  with
  | Sys_error msg -> Error msg
  | Unix.Unix_error (code, fn_name, arg) ->
    Error (Unix.error_message code)
```

The `Unix.error` type is a variant with constructors like `ENOENT`, `EACCES`, etc., analogous to `io::ErrorKind`.

## Key Differences

1. **Exception vs Result**: OCaml's Unix file functions raise exceptions by default; Rust's `std::fs` functions always return `Result`.
2. **Error classification**: Both use OS error codes internally; Rust exposes them via `ErrorKind` enum, OCaml via `Unix.error` variant.
3. **Error composition**: Rust's `?` propagates `io::Error` through call stacks uniformly; OCaml's `try/with` scoping is more explicit.
4. **Cross-platform**: Rust's `io::ErrorKind` abstracts OS differences; OCaml's `Unix.error` is more Unix-specific.

## Exercises

1. Write a `read_or_create(path: &str, default: &str) -> Result<String, io::Error>` function that reads a file if it exists, or creates it with the default content if it does not.
2. Implement a `safe_copy(src: &str, dst: &str) -> Result<u64, FileError>` function that copies a file, converting all `io::Error`s to `FileError`.
3. Write a function that lists all `.txt` files in a directory using `fs::read_dir`, collecting errors and file paths separately.
