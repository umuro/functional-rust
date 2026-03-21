📖 **[View on hightechmind.io →](https://hightechmind.io/rust/405-iterator-trait-deep)**

---

# 405: Iterator Trait Deep Dive
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Rust's `Iterator` trait is one of the most carefully designed APIs in the standard library. Implementing just `fn next(&mut self) -> Option<Self::Item>` unlocks 75+ adapter methods: `map`, `filter`, `fold`, `take`, `skip`, `chain`, `zip`, `enumerate`, `flat_map`, and more — all built as default methods on top of `next`. This design means any custom data structure that implements `Iterator` gains the entire rich functional pipeline for free, with zero-cost abstraction through lazy evaluation and monomorphization.

Iterators power Rust's approach to functional programming without allocations: entire processing pipelines execute lazily, only materializing with `.collect()` or `.fold()`.

## Learning Outcomes

- Understand how implementing only `fn next` unlocks the entire iterator adapter library
- Learn how to create stateful iterators using struct fields
- See how `size_hint` enables efficient pre-allocation in `.collect()`
- Understand lazy evaluation: adapters are zero-cost until consumed by `for` or a terminal operation
- Learn how custom iterators compose with `std` adapters (`take`, `zip`, `enumerate`)

## Rust Application

In `src/lib.rs`, `Fibonacci` and `Countdown` each implement `Iterator` with a single `next` method. `Fibonacci` uses two `u64` fields for state. `Countdown` overrides `size_hint()` returning an exact count — enabling `Vec::with_capacity` optimizations in `.collect()`. Both immediately work with `take(10).collect::<Vec<_>>()`, `zip`, `sum`, `enumerate`, and all other adapters. The `Default` derive enables `Fibonacci::default()` construction.

## OCaml Approach

OCaml's `Seq.t` is the lazy sequence type: `type 'a t = unit -> 'a node` where `node = Nil | Cons of 'a * 'a t`. A Fibonacci sequence is `let rec fib a b = fun () -> Seq.Cons (a, fib b (a+b))`. The `Seq` module provides `map`, `filter`, `take`, `fold_left` adapters. Unlike Rust's iterator, OCaml sequences are persistent (can be consumed multiple times). The `Iter` module provides mutable imperative iterators closer to Rust's model.

## Key Differences

1. **One method**: Rust requires implementing only `next`; OCaml's `Seq.t` is a recursive type that requires understanding the `node` type structure.
2. **Mutability**: Rust's iterators are mutable (progress is tracked in `&mut self`); OCaml's `Seq.t` is immutable and persistent.
3. **Eagerness**: Both are lazy by default; Rust materializes with `.collect()`, OCaml with `Seq.to_list` or `Seq.fold_left`.
4. **Adapter count**: Rust's `Iterator` has 75+ adapters as defaults; OCaml's `Seq` has ~20 functions, with more in `Base.Sequence`.

## Exercises

1. **Primes iterator**: Implement `struct Primes { current: u64 }` implementing `Iterator<Item = u64>` using trial division. The iterator yields successive prime numbers. Write a test collecting the first 10 primes.
2. **Zip with index (enumerate)**: Without using `.enumerate()`, implement a `Numbered<I: Iterator>` wrapper that implements `Iterator<Item = (usize, I::Item)>`. Verify it matches `.enumerate()` output exactly.
3. **Infinite spiral**: Implement a `Spiral` iterator that yields `(i32, i32)` coordinates in a clockwise spiral from (0,0): (0,0), (1,0), (1,-1), (0,-1), (-1,-1), (-1,0), (-1,1), (0,1), (1,1), (2,1), ... Use it with `.take(25)` to verify the spiral pattern.
