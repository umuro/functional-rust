📖 **[View on hightechmind.io →](https://hightechmind.io/rust/005-currying)**

---

# 005 — Currying and Partial Application

## Problem Statement

Currying (named after mathematician Haskell Curry, though independently discovered by Moses Schonfinkel) is the technique of transforming a multi-argument function into a chain of single-argument functions. `add(a, b)` becomes `add(a)(b)`, where `add(5)` returns a new function that adds 5 to its argument. This makes partial application — fixing some arguments now and the rest later — a natural consequence of function application.

Currying is the foundation of functional composition. It enables point-free style, makes predicate factories (like `greater_than(threshold)`) trivial, and is how Haskell and OCaml's standard libraries are designed. In practice it appears in React's higher-order components, Lodash's `_.curry`, and functional Redux reducers.

## Learning Outcomes

- Understand the difference between currying (transforming function arity) and partial application (fixing arguments)
- Return closures from functions to achieve curried-style APIs in Rust
- Use `move` closures to capture environment by value
- Build predicate factories and transformation pipelines with partial application
- Understand why OCaml's automatic currying requires explicit closures in Rust

## Rust Application

`add(n: i64) -> impl Fn(i64) -> i64` returns a closure that captures `n` by move. This is the Rust equivalent of OCaml's `let add n = fun x -> x + n`. `greater_than(threshold)` returns a predicate closure suitable for use with `.filter()`. `curried_add3` demonstrates fully nested closures: `curried_add3(1)(2)(3) == 6`. The generic `partial<A,B,C,F>` function fixes the first argument of any two-argument function, requiring `A: Clone` because the captured value may be called multiple times.

## OCaml Approach

In OCaml, ALL multi-argument functions are automatically curried. `let add a b = a + b` is syntactic sugar for `let add = fun a -> fun b -> a + b`. Partial application is free: `let add5 = add 5` immediately produces a function. `let double = ( * ) 2` partially applies multiplication. The standard library exploits this everywhere: `List.map (fun x -> x + 1)` partially applies `List.map`.

## Key Differences

1. **Default currying**: OCaml curries all functions automatically; every `let f a b = ...` is implicitly `let f = fun a -> fun b -> ...`. Rust requires explicit closure-returning functions.
2. **Closure syntax**: OCaml: `fun x -> x + n`. Rust: `move |x| x + n`. The `move` keyword is required in Rust to transfer ownership of captured variables into the closure.
3. **Type inference**: Both infer closure types from usage, but Rust's `impl Fn(T) -> T` return type must be written explicitly when returning closures from functions.
4. **`FnOnce` vs `Fn`**: Rust distinguishes closures that can be called once (`FnOnce`), once at a time (`FnMut`), and multiple times (`Fn`). OCaml has no such distinction.

## Exercises

1. **Pipeline factory**: Write `make_pipeline(steps: Vec<Box<dyn Fn(i64) -> i64>>) -> impl Fn(i64) -> i64` that returns a function composing all steps in sequence.
2. **Memoized partial**: Write a version of `partial` that memoizes the result of calling the inner function with a particular `a` value using a `HashMap`.
3. **Uncurry**: Write `uncurry<A,B,C>(f: impl Fn(A) -> impl Fn(B) -> C) -> impl Fn(A, B) -> C` that converts a curried function back into a two-argument function.
