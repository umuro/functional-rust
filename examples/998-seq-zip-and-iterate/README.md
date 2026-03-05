# Example 998: Seq — Zip and Iterate

**Difficulty:** ⭐⭐
**Category:** stdlib-seq
**OCaml Source:** OCaml standard library `Seq` module — `Seq.zip`, `Seq.iterate`

## Problem Statement

Combine two lazy sequences element-by-element with `Seq.zip`, then generate an
infinite sequence of repeated function applications with `Seq.iterate`. The
Collatz sequence (n → n/2 if even, 3n+1 if odd) is the canonical demonstration.

## Learning Outcomes

- How OCaml's `Seq.zip` maps to Rust's `Iterator::zip`
- How OCaml's `Seq.iterate f x` maps to `std::iter::successors`
- Why Rust iterators and OCaml sequences share the same lazy evaluation model
- How to encode a generic `iterate` function with trait bounds `T: Clone` and `F: Fn(&T) -> T`

## OCaml Approach

OCaml uses the `Seq` module for lazy sequences. `Seq.zip` takes two sequences
and produces a sequence of pairs, stopping at the shorter. `Seq.iterate f x`
generates the infinite sequence `x, f(x), f(f(x)), …` — useful for fixed-point
iteration and mathematical sequences. Both are lazy: elements are only computed
when consumed by `Seq.take` or `List.of_seq`.

## Rust Approach

Rust iterators are structurally identical to OCaml sequences in their laziness.
`Iterator::zip` zips two iterators. `std::iter::successors(Some(start), |prev|
Some(f(prev)))` is the direct encoding of `Seq.iterate`: it produces an infinite
iterator whose each element is derived from the previous. Calling `.take(n)` and
`.collect()` materialises exactly the needed prefix.

## Key Differences

1. **Laziness location:** OCaml sequences are heap-allocated closures; Rust
   iterators are zero-cost state machines on the stack — no allocation.
2. **Zip arity:** OCaml `Seq.zip` takes two sequences; Rust's `.zip()` is a
   method on any `Iterator`, so it chains naturally.
3. **`Seq.iterate` vs `successors`:** OCaml passes the value directly; Rust
   passes `&T` to the closure (avoiding a move), requiring the result to be
   a new `T` (hence `T: Clone` when the value must be re-used).
4. **Termination:** OCaml `Seq.iterate` is always infinite; Rust
   `successors` can return `None` to terminate, making it strictly more general.
