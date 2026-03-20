# Example 1181: List.partition — Divide List by Predicate

**Difficulty:** ⭐
**Category:** Lists & HOF
**OCaml Source:** OCaml standard library `List.partition`

## Problem Statement

Split a list (or slice) into two groups based on a predicate: elements satisfying the predicate go into the first group, the rest into the second.

## Learning Outcomes

- How Rust's `Iterator::partition` mirrors OCaml's `List.partition` directly
- How to write the same logic as a fold accumulator (functional Rust style)
- How slice pattern matching enables recursive list processing in Rust
- The difference between `Fn(&T) -> bool` (idiomatic) and double-deref in iterator closures

## OCaml Approach

OCaml provides `List.partition : ('a -> bool) -> 'a list -> 'a list * 'a list` in the standard library. The recursive implementation uses structural pattern matching on the cons list, prepending to the matching or non-matching accumulator on each step.

## Rust Approach

Rust's `Iterator::partition` mirrors OCaml's `List.partition` almost exactly: it consumes the iterator and returns a pair of collections. Working on slices (`&[T]`) rather than owned lists preserves the data without allocation, while the predicate receives `&T` (a reference to each element).

## Key Differences

1. **Types:** OCaml returns `'a list * 'a list` (owned copies); Rust returns `(Vec<&T>, Vec<&T>)` (references into the original slice) — zero copying.
2. **Predicate arity:** OCaml predicate takes `'a`; Rust predicate takes `&T` because we iterate over references.
3. **Recursion depth:** OCaml's recursive `partition_rec` is idiomatic; in Rust, deep recursion on slices risks stack overflow — prefer `Iterator::partition` or fold.
4. **Allocation:** OCaml lists are singly-linked and always heap-allocated; Rust slices are contiguous and borrowed, allocation only happens for the result `Vec`s.
