# Example 103: Unfold — Generating Sequences from Seeds

**Difficulty:** ⭐⭐
**Category:** Higher-Order Functions
**OCaml Source:** [CS3110 — Streams](https://cs3110.github.io/textbook/chapters/ds/streams.html)

## Problem Statement

Implement `unfold` — the dual of `fold`. Given a seed value and a function that produces (value, next_seed) or None, generate a sequence.

## Learning Outcomes

- Understand unfold as the categorical dual of fold
- Compare eager (Vec) vs lazy (Iterator) sequence generation
- Use `std::iter::from_fn` and `successors` for Rust unfold patterns

## Key Insight

OCaml's recursive `unfold` builds a list eagerly. Rust can do the same with a loop, but the idiomatic approach returns a lazy iterator using `std::iter::from_fn` — computing values only when consumed. This is more memory-efficient for large or infinite sequences.
