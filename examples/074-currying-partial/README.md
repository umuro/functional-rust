📖 **[View on hightechmind.io →](https://hightechmind.io/rust/074-currying-partial)**

---

# 074 — Currying and Partial Application (Applied)
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  



## Problem Statement

This example applies currying and partial application to practical patterns — building factories, greeting generators, and transformation pipelines. Where example 005 introduced the theory, this example shows the patterns in context: `make_adder(5)` returns a reusable function, `apply_twice` demonstrates function composition from first principles.

Partial application is ubiquitous in functional programming: event handler factories in UIs, middleware pipelines in web frameworks, predicate factories in query builders, and configuration-bound operations. Understanding how to build and compose these in Rust is essential for ergonomic API design.

## Learning Outcomes

- Build function factories with `make_adder(n) -> impl Fn(i32) -> i32`
- Use closures for multi-level currying: function returning function returning value
- Apply `apply_twice` to demonstrate functions as values
- Use `compose` to build pipelines
- Understand the `move` keyword for capturing environment in closures

## Rust Application

`make_adder(x: i32) -> impl Fn(i32) -> i32` returns `move |y| x + y` — `x` is captured by move. `make_multiplier(x)` is analogous. `make_greeting(prefix)` returns a two-level curried closure for building greeting strings. `apply_twice(f, x)` applies `f` twice. `compose(f, g)` returns `move |x| f(g(x))`. All demonstrate the pattern of functions as values.

## OCaml Approach

OCaml: `let make_adder x = fun y -> x + y`. `let add5 = make_adder 5 in add5 3` evaluates to 8. `let apply_twice f x = f (f x)`. `let compose f g = fun x -> f (g x)`. OCaml's automatic currying makes `make_adder` equivalent to `let make_adder x y = x + y` — both are curried functions.

## Key Differences

1. **`move` requirement**: Rust requires `move` in the closure to capture `x` by value. OCaml captures by closure environment automatically (GC-managed). Without `move`, Rust would borrow `x` — problematic when the closure outlives the function.
2. **`impl Fn` return type**: Rust's `-> impl Fn(i32) -> i32` return type is required because closures have unique anonymous types. OCaml's `fun x -> ...` return type is inferred as `int -> int`.
3. **Lifetime**: The returned closure borrows nothing from the outer scope (because of `move`), so it has `'static` lifetime. OCaml closures can safely reference the outer scope due to GC.
4. **Higher-order composition**: `compose(f, g)` in Rust requires `+ 'static` bounds if stored. OCaml's `fun x -> f (g x)` composes naturally.

## Exercises

1. **Tax calculator**: Write `make_tax_calculator(rate: f64) -> impl Fn(f64) -> f64` and `make_discount(pct: f64) -> impl Fn(f64) -> f64`. Compose them into a `calculate_price` pipeline.
2. **Memoized factory**: Write `memoized_make_adder(cache: &mut HashMap<i32, Box<dyn Fn(i32) -> i32>>, n: i32) -> &Box<dyn Fn(i32) -> i32>` that caches the created adder function.
3. **Pipeline DSL**: Using `compose`, build a pipeline for data transformation: `trim -> lowercase -> split_words -> filter_short_words -> join_with_comma`. Each step is a partial application of a generic combinator.
