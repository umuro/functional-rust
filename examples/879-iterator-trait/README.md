📖 **[View on hightechmind.io →](https://hightechmind.io/rust/879-iterator-trait)**

---

# 879-iterator-trait — Iterator Trait
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  


## Problem Statement

Iteration is fundamental to every program. Languages handle it differently: C uses index-based loops, Java uses `Iterable`/`Iterator` interfaces, Python uses `__iter__`/`__next__`. Rust's `Iterator` trait requires only one method — `fn next(&mut self) -> Option<Self::Item>` — and provides over 70 adapter and consumer methods for free. This design unifies all iteration patterns: ranges, collections, generators, and lazy sequences all implement the same interface. OCaml's `Seq` module serves a similar role for lazy sequences, and `List` provides eager equivalents. Understanding the `Iterator` trait is essential for idiomatic Rust.

## Learning Outcomes

- Implement the `Iterator` trait with only the `next` method
- Understand that all other iterator methods are provided automatically via default implementations
- Build both finite and infinite custom iterators
- Implement `IntoIterator` to make custom types work in `for` loops
- Compare Rust's `Iterator` with OCaml's `Seq` lazy sequence abstraction

## Rust Application

The code defines a custom `Range` struct implementing `Iterator<Item = i32>` with a finite `next`. A `Counter` struct implementing an infinite iterator (always returning `Some`) demonstrates lazy infinite sequences. Because both implement `Iterator`, they automatically gain `.map()`, `.filter()`, `.take()`, `.collect()`, etc. without any additional implementation. The `IntoIterator` pattern is shown for making custom collections work in `for` loops. The key insight: one method enables the entire iterator ecosystem.

## OCaml Approach

OCaml's `Seq` module uses a lazy list: `type 'a t = unit -> 'a node` where `node = Nil | Cons of 'a * 'a t`. A custom sequence is a function returning the next node lazily. The `Seq` module provides `map`, `filter`, `take`, `flat_map` as standalone functions rather than methods. For eager iteration, OCaml uses `List.iter`, `Array.iter`, or explicit recursion. `Seq.of_list` and `List.of_seq` convert between representations.

## Key Differences

1. **Method vs function**: Rust iterator methods are called on the value (`iter.map(f)`); OCaml uses module-level functions (`Seq.map f seq`).
2. **Infinite sequences**: Both support infinite iterators; Rust via `Option::Some` always, OCaml via `Seq.Cons` always.
3. **Laziness**: Rust iterators are lazy by default (adapters don't evaluate until consumed); OCaml `Seq` is explicitly lazy via `unit ->`.
4. **Mutability**: Rust `Iterator::next` takes `&mut self` (mutable internal state); OCaml sequences are immutable — each step returns a new sequence.

## Exercises

1. Implement a `Primes` iterator that yields prime numbers lazily using a sieve or trial division.
2. Implement `IntoIterator` for a custom `Grid<T>` type that yields elements in row-major order.
3. Implement a `Zip2<A: Iterator, B: Iterator>` struct that pairs elements from two iterators, stopping when either is exhausted.
