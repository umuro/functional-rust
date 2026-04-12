📖 **[View on hightechmind.io →](https://hightechmind.io/rust/265-caesar-cipher)**

---

# Example 265: Caesar Cipher — Functional Encryption
**Difficulty:** ⭐  
**Category:** Functional Programming  



## Problem Statement

Implement a Caesar cipher that shifts each letter by a fixed number of positions in the alphabet. Non-letter characters pass through unchanged. Provide both encryption and decryption.

## Learning Outcomes

- Character arithmetic in Rust using `as u8` and `as char` conversions
- Range patterns in match arms (`'a'..='z'`)
- String transformation with `.chars().map().collect()`
- Function composition: decryption as a shifted encryption

## OCaml Approach

OCaml uses `Char.code` and `Char.chr` for character arithmetic and `String.map` to apply the shift function to every character. `decrypt` is elegantly defined as `caesar (26 - n)` using partial application.

## Rust Approach

Rust uses `as u8` / `as char` for character arithmetic and range patterns `'a'..='z'` in match arms. The iterator chain `.chars().map().collect()` replaces `String.map`. Decryption uses the same shift-reversal technique.

## Key Differences

1. **Char arithmetic:** OCaml uses `Char.code`/`Char.chr`; Rust casts with `as u8`/`as char`
2. **Pattern matching:** OCaml uses `if/else` on char comparisons; Rust uses range patterns `'a'..='z'`
3. **String mapping:** OCaml's `String.map` applies a function per char; Rust uses `.chars().map().collect()`
4. **Partial application:** OCaml's `let decrypt n = caesar (26 - n)` is more concise; Rust needs a full function definition

## Exercises

1. Implement `caesar_crack` that performs brute-force decryption: try all 25 shifts and return the one whose output most closely matches English letter frequencies.
2. Extend the Caesar cipher to the Vigenère cipher: accept a keyword and apply a different shift for each position, cycling through the keyword letters.
3. Implement ROT13 as a special case of Caesar cipher with shift 13 and verify that applying it twice returns the original text; then generalize to an arbitrary involution cipher.
