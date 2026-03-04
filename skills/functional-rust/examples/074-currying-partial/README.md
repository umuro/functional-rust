# 074: Currying and Partial Application

**Difficulty:** 2  **Level:** Beginner

OCaml curries everything automatically. Rust doesn't — but closures capture arguments explicitly with equal power and more transparency.

## The Problem This Solves

You want to reuse a multi-argument function with one argument fixed: an adder with a fixed offset, a comparator with a fixed threshold, a formatter with a fixed template. In a higher-order pipeline, you need to produce a single-argument function from a multi-argument one.

In OCaml this is free: `let add5 = add 5` partially applies `add` instantly. In Rust you write `let add5 = |y| add(5, y)` — explicit, but it makes the capture visible. The power is the same; the ceremony differs.

## The Intuition

Currying means every function takes exactly one argument and returns either a value or another function. `add 5 3` is really `(add 5) 3` — apply `add` to `5`, get a new function, apply that to `3`.

Rust's closures capture bindings explicitly: `move |y| x + y` captures `x` by value and takes `y` as argument. This is manual currying — more explicit but also clearer about *what* is captured.

The `move` keyword is the Rust equivalent of OCaml's value capture — it ensures the closure owns its captured variables and can outlive the scope that created them.

## How It Works in Rust

```rust
// Regular two-argument function — NOT curried
pub fn add(x: i32, y: i32) -> i32 { x + y }

// Partial application via closure — the idiomatic Rust way
pub fn add5() -> impl Fn(i32) -> i32 {
    move |y| add(5, y)   // captures 5, takes y as argument
}

// Curried form: takes x, returns a function that takes y
pub fn add_curried(x: i32) -> impl Fn(i32) -> i32 {
    move |y| x + y   // `move` captures x by value into the closure
}

// Operator sections via closures
pub fn double()    -> impl Fn(i32) -> i32 { |x| x * 2 }
pub fn increment() -> impl Fn(i32) -> i32 { |x| x + 1 }

// Use them in a pipeline
fn pipeline(data: &[i32]) -> Vec<i32> {
    data.iter()
        .copied()
        .map(add_curried(10))  // partial: add 10 to each
        .map(double())          // double each result
        .collect()
}

// Generic curry converter: (A, B) -> C becomes A -> (B -> C)
pub fn curry<A, B, C>(f: impl Fn(A, B) -> C + 'static) -> impl Fn(A) -> Box<dyn Fn(B) -> C>
where A: Copy + 'static, B: 'static, C: 'static {
    move |a| Box::new(move |b| f(a, b))
}

// Compose two functions: g(f(x))
pub fn compose<A, B, C>(
    f: impl Fn(A) -> B,
    g: impl Fn(B) -> C,
) -> impl Fn(A) -> C {
    move |x| g(f(x))
}
```

Note: returning `impl Fn(i32) -> i32` works for simple cases. For storing in a struct or returning from a trait object, use `Box<dyn Fn(i32) -> i32>`.

## What This Unlocks

- **Pipeline factories**: generate configured `.map()` functions with fixed parameters.
- **Dependency injection light**: pass partially-applied functions instead of full objects.
- **Point-free style**: compose operations without naming intermediate values.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Default currying | Automatic — every function | Not automatic — must use closures |
| Partial apply | `let add5 = add 5` | `let add5 = \|y\| add(5, y)` |
| Capture | Implicit (lexical scope) | Explicit `move` for owned capture |
| Compose | `\|>` pipe or `Fun.compose` | Custom `compose()` or `\|>` macro |
| Return type | `int -> int` | `impl Fn(i32) -> i32` or `Box<dyn Fn>` |
