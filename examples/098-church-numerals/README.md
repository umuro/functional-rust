# Example 098: Church Numerals — Functions as Numbers

**Difficulty:** ⭐⭐⭐
**Category:** Higher-Order Functions
**OCaml Source:** [CS3110 — Lambda Calculus](https://cs3110.github.io/textbook/chapters/hop/lambda.html)

## Problem Statement

Implement Church numerals: natural numbers encoded as higher-order functions. Zero applies a function 0 times, one applies it once, etc. Implement successor, addition, and multiplication.

## Learning Outcomes

- Understand lambda calculus encoding in a typed language
- See how OCaml's implicit polymorphism makes Church numerals elegant
- Confront Rust's ownership challenges with deeply nested closures
- Learn practical alternatives (struct-based) when pure function encoding is awkward

## Key Insight

OCaml's `let zero _f x = x` is polymorphic in `f` — it just works. Rust's closures are concrete types with ownership, making pure Church encoding verbose and requiring `Box<dyn Fn>`. The practical Rust approach wraps a `usize` and provides an `apply` method — same semantics, better ergonomics.
