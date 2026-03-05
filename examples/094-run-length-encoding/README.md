📖 **[View on hightechmind.io →](https://hightechmind.io/rust/094-run-length-encoding)**

---

# Example 094: Run-Length Encoding — String Compression

**Difficulty:** ⭐
**Category:** String Processing
**OCaml Source:** [Exercism — Run-Length Encoding](https://exercism.org/tracks/ocaml/exercises/run-length-encoding)

## Problem Statement

Encode a string by replacing consecutive identical characters with count+character (e.g., "AABCCCDEEEE" → "2AB3CD4E"). Single characters have no count prefix.

## Learning Outcomes

- Compare OCaml's `Buffer` with Rust's `String` for incremental building
- Practice grouping consecutive elements in both languages
- See fold-based approaches to run-length grouping

## Key Insight

OCaml uses `Buffer.create` for efficient string building; Rust's `String` is already a growable buffer. The grouping logic (track current char + count) is nearly identical in both languages, but Rust's `match` on mutable last element enables elegant in-place grouping.
