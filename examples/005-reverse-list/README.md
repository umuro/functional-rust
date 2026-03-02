# Example 005: Reverse a List

**Difficulty:** ⭐  
**Category:** Lists & Accumulator Pattern  
**OCaml Source:** OCaml.org 99 Problems #5

## Problem Statement

Reverse the order of elements in a list. Return a new reversed list (immutable semantics in OCaml).

## Learning Outcomes

- The accumulator pattern: building results incrementally
- `iter().rev()` and lazy iterator reversal in Rust
- `fold` for constructing reversed collections
- In-place mutation (`slice.reverse()`) as the zero-allocation alternative
- Clone trait requirements when creating new collections from references

## OCaml Approach

Classic tail-recursive accumulator: `aux (h :: acc) t` prepends each element to the accumulator, naturally reversing the order. This is O(n) time and O(n) space, and is how `List.rev` is implemented.

## Rust Approach

Four approaches: (1) `iter().rev().cloned().collect()` — idiomatic and lazy, (2) `fold` with `insert(0, ...)` — mirrors OCaml's prepend pattern (but O(n²)), (3) recursive with accumulator, and (4) in-place `slice.reverse()` for owned data.

## Key Differences

1. **Mutation option:** Rust can reverse in-place with zero allocation; OCaml always creates a new list
2. **Iterator laziness:** `iter().rev()` doesn't traverse — it reverses the iteration direction. Only `collect()` materializes
3. **Clone requirement:** Creating a new `Vec<T>` from `&[T]` requires `T: Clone`; OCaml copies implicitly
4. **Fold efficiency:** OCaml's `h :: acc` is O(1) prepend (cons cell); Rust's `insert(0, ...)` is O(n) (shifts elements)
5. **Ownership model:** Rust distinguishes "reverse a borrowed slice" (returns new Vec) from "reverse owned data" (in-place) — OCaml has only the former
