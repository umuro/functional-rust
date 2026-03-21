📖 **[View on hightechmind.io →](https://hightechmind.io/rust/062-isogram-check)**

---

# 062 — Isogram Check
**Difficulty:** ⭐  
**Category:** Functional Programming  



## Problem Statement

An isogram is a word where no letter appears more than once (ignoring case, hyphens, and spaces). Examples: "lumberjacks", "background", "downstream". Non-examples: "hello" (two l's), "Alabama" (three a's). This problem from Exercism exercises set-based duplicate detection.

Isogram detection appears in spell-checking (detecting repeated characters in passwords), password strength metrics (no repeated chars = higher entropy), and linguistics analysis. The problem has three O(n) solutions with different constant factors: sort-and-check (O(n log n)), HashSet, and bitset (26 bits for 26 letters).

## Learning Outcomes

- Use `HashSet` for O(1) membership testing and duplicate detection
- Filter non-alphabetic characters with `is_ascii_alphabetic()`
- Normalize case with `to_ascii_lowercase()`
- Implement the early-exit version for better performance on failing inputs
- Use a 32-bit integer as a bitset for 26 letters (most cache-friendly approach)

## Rust Application

`is_isogram` collects lowercase letters to a `Vec<char>`, builds a `HashSet<char>`, and compares lengths — equal iff no duplicates. `is_isogram_early_exit` inserts into a `HashSet` char by char, returning `false` immediately on first duplicate (`insert` returns `false` if already present). `is_isogram_bitflag` uses `seen: u32` with `1 << (c - 'a')` bit positions — fastest for ASCII, no heap allocation.

## OCaml Approach

OCaml's version using a module set: `module CharSet = Set.Make(Char)`. `let is_isogram s = let chars = String.to_seq s |> Seq.filter Char.is_letter |> Seq.map Char.lowercase_ascii |> List.of_seq in List.length chars = CharSet.cardinal (CharSet.of_list chars)`. The `Set` approach mirrors the `HashSet` approach but with a balanced BST (O(log n) operations, O(n log n) total).

## Key Differences

1. **`HashSet` vs `Set.Make`**: Rust's `HashSet` has O(1) average insert/lookup. OCaml's `Set.Make` is a balanced BST with O(log n) operations. For 26 possible keys, the difference is negligible but structural.
2. **`insert` return value**: Rust's `HashSet::insert` returns `bool` — `false` if already present. This enables the early-exit pattern. OCaml's `Set.add` does not return whether the element was new.
3. **Bitset**: The u32 bitflag approach is not idiomatically OCaml (it's C-style). OCaml functional code prefers `Set`. Rust's systems heritage makes bitsets natural and common.
4. **Unicode**: `is_ascii_alphabetic()` handles only ASCII letters. For Unicode isogram checking, use `char::is_alphabetic()` and `to_lowercase()` (which returns an iterator for multi-char lowercasing).

## Exercises

1. **Anagram check**: Two words are anagrams if they use the same letters (with repetition). Write `is_anagram(a: &str, b: &str) -> bool` using sorted character vectors.
2. **Unique characters count**: Write `unique_letter_count(s: &str) -> usize` that counts distinct alphabetic characters. Use `HashSet` or bitset.
3. **Pangram from isogram**: What is the shortest English sentence that is both a pangram (uses all 26 letters) and an isogram (no repeated letters)? Research "the quick brown fox" and verify it is not an isogram.
