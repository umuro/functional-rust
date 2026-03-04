# 481: bytes() and Byte-Level Operations

**Difficulty:** 1  **Level:** Intermediate

Work with raw UTF-8 bytes — for binary protocols, checksums, and ASCII-only processing.

## The Problem This Solves

Most string operations in Rust work at the character (Unicode scalar) level. But sometimes you need to work at the byte level: computing checksums, implementing binary protocols, parsing ASCII-only formats, or interfacing with C libraries that think in bytes.

Python has `str.encode('utf-8')` to get bytes, and `bytes` is a distinct type. In Rust, a `String` *is* a `Vec<u8>` with a UTF-8 guarantee. You can always get at those raw bytes — `s.as_bytes()` gives you a `&[u8]` view, and `s.bytes()` gives you an iterator of `u8`.

The critical issue: once you go to bytes, you must stay byte-correct. UTF-8 means that modifying raw bytes can break the string. Rust's `String::from_utf8()` validates UTF-8 when converting bytes back to a string — and it returns a `Result`, forcing you to handle the case where the bytes aren't valid UTF-8. `from_utf8_lossy()` replaces bad sequences with the replacement character `<0xEF><0xBF><0xBD>` instead of returning an error.

## The Intuition

`.bytes()` is Python's `s.encode('utf-8')` iteration — each element is a `u8` raw byte. `.as_bytes()` is a zero-copy view of the same data as a `&[u8]` slice.

Going the other direction: `String::from_utf8(vec_of_bytes)` is Python's `bytes.decode('utf-8')` — it validates and returns `Result<String, FromUtf8Error>`. `String::from_utf8_lossy(&bytes)` is Python's `bytes.decode('utf-8', errors='replace')`.

For **ASCII-only processing**, bytes are safe and efficient. ASCII characters always fit in a single byte in UTF-8, so byte-level operations like `b.to_ascii_lowercase()` are both correct and fast. For any multi-language text, use `.chars()` instead.

## How It Works in Rust

```rust
let s = "Hello, World!";

// bytes() — iterator of u8
for b in s.bytes() {
    print!("{:02x} ", b);  // 48 65 6c 6c 6f ...
}

// as_bytes() — zero-copy &[u8] view (no allocation)
let bytes: &[u8] = s.as_bytes();
let sum: u32 = bytes.iter().map(|&b| b as u32).sum();
let spaces = bytes.iter().filter(|&&b| b == b' ').count();

// ASCII-only operations on bytes (safe for ASCII)
let lower: Vec<u8> = s.bytes()
    .map(|b| b.to_ascii_lowercase())
    .collect();
let lower_str = String::from_utf8(lower).unwrap();  // "hello, world!"

// from_utf8 — validates, returns Result
String::from_utf8(vec![72, 105])        // Ok("Hi")
String::from_utf8(vec![0xFF])           // Err — invalid UTF-8

// from_utf8_lossy — replaces bad sequences (returns Cow<str>)
let bytes = b"hell\xFF world";
let s = String::from_utf8_lossy(bytes); // "hell<replacement char> world"

// Build a String from known-good bytes
let bytes = vec![72u8, 101, 108, 108, 111];
let s = String::from_utf8(bytes).unwrap(); // "Hello"

// Literal byte string: b"hello" is &[u8]
let bs: &[u8] = b"hello";
```

## What This Unlocks

- **Checksums and hashing** — sum, XOR, or hash raw bytes of any string efficiently.
- **Binary protocol parsing** — parse wire formats, file headers, or network packets at the byte level.
- **FFI data exchange** — pass `as_bytes()` to C functions expecting `const uint8_t*`.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Byte-level access | `Bytes.of_string s` | `s.as_bytes()` → `&[u8]` |
| Iterate bytes | `Bytes.iter` | `s.bytes()` |
| String is bytes | Mutable `Bytes` separate | `String` = UTF-8 `Vec<u8>` internally |
| Bytes to string | `Bytes.to_string` (no validation!) | `String::from_utf8(vec)` → `Result` |
| Invalid bytes | No validation | `from_utf8_lossy()` — replaces bad bytes |
| Byte literal | `"\xNN"` | `b'\xNN'` (char) / `b"..."` (&[u8]) |
