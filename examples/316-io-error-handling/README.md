📖 **[View on hightechmind.io →](https://hightechmind.io/rust/316-io-error-handling)**

---

# 316: std::io::Error Patterns

## Problem Statement

File I/O, network operations, and process management all produce `std::io::Error`. This error type wraps OS-level errors (errno codes) and classifies them by `ErrorKind` — enabling portable handling of "file not found", "permission denied", or "connection refused" without dealing with raw OS codes. Understanding `io::Error` and `ErrorKind` is essential for writing robust system-level Rust code.

## Learning Outcomes

- Use `io::Error::new(kind, message)` to create IO errors with descriptive messages
- Match on `ErrorKind` for portable, OS-independent error classification
- Use `io::Error::kind()` to categorize errors in match arms
- Implement functions returning `io::Result<T>` (alias for `Result<T, io::Error>`)

## Rust Application

`io::ErrorKind` provides portable classifications for OS errors:

```rust
use std::io::{self, ErrorKind};

pub fn validate_port(port: u16) -> io::Result<u16> {
    if port == 0 {
        return Err(io::Error::new(ErrorKind::InvalidInput, "port cannot be zero"));
    }
    if port < 1024 {
        return Err(io::Error::new(ErrorKind::PermissionDenied,
                                   format!("port {} requires root", port)));
    }
    Ok(port)
}

// Match on ErrorKind for portable handling:
match read_file("config.txt") {
    Ok(content) => process(content),
    Err(e) if e.kind() == ErrorKind::NotFound => use_defaults(),
    Err(e) if e.kind() == ErrorKind::PermissionDenied => request_elevation(),
    Err(e) => return Err(e),
}
```

## OCaml Approach

OCaml's `Unix.error` type enumerates POSIX errors, and `Unix.Unix_error(code, fn, arg)` exceptions carry the OS error code, function name, and argument:

```ocaml
let () =
  try
    let _ = open_in "nonexistent" in ()
  with
  | Sys_error msg -> Printf.printf "Error: %s\n" msg
  | Unix.Unix_error(Unix.ENOENT, fn, arg) ->
    Printf.printf "Not found in %s(%s)\n" fn arg
```

## Key Differences

1. **Error classification**: Rust's `ErrorKind` is portable across OSes; OCaml's `Unix.error` closely follows POSIX errno codes.
2. **`io::Result<T>`**: Rust provides `io::Result<T>` as a type alias for `Result<T, io::Error>` — a common convenience in I/O-heavy code.
3. **OS error access**: `e.raw_os_error()` retrieves the underlying errno; `e.kind()` gives the portable classification.
4. **Custom IO errors**: `io::Error::new(kind, msg)` creates custom errors that integrate naturally with the IO error ecosystem.

## Exercises

1. Write a function that opens a file and classifies the error: if `NotFound`, create a default file; if `PermissionDenied`, return a clear user-facing message; otherwise propagate.
2. Implement a port validator that returns specific `ErrorKind` variants for each validation failure and verify the kinds match expectations.
3. Write a retry loop that retries an IO operation on `WouldBlock` / `Interrupted` errors but propagates all other errors immediately.
