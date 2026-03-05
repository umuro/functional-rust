# Example 997: Seq.map, Seq.filter — Lazy Transformations

**Difficulty:** ⭐⭐
**Category:** stdlib-seq
**OCaml Source:** OCaml standard library `Seq` module — lazy sequences

## Problem Statement

Produce the first `k` even perfect squares by generating an infinite sequence of
natural numbers, squaring each one, keeping only the even results, and collecting
the first `k` values — all without materializing the full sequence.

## Learning Outcomes

- Rust `Iterator` is the direct equivalent of OCaml's `Seq`: both are lazy, pull-based, and composable.
- `std::iter::successors` mirrors `Seq.unfold` for building infinite generators.
- Iterator adapters (`.map()`, `.filter()`, `.take()`) are zero-cost: they chain closures with no intermediate allocation.
- Generic iterator functions encode `Seq.map f |> Seq.filter p` as reusable, monomorphised Rust.

## OCaml Approach

OCaml's `Seq` module uses a thunk-based lazy linked list. `Seq.unfold` generates
an infinite sequence; `Seq.map` and `Seq.filter` return new lazy sequences without
evaluating elements. Only `List.of_seq` (or `Seq.take`) forces evaluation.

## Rust Approach

Rust's `Iterator` trait is a state machine: `.map()` and `.filter()` wrap the
source iterator with an adapter struct. No elements are computed until the consumer
calls `.next()`. The `(1u64..).map(...).filter(...).take(k).collect()` chain is
exactly as lazy as the OCaml pipeline, and the compiler inlines all closures.

## Key Differences

1. **Lazy mechanism:** OCaml uses heap-allocated thunks (`unit -> 'a Seq.node`); Rust uses stack-allocated adapter structs (zero heap overhead).
2. **Infinite sequences:** OCaml `Seq.unfold` needs an explicit seed + step; Rust has built-in infinite ranges (`1u64..`) and `std::iter::successors`.
3. **Type signatures:** OCaml infers `int Seq.t`; Rust requires explicit element type (`u64`) to prevent overflow ambiguity.
4. **Forcing evaluation:** OCaml uses `List.of_seq`; Rust uses `.collect::<Vec<_>>()`.
