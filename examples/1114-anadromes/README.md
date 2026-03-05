# Example 1114: Anadromes

**Difficulty:** ⭐⭐
**Category:** String Processing | Sets | Iterator Combinators
**OCaml Source:** Rosetta Code — Anadromes task

## Problem Statement

Find all pairs of words in a dictionary where one word is the reverse of the other (e.g., "stressed" / "desserts", "repaid" / "diaper"). Each pair should appear only once and only words longer than a given minimum length are considered.

## Learning Outcomes

- How to reverse a string idiomatically using `.chars().rev().collect()` instead of byte-level indexing
- Using `BTreeSet` as an ordered set for deterministic iteration and O(log n) membership testing
- Applying `filter_map` to combine the filter and transform steps in one clean combinator chain
- Avoiding duplicate pairs by exploiting lexicographic ordering (`s < reverse(s)`)

## OCaml Approach

OCaml uses a functor-based set (`Set.Make(String)`) for ordered, immutable membership testing. The `Seq` module provides lazy sequences, so the pipeline — read, filter by length, lowercase, insert into set, scan for anadromes — is expressed as a chain of sequence combinators that mirrors Haskell-style data-flow. The duplicate-elimination trick (`s < r`) works identically to the Rust version.

## Rust Approach

Rust uses `BTreeSet<String>` (a sorted, owned set) which provides the same ordered iteration and `O(log n)` `.contains()` as OCaml's `Set`. The pipeline is built from iterator adapters: `.map()` for lowercasing, `.filter()` for length, and `.filter_map()` for the anadrome detection step. Ownership is handled by cloning only when a pair is found, keeping allocation minimal.

## Key Differences

1. **String reversal:** OCaml uses `String.init` with index arithmetic; Rust uses `.chars().rev().collect()` — char-safe, no manual index math.
2. **Set type:** OCaml's `Set.Make` is a persistent functional balanced BST; Rust's `BTreeSet` is an imperative B-tree, but both give deterministic (sorted) iteration.
3. **Laziness:** OCaml `Seq` is lazy by default; Rust iterators are also lazy, only evaluated when `.collect()` or a consuming adapter is called.
4. **Duplicate avoidance:** Both use the same `s < rev(s)` trick — because iteration is in sorted order, the smaller string always appears first, eliminating the need for a visited set.
