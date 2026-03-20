# Example 1093: Currying and Partial Application

**Difficulty:** ⭐
**Category:** Functions & Higher-Order Programming
**OCaml Source:** Core language feature — every OCaml function is curried by default

## Problem Statement

Implement `let add x y = x + y` in Rust, showing how OCaml's automatic currying translates to Rust's explicit closures for partial application.

## Learning Outcomes

- How OCaml's auto-currying maps to Rust's `impl Fn` return types
- Using `move` closures to capture values for partial application
- Writing generic curried functions with trait bounds (`Add + Copy`)
- Building `curry` / `uncurry` combinators as higher-order functions

## OCaml Approach

In OCaml, `let add x y = x + y` is sugar for `let add = fun x -> fun y -> x + y`. Every function is automatically curried — `add 5` returns a function. No special syntax needed for partial application.

## Rust Approach

Rust functions take all arguments at once. To get partial application, you return a closure: `fn add_partial(x: i64) -> impl Fn(i64) -> i64`. The `move` keyword transfers ownership of captured values into the closure.

## Key Differences

1. **Currying:** OCaml auto-curries all functions; Rust requires explicit closure returns
2. **Type signatures:** OCaml infers `int -> int -> int`; Rust needs `impl Fn(i64) -> i64` return type
3. **Capture semantics:** OCaml closures capture by GC reference; Rust uses `move` for ownership transfer
4. **Polymorphism:** OCaml uses parametric polymorphism implicitly; Rust requires `<T: Add + Copy>` bounds

## Exercises

1. Write a curried `add3` function `i32 -> i32 -> i32 -> i32` and use partial application to create an `increment` and a `add10` specialization.
2. Implement `flip` — a higher-order function that takes `f: A -> B -> C` and returns `B -> A -> C` — and use it to partially apply the second argument of a binary function.
3. Use currying and partial application to build a configurable string formatter: a curried `format_field` that accepts a padding width, an alignment, and finally a value, returning the formatted string.
