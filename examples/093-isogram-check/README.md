[![hightechmind.io](https://img.shields.io/badge/hightechmind.io-functional--rust-blue)](https://hightechmind.io)

# 093 — Isogram Check

## Problem Statement

Determine whether a word is an isogram — no letter appears more than once (ignoring case and non-alphabetic characters). Implement three approaches: sort + dedup, `HashSet` with early exit via `all`, and bitset with early exit on duplicate bit. Compare with OCaml's `List.sort_uniq` approach.

## Learning Outcomes

- Use `sort_unstable` + `dedup` and compare lengths to detect duplicates
- Exploit `HashSet::insert` returning `false` on duplicates with `.all(|c| seen.insert(…))`
- Use a bitset with `bits & mask != 0` for O(1) duplicate detection with early exit
- Apply `is_ascii_alphabetic()` + `to_ascii_lowercase()` for letter normalisation
- Map Rust's three approaches to OCaml's `List.sort_uniq Char.compare`
- Identify when early-exit (HashSet/bitset) outperforms sort-based approaches

## Rust Application

The sort approach collects letters, sorts, deduplicates, and checks the length is unchanged — O(n log n). The `HashSet` approach uses `seen.insert(c)` — which returns `false` if `c` was already in the set — and `.all(…)` short-circuits on the first `false`. This is O(n) average with early exit. The bitset approach sets and tests individual bits — O(n) with early exit and O(1) space (just a `u32`). All three ignore non-alphabetic characters via `filter` or `is_ascii_alphabetic()`.

## OCaml Approach

OCaml's `List.sort_uniq Char.compare` sorts and removes duplicates in one pass (O(n log n)). Comparing `List.length chars = List.length unique` is the same length-based test. OCaml lacks a direct `HashSet::insert`-returns-bool idiom; the functional approach is more natural with the sort-based method.

## Key Differences

| Aspect | Rust Sort | Rust HashSet | Rust Bitset | OCaml |
|--------|-----------|-------------|-------------|-------|
| Time | O(n log n) | O(n) avg | O(n) | O(n log n) |
| Early exit | No | Yes | Yes | No |
| Space | O(n) | O(n) | O(1) | O(n) |
| Code length | Short | Short | Short | Short |
| Readability | High | High | Medium | High |

For correctness and simplicity, the `HashSet` version is preferred. For performance-critical code with short strings, the bitset version wins. The sort version matches OCaml's natural idiom.

## Exercises

1. Extend to Unicode: use `to_lowercase().next()` for multi-codepoint character normalisation.
2. Write `isogram_score(s: &str) -> usize` that returns the number of unique letters.
3. Implement a version that returns the first repeated character as `Option<char>`.
4. Add a `longest_isogram(words: &[&str]) -> Option<&str>` that finds the longest isogram in a word list.
5. In OCaml, implement the `HashSet`-equivalent using `Hashtbl` and early exit via an exception.
