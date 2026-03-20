[![hightechmind.io](https://img.shields.io/badge/hightechmind.io-functional--rust-blue)](https://hightechmind.io)

# 085 — Iterator Trait

## Problem Statement

Implement the `Iterator` trait from scratch for two custom types: a bounded `MyRange` iterator and an infinite `Fibonacci` iterator. By implementing only `next`, gain access to the entire iterator adapter ecosystem (`filter`, `map`, `take`, `collect`). Compare with OCaml's `Seq` type and lazy sequence operations.

## Learning Outcomes

- Implement `Iterator` by defining only `fn next(&mut self) -> Option<Self::Item>`
- Understand that all other iterator methods (`map`, `filter`, `collect`) come for free
- Create an infinite iterator and use `.take(n)` to bound it safely
- Use `type Item = i32` as the associated type required by `Iterator`
- Map Rust's iterator protocol to OCaml's `'a Seq.t = unit -> 'a Seq.node` lazy type
- Recognise when to implement `Iterator` vs using `iter()` on existing collections

## Rust Application

`MyRange` stores `current` and `end_` and increments in `next`, returning `None` when `current >= end_`. `Fibonacci` stores `(a, b)` and always returns `Some(val)` — an infinite iterator. The `demo_free_methods` function shows that once `next` is implemented, the full adapter chain `MyRange::new(0,10).filter(…).map(…).collect()` works without any additional implementation. `Fibonacci::new().take(10).collect::<Vec<_>>()` materialises the first 10 Fibonacci numbers.

## OCaml Approach

OCaml's `Seq` type represents lazy sequences: `type 'a my_seq = unit -> 'a my_node` with `Nil | Cons of 'a * 'a my_seq`. `range_seq a b` is a thunk that produces the next element on demand. `seq_map`, `seq_filter`, and `seq_fold` mirror Rust's adapters. The key difference: OCaml sequences are lazy by default (thunks), while Rust's iterators are lazy by protocol (pull-based `next`). Both achieve the same deferred evaluation.

## Key Differences

| Aspect | Rust | OCaml |
|--------|------|-------|
| Interface | `trait Iterator { fn next(&mut self) -> Option<Item> }` | `type 'a t = unit -> 'a node` |
| Infinite sequences | `Fibonacci` with no termination | `Seq` with thunks |
| Bounding | `.take(n)` | `seq_take n` |
| Adapters | Free from trait (`map`, `filter`, etc.) | Explicit `seq_map`, `seq_filter` |
| Mutability | `&mut self` (mutable state) | Thunks (immutable, creates new seq) |
| Collection | `.collect::<Vec<_>>()` | `seq_fold` or `List.of_seq` |

Implementing `Iterator` is one of Rust's most powerful patterns: a single method unlocks dozens of combinators. Any data source — database cursor, file reader, tree traversal — becomes a first-class iterator with zero overhead.

## Exercises

1. Implement a `Cycle<I: Iterator + Clone>` iterator that wraps another iterator and repeats it infinitely.
2. Add a `step_by` field to `MyRange` and implement stepping (e.g. every 3rd element) in `next`.
3. Implement `DoubleEndedIterator` for `MyRange` by adding `next_back(&mut self) -> Option<i32>`.
4. Write a `ZipIter<A: Iterator, B: Iterator>` that zips two iterators into pairs without using `std::iter::Zip`.
5. In OCaml, implement an infinite sequence of prime numbers using the `Seq` module and the sieve of Eratosthenes.
