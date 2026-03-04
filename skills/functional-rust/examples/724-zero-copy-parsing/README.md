# 724: Zero-Copy Parsing with Byte Slices

**Difficulty:** 3  **Level:** Expert

Parse structured data by returning slices into the input buffer — no allocation, lifetimes as safety contracts.

## The Problem This Solves

Traditional parsers copy substrings into freshly-allocated `String` values: one allocation per token, per field, per record. For small inputs this is negligible. For high-throughput servers parsing thousands of HTTP requests per second, or embedded systems with kilobytes of RAM, every allocation matters — it costs time (heap traversal, zero-init, potential GC) and space.

Zero-copy parsing returns `&str` or `&[u8]` slices that borrow directly from the input buffer. No new memory is allocated; the parsed "values" are just view references pointing into bytes that already exist. Rust's lifetime system enforces the safety contract: the parsed references cannot outlive the input buffer. If you try to store a parsed field after the input goes out of scope, the compiler rejects it at compile time — no use-after-free possible.

## The Intuition

Imagine the input buffer as a long roll of ticker tape. Zero-copy parsing marks start and end positions on the tape and hands you a window to look through — it doesn't cut out a piece and give you a photocopy. The window is a `&[u8]` slice: the same memory, a different view.

The lifetime annotation `<'a>` on the parser result types is the mechanism: `struct RequestLine<'a> { method: &'a str, path: &'a str, version: &'a str }`. The `'a` says "these string slices borrow from whatever you parsed them out of." You can't outlive the source. The parser combinators (`take`, `take_until`, `skip_whitespace`) are tiny functions that split a byte slice at a position and return both halves — cheap pointer arithmetic, no allocation.

## How It Works in Rust

```rust
use std::str;

/// Parsed HTTP request line — all slices borrow from the input buffer.
pub struct RequestLine<'a> {
    pub method:  &'a str,
    pub path:    &'a str,
    pub version: &'a str,
}

/// Take bytes until `delimiter`, returning (before, after_delimiter).
pub fn take_until(buf: &[u8], delimiter: u8) -> Result<(&[u8], &[u8]), ParseError> {
    match buf.iter().position(|&b| b == delimiter) {
        Some(pos) => Ok((&buf[..pos], &buf[pos + 1..])),
        None => Err(ParseError::MissingDelimiter(delimiter)),
    }
}

/// Parse "GET /path HTTP/1.1" — zero allocations, zero copies.
pub fn parse_request_line(input: &[u8]) -> Result<RequestLine<'_>, ParseError> {
    // method: bytes before first space
    let (method_bytes, rest) = take_until(input, b' ')?;
    let method = str::from_utf8(method_bytes).map_err(|_| ParseError::InvalidUtf8)?;

    // path: bytes before second space
    let (path_bytes, rest) = take_until(rest, b' ')?;
    let path = str::from_utf8(path_bytes).map_err(|_| ParseError::InvalidUtf8)?;

    // version: rest of the line
    let (version_bytes, _) = take_until(rest, b'\n')?;
    let version = str::from_utf8(version_bytes.trim_ascii_end())
        .map_err(|_| ParseError::InvalidUtf8)?;

    Ok(RequestLine { method, path, version })
    // method, path, version all point into `input` — no String allocated
}
```

The lifetime `'_` (elided `'a`) on the return type communicates: "the returned struct borrows from the input argument." Attempting to return parsed slices while moving the input buffer out of scope is a compile error.

## What This Unlocks

- **High-throughput network servers** — HTTP/1.1, HTTP/2 HPACK header parsing, Redis protocol (RESP), DNS packet parsing — all amenable to zero-copy; popular crates (`httparse`, `nom`) use this approach.
- **Embedded and no-std** — parse binary sensor data, protocol frames, and configuration headers from flash memory without dynamic allocation.
- **Log and analytics pipelines** — parse TSV/CSV, JSON field extraction, and log line tokenisation at high volume without allocating per-field strings.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| String slice | `String.sub` — always copies | `&str` / `&[u8]` — borrow, no copy |
| Parser return type | `string * string` (copies) | `(&'a str, &'a [u8])` — lifetime-tied to input |
| Use-after-free safety | GC prevents most cases | Lifetime system: compile error if input outlived |
| Zero-copy idiom | Angstrom parser (lazy), Bigstring | `nom`, `winnow`, hand-rolled slice combinators |
| Allocation cost | GC amortises; still copies | Truly zero — pointer arithmetic only |
