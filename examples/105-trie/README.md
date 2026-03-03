# Example 105: Trie — Prefix Tree for Strings

**Difficulty:** ⭐⭐⭐
**Category:** Trees
**OCaml Source:** [Rosetta Code — Trie](https://rosettacode.org/wiki/Trie#OCaml)

## Problem Statement

Implement a trie (prefix tree) for storing and looking up strings. Support insert, membership check, and prefix search.

## Learning Outcomes

- Map OCaml's `Map.Make(Char)` to Rust's `HashMap<char, Trie>`
- Compare functional (immutable) vs mutable trie implementations
- See array-based tries for performance-critical ASCII workloads
- Understand recursive tree construction in both languages

## Key Insight

OCaml's trie uses `Map.Make(Char)` for children — an immutable sorted map created via functor. Rust offers three natural choices: `HashMap` (fast, unordered), `BTreeMap` (sorted, like OCaml), or `[Option<Box<Trie>>; 26]` (array-based, fastest for ASCII). The mutable `HashMap` version is idiomatic Rust; the `BTreeMap` version mirrors OCaml's semantics.
