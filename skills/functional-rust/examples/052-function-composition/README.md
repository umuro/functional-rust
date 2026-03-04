# 052: Function Composition

**Difficulty:** 1  **Level:** Beginner

Combine two functions into one: `compose(f, g)(x) = f(g(x))`.

## The Problem This Solves

You have a `validate` function and a `trim` function. You want a single `clean` function that trims then validates. Without composition, you write `validate(trim(s))` everywhere — three names, nested syntax, reading right-to-left.

Function composition solves this by building new functions from existing ones. `compose(validate, trim)` produces a single `clean` function you can store, pass around, and call without seeing the internals. When you add a `normalize` step later, you compose again rather than changing all the call sites.

This is the functional programmer's version of a pipeline: instead of threading data through a sequence, you thread functions together first, then run the composed function once.

## The Intuition

Mathematical composition: f∘g means "apply g, then f." `compose(f, g)` returns a new function that does exactly that. The argument order might feel backwards (`f` before `g` but `g` runs first) — that's the mathematical convention.

If the backwards order bothers you, use `pipe(g, f)` instead: same result, argument order matches the data flow (left to right). `pipe(trim, validate)` reads as "trim first, then validate."

## How It Works in Rust

```rust
// compose(f, g)(x) = f(g(x))  — mathematical convention (f applied last)
pub fn compose<A, B, C, F, G>(f: F, g: G) -> impl Fn(A) -> C
where
    F: Fn(B) -> C,
    G: Fn(A) -> B,
{
    move |x| f(g(x))  // f and g are moved into the closure
}

// pipe(g, f)(x) = f(g(x))  — data-flow convention (g applied first)
pub fn pipe<A, B, C, F, G>(g: G, f: F) -> impl Fn(A) -> C
where
    F: Fn(B) -> C,
    G: Fn(A) -> B,
{
    move |x| f(g(x))
}

// Trait extension: any Fn gets a .then_apply(next) method
pub trait Compose<A, B>: Fn(A) -> B + Sized {
    fn then_apply<C, H: Fn(B) -> C>(self, next: H) -> impl Fn(A) -> C {
        move |x| next(self(x))
    }
}
impl<A, B, F: Fn(A) -> B> Compose<A, B> for F {}

// Usage:
let square_then_double = compose(double, square);  // square first, double second
let square_then_double = square.then_apply(double); // method chain style
```

The `impl Fn` return type hides the concrete closure type, which the compiler knows but can't name. The `move` keyword is required because the returned closure lives beyond the function call.

## What This Unlocks

- **Pipeline construction** — compose a sequence of transformations once, reuse the resulting function everywhere.
- **Adapter pattern** — wrap a function with pre/post-processing without touching its implementation.
- **Trait extension** — adding `.then_apply` to any `Fn` shows how Rust traits enable expressive APIs.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Syntax | `let compose f g x = f (g x)` | Explicit `move` closure required |
| Return type | Inferred polymorphic function | `impl Fn(A) -> C` |
| Partial application | `let f = compose double` | Requires explicit closure or `twice_partial`-style wrapper |
| Argument order | `compose f g` (f last, mathematical) | Same, or `pipe(g, f)` for data-flow order |
| Trait method | Not available | `.then_apply(next)` via trait extension |
