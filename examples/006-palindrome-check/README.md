# Example 006: Palindrome Check

**Difficulty:** ⭐
**Category:** Lists & Comparison
**OCaml Source:** OCaml 99 Problems #6

## Problem Statement

Determine whether a list is a palindrome — i.e., it reads the same forwards and backwards.

## Learning Outcomes

- Compare forward and backward iteration without allocation
- Understand `DoubleEndedIterator` in Rust vs `List.rev` in OCaml
- See how Rust slices enable O(1) indexed access (unlike OCaml linked lists)
- Appreciate the difference between owned reversal (`Vec`) and lazy reversal (iterators)
- Practice generic functions with `PartialEq` bounds

## OCaml Approach

OCaml uses `List.rev` to reverse the linked list, then compares structurally with `=`. The manual version explicitly recurses through both lists. Because OCaml lists are immutable and GC-managed, the reversed copy is cheap to create and automatically freed.

## Rust Approach

Three approaches offered:
1. **Index-based**: Direct slice indexing — compare `list[i]` with `list[n-1-i]`, only half the list
2. **Rev + clone**: Build a reversed `Vec` (mirrors OCaml), requires `Clone`
3. **Iterator zip**: `iter().eq(iter().rev())` — zero allocation, lazy evaluation

## Key Differences

1. **No allocation needed**: Rust's `DoubleEndedIterator` on slices means we can iterate backwards without creating a reversed copy
2. **Trait bounds explicit**: Rust requires `PartialEq` (and `Clone` for the rev approach) — OCaml's structural equality is implicit
3. **Slices vs linked lists**: Rust slices give O(1) random access; OCaml lists are O(n) for indexing, so reversal is the natural approach
4. **Lazy vs eager**: The iterator approach in Rust short-circuits on first mismatch; OCaml's `List.rev` always builds the full reversed list
5. **No GC**: Rust's stack-allocated slice references need no garbage collection
