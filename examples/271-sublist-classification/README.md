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
