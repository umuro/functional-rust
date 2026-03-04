# 381: Blanket Implementations

**Difficulty:** 3  **Level:** Advanced

Implement a trait for every type that satisfies a bound — at once, forever.

## The Problem This Solves

You've defined a trait, and you want it available on many types — but you don't want to write `impl MyTrait for i32`, `impl MyTrait for String`, `impl MyTrait for f64` one by one. Worse, new types keep being added: yours, library types, user types. Maintaining a list of impls is futile.

The standard library faces this constantly. `Iterator` has 70+ adapter methods — they work on every iterator, whether it's a `Vec`, a file, a network stream, or something invented tomorrow. The library authors couldn't have written an impl for each. Instead they wrote one blanket impl: *if you implement `Iterator`, you get all the adapter methods for free*.

Blanket implementations solve this by saying: "for all `T` satisfying bound `B`, here is how `T` implements `Trait`." One impl covers infinite types.

## The Intuition

A blanket impl is just a generic `impl` where the type parameter is unconstrained except by trait bounds. Instead of `impl Foo for Bar`, you write `impl<T: Something> Foo for T`. That single declaration automatically covers every type — present and future — that implements `Something`.

Think of it as a universal factory: the bounds are the inputs, the trait impl is the output, and the compiler runs the factory at compile time for every qualifying type it encounters.

## How It Works in Rust

```rust
use std::fmt;

trait Summary {
    fn summarize(&self) -> String;
}

// Blanket impl: EVERY type that is Display automatically gets Summary
impl<T: fmt::Display> Summary for T {
    fn summarize(&self) -> String {
        format!("Summary: {}", self)  // we can call Display because T: Display
    }
}

// Now i32, f64, &str, String, etc. all have .summarize() — no extra code needed
fn print_summary<T: Summary>(item: &T) {
    println!("{}", item.summarize());
}

fn main() {
    print_summary(&42);           // i32 gets Summary via blanket impl
    print_summary(&3.14f64);      // f64 too
    print_summary(&"hello");      // &str too
}
```

The compiler checks: does `i32: fmt::Display`? Yes. So `i32: Summary`. It generates the impl on demand — there's no runtime cost.

You can stack bounds: `impl<T: Display + Clone> MyTrait for T {}` means "only types that are both Display and Clone qualify."

## What This Unlocks

- **Zero-cost abstraction over open type sets** — write the trait once, it works for all current and future types that satisfy the bounds.
- **Standard library patterns** — `impl<I: Iterator> IteratorExt for I` is how all iterator adapters work; `impl<T: Display> ToString for T` is how `.to_string()` works on anything printable.
- **Composable capability towers** — stack trait bounds to carve out exactly the types that qualify, without listing them.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Blanket impl | `MakeDescribable(P: Printable)` functor — explicit per-module application | `impl<T: Display> Trait for T` — implicit, compiler-driven, applies globally |
| Open-world extensibility | New type needs a new functor application | New type gets the impl automatically if it satisfies the bound |
| Coherence | Multiple functor applications can coexist | Only one blanket impl per (Trait, Type) pair — compiler enforces uniqueness |
| Syntax | `module M = MakeDescribable(P)` | `impl<T: Bound> Trait for T {}` |
