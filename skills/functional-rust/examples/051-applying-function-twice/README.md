# 051: Applying a Function Twice

**Difficulty:** 1  **Level:** Beginner

Pass a function as a value and apply it twice: `twice(f, x) = f(f(x))`.

## The Problem This Solves

Most languages let you call functions, but can you *pass* a function to another function? Can you *return* a function from a function? These are the hallmarks of **higher-order functions** — the foundation of functional programming.

Without higher-order functions, you'd write `double(double(x))` and `square(square(x))` as separate concrete functions. With `twice`, you write the pattern once and parameterize over the function. The same logic works for any transformation you want to apply twice: encryption rounds, normalisation steps, geometric transforms.

This example is deliberately tiny — `twice(f, x) = f(f(x))` — so the mechanics of higher-order functions are visible without distracting complexity.

## The Intuition

Think of `twice` like a recipe instruction: "do this step, then do it again." The recipe doesn't care what the step is. It just applies it twice. `twice(stir, bowl)` stirs twice; `twice(double, 3)` gives 12.

Partial application goes one step further: `twice_partial(double)` bakes "double" into a new function `quad` that you can call later with any number. `quad(3)` gives 12, `quad(5)` gives 20 — the step is frozen, only the input varies.

## How It Works in Rust

```rust
// Generic: F must be Fn(T)->T so it can be called multiple times
pub fn twice<T, F: Fn(T) -> T>(f: F, x: T) -> T {
    f(f(x))  // first call produces intermediate, second consumes it
}

// Partial application: bind f, return a new closure
pub fn twice_partial<T, F: Fn(T) -> T>(f: F) -> impl Fn(T) -> T {
    move |x| f(f(x))  // f is moved into the closure; safe to call many times
}

pub fn double(x: i32) -> i32 { 2 * x }
pub fn square(x: i32) -> i32 { x * x }

// Usage:
let quad   = twice_partial(double);  // quad(3) == 12
let fourth = twice_partial(square);  // fourth(2) == 16
```

`Fn(T) -> T` (not `FnOnce`) means the closure can be called repeatedly. The `move` keyword transfers ownership of `f` into the returned closure.

## What This Unlocks

- **Composable transformations** — apply any `Fn(T) -> T` twice without rewriting it: encryption rounds, string normalisation, numeric iteration.
- **Partial application pattern** — freezing one argument to build specialized functions; the Rust idiom for what OCaml does automatically with currying.
- **Understanding `Fn`/`FnMut`/`FnOnce`** — `twice` needs `Fn` (not `FnOnce`) because `f` is called twice. Recognizing which trait bound you need is a core Rust skill.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Partial application | Built-in: `let quad = twice double` | Explicit closure: `twice_partial(double)` |
| Function type | `('a -> 'a) -> 'a -> 'a` | `fn<T, F: Fn(T)->T>(f: F, x: T) -> T` |
| Returning a function | Currying is automatic | `impl Fn(T) -> T` return type |
| Capturing in closure | Automatic under GC | `move` keyword required |
| Function pointers | No distinction | `fn(i32) -> i32` (no captures, zero overhead) |
