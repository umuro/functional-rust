[![hightechmind.io](https://img.shields.io/badge/hightechmind.io-functional--rust-blue)](https://hightechmind.io)

# 096 — Exact Size Iterator
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  


## Problem Statement

Demonstrate Rust's `ExactSizeIterator` trait — iterators that know their length in O(1). Show that `Vec::iter()`, ranges, and `enumerate` all implement `ExactSizeIterator`, enabling `.len()` without traversal. Demonstrate `chunks_exact` for obtaining only full-sized chunks plus a remainder. Compare with OCaml's O(1) `Array.length` versus O(n) `List.length`.

## Learning Outcomes

- Call `.len()` on any `ExactSizeIterator` in O(1)
- Understand that `.len()` on iterators is distinct from `.count()` (which consumes)
- Use `chunks_exact(n)` to iterate only complete chunks and access the remainder
- Distinguish `ExactSizeIterator` (knows length) from `Iterator` (does not)
- Map Rust's array/Vec O(1) length to OCaml's `Array.length` O(1)
- Recognise when knowing size ahead of time enables `Vec::with_capacity`

## Rust Application

`v.iter().len()` returns the number of remaining elements in O(1) — the slice iterator stores its length. `(0..10).len()` is O(1) by subtraction. Both implement `ExactSizeIterator`, which extends `Iterator` with `fn len(&self) -> usize`. `chunks_exact(2)` yields only complete 2-element chunks; `chunks_exact(2).remainder()` provides the leftover elements that didn't fit a full chunk. `enumerate` wraps another `ExactSizeIterator` and preserves `ExactSizeIterator`.

## OCaml Approach

OCaml arrays have O(1) `Array.length`; lists require O(n) `List.length`. There is no `ExactSizeIterator` protocol — arrays inherently carry their length, and sequential iteration with `for i = 0 to Array.length a - 1 do` naturally provides sized bounds. `List.mapi` provides enumerate-style indexed iteration.

## Key Differences

| Aspect | Rust | OCaml |
|--------|------|-------|
| O(1) length | `ExactSizeIterator::len()` | `Array.length` |
| O(n) length | `Iterator::count()` (consumes!) | `List.length` |
| Chunks with remainder | `chunks_exact(n).remainder()` | Manual split |
| Enumerate | `.enumerate()` (preserves ExactSize) | `List.mapi` |
| Protocol | Trait `ExactSizeIterator` | No equivalent |
| Consuming length | `.count()` drains iterator | `Seq.length` or `List.length` |

`ExactSizeIterator::len()` is non-consuming — unlike `.count()`, it does not advance the iterator. This is critical when you want to pre-allocate (`Vec::with_capacity(iter.len())`) without losing the ability to iterate.

## Exercises

1. Write `collect_with_capacity<I: ExactSizeIterator<Item=T>>(iter: I) -> Vec<T>` that pre-allocates exactly the right capacity.
2. Implement `ExactSizeIterator` for a custom `Counter` struct that counts from `start` to `end`.
3. Show that `.map(f)` on an `ExactSizeIterator` preserves `ExactSizeIterator` by checking that the mapped iterator also has `.len()`.
4. Use `chunks_exact(3).remainder()` to process a byte buffer in 3-byte UTF-8 sequences, handling the partial final chunk separately.
5. In OCaml, implement a `sized_seq` type that pairs a `Seq.t` with a known `int` length and implement `map` that preserves the size.
