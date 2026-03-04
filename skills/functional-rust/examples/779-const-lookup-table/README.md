# 779: Compile-Time Lookup Tables with const Arrays

**Difficulty:** 3  **Level:** Intermediate

Build CRC-32, ASCII classification, and trigonometric lookup tables entirely at compile time — complex initialization logic runs once in the compiler, O(1) table access at runtime.

## The Problem This Solves

Lookup tables are a classic performance technique: precompute expensive values, store them in an array, look them up by index at runtime. CRC-32 checksum calculation, for example, uses a 256-entry table computed from the CRC polynomial. Computing this table at program startup works but has several problems: the table must be mutable or initialized in `lazy_static`, it costs startup time, and in no-std/embedded contexts there may be no heap allocator.

With `const` arrays computed by `const` blocks, the initialization logic runs once — at compile time — and the table is placed in the binary's `.rodata` segment. Zero startup cost. No mutable state. No allocator required. The exact same bytes on every platform.

This pattern appears in real production code: PNG and zlib use CRC-32 tables, network packet checksums use CRC tables, text processing uses ASCII classification tables, signal processing uses sin/cos lookup tables for performance-critical inner loops.

## The Intuition

Think of it like a giant `match` expression collapsed into an array. Instead of `match byte { 0..=9 => is_digit(), 'a'..='z' => is_alpha(), ... }` you compute a `[bool; 256]` table at compile time and look up `IS_ALPHA[byte]` at runtime. One array access instead of a chain of comparisons.

The const block `{ let mut t = [0; 256]; while i < 256 { t[i] = compute(i); i+=1; } t }` is how you write complex initialization that can't be expressed as a single formula. The block is an expression whose value is the last statement — the completed array.

## How It Works in Rust

```rust
// CRC-32 table (IEEE polynomial) — 256 entries computed from the polynomial
const CRC32_TABLE: [u32; 256] = {
    let mut t = [0u32; 256];
    let mut i = 0usize;
    while i < 256 {
        let mut crc = i as u32;
        let mut j = 0;
        while j < 8 {                           // process each bit
            if crc & 1 != 0 {
                crc = (crc >> 1) ^ 0xEDB88320;  // IEEE polynomial
            } else {
                crc >>= 1;
            }
            j += 1;
        }
        t[i] = crc;
        i += 1;
    }
    t                                           // completed table is the const value
};

// CRC-32 computation — just table lookups, no polynomial arithmetic at runtime
pub fn crc32(data: &[u8]) -> u32 {
    let mut crc: u32 = 0xFFFF_FFFF;
    for &byte in data {
        let idx = ((crc ^ byte as u32) & 0xFF) as usize;
        crc = (crc >> 8) ^ CRC32_TABLE[idx];    // O(1) per byte
    }
    crc ^ 0xFFFF_FFFF
}

// ASCII uppercase conversion table — fast O(1) toUpperCase
const ASCII_UPPER: [u8; 256] = {
    let mut t = [0u8; 256];
    let mut i = 0usize;
    while i < 256 {
        t[i] = if i >= b'a' as usize && i <= b'z' as usize {
            (i - 32) as u8                      // shift lowercase to uppercase
        } else {
            i as u8                             // everything else unchanged
        };
        i += 1;
    }
    t
};

// At runtime: just an array index — no branching
pub fn to_upper_byte(b: u8) -> u8 { ASCII_UPPER[b as usize] }

// Classification table — IS_ALPHA[b] instead of ('a'..='z').contains(&(b as char))
const IS_ALPHA: [bool; 256] = { ... };  // see example.rs

// Fixed-point sin table — 256 entries covering 0..2π, scaled to i16
// const SIN_TABLE: [i16; 256] = { ... }  // typical embedded graphics pattern

// Using the tables
assert_eq!(crc32(b"hello world"), 0xD4A1185A);
assert_eq!(to_upper_byte(b'a'), b'A');
assert!(IS_ALPHA[b'z' as usize]);
assert!(!IS_ALPHA[b'0' as usize]);
```

Key points:
- `const TABLE: [T; 256] = { let mut t = [...]; while ... { ... } t }` — the const block pattern
- Nested `while` loops are fine in const context — each inner loop runs at compile time
- The polynomial constant `0xEDB88320` is the IEEE CRC-32 polynomial in reversed bit order — a mathematical constant, not magic
- `IS_ALPHA[b as usize]` is a single memory read — faster than any range comparison on hot paths
- For floating-point sin/cos tables in embedded contexts, use `f64` arithmetic in the const block to compute scaled fixed-point values

## What This Unlocks

- **Network checksums**: CRC-32, CRC-16, and Adler-32 can all be implemented as O(n) byte loops over precomputed tables — no polynomial arithmetic in the hot path
- **High-performance text processing**: character classification (`is_alpha`, `is_digit`, `is_hex`), case conversion, and character mapping with zero branching
- **Embedded signal processing**: sin/cos/exp lookup tables for DSP code where `f32::sin()` is too slow or unavailable

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Const initialization | Module-level `let table = Array.init ...` (startup) | `const TABLE: [T; N] = { while ... }` (compile time) |
| CRC polynomial loop | Nested `for` at module init | Nested `while` in const block |
| Startup cost | Runs every process start | Zero — baked into binary `.rodata` |
| Mutable state needed | Yes — `Array.make` then mutate | Mutation only inside the const block |
| No-std compatibility | N/A | Full — no allocator, no runtime needed |
| Table in binary | Heap-allocated at startup | Stack/`.rodata` — determined at link time |
