📖 **[View on hightechmind.io →](https://hightechmind.io/rust/256-iterator-chain)**

---

# 256: Chaining Iterators with chain()
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Sequential processing of multiple separate collections is a universal programming need. Before lazy iterator composition, programmers allocated a new combined collection just to iterate over it — wasteful in both time and memory. The `chain()` combinator solves this by creating a single iterator that moves through one source, then continues with another, producing zero intermediate allocations. This is the functional programming principle of iterator composition: build complex traversal logic from small, single-purpose pieces.

## Learning Outcomes

- Understand how `chain()` concatenates two iterators lazily with no intermediate allocation
- Recognize when to use `chain()` instead of concatenating into a new `Vec`
- Combine `chain()` with other adapters (`map`, `filter`, `sum`) in pipelines
- Chain more than two sources by applying `chain()` multiple times

## Rust Application

Rust's `Iterator::chain()` takes any `IntoIterator` and returns a `Chain<A, B>` struct implementing `Iterator`. The chain switches from the first source to the second only when the first is exhausted. No data is copied:

```rust
let a = [1i32, 2, 3];
let b = [4i32, 5, 6];
let result: Vec<i32> = a.iter().chain(b.iter()).copied().collect();
// [1, 2, 3, 4, 5, 6] — no intermediate allocation
```

Chaining with an empty iterator is always safe. The `sum()` example in the tests demonstrates that you can fold over chained iterators without ever collecting into a `Vec`.

## OCaml Approach

OCaml uses `List.append` (the `@` operator) for strict list concatenation, which copies the left spine. For lazy sequences, `Seq.append` is the true equivalent of `chain()`:

```ocaml
let chained = Seq.append (List.to_seq [1;2;3]) (List.to_seq [4;5;6])
(* Lazy: nothing runs until consumed *)
```

## Key Differences

1. **Laziness**: Rust's `chain()` is always lazy; OCaml's `List.append` / `@` is strict and allocates immediately.
2. **Type homogeneity**: Both iterators must yield the same `Item` type in Rust; OCaml's polymorphic lists handle this naturally.
3. **Source flexibility**: Rust's `chain()` works on any `Iterator` implementation — slices, ranges, custom types — not only lists.
4. **Lifetime tracking**: Rust's borrow checker ensures chained references remain valid; OCaml relies on GC.

## Exercises

1. Chain three slices of strings and collect all words into a single `Vec<&str>` without allocating an intermediate combined slice.
2. Use `chain()` to prepend a sentinel header element and append a footer element to an iterator of body items.
3. Build a function `chain_n(slices: &[&[i32]]) -> Vec<i32>` that chains an arbitrary number of slices using `Iterator::flatten` or repeated `chain()` calls.
