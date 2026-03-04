# 265: Caesar Cipher

**Difficulty:** 1  **Level:** Beginner

Encrypt and decrypt text by shifting each letter a fixed number of positions in the alphabet, leaving non-letter characters unchanged.

## The Problem This Solves

You're learning character manipulation in Rust — how to treat `char` as a number, apply arithmetic, and map transformations over strings. The Caesar cipher is the perfect vehicle: the logic is simple enough to understand immediately, but it forces you to work with character ranges, modular arithmetic, and string collection.

Without range patterns and `as u8` casting, you'd write verbose `if/else` chains checking `c >= 'a' && c <= 'z'`. Without `.chars().map().collect()`, you'd iterate over a string building a new one character by character. The Caesar cipher shows you both idioms in under 10 lines.

Decryption as `caesar(26 - n, s)` demonstrates a key functional insight: inverse operations can often be expressed as the original operation with a different parameter, avoiding duplicated logic.

## The Intuition

Map each letter to its shifted position in the alphabet using modular arithmetic; pass non-letter characters through unchanged.

## How It Works in Rust

```rust
fn shift_char(n: u8, c: char) -> char {
    match c {
        // Range pattern matches entire alphabet in one arm
        'a'..='z' => ((c as u8 - b'a' + n) % 26 + b'a') as char,
        'A'..='Z' => ((c as u8 - b'A' + n) % 26 + b'A') as char,
        _ => c,  // non-letters pass through unchanged
    }
}

pub fn caesar(n: u8, s: &str) -> String {
    s.chars()           // iterate over Unicode code points
     .map(|c| shift_char(n, c))  // shift each letter
     .collect()         // reassemble into a String
}

pub fn decrypt(n: u8, s: &str) -> String {
    caesar(26 - (n % 26), s)  // shift by the complement; n % 26 handles n=26
}

pub fn rot13(s: &str) -> String {
    caesar(13, s)  // ROT13 is its own inverse: rot13(rot13(s)) == s
}
```

The key arithmetic: `(c as u8 - b'a' + n) % 26 + b'a'`
1. Subtract `b'a'` → 0-based index (0–25)
2. Add shift `n`, take `% 26` → stays in alphabet
3. Add `b'a'` → back to ASCII code point
4. Cast to `char`

## What This Unlocks

- **Character arithmetic patterns:** The `as u8` / `as char` dance applies to any char-to-number transformation.
- **Range patterns:** `'a'..='z'` in match arms replaces three separate comparisons — reuse this anywhere.
- **Functional string transformation:** `.chars().map().collect()` is the idiomatic Rust `String.map` — memorize it.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Char to int | `Char.code c` | `c as u8` |
| Int to char | `Char.chr n` | `n as char` |
| Alphabet check | `if c >= 'a' && c <= 'z'` | `'a'..='z'` range pattern |
| String mapping | `String.map (shift_char n) s` | `s.chars().map(\|c\| shift_char(n, c)).collect()` |
| Decryption | `let decrypt n = caesar (26 - n)` (partial app) | `fn decrypt(n, s) { caesar(26 - n % 26, s) }` |
