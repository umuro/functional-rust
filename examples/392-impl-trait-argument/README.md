📖 **[View on hightechmind.io →](https://hightechmind.io/rust/392-impl-trait-argument)**

---

# 392: `impl Trait` in Argument Position

## Problem Statement

Generic function signatures with multiple trait bounds become syntactically heavy: `fn process<I: Iterator<Item = i32>, F: Fn(i32) -> i32>(items: I, f: F)`. Argument-position `impl Trait` provides syntactic sugar: `fn process(items: impl Iterator<Item = i32>, f: impl Fn(i32) -> i32)`. Unlike return-position `impl Trait`, argument-position `impl Trait` is exactly equivalent to a generic type parameter — each call site can pass a different concrete type, and the function is monomorphized for each one. The syntax is cleaner but the semantics are identical to bounded generics.

This syntactic pattern appears throughout `std`, `tokio`, `rayon`, and modern Rust APIs as a more readable alternative to explicit type parameters.

## Learning Outcomes

- Understand that argument-position `impl Trait` is syntactic sugar for bounded generics
- Learn how `impl Iterator<Item = T>` accepts any iterator type producing `T` values
- See how `impl Fn(i32) -> i32` accepts any callable (closure, function pointer, custom `Fn` impl)
- Understand the difference from return-position `impl Trait` (argument position allows multiple types per parameter)
- Learn when argument-position `impl Trait` is clearer than explicit type parameters

## Rust Application

In `src/lib.rs`, `print_all(items: impl Iterator<Item = impl std::fmt::Display>)` accepts any iterator of displayable items — nested `impl Trait`. `sum_all(nums: impl Iterator<Item = i32>) -> i32` accepts any `i32` iterator. `process<F: Fn(i32) -> i32>(items: impl Iterator<Item = i32>, f: F)` mixes both styles. `debug_any(val: impl fmt::Debug)` accepts any debuggable value. Each call is monomorphized separately.

## OCaml Approach

OCaml's polymorphic functions achieve the same without special syntax. `let sum_all nums = Seq.fold_left (+) 0 nums` works for any integer sequence. OCaml uses structural typing and type inference — the function type is inferred to be `int Seq.t -> int` automatically. Higher-order functions accepting callbacks use `('a -> 'b) -> ...` types, which are equivalent to Rust's `impl Fn(A) -> B`.

## Key Differences

1. **Syntax**: Rust requires explicit `impl Trait` or type parameter syntax; OCaml infers constraint from usage automatically.
2. **Monomorphization**: Rust generates separate code for each concrete type; OCaml uses uniform representation (boxing) for most polymorphic functions.
3. **Multiple impls**: Rust can use the same `impl Trait` parameter position with different concrete types in different calls; OCaml's polymorphism is implicit.
4. **Type inference**: Rust requires at least the trait bound to be stated; OCaml often requires no annotation at all for equivalent generic functions.

## Exercises

1. **Sorted collector**: Write `fn collect_sorted<T: Ord>(items: impl Iterator<Item = T>) -> Vec<T>` that collects all items and sorts them. Show it works with `i32`, `String`, and a custom `Ord` type.
2. **Higher-order pipeline**: Write `fn pipeline<A, B, C>(items: impl Iterator<Item = A>, f: impl Fn(A) -> B, g: impl Fn(B) -> C) -> Vec<C>` that applies two transformations. Test with string length then squaring.
3. **Trait object comparison**: Rewrite `sum_all` and `process` using explicit generic type parameters instead of `impl Trait`. Then try rewriting them using `Box<dyn Trait>`. Write a comment explaining which form is best for each use case.
