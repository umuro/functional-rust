# 123: impl Trait

**Difficulty:** 2  **Level:** Intermediate

Ergonomic generics in argument position and opaque return types in return position — without naming the concrete type.

## The Problem This Solves

Generic functions are powerful but verbose. `fn print_all<T: Display>(items: &[T])` works, but for simple cases the `<T: ...>` syntax adds ceremony without value. More importantly, Rust's type system has a real problem with return types: you can't write `fn make_iter() -> Iterator<Item = u32>` as a return type — `Iterator` is a trait, not a type. You'd have to name the concrete iterator type, which for complex chains is impossibly long (`Map<Filter<Range<u32>, fn(&u32) -> bool>, fn(u32) -> u32>`).

`impl Trait` solves both. In argument position it's shorthand for a generic — `fn f(x: impl Display)` means the same as `fn f<T: Display>(x: T)` with a cleaner surface. In return position it creates an *opaque type*: the function promises to return *some* type that implements the trait, but the caller doesn't see the concrete type. This lets you return closures, complex iterators, and other unnameable types without boxing them on the heap.

The key limitation: if you need to return *different* concrete types from different branches of a function, `impl Trait` won't work — the compiler needs to monomorphize at compile time, which requires a single concrete type. Use `Box<dyn Trait>` for that case.

## The Intuition

`impl Trait` in arguments = "I accept any type that implements this trait." `impl Trait` in return position = "I return some specific type that implements this trait — figure it out at compile time."

## How It Works in Rust

```rust
use std::fmt::Display;

// Argument position — clean sugar for generics
fn stringify_all(items: &[impl Display]) -> Vec<String> {
    items.iter().map(|x| x.to_string()).collect()
}
// Identical to: fn stringify_all<T: Display>(items: &[T]) -> Vec<String>

stringify_all(&[1, 2, 3]);      // works
stringify_all(&[1.0, 2.0]);     // works — any Display type

// Return position — opaque iterator type (compiler knows; caller doesn't)
fn even_squares(n: u32) -> impl Iterator<Item = u32> {
    (0..n).filter(|x| x % 2 == 0).map(|x| x * x)
    // The concrete type is something like Map<Filter<Range<u32>, ...>, ...>
    // We don't have to write it. Zero overhead — no Box, no vtable.
}
let squares: Vec<u32> = even_squares(10).collect();

// Return position — returning a closure
fn make_formatter(uppercase: bool) -> impl Fn(&str) -> String {
    if uppercase {
        |s: &str| s.to_uppercase()   // both branches must be the SAME type!
    } else {
        |s: &str| s.to_lowercase()   // here both are closures with the same signature
    }
}
let fmt = make_formatter(true);
assert_eq!(fmt("hello"), "HELLO");

// When you NEED different types in branches, use Box<dyn Trait>:
fn make_content(is_article: bool) -> Box<dyn Summarizable> {
    if is_article { Box::new(Article { ... }) }
    else          { Box::new(Tweet { ... }) }
}
```

## What This Unlocks

- **Clean generic APIs** — remove `<T: Trait>` boilerplate from simple functions with a single generic parameter.
- **Return complex iterators** — `impl Iterator<Item = T>` hides multi-adapter chains without heap allocation.
- **Return closures** — `impl Fn(X) -> Y` lets you build factory functions that return closures the caller can call directly.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Generic arguments | Parametric polymorphism — inferred | `impl Trait` or explicit `<T: Trait>` |
| Opaque return types | Module signatures | `impl Trait` return position |
| Multiple concrete return types | Module functors / first-class modules | Must use `Box<dyn Trait>` |
| Runtime cost | Closures heap-allocated | `impl Trait` = zero cost, compile-time dispatch |
