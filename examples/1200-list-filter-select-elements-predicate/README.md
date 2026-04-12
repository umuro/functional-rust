# Example 1200: List Filter — Select Elements by Predicate

**Difficulty:** ⭐  
**Category:** Lists & HOF  
**OCaml Source:** Standard OCaml — `List.filter`

## Problem Statement

Given a list of integers, keep only those elements satisfying a boolean predicate. Demonstrates `List.filter` as a higher-order function that decouples the iteration strategy from the selection logic.

## Learning Outcomes

- How OCaml's `List.filter` maps to Rust's `Iterator::filter` combinator
- The difference between returning borrowed refs (`Vec<&T>`) vs owned values (`Vec<T>`) in Rust
- How to write a recursive Rust filter without hitting infinite type instantiation (use an inner `fn go` with `&dyn Fn`)
- Closures as first-class predicates in both languages

## OCaml Approach

OCaml's `List.filter pred lst` is a standard library function implemented by structural recursion. The predicate is a plain closure `fun x -> x mod 2 = 0`. The result is a new list — OCaml lists are immutable and singly-linked, so this allocates fresh cons cells.

## Rust Approach

Rust's iterator chain `list.iter().filter(|x| predicate(x)).collect()` mirrors OCaml semantically. The key decision is whether to return `Vec<&T>` (borrowed, zero-copy) or `Vec<T>` (owned, requires `Clone`). The idiomatic choice depends on the caller's ownership needs. For the recursive variant, a nested `fn go` with `&dyn Fn` avoids the infinite monomorphization that arises from passing `&predicate` recursively with a generic `F`.

## Key Differences

1. **Allocation model:** OCaml creates a new linked list; Rust collects into a `Vec` (contiguous memory).
2. **Ownership:** Rust distinguishes `Vec<&T>` (borrowed refs) from `Vec<T>` (owned copies) — OCaml has no such split.
3. **Recursion helper:** Rust's monomorphic generics require an inner `fn go(&dyn Fn)` to recurse without type explosion; OCaml recurses freely.
4. **Predicate type:** OCaml uses `'a -> bool`; Rust uses `Fn(&T) -> bool`, explicitly borrowed.
