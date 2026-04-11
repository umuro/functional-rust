📖 **[View on hightechmind.io →](https://hightechmind.io/rust/051-applying-function-twice)**

---

# 051 — Applying a Function Twice

## Problem Statement

Applying a function twice — `f(f(x))` — is a simple but illuminating exercise in higher-order functions and partial application. It demonstrates that functions are first-class values: `twice` takes a function as a parameter and returns a value (or a new function via partial application). This is the simplest non-trivial higher-order function, used as an introduction in OCaml's CS3110 course at Cornell.

The `twice` combinator generalizes to `apply_n_times(f, n, x)` which applies `f` exactly `n` times — the basis for iterative computation in functional languages, Church numerals (see example 098), and fixed-point iteration in numerical methods.

## Learning Outcomes

- Write a function that takes another function as a parameter
- Implement both `twice(f, x) -> T` and the partially applied `twice_partial(f) -> Fn(T) -> T`
- Use `impl Fn(T) -> T` as a generic function type
- Understand the difference between `Fn`, `FnMut`, and `FnOnce` for this use case
- Demonstrate that function-returning functions (curried style) work in Rust via closures

- Use `F: Fn(T) -> T` (not `FnOnce`) as the trait bound since the function is called twice in `twice(f, x)`
- Return `impl Fn(T) -> T` from `twice_partial(f)` using `move |x| f(f(x))`

## Rust Application

`twice<T, F>(f: F, x: T) -> T where F: Fn(T) -> T` applies `f` twice: `f(f(x))`. `twice_partial<T, F>(f: F) -> impl Fn(T) -> T` returns a closure: `move |x| f(f(x))` — the partially applied version. `twice_fp(f: fn(i32) -> i32, x: i32) -> i32` uses a bare function pointer for the no-closure case. `F: Fn(T) -> T` (not `FnOnce`) is required because `f` is called twice — `FnOnce` could only be called once.

## OCaml Approach

OCaml's version from CS3110: `let twice f x = f (f x)`. This is automatically curried: `let quad = twice double` — partially applies `twice` with `double`, producing a function `int -> int` that quadruples its argument. `let apply_n_times f n x = if n = 0 then x else apply_n_times f (n-1) (f x)` generalizes to n applications.

## Key Differences

1. **Automatic currying**: OCaml's `twice f x` is automatically `twice f` (partial application). Rust needs `twice_partial(f)` as a separate function returning a closure.
2. **`Fn` vs `FnOnce`**: Rust distinguishes single-use (`FnOnce`) and multi-use (`Fn`) closures. Calling `f` twice requires `F: Fn(T) -> T`. OCaml has no such distinction.
3. **Type inference**: Both infer T from the function's return type. Rust infers the closure type from how `f` is called.
4. **`move` closure**: `twice_partial` captures `f` by move into the returned closure. The `move` keyword transfers ownership of `f` into the closure.

1. **`Fn` vs `FnOnce`:** `twice` requires `F: Fn(T) -> T` (not `FnOnce`) because it calls `f` twice. `FnOnce` can only be called once — using it here would cause a compile error on the second call.
2. **Return type `impl Fn`:** `twice_partial` returns `impl Fn(T) -> T`. The concrete closure type is unnameable, so `impl Fn` is used. In a trait object context, `Box<dyn Fn(T) -> T>` would be needed.
3. **Church numerals:** `twice` applied to itself computes 4 times: `twice(twice, f)` applies `f` four times. This is the basis of Church numerals: `0 = id`, `1 = once`, `2 = twice`, `n = apply_n`.
4. **OCaml's `@@`:** OCaml's function application operator `@@` reads as `f @@ x` instead of `f x`. `twice f @@ x` applies `twice f` to `x`. Rust has no equivalent — use `f(x)` or the explicit `pipe` function.

## Exercises

1. **Apply n times**: Write `apply_n<T: Clone, F: Fn(T) -> T>(f: F, n: usize, x: T) -> T` that applies `f` exactly `n` times. Handle `n=0` (return `x` unchanged).
2. **Fixed point**: Write `fixed_point<T: Clone + PartialEq, F: Fn(T) -> T>(f: F, x: T) -> T` that applies `f` repeatedly until `f(x) == x` (convergence). Use this to compute square roots by Newton's method.
3. **Compose from twice**: Express `compose(f, g)` in terms of `twice` — or explain why it cannot be expressed that way without additional combinators.

4. **Apply N times**: Implement `apply_n<T>(f: impl Fn(T) -> T, n: usize, x: T) -> T` that applies `f` exactly `n` times. For `n=0`, return `x`. For `n=1`, return `f(x)`. Connect this to Church numeral representation.
5. **Fixed point**: Implement `fixed_point<T: PartialEq + Clone>(f: impl Fn(T) -> T, x: T, max_iter: usize) -> Option<T>` that keeps applying `f` until the output equals the input (or exceeds `max_iter`). Useful for iterative numerical methods.
