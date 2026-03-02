# Example 052: Function Composition

**Difficulty:** ⭐
**Category:** Higher-Order Functions
**OCaml Source:** CS3110

## Problem Statement

Implement `compose f g` — a higher-order function that returns a new function applying `g` first, then `f`. Demonstrate with `square_then_double`, which squares a number and then doubles it.

## Learning Outcomes

- Return closures from functions using `impl Fn` in Rust
- Understand how Rust's type system handles higher-order functions
- Compare three composition patterns: generic `compose`, pipeline `pipe`, and trait extension
- See why Rust requires `move` closures when capturing function arguments

## OCaml Approach

A simple three-argument curried function: `let compose f g x = f (g x)`. Partial application with `compose double square` produces a new function that captures `f` and `g` in a closure automatically — OCaml handles this transparently.

## Rust Approach

1. **Generic `compose`**: Returns `impl Fn(A) -> C`, capturing `f` and `g` by move — direct translation of OCaml
2. **`pipe`**: Argument order flipped (`pipe(g, f)`) so the pipeline reads left-to-right
3. **Trait extension**: A `Compose` trait adds `.then_apply(next)` to any `Fn`, enabling method-chaining style

## Key Differences

1. **`impl Fn` return type**: Rust cannot name the closure type, so `impl Fn(A) -> C` is used; OCaml infers a polymorphic function type transparently
2. **`move` closures**: Rust requires `move` to transfer ownership of `f` and `g` into the returned closure; OCaml captures values automatically
3. **Monomorphisation**: Rust generates a concrete function for each type combination at compile time; OCaml uses a uniform representation
4. **Argument order conventions**: OCaml's `compose f g` matches mathematical notation (f∘g); Rust's `pipe(g, f)` matches data-flow reading order
5. **Trait extension**: Rust's trait system allows adding `.then_apply` to any `Fn` type — a pattern unavailable in OCaml without a module functor
