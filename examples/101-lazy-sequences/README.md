# Example 101: Lazy Sequences

**Difficulty:** ⭐⭐
**Category:** Lazy/Infinite Sequences
**OCaml Source:** [OCaml Docs — Seq Module](https://ocaml.org/docs/)

## Problem Statement

Create infinite lazy sequences: natural numbers, Fibonacci numbers, and primes. Take only what you need without computing the rest.

## Learning Outcomes

- Map OCaml's `Seq` module to Rust's `Iterator` trait
- Use `std::iter::successors` and `std::iter::from_fn` for custom infinite iterators
- Understand that Rust iterators are lazy by default (no `Seq` module needed)
- Implement `unfold` — the dual of `fold`

## Key Insight

OCaml added lazy sequences (`Seq`) in 4.07 as an explicit opt-in. Rust's iterators are lazy by default — `(0..)` creates an infinite range that only computes values when consumed. This means Rust doesn't need a separate "lazy" abstraction; the standard iterator *is* the lazy sequence.
