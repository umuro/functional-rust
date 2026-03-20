📖 **[View on hightechmind.io →](https://hightechmind.io/rust/390-type-alias-impl-trait)**

---

# 390: Type Alias and `impl Trait`
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  


## Problem Statement

Complex iterator chains in Rust have unwritable types: `Filter<Map<IntoIter<i32>, fn(i32) -> i32>, fn(&i32) -> bool>` is the actual return type of a filtered map. Before `impl Trait` (stabilized in Rust 1.26), returning such types required boxing with `Box<dyn Iterator>`, adding heap allocation and vtable overhead. `impl Trait` in return position lets the compiler infer the concrete type while hiding it from the caller — giving static dispatch performance with ergonomic opaque types. Type aliases make these patterns reusable.

`impl Trait` return types appear throughout `std` (`.filter()`, `.map()`, `.chain()`), `tokio`'s `async fn` desugaring, and any performance-sensitive API that returns complex iterator or future types.

## Learning Outcomes

- Understand `impl Trait` in return position as a way to hide complex concrete types
- Learn the difference between `impl Trait` (static dispatch, one concrete type) and `Box<dyn Trait>` (dynamic dispatch, any type)
- See how type aliases (`type BoxedIter<T> = Box<dyn Iterator<Item = T>>`) improve readability
- Understand when to choose `impl Trait` vs. `Box<dyn Trait>` (lifetime, heterogeneous storage, recursion)
- Learn how closures captured in `impl Iterator` prevent naming the return type

## Rust Application

In `src/lib.rs`, `make_counter` returns `impl Iterator<Item = i32>` — actually a `Range<i32>`, but the caller only sees the trait. `make_even_filter` returns `impl Iterator<Item = i32>` wrapping a `Filter` over a `Vec` iterator. `squares` chains `.map()` for squares. `BoxedIter<T>` aliases `Box<dyn Iterator<Item = T>>`, and `range_boxed` shows the boxed alternative which is heap-allocated but allows storage in structs and heterogeneous use.

## OCaml Approach

OCaml handles abstract return types through module signatures and functors. A function returning an abstract `'a Seq.t` hides the concrete sequence implementation. OCaml's lazy sequences (`Seq.t`) naturally compose like iterators. OCaml doesn't need `impl Trait` because function types are already abstract in module interfaces — the `.mli` file determines what's exposed.

## Key Differences

1. **Inference**: Rust infers the concrete type behind `impl Trait` and enforces it's a single type; OCaml infers the full concrete type at definition and can expose or hide it via `.mli`.
2. **Dynamic vs. static**: Rust explicitly chooses `impl Trait` (static) or `Box<dyn Trait>` (dynamic); OCaml uses dynamic dispatch for objects, static for modules.
3. **Closure types**: Rust closures are unique anonymous types requiring `impl Trait` or `Box<dyn Fn()>` in return positions; OCaml function types are first-class and returnable as `'a -> 'b`.
4. **Type aliases**: Rust's `type BoxedIter<T> = Box<dyn Iterator<Item = T>>` creates a generic alias; OCaml's `type 'a iter = 'a Seq.t` is equivalent and idiomatic.

## Exercises

1. **Generator pattern**: Write a function `fibonacci() -> impl Iterator<Item = u64>` using `std::iter::from_fn` with captured mutable state. The iterator should produce Fibonacci numbers indefinitely.
2. **Return multiple iterators**: Write a function `choose_iter(flag: bool) -> Box<dyn Iterator<Item = i32>>` that returns either `(0..10)` or `(10..0)` based on the flag. Explain why `impl Trait` cannot be used here but `Box<dyn>` can.
3. **Chaining with impl Trait**: Implement a `Pipeline<T>` builder that accumulates `impl Fn(T) -> T` transformations and has a `run(input: T) -> T` method. Use `impl Trait` bounds to avoid boxing.
