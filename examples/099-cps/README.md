# Example 099: CPS — Continuation-Passing Style

**Difficulty:** ⭐⭐⭐
**Category:** Higher-Order Functions
**OCaml Source:** [CS3110 — CPS](https://cs3110.github.io/textbook/chapters/hop/cps.html)

## Problem Statement

Transform recursive functions into continuation-passing style, where each function takes an extra argument: "what to do with the result." This enables tail recursion.

## Learning Outcomes

- Understand CPS transformation and why it matters for stack safety
- See why CPS is natural in OCaml but awkward in Rust
- Learn that Rust's iterators and explicit stacks solve the same problem more idiomatically
- Compare `Box<dyn FnOnce>` costs with OCaml's lightweight closures

## Key Insight

CPS transforms stack usage into heap allocation (closures). In OCaml, closures are cheap (GC-managed), making CPS a practical technique. In Rust, each `Box<dyn FnOnce>` heap-allocates, so CPS is educational but not idiomatic — iterators and explicit stacks are preferred.
