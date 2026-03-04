# 500: String Compression (Run-Length Encoding)

**Difficulty:** 2  **Level:** Beginner-Intermediate

Compress a string by replacing runs of identical characters with a count and character: `"aabbbcc"` → `"2a3b2c"`.

## The Problem This Solves

Run-length encoding (RLE) is the simplest useful compression algorithm. Where data contains long runs of repeated values — bitmap images, simple logs, test fixtures — RLE can cut size dramatically with zero dependencies and O(n) time.

Beyond compression, the encode/decode cycle is a clean exercise in iterator-driven state machines. Real-world uses include BMP pixel data, PCX image format, certain network protocols, and any domain where you want a human-readable compact form of repeated values.

This example also shows functional-style accumulation with `fold` instead of imperative loops with manual index management.

## The Intuition

Walk the string one character at a time. Keep a "current character" and a running count. When the character changes, flush the current run to the output and start a new one. At the end, flush the last run.

For decoding, just repeat each character by its count.

## How It Works in Rust

**Encoding with `fold`** — treats the accumulator as `(current_char, count, output_vec)`:
```rust
fn encode(s: &str) -> Vec<(usize, char)> {
    let mut chars = s.chars();
    let first = match chars.next() {
        None => return vec![],
        Some(c) => c,
    };
    let (cur, count, mut acc) =
        chars.fold((first, 1usize, Vec::new()), |(cur, count, mut acc), c| {
            if c == cur {
                (cur, count + 1, acc)
            } else {
                acc.push((count, cur));
                (c, 1, acc)
            }
        });
    acc.push((count, cur));
    acc
}
```
The `fold` carries three pieces of state without any mutable variable in scope — it threads state purely through the accumulator tuple.

**Decoding with `fold`** — builds a `String` by repeating each char:
```rust
fn decode(pairs: &[(usize, char)]) -> String {
    pairs.iter().fold(String::new(), |mut s, &(n, c)| {
        for _ in 0..n { s.push(c); }
        s
    })
}
```

**Formatting** — `map` + `collect` over the pairs:
```rust
fn show_encoded(pairs: &[(usize, char)]) -> String {
    pairs.iter().map(|(n, c)| format!("{}{}", n, c)).collect()
}
```

The roundtrip invariant `decode(encode(s)) == s` holds for all valid ASCII and UTF-8 strings.

## What This Unlocks

- **Functional state machines** — `fold` with a compound accumulator models step-by-step state transitions without mutable variables.
- **Codec pattern** — encode/decode as inverse functions tested by roundtrip property.
- **Iterator composition** — building output strings from iterators without intermediate allocations for each element.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Fold with tuple state | `List.fold_left` with tuple | `Iterator::fold` with tuple accumulator |
| Mutable output in fold | `Buffer.t` passed through | `mut` captured inside closure, or `Vec` in accumulator |
| Repeat a char n times | `String.make n c` | `for _ in 0..n { s.push(c) }` |
| Char comparison | `Char.(=)` | `==` on `char` |
