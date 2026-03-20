# Example 1182: List.flatten — Flatten Nested Lists

**Difficulty:** ⭐
**Category:** Lists & HOF
**OCaml Source:** OCaml standard library — `List.flatten`, `List.concat_map`

## Problem Statement

Given a list of lists, produce a single flat list containing all elements in order. Also demonstrate `concat_map` (flat_map), which maps each element to a list and flattens the results.

## Learning Outcomes

- How `Iterator::flatten` mirrors OCaml's `List.flatten` with zero manual recursion
- How `flat_map` / `Iterator::flat_map` corresponds to OCaml's `List.concat_map`
- Recursive slice pattern matching with `[head, rest @ ..]` to process nested structure
- Why `Clone` is needed when flattening slices of owned `Vec<T>` values

## OCaml Approach

OCaml provides `List.flatten` as a built-in that concatenates a list of lists into one. `List.concat_map f xs` is equivalent to `List.flatten (List.map f xs)` but more efficient. The recursive implementation uses the `@` operator to append the head list to the recursively flattened tail.

## Rust Approach

Rust's `Iterator::flatten` works on any iterator of iterables. Combined with `.cloned()` it converts `&[Vec<T>]` to `Vec<T>` without explicit loops. `flat_map` is the direct counterpart to `concat_map`: it maps each element to an iterator and flattens the results. The recursive version uses Rust's slice pattern `[head, rest @ ..]` to destructure the nested structure idiomatically.

## Key Differences

1. **Flattening:** OCaml uses `List.flatten`; Rust uses `.iter().flatten().cloned().collect()`
2. **concat_map:** OCaml has `List.concat_map f xs`; Rust uses `.iter().flat_map(f).collect()`
3. **Append:** OCaml's `@` operator concatenates lists; Rust uses `Vec::extend` or iterator chaining
4. **Ownership:** Rust requires `.cloned()` because iterating `&Vec<T>` yields `&T` references, not owned values
