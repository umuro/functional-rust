📖 **[View on hightechmind.io →](https://hightechmind.io/rust/120-closure-types)**

---

# Closure Types: Fn, FnMut, FnOnce

## Problem Statement

Closures that capture their environment behave differently depending on what they do with captured values — read them, mutate them, or consume them. Rust encodes this in three traits: `Fn` (shared borrow), `FnMut` (mutable borrow), `FnOnce` (takes ownership). The hierarchy `Fn ⊆ FnMut ⊆ FnOnce` means the compiler selects the most restrictive trait that still allows the closure to be called correctly, preventing data races and use-after-move bugs at compile time.

## Learning Outcomes

- Understand the three closure traits and when each is inferred
- Learn how the trait hierarchy (`Fn` implies `FnMut` implies `FnOnce`) affects function bounds
- See why `FnOnce` closures can only be called once and how the type system enforces this
- Practice writing higher-order functions that accept each closure kind

## Rust Application

`make_greeter` returns `impl Fn(&str) -> String` — the closure only reads `prefix`, so `Fn` is inferred. `make_counter` returns `impl FnMut() -> u32` — the closure mutates `count`, requiring `FnMut`. A closure that moves a `Vec` out of its capture implements only `FnOnce`, and the compiler rejects a second call at compile time. `apply_twice<F: Fn()>` correctly signals that `f` will be invoked multiple times without mutation.

## OCaml Approach

OCaml functions always implement the equivalent of `Fn` — closures in OCaml capture variables by reference to a heap-allocated environment and can be called any number of times. There is no distinction between `Fn`, `FnMut`, and `FnOnce` because OCaml's GC manages the environment and mutation is controlled separately via `ref` cells.

## Key Differences

1. **Trait stratification**: Rust distinguishes three calling conventions at the type level; OCaml has one uniform closure type.
2. **Mutation safety**: Rust's `FnMut` bound signals that a closure mutates state, preventing concurrent sharing; OCaml relies on the programmer to avoid unsafe concurrent mutation.
3. **Consume on call**: `FnOnce` enforces single-call semantics in the type system; OCaml has no equivalent — a closure returning a consumed value would panic at runtime, not compile time.
4. **Inference**: Rust infers the tightest trait that fits; OCaml infers a single function type `'a -> 'b`.

## Exercises

1. Write a function `apply_once<F: FnOnce() -> String>(f: F) -> String` and verify that calling `f` twice causes a compile error.
2. Implement a memoizing wrapper: `memoize<F: FnMut(u32) -> u32>(f: F) -> impl FnMut(u32) -> u32` that caches results.
3. Create a pipeline combinator that takes a `Vec<Box<dyn FnMut(i32) -> i32>>` and applies each closure in sequence.
