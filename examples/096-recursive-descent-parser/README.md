# Example 096: Simple Recursive Descent Parser

**Difficulty:** ⭐⭐⭐
**Category:** Parsing
**OCaml Source:** [CS3110 — Parsing](https://cs3110.github.io/textbook/chapters/interp/parsing.html)

## Problem Statement

Parse arithmetic expressions with `+` and `*` operators into an AST, respecting operator precedence (`*` binds tighter than `+`). Evaluate the AST.

## Learning Outcomes

- Implement mutual recursion in Rust (functions calling each other)
- Use `Box<T>` for recursive enum variants (Rust's heap allocation for tree nodes)
- Map OCaml's list consumption pattern to Rust's `split_first()` on slices
- Handle lifetimes in parser return types

## Key Insight

OCaml's `and` keyword for mutual recursion has no Rust equivalent — Rust functions can simply call each other. The bigger difference is `Box<Expr>`: Rust needs explicit heap allocation for recursive types because it must know sizes at compile time. OCaml allocates everything on the heap implicitly.
