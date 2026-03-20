# Example 003: Pipeline Operator

**Difficulty:** ⭐
**Category:** Higher-Order Functions | Function Composition
**OCaml Source:** OCaml manual (standard library pattern)

## Problem Statement

The pipeline operator `|>` is a function composition tool that threads a value through a series of transformations from left-to-right. In OCaml, it's defined simply as `let (|>) x f = f x`, which applies function `f` to value `x`. This example demonstrates how to chain function calls in a readable sequence.

## Learning Outcomes

- Understand function application and composition in Rust vs OCaml
- Learn three approaches to chaining transformations: nested calls, pipe functions, and function composition
- Recognize that Rust's method chaining is the idiomatic equivalent to OCaml's pipe operator
- See how FnOnce, closures, and trait bounds enable higher-order programming in Rust

## OCaml Approach

OCaml's `|>` operator provides a natural left-to-right reading order for chained transformations. It's defined as a simple infix operator that applies the right-hand function to the left-hand value. This makes complex pipelines easy to read: `5 |> double |> add_one` reads as "take 5, double it, add one".

## Rust Approach

Rust achieves similar composability through several idiomatic patterns: nested function calls for simple cases, the pipe function pattern for explicit composition, and function composition combinators. While Rust doesn't have a built-in `|>` operator, the same semantics are easily expressed using higher-order functions and closures.

## Key Differences

1. **Syntax:** OCaml's `|>` is syntactic sugar for right-associative function application. Rust requires explicit function calls or a custom pipe function.
2. **Method Chaining:** Rust's idiomatic style uses method chaining (`.map()`, `.filter()`, etc.), which is the natural equivalent. OCaml doesn't have methods on built-in types.
3. **Ownership:** Rust's pipe function uses `FnOnce` for one-shot transformations, enforcing move semantics when values are consumed. OCaml handles this implicitly.
4. **Type Inference:** Both languages infer the intermediate types in a pipeline, but Rust requires explicit type bounds for generic higher-order functions.

## Exercises

1. Define a `pipe2` macro (or function pair) that chains two single-argument closures and apply it to a string processing pipeline: parse → validate → format.
2. Extend the pipeline pattern to support error propagation: write `pipe_result` that threads a `Result<T, E>` through a sequence of `FnOnce(T) -> Result<U, E>` steps, short-circuiting on the first error.
3. Build a numeric pipeline using `pipe` that computes a statistical summary (min, max, mean, standard deviation) over a `Vec<f64>` in a single readable chain.
