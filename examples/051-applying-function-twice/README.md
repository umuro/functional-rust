📖 **[View on hightechmind.io →](https://hightechmind.io/rust/051-applying-function-twice)**

---

# 051 — Applying a Function Twice
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  


## Problem Statement

Applying a function twice — `f(f(x))` — is a simple but illuminating exercise in higher-order functions and partial application. It demonstrates that functions are first-class values: `twice` takes a function as a parameter and returns a value (or a new function via partial application). This is the simplest non-trivial higher-order function, used as an introduction in OCaml's CS3110 course at Cornell.

The `twice` combinator generalizes to `apply_n_times(f, n, x)` which applies `f` exactly `n` times — the basis for iterative computation in functional languages, Church numerals (see example 098), and fixed-point iteration in numerical methods.

## Learning Outcomes

- Write a function that takes another function as a parameter
- Implement both `twice(f, x) -> T` and the partially applied `twice_partial(f) -> Fn(T) -> T`
- Use `impl Fn(T) -> T` as a generic function type
- Understand the difference between `Fn`, `FnMut`, and `FnOnce` for this use case
- Demonstrate that function-returning functions (curried style) work in Rust via closures

## Rust Application

`twice<T, F>(f: F, x: T) -> T where F: Fn(T) -> T` applies `f` twice: `f(f(x))`. `twice_partial<T, F>(f: F) -> impl Fn(T) -> T` returns a closure: `move |x| f(f(x))` — the partially applied version. `twice_fp(f: fn(i32) -> i32, x: i32) -> i32` uses a bare function pointer for the no-closure case. `F: Fn(T) -> T` (not `FnOnce`) is required because `f` is called twice — `FnOnce` could only be called once.

## OCaml Approach

OCaml's version from CS3110: `let twice f x = f (f x)`. This is automatically curried: `let quad = twice double` — partially applies `twice` with `double`, producing a function `int -> int` that quadruples its argument. `let apply_n_times f n x = if n = 0 then x else apply_n_times f (n-1) (f x)` generalizes to n applications.

## Key Differences

1. **Automatic currying**: OCaml's `twice f x` is automatically `twice f` (partial application). Rust needs `twice_partial(f)` as a separate function returning a closure.
2. **`Fn` vs `FnOnce`**: Rust distinguishes single-use (`FnOnce`) and multi-use (`Fn`) closures. Calling `f` twice requires `F: Fn(T) -> T`. OCaml has no such distinction.
3. **Type inference**: Both infer T from the function's return type. Rust infers the closure type from how `f` is called.
4. **`move` closure**: `twice_partial` captures `f` by move into the returned closure. The `move` keyword transfers ownership of `f` into the closure.

## Exercises

1. **Apply n times**: Write `apply_n<T: Clone, F: Fn(T) -> T>(f: F, n: usize, x: T) -> T` that applies `f` exactly `n` times. Handle `n=0` (return `x` unchanged).
2. **Fixed point**: Write `fixed_point<T: Clone + PartialEq, F: Fn(T) -> T>(f: F, x: T) -> T` that applies `f` repeatedly until `f(x) == x` (convergence). Use this to compute square roots by Newton's method.
3. **Compose from twice**: Express `compose(f, g)` in terms of `twice` — or explain why it cannot be expressed that way without additional combinators.
