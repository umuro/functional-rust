📖 **[View on hightechmind.io →](https://hightechmind.io/rust/010-run-length-encoding)**

---

# Example 010: Run-Length Encoding
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Use the result of Problem 9 (pack consecutive duplicates) to implement run-length encoding: replace consecutive duplicate elements with `(count, element)` pairs.

## Learning Outcomes

- Compose solutions by building on previous problems (pack → encode)
- Use tuple types `(usize, T)` as lightweight data containers
- Practice the fold pattern for single-pass encoding
- Compare OCaml's tuple-based encoding with Rust's strongly-typed tuples
- Understand function composition: `pack().map()` vs direct iteration

## OCaml Approach

First packs consecutive elements (reusing Problem 9's `pack`), then maps each group to a `(count, element)` tuple. The direct version counts in a single pass with a recursive helper.

## Rust Approach

1. **Compose with pack**: Pack first, then `map` groups to `(len, first)` — mirrors OCaml
2. **Fold**: Single-pass fold, matching on `last_mut()` to increment count or start new run
3. **Direct**: Imperative single-pass with counter — most efficient, no intermediate structures

## Key Differences

1. **Tuple types**: Rust's `(usize, T)` is structurally typed like OCaml's `int * 'a`, but Rust tuples own their data
2. **Composition**: `pack().into_iter().map()` chains naturally — Rust's iterator adaptors mirror OCaml's `List.map`
3. **Single-pass efficiency**: The fold and direct versions avoid creating intermediate packed groups
4. **`usize` vs `int`**: Rust uses `usize` for counts (unsigned, pointer-sized); OCaml uses `int` (signed, word-sized)
5. **No intermediate allocation**: The direct version builds the result in one pass — important for large inputs

## Exercises

1. Implement the decoder: given a run-length encoded `Vec<(usize, T)>`, reconstruct the original `Vec<T>`.
2. Write a modified RLE that only encodes runs of length greater than 1 — single elements are stored as-is using an enum `Rle<T> { Single(T), Run(usize, T) }`.
3. Implement RLE for a stream of bytes and measure its compression ratio on a sample ASCII text; then try to compress already-compressed data and observe the result.
