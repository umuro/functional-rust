📖 **[View on hightechmind.io →](https://hightechmind.io/rust/391-impl-trait-return)**

---

# 391: `impl Trait` in Return Position
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  


## Problem Statement

Closures and complex iterators have unnameable types in Rust. A closure `move |x| x + n` has a unique anonymous type that cannot be written in a function signature. Before `impl Trait` (Rust 1.26), returning closures required `Box<dyn Fn>` with heap allocation. Return-position `impl Trait` (RPIT) tells the compiler "return some concrete type implementing this trait" — the caller doesn't know the exact type, but gets static dispatch with no heap allocation. This is essential for returning closures, lazy iterators, and async futures.

RPIT appears in `std::iter` adapters, `async fn` desugaring (which returns `impl Future`), Rust's generator proposal, and any API that wants to hide implementation types while avoiding boxing costs.

## Learning Outcomes

- Understand return-position `impl Trait` as a way to return unnameable types with static dispatch
- Learn how closures captured with `move` can be returned as `impl Fn`
- See how `std::iter::from_fn` creates stateful iterators without defining new types
- Understand lifetime annotations with `impl Trait + '_` for borrowed return types
- Learn the constraint: a function can only return ONE concrete type behind `impl Trait`

## Rust Application

In `src/lib.rs`, `make_adder` returns `impl Fn(i32) -> i32` — the actual type is an anonymous closure capturing `n`. `make_counter` returns `impl Iterator<Item = i32>` — the concrete `Range<i32>`. `fibonacci` uses `std::iter::from_fn` with two captured mutable variables for state, producing an infinite sequence of Fibonacci numbers without defining a struct. `make_greeting` uses `+ '_` lifetime to return a `String` borrowing from the input.

## OCaml Approach

OCaml functions can return any value including closures without special syntax — closures are first-class values. `let make_adder n = fun x -> x + n` naturally returns a function. Iterators are typically `'a Seq.t` values with lazy evaluation. OCaml's type inference automatically determines the concrete return type without annotation, similar to what Rust achieves with `impl Trait`.

## Key Differences

1. **Necessity**: OCaml doesn't need `impl Trait` for closures since functions are first-class and polymorphically typed; Rust needs it because closures have unique anonymous types.
2. **Single-type constraint**: Rust's `impl Trait` return must be one concrete type; OCaml functions can return different types in different branches (via union types or polymorphism).
3. **Async desugaring**: Rust's `async fn` implicitly uses RPIT (`-> impl Future`); OCaml's `Lwt` or `Eio` use explicit promise types.
4. **Performance**: Both achieve zero allocation for closure returns; Rust makes this explicit with `impl Fn` vs. `Box<dyn Fn>`, OCaml always uses the same value representation.

## Exercises

1. **Memoized adder factory**: Write `make_memoized_adder(n: i32) -> impl FnMut(i32) -> i32` that caches the most recent result using captured mutable state, returning the cached value when called with the same input.
2. **Iterator pipeline**: Write `pipeline(data: Vec<i32>) -> impl Iterator<Item = String>` that filters evens, squares them, and converts to strings — all lazily without collecting intermediate results.
3. **Stateful counter**: Implement `make_counter(step: i32) -> impl FnMut() -> i32` returning a closure that increments by `step` each call, starting at 0. Write tests verifying the state is independent between different closures.
