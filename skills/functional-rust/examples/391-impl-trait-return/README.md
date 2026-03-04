# 391: impl Trait in Return Position

**Difficulty:** 2  **Level:** Intermediate

Return `impl Iterator` (or any trait) to hide the concrete type — zero-cost abstraction without naming the type.

## The Problem This Solves

Before `impl Trait` in return position, hiding a concrete return type required either exposing it explicitly (tying your API to implementation details) or boxing it (`Box<dyn Iterator>` — heap allocation, dynamic dispatch, runtime cost). Neither was satisfying for iterator-heavy code.

The problem is common: you chain several iterator adapters inside a function and the concrete type becomes something like `Filter<Map<std::ops::Range<i32>, fn(i32) -> i32>, fn(&i32) -> bool>`. This is an implementation detail. Callers don't need to know it, and you don't want to commit to it — it changes every time you refactor the internals.

`impl Trait` in return position (RPIT) says: "this function returns *some* concrete type that implements this trait." The type is fixed and known at compile time — the compiler sees through the abstraction — but callers only see the trait bound. No allocation, no dynamic dispatch, no exposed internals.

## The Intuition

`impl Trait` is a promise: "I'll give you something that can do X — you don't need to know what it is, just that it can." The concrete type is locked in at compile time (monomorphic), which means the optimizer sees everything. You get the ergonomics of abstraction with the performance of concrete types.

The constraint: every code path in the function must return the *same* concrete type. If you need to return different types conditionally (odd path vs. even path), you need `Box<dyn Trait>` or an enum.

## How It Works in Rust

```rust
// Return impl Iterator — concrete type is hidden from callers
fn make_range(start: i32, end: i32) -> impl Iterator<Item = i32> {
    start..end  // concrete type: Range<i32>
}

// Return impl Fn — hide closure type
fn make_adder(n: i32) -> impl Fn(i32) -> i32 {
    move |x| x + n  // concrete type: [closure@...]
}

// Chain adapters — concrete type gets complex, impl hides it
fn evens_doubled(start: i32, end: i32) -> impl Iterator<Item = i32> {
    (start..end).filter(|x| x % 2 == 0).map(|x| x * 2)
}

// Can't return different types from branches — use Box<dyn> for that
fn either_iter(flag: bool) -> Box<dyn Iterator<Item = i32>> {
    if flag { Box::new(0..5) } else { Box::new(vec![10, 20, 30].into_iter()) }
}

// Use it just like any iterator
let sum: i32 = evens_doubled(1, 11).sum(); // → 60
```

## What This Unlocks

- **Stable, zero-cost abstraction** — hide complex concrete types (especially chained iterator types) without paying for heap allocation or dynamic dispatch
- **API stability** — change your internal implementation without changing your function signatures; callers just see `impl Iterator<Item = T>`
- **Closure returns** — `impl Fn(...)` lets you return closures from functions without boxing, enabling functional-style composition with zero overhead

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Opaque return type | Module type ascription | `fn f() -> impl Trait` |
| Concrete type hidden from caller | Yes (module signature) | Yes (compiler knows, caller doesn't) |
| Runtime cost | Zero (monomorphic modules) | Zero (monomorphized) |
| Conditional different types | Variant / functor | Requires `Box<dyn Trait>` or enum |
| Closure return | First-class values (no boxing) | `impl Fn(...)` (no boxing) |
