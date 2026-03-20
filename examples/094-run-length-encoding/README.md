[![hightechmind.io](https://img.shields.io/badge/hightechmind.io-functional--rust-blue)](https://hightechmind.io)

# 094 — Run-Length Encoding

## Problem Statement

Compress a string using run-length encoding: replace consecutive repeated characters with a count followed by the character (e.g. `"AAABCC"` → `"3AB2C"`). Implement both an imperative index-based version and a functional grouping version. Compare with OCaml's `Buffer`-based recursive implementation.

## Learning Outcomes

- Traverse a character vector with indices to detect character boundaries
- Build a `Vec<(char, usize)>` of groups using `last_mut` and pattern matching
- Use `format!` conditionally to omit the count `1` from single-character runs
- Understand `String::push` and `push_str` for incremental string building
- Map Rust's `String` building to OCaml's `Buffer.add_char`/`Buffer.add_string`
- Recognise `decode` as the inverse: parse optional count, then character

## Rust Application

The imperative `encode` collects `chars()` into a `Vec<char>`, then scans with an index comparing `chars[i]` to `chars[i-1]`. When a boundary is reached, it writes the run. The functional `encode_functional` builds a `Vec<(char, usize)>` groups using `last_mut()` — if the last group matches the current char, increment count; otherwise push a new group. Both finish with `.map(|(c, n)| if n > 1 { format!("{}{}", n, c) } else { c.to_string() }).collect::<String>()`.

## OCaml Approach

OCaml uses `Buffer.create` and `Buffer.add_char`/`Buffer.add_string` for efficient incremental building. The recursive `go i c count` function scans the string, writing to the buffer at boundaries. `Buffer.contents` materialises the final string. OCaml's `Buffer` is semantically equivalent to Rust's `String::push`/`push_str` pattern.

## Key Differences

| Aspect | Rust | OCaml |
|--------|------|-------|
| String builder | `String` with `push`/`push_str` | `Buffer` with `add_char`/`add_string` |
| Group detection | `last_mut()` + match | Recursive `go i c count` |
| Format number | `count.to_string()` | `string_of_int count` |
| Collect groups | `Vec<(char, usize)>` | Implicit in recursion |
| Single chars | `c.to_string()` | `Buffer.add_char buf c` |
| Pre-allocation | `String::with_capacity` | `Buffer.create n` |

Run-length encoding is a classic interview problem that tests string building, grouping, and edge-case handling (empty string, single characters, long runs). The `decode` inverse is equally instructive: parse optional digits, then a mandatory character.

## Exercises

1. Implement `decode(s: &str) -> String` that reverses the encoding: parse the optional number prefix and repeat the following character.
2. Handle multi-digit run counts (e.g. `"100A"` → 100 `A`s) in `decode`.
3. Write a property test: `decode(encode(s)) == s` for any string of alphabetic characters.
4. Implement encode for `Vec<T: PartialEq>` (not just strings) returning `Vec<(T, usize)>`.
5. In OCaml, implement `decode` using `Scanf` to parse the optional integer and mandatory character.
