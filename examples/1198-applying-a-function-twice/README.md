# Example 1198: Applying a Function Twice

**Difficulty:** ⭐⭐
**Category:** Higher-Order Functions | Currying & Partial Application
**OCaml Source:** Classic functional programming exercise

## Problem Statement

Define a higher-order function `twice` that takes a function `f` and a value `x`, and applies `f` to `x` twice: `f(f(x))`. Use it to build new functions via partial application.

## Learning Outcomes

- How Rust expresses higher-order functions using `Fn` trait bounds
- Why `F: Fn(T) -> T` requires the input and output types to match
- How to simulate OCaml-style currying with `impl Fn(T) -> T` return types
- The difference between direct application and returning a closure for partial application

## OCaml Approach

OCaml's `twice` is automatically curried: `let twice f x = f (f x)` has type `('a -> 'a) -> 'a -> 'a`. Partial application (`let quad = twice double`) is free — you just omit the last argument and OCaml gives you back a function.

## Rust Approach

Rust requires explicit generic bounds: `fn twice<T, F: Fn(T) -> T>(f: F, x: T) -> T`. For partial application, a separate `twice_curried` function returns `impl Fn(T) -> T`, capturing `f` in a `move` closure. The type constraint `Fn(T) -> T` enforces that the function is an endomorphism (maps a type to itself).

## Key Differences

1. **Currying:** OCaml curries all functions automatically; Rust requires a distinct `twice_curried` wrapper that returns a closure.
2. **Type annotation:** OCaml infers `('a -> 'a) -> 'a -> 'a` silently; Rust spells out `F: Fn(T) -> T` explicitly.
3. **Closure capture:** The `move` keyword in Rust transfers ownership of `f` into the returned closure; OCaml handles this via GC.
4. **Endomorphism constraint:** Both languages enforce that `f` must map a type to itself — OCaml via unification, Rust via the bound `Fn(T) -> T`.
