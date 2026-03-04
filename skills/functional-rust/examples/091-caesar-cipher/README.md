# 091: Caesar Cipher

**Difficulty:** 2  **Level:** Beginner

Rotate each letter in a string by N positions using modular arithmetic on ASCII values.

## The Problem This Solves

Encrypting or obfuscating text by shifting characters is one of the oldest ciphers. It shows up in simple encoding tasks, puzzle games, ROT-13 utilities, and anywhere you want a reversible, position-based character transformation.

Without modular arithmetic, wrapping `'z'` back around to `'a'` requires explicit range checks. With it, a single expression handles the full rotation cleanly.

The pattern generalizes beyond letters: any cyclic substitution over a finite alphabet uses the same formula — shift, mod, rebase.

## The Intuition

Think of the 26-letter alphabet as a clock face. Shifting by `n` means walking `n` steps clockwise. When you go past 'z', you wrap back to 'a'.

In Python: `chr((ord(c) - ord('a') + n) % 26 + ord('a'))`. Rust does exactly the same thing on bytes: `(c as u8 - b'a' + n) % 26 + b'a'`. The `b'a'` syntax is a byte literal — clean ASCII math without casting noise.

Decryption is just `shift(26 - n)` because shifting forward 26 steps is a full rotation back to start.

## How It Works in Rust

```rust
pub fn shift_char(n: u8, c: char) -> char {
    match c {
        // Lowercase: subtract 'a' to get 0-25, add shift, wrap, re-add 'a'
        'a'..='z' => (b'a' + (c as u8 - b'a' + n) % 26) as char,
        // Uppercase: same formula using 'A' as base
        'A'..='Z' => (b'A' + (c as u8 - b'A' + n) % 26) as char,
        // Leave spaces, digits, punctuation untouched
        _ => c,
    }
}

pub fn caesar(n: u8, s: &str) -> String {
    s.chars().map(|c| shift_char(n, c)).collect()
}

// Decryption = shift by (26 - n) to go backwards
pub fn decrypt(n: u8, s: &str) -> String {
    caesar(26 - n, s)
}
```

Range patterns (`'a'..='z'`) replace verbose `>= 'a' && <= 'z'` guards. The `chars().map().collect::<String>()` chain is the Rust equivalent of OCaml's `String.map`.

## What This Unlocks

- **ROT-13 in one line**: `caesar(13, s)` — self-inverse since 26/2 = 13.
- **Custom alphabets**: swap the base and modulus for Base64-like encodings.
- **Iterator composition**: chain `.map(shift_char)` with other transforms (uppercase, filter) without intermediate allocations.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| String transform | `String.map f s` | `s.chars().map(f).collect()` |
| Char to byte | `Char.code c` | `c as u8` |
| Byte literal | `Char.code 'a'` | `b'a'` |
| Range pattern | `c >= 'a' && c <= 'z'` | `'a'..='z'` match arm |
| Byte arithmetic | `mod 26` | `% 26` (same) |
