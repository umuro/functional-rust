# Example 982: List.flatten — Flatten Nested Lists

**Difficulty:** ⭐
**Category:** stdlib-list
**OCaml Source:** OCaml standard library `List.flatten` / `List.concat_map`

## Problem Statement

Given a list of lists, concatenate them all into a single flat list. Also demonstrates `concat_map`, which applies a function to each element and flattens the results simultaneously.

## Learning Outcomes

- How OCaml's `List.flatten` maps directly to Rust's `.flatten()` iterator adapter
- How slice pattern matching (`[head, rest @ ..]`) enables recursive list processing in Rust
- How `List.concat_map` maps to `.flat_map()` in Rust iterators
- The difference between owned (`Vec<T>`) and borrowed (`&[Vec<T>]`) representations of nested lists

## OCaml Approach

OCaml provides `List.flatten : 'a list list -> 'a list` as a stdlib function that concatenates a list of lists. `List.concat_map` extends this by applying a function first, then flattening — equivalent to `List.flatten (List.map f lst)` but in a single pass.

## Rust Approach

Rust's iterator provides `.flatten()` which works on any iterator of iterables, and `.flat_map()` which combines map and flatten. The idiomatic approach chains these adapters. A recursive solution uses slice patterns to destructure the nested structure head-by-head, mirroring OCaml's recursive list decomposition.

## Key Differences

1. **List representation:** OCaml uses linked lists (`'a list list`); Rust uses `&[Vec<T>]` — a slice of owned vectors
2. **Ownership:** The idiomatic Rust solution borrows the input and clones elements; OCaml copies freely via GC
3. **concat_map:** OCaml's `List.concat_map` has a direct Rust equivalent in `.flat_map()` on iterators
4. **Pattern matching:** Both languages support head/tail decomposition, but Rust uses `[head, rest @ ..]` slice patterns
