# 483: UTF-8 Encoding Patterns

**Difficulty:** 1  **Level:** Intermediate

Rust strings are always UTF-8. Understanding that invariant unlocks safe, efficient text handling.

## The Problem This Solves

Many languages let you store arbitrary bytes in strings and discover encoding problems at runtime — a `UnicodeDecodeError` in Python, a garbled web page from a Latin-1 mismatch in older Java code. These bugs are subtle: code works fine in English, then breaks on an é or a Japanese character.

Rust enforces UTF-8 at the type level. `str` and `String` *guarantee* valid UTF-8. You can't accidentally create an invalid string — the constructors check at the boundary. If you receive bytes from the outside world and they're not valid UTF-8, you find out immediately with a `Result`, not silently later.

When you genuinely need other encodings (reading Latin-1 files, speaking to Windows APIs, parsing network protocols), you convert explicitly at the boundary and work with `String` inside your program.

## The Intuition

Think of UTF-8 as the contract your application enforces internally. Anything that comes in from outside — file bytes, network packets, OS strings — must pass through a border check. Valid UTF-8 enters as `String`. Invalid bytes stay as `Vec<u8>` or `[u8]` until you decide how to convert them. There's no way for an encoding bug to quietly corrupt data downstream.

## How It Works in Rust

1. **Validate bytes as UTF-8**:
   ```rust
   let bytes: &[u8] = b"caf\xc3\xa9"; // "café" in UTF-8
   let s = std::str::from_utf8(bytes)?; // returns &str or Utf8Error
   ```
2. **Lossy conversion** — replace invalid sequences with `U+FFFD`:
   ```rust
   let s = String::from_utf8_lossy(bytes); // Cow<str>
   ```
3. **Byte access** — when you need raw bytes back:
   ```rust
   let bytes: &[u8] = s.as_bytes();
   ```
4. **Character iteration** — iterate Unicode scalar values, not bytes:
   ```rust
   for ch in "café".chars() { /* ch is a char (Unicode scalar value) */ }
   ```
5. **Other encodings** — use the `encoding_rs` crate for Latin-1, UTF-16, Shift-JIS, etc.:
   ```rust
   let (decoded, _enc, had_errors) = encoding_rs::WINDOWS_1252.decode(latin1_bytes);
   ```

## What This Unlocks

- **Encoding safety** — encoding bugs become compiler/type errors or explicit `Result`s, not silent corruption.
- **Zero-cost string views** — `&str` is just a pointer + length into UTF-8 bytes; slicing is O(1).
- **Interop confidence** — knowing exactly where to convert means your UTF-8 core stays clean.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| String encoding | Bytes (no guarantee) | Guaranteed UTF-8 |
| Validation | Manual | `str::from_utf8()` at boundary |
| Char type | `char` (byte in `Bytes`) | `char` (Unicode scalar, 4 bytes) |
| Other encodings | `Uutf` crate | `encoding_rs` crate |
