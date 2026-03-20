📖 **[View on hightechmind.io →](https://hightechmind.io/rust/271-sublist-classification)**

---

# Example 271: Sublist Classification

**Difficulty:** ⭐⭐  
**Category:** Lists & HOF | Pattern Matching  
**OCaml Source:** Classic list relationship problem

## Problem Statement

Given two lists, classify their relationship: one list is equal to, a sublist of, a superlist of, or completely unequal to the other.

## Learning Outcomes

- How `slice::windows` enables elegant contiguous subsequence searching
- How slice pattern matching (`[h, rest @ ..]`) mirrors OCaml's `h :: t` destructuring
- The idiomatic Rust approach to replacing recursive list walks with iterator combinators
- How a sum type (`enum`) maps directly from OCaml's variant type to Rust

## OCaml Approach

OCaml defines a recursive `starts_with` helper and `is_sublist` that walks the list head-by-head, checking at every position whether the prefix matches. The `classify` function then uses equality (`=`) as a structural check before delegating to the sublist tests.

## Rust Approach

The idiomatic Rust solution uses `slice::windows(n)` to generate all contiguous sub-slices of length `n` and checks if any equals the target — a single iterator expression replaces the recursive walk. The recursive solution preserves the OCaml structure exactly using slice patterns with rest bindings (`[h, rest @ ..]`).

## Key Differences

1. **List walking:** OCaml recurses on `h :: t`; Rust iterates via `.windows()` or uses slice patterns
2. **Equality:** OCaml's structural `=` works on lists; Rust requires `PartialEq` bound on the generic `T`
3. **Empty check:** OCaml matches `sub = []`; Rust uses `sub.is_empty()`
4. **Type:** OCaml `type relation = ...` is a sum type; Rust `enum Relation` is identical in concept

## Exercises

1. Extend the sublist classifier to return the starting index at which the first list appears within the second (for the `Sublist` case), returning `None` if not present.
2. Implement a `longest_common_subsequence` function that returns the LCS of two lists, and use it to check that the LCS of a sublist with its superlist equals the sublist.
3. Define a lattice of sublist relations (Equal < Sublist < Superlist < Unordered) and implement a `most_specific` function that classifies the relationship between a query list and a collection of lists.
