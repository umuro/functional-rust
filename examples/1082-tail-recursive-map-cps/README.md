# Example 1082: Tail-Recursive Map with CPS

**Difficulty:** ⭐⭐
**Category:** Higher-Order Functions
**OCaml Source:** Cornell CS3110 — Functional Programming in OCaml

## Problem Statement

Implement `List.map` in three ways: naive recursive (stack-unsafe), tail-recursive with an accumulator and reverse, and continuation-passing style (CPS). Show how each approach handles the stack-safety problem.

## Learning Outcomes

- Understanding why naive recursive map overflows the stack on large inputs
- Translating OCaml's tail-recursive accumulator pattern to Rust's loop-based equivalent
- Building continuation chains with boxed closures as a Rust analog to OCaml CPS
- Recognizing that Rust's lack of TCO makes explicit loops the practical choice over structural tail recursion

## OCaml Approach

OCaml guarantees tail-call optimization, so rewriting `map` with an accumulator parameter (`go acc = function ...`) makes it stack-safe. The CPS version achieves the same by passing a continuation closure — all recursive calls are in tail position. Both `map_tr` and `map_cps` handle million-element lists without overflow.

## Rust Approach

Rust does not guarantee TCO, so even structurally tail-recursive functions can overflow. The idiomatic Rust translation of OCaml's accumulator pattern is a `for` loop with `Vec::push`. The CPS version uses `Box<dyn FnOnce>` to build a closure chain, which is educational but applies the chain via nested calls (still O(n) stack). The truly idiomatic solution is `iter().map().collect()`.

## Key Differences

1. **Tail-call optimization:** OCaml guarantees TCO; Rust does not. Structural tail recursion in Rust still overflows.
2. **List construction:** OCaml cons (`::`) prepends in O(1), requiring `List.rev` at the end. Rust's `Vec::push` appends in O(1) amortized — no reverse needed.
3. **Continuations:** OCaml closures are lightweight GC-managed values. Rust requires `Box<dyn FnOnce>` heap allocation for each continuation in the chain.
4. **Idiomatic solution:** OCaml developers write `List.map`. Rust developers write `iter().map(f).collect()` — both are zero-overhead abstractions over the same pattern.

## Exercises

1. Rewrite `filter` in CPS style so it is tail-recursive without relying on Rust's stack; verify it handles a list of 100,000 elements without stack overflow.
2. Implement `fold_left` using CPS transformation, then compare its performance to the direct iterative version on a list of one million integers.
3. Combine CPS-style `map` and `filter` into a single CPS `filter_map` that applies a `T -> Option<U>` function and collects `Some` results, avoiding intermediate allocations.
