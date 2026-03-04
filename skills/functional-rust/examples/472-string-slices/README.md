# 472: String Slices and Byte Boundaries

**Difficulty:** 1  **Level:** Intermediate

Understand `&str` as a borrowed view into UTF-8 bytes — and why safe slicing matters.

## The Problem This Solves

In Python or JavaScript, you can slice a string with `s[0:3]` and it just works. The runtime figures out character boundaries for you. In Rust, strings are raw UTF-8 bytes under the hood — and if you try to slice in the middle of a multi-byte character like `é` (which takes 2 bytes), your program panics at runtime.

This isn't Rust being difficult — it's Rust being honest. UTF-8 means one character can be 1–4 bytes wide. A slice like `&s[3..4]` on `"café"` would land you in the middle of `é`'s two-byte sequence. That's not a valid string. Rust refuses to give you one.

Understanding the difference between `String` (owned, heap-allocated, growable) and `&str` (borrowed slice — just a pointer + length) is the foundation for all string work in Rust. This example shows safe slicing with `.get()`, character counting with `.chars()`, and byte-accurate iteration with `.char_indices()`.

## The Intuition

Think of a `String` as a `Vec<u8>` with a UTF-8 promise attached. A `&str` is a view into that buffer — it borrows a range of bytes. It's two machine words: a pointer to the start, and a length in bytes.

In Python, `"café"[3]` gives you `é`. In Rust, `"café"[3..]` panics — byte 3 is *inside* the `é` codepoint. You must either use `.chars().nth(3)` (character index) or `.char_indices()` to find valid byte boundaries first.

Use `.get(start..end)` instead of `&s[start..end]` whenever you're not 100% sure the indices are on character boundaries. It returns `Option<&str>` — `None` instead of panic.

## How It Works in Rust

```rust
let s = "Hello, World!";
// ASCII-only: safe to slice by byte index
let hello = &s[0..5];  // "Hello"

// UTF-8: café is 5 bytes, 4 chars
let cafe = "café";
println!("bytes={} chars={}", cafe.len(), cafe.chars().count()); // 5, 4

// char_indices gives you SAFE byte positions + char pairs
for (byte_pos, ch) in cafe.char_indices() {
    println!("byte {} → '{}'", byte_pos, ch);
}
// byte 0 → 'c', byte 1 → 'a', byte 2 → 'f', byte 3 → 'é'

// Safe get — returns None instead of panicking
cafe.get(0..3);   // Some("caf")
cafe.get(3..4);   // None  ← mid-codepoint boundary!
cafe.get(0..999); // None  ← out of bounds

// split_at by byte position (panics if not on boundary)
let (left, right) = s.split_at(7); // "Hello, " / "World!"
```

## What This Unlocks

- **Safe string slicing** in parsers and text processors — never panic on user input.
- **Correct character counting** for display widths, truncation, and pagination.
- **FFI and binary protocols** where you need to know exact byte positions.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| String type | `string` (immutable byte sequence) | `String` (owned, heap) / `&str` (borrowed view) |
| Slice by index | `String.sub s pos len` (bytes) | `&s[start..end]` (bytes, panics) or `.get()` (safe) |
| Character count | Manual UTF-8 loop | `.chars().count()` |
| Byte length | `String.length s` | `s.len()` |
| Safe subslice | Custom `safe_sub` function | `.get(start..end)` → `Option<&str>` |
| Char + position | `String.index` (char, panics) | `.char_indices()` → `(byte_pos, char)` |
