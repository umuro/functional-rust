# Example 1208: Applying a Function Twice

**Difficulty:** ⭐  
**Category:** Higher-Order Functions  
**OCaml Source:** Classic HOF — `twice f x = f (f x)`

## Problem Statement

Define a higher-order function `twice` that applies a function `f` to a value `x` two times: `f(f(x))`. Show how partial application turns `twice` into a factory for new functions.

## Learning Outcomes

- How Rust's `Fn` trait expresses higher-order functions generically
- Why `impl Fn(T) -> T` as a return type enables OCaml-style partial application
- The difference between `Fn` (shared reference), `FnMut` (exclusive reference), and `fn` (function pointer)
- How closures capture function arguments via `move`, mirroring OCaml's currying

## OCaml Approach

OCaml functions are automatically curried, so `twice double` is valid and produces a new function. The type of `twice` is `('a -> 'a) -> 'a -> 'a` — it takes a function and returns a function. Partial application is built into the language with no extra syntax.

## Rust Approach

Rust uses trait bounds (`F: Fn(T) -> T`) to accept any callable. The partial application style is achieved with `twice_compose`, which takes `f` by move into a returned closure (`impl Fn(T) -> T`). The direct style (`twice`) takes both `f` and `x` together, which is more common in Rust.

## Key Differences

1. **Currying:** OCaml curries automatically; Rust requires an explicit closure or separate function to achieve partial application.
2. **Type signature:** OCaml's `('a -> 'a) -> 'a -> 'a` vs Rust's `fn<T, F: Fn(T) -> T>(f: F, x: T) -> T`.
3. **Ownership:** Rust must `move` the function into the returned closure; OCaml's GC handles this transparently.
4. **Function pointers vs closures:** Rust distinguishes `fn(T) -> T` (bare pointer, Copy) from `impl Fn(T) -> T` (closure, may capture state).
