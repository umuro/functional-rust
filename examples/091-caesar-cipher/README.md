# Example 091: Caesar Cipher — Functional Encryption

**Difficulty:** ⭐
**Category:** String Processing
**OCaml Source:** [Exercism — Rotational Cipher](https://exercism.org/tracks/ocaml/exercises/rotational-cipher)

## Problem Statement

Implement a Caesar cipher that shifts each letter by N positions in the alphabet, wrapping around. Non-alphabetic characters remain unchanged.

## Learning Outcomes

- Map OCaml's `String.map` to Rust's `chars().map().collect()`
- Use Rust's range patterns (`'a'..='z'`) for character classification
- Compare byte-level vs char-level string processing
- Understand modular arithmetic for wrapping

## Key Insight

OCaml's `String.map` applies a function to each character, returning a new string. Rust has no direct `String::map`, but `chars().map(f).collect::<String>()` is the idiomatic equivalent. For ASCII-only work, operating on bytes is faster.
