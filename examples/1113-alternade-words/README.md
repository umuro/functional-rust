# Example 1113: Alternade Words

**Difficulty:** ⭐⭐
**Category:** Strings
**OCaml Source:** [Rosetta Code — Alternade words](https://rosettacode.org/wiki/Alternade_words)

## Problem Statement

An alternade word is a word that can be split into two other valid words by taking alternating characters: even-indexed characters form one word, odd-indexed characters form another, and both must appear in the same dictionary.

## Learning Outcomes

- Using `.step_by()` and `.skip()` to stride through iterators — a direct, zero-allocation way to interleave characters
- Building a `HashSet<&str>` from a slice for O(1) membership tests, borrowing strings from the input
- Chaining `.filter()` and `.filter_map()` to express a multi-condition pipeline without intermediate allocations
- How OCaml's `String.init` with an index closure maps cleanly to Rust's iterator-based character collection

## OCaml Approach

OCaml builds the two alternades with `String.init`, computing the target length as `(n+1) lsr 1` (ceil) and `n lsr 1` (floor), and indexing directly into the string with `s.[i + i]` and `s.[i + succ i]`. The word set is a `StrSet` (a balanced BST via `Set.Make(String)`), and the pipeline uses `Seq` for lazy streaming.

## Rust Approach

Rust uses iterator combinators throughout: `chars().step_by(2)` for even characters and `chars().skip(1).step_by(2)` for odd. The word set is a `HashSet<&str>`, borrowing the original string data. The entire lookup pipeline is a single `.filter_map()` chain returning formatted strings.

## Key Differences

1. **Character indexing:** OCaml indexes bytes with `s.[i]`; Rust uses `chars().step_by()` for correct Unicode support
2. **Set type:** OCaml uses a functional BST (`Set.Make`); Rust uses `HashSet` for O(1) lookup (vs O(log n))
3. **String building:** OCaml uses `String.init` with a length and index function; Rust collects a filtered iterator directly with `.collect::<String>()`
4. **Lifetime borrowing:** Rust's `HashSet<&str>` borrows from the input slice, requiring no heap copies; OCaml strings are always heap-allocated values
