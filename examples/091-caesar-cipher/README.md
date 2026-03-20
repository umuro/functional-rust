[![hightechmind.io](https://img.shields.io/badge/hightechmind.io-functional--rust-blue)](https://hightechmind.io)

# 091 — Caesar Cipher

## Problem Statement

Implement the Caesar cipher: rotate each alphabetic character by `n` positions, wrapping around with modular arithmetic (`% 26`). Preserve non-alphabetic characters unchanged. Implement three versions: iterator-based, fold-based, and in-place byte mutation — and compare with OCaml's single-pass `String.map` approach.

## Learning Outcomes

- Use byte arithmetic `(c as u8 - b'a' + n) % 26` for letter rotation
- Apply `s.chars().map(shift_char).collect::<String>()` for idiomatic transformation
- Use `fold` with `String::with_capacity` to avoid repeated reallocation
- Understand the in-place byte mutation approach and when it is safe (ASCII only)
- Map Rust's `.chars().map().collect()` to OCaml's `String.map f s`
- Recognise that `decrypt(n)` is just `caesar(26 - n)` — a clean mathematical inverse

## Rust Application

`shift_char(n, c)` pattern-matches on character ranges `'a'..='z'` and `'A'..='Z'`, applying modular rotation. The idiomatic version chains `.chars().map(|c| shift_char(n, c)).collect()`. The fold version builds a `String` with `String::with_capacity(s.len())` pre-allocated, then `push`es each shifted character — avoiding reallocations. The byte version converts to `Vec<u8>`, mutates each byte in-place, and converts back — only safe for ASCII input. `decrypt(n)` uses `caesar(26 - n)` as the inverse.

## OCaml Approach

OCaml's `String.map (shift_char n) s` applies the character function uniformly. `shift_char` uses `Char.code` and `Char.chr` for arithmetic on character codes. The decryption is `caesar (26 - n)`. The code is more concise because `String.map` is a one-liner abstraction and `Char.code`/`Char.chr` avoid manual casting.

## Key Differences

| Aspect | Rust | OCaml |
|--------|------|-------|
| Map over string | `.chars().map(f).collect()` | `String.map f s` |
| Char to int | `c as u8` | `Char.code c` |
| Int to char | `x as char` | `Char.chr x` |
| Byte ranges | `b'a'..=b'z'` | `c >= 'a' && c <= 'z'` |
| In-place | `Vec<u8>` + `from_utf8` | Not idiomatic |
| Decrypt | `caesar(26 - n)` | `caesar (26 - n)` |

The three Rust implementations cover three idioms: functional map/collect, fold with pre-allocated buffer, and low-level byte mutation. In practice, the iterator version is the most idiomatic; the byte version is only useful when profiling confirms allocation overhead is significant.

## Exercises

1. Add a `rot13(s: &str) -> String` convenience function that calls `caesar(13, s)`.
2. Implement a Vigenère cipher: instead of a fixed shift, accept a `key: &str` and cycle through its characters as shifts using `.cycle()`.
3. Write `shift_char_wrapping` that also handles digits (0–9 shifted mod 10) in addition to letters.
4. Benchmark the three implementations for a 1MB string and compare throughput.
5. In OCaml, implement the Vigenère cipher using `Seq.zip` to pair message characters with cycled key characters.
