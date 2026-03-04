# 316: std::io::Error Patterns

**Difficulty:** 2  **Level:** Intermediate

Work with the standard I/O error type — create, inspect, classify, and wrap.

## The Problem This Solves

You're writing filesystem or network code and getting back `std::io::Error`. You need to handle "file not found" differently from "permission denied" differently from "would block" — but raw OS error codes like `ENOENT` (2) are platform-specific and unreadable. You also need to create your own `io::Error` values for custom validation, and wrap existing errors with additional context.

`std::io::Error` is the universal error type for anything that touches the OS. It wraps OS error codes with `ErrorKind` — a portable enum that names the common cases. Your code can match on `ErrorKind::NotFound` without caring whether it's Linux, macOS, or Windows. And you can construct `io::Error` values from scratch for custom error conditions that fit the I/O error model.

This matters because `io::Result<T>` is used everywhere in the standard library. Understanding how to create and classify `io::Error` values is as fundamental to systems Rust as `Option` and `Result` are to application Rust.

## The Intuition

`io::ErrorKind` is a portable OS-error classifier: match on it instead of raw error codes, and construct `io::Error::new(kind, message)` when you need to signal I/O-like errors from your own code.

## How It Works in Rust

```rust
use std::io::{self, ErrorKind};

// Creating custom io::Error values
fn validate_port(port: u16) -> io::Result<u16> {
    if port == 0 {
        return Err(io::Error::new(ErrorKind::InvalidInput, "port cannot be zero"));
    }
    if port < 1024 {
        return Err(io::Error::new(
            ErrorKind::PermissionDenied,
            format!("port {} requires root", port),
        ));
    }
    Ok(port)
}

// Matching on ErrorKind for portable error handling
fn handle_io_error(e: &io::Error) {
    match e.kind() {
        ErrorKind::NotFound       => eprintln!("File not found"),
        ErrorKind::PermissionDenied => eprintln!("Access denied"),
        ErrorKind::WouldBlock     => eprintln!("Not ready — try again"),
        ErrorKind::InvalidInput   => eprintln!("Invalid: {}", e),
        other => eprintln!("I/O error ({:?}): {}", other, e),
    }
}

// Wrapping an io::Error with context (preserve the kind)
match std::fs::read_to_string("config.toml") {
    Err(e) => {
        let wrapped = io::Error::new(
            e.kind(),  // preserve the original ErrorKind
            format!("loading config: {}", e),
        );
        return Err(wrapped);
    }
    Ok(s) => s,
}

// From OS error code (useful for FFI)
let not_found = io::Error::from_raw_os_error(2);  // ENOENT
assert_eq!(not_found.kind(), ErrorKind::NotFound);
```

`e.raw_os_error()` returns the original OS code if there is one — useful for debugging but not for matching logic (always prefer `ErrorKind`).

## What This Unlocks

- **Portable error handling** — match `ErrorKind` variants instead of platform-specific error codes
- **Custom I/O-shaped errors** — `io::Error::new(kind, message)` creates synthetic errors that integrate with the `io::Result` ecosystem
- **Contextual wrapping** — wrap IO errors with a human-readable context message while preserving the original `ErrorKind` for callers

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| I/O error type | `Unix.error` exceptions | `std::io::Error` value |
| Error classification | `Unix.ENOENT`, `Unix.EACCES` etc. | `io::ErrorKind` enum — portable |
| Custom I/O error | Manual exception type | `io::Error::new(kind, msg)` |
| OS error code | `Unix.error_message` | `e.raw_os_error()` — use `kind()` for logic |
