# Example 001: Applying a Function Twice
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Given a function `f` and a value `x`, apply the function `f` to `x`, and then apply `f` again to the result. This is a fundamental pattern in functional programming that demonstrates how functions can be treated as first-class values.

## Learning Outcomes

- Understand **higher-order functions** — functions that take other functions as arguments
- Master **generic function types** in Rust using trait bounds and function pointers
- Learn the difference between **closures** and **function pointers** for passing functions
- Explore **function composition** as a way to build reusable transformations

## OCaml Approach

OCaml's `twice` function is elegantly simple:

```ocaml
let twice f x = f (f x)
```

This takes advantage of OCaml's currying: `f` and `x` are separate parameters, and partial application is implicit. You can write `let quad = twice double` to bind `double` as the function argument, creating a partially-applied function.

## Rust Approach

Rust requires explicit handling of function types through:

1. **Closures with trait bounds** — `impl Fn(T) -> T` captures any callable type
2. **Function pointers** — `fn(T) -> T` for concrete function references
3. **Closure composition** — returning closures that capture the original function

We provide three implementations that grow in expressiveness:

- **`twice`** — accepts any callable (closure or function) via trait bound
- **`twice_fn`** — accepts explicit function pointers
- **`twice_compose`** — returns a new closure that can be stored and reused

## Key Differences

1. **Currying:** OCaml's automatic currying means `twice double` is syntactic sugar for `(twice double)`. In Rust, we write `|x| twice(double, x)` or use `twice_compose` to achieve similar partial application.

2. **Function Types:** OCaml treats all functions uniformly with type `'a -> 'b`. Rust distinguishes between concrete function pointers `fn(T) -> T` and closures captured via `impl Fn(T) -> T`, giving more control but requiring explicit choices.

3. **Generic Flexibility:** Rust's `impl Fn(T) -> T` trait bound is more flexible than a bare function pointer, accepting closures with captured state.

4. **Composition as a First-Class Pattern:** Rust's `twice_compose` returns a closure, enabling functional composition patterns. OCaml achieves this naturally through function composition.

## Exercises

1. Implement `thrice` (apply a function three times) in both languages
2. Create a composition operator `|>` in Rust that mimics OCaml's `|>` pipeline
3. Implement a generic `apply_n_times` function that applies a function `n` times
