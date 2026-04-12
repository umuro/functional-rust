📖 **[View on hightechmind.io →](https://hightechmind.io/rust/521-closure-map-reduce)**

---

# Map-Reduce with Closures
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Map-reduce was popularized by Google's 2004 paper as a framework for processing petabytes of data across thousands of machines. But the pattern predates distributed computing — it is a direct application of `map` and `fold` from functional programming, rooted in lambda calculus. Even in single-threaded Rust, the map-reduce idiom cleanly separates the transformation of individual elements from their aggregation, making code more composable and testable than equivalent imperative loops.

## Learning Outcomes

- How to implement a generic `map_reduce` combining a mapper and reducer closure
- How `fold` serves as the universal aggregation primitive
- How to express word frequency counting, sum of squares, and string joining as map-reduce pipelines
- How `group_by_key` generalizes the reduce step to produce grouped collections
- Why separating map from reduce enables parallelism (Rayon, async tasks)

## Rust Application

`map_reduce<T, U, V, M, R>(items, mapper, reducer, init)` composes `iter().map(mapper).fold(init, reducer)` in a single generic function. `word_count` uses `fold` to build a `HashMap<&str, usize>` by counting occurrences. `sum_of_squares` calls `map_reduce` with `|&x| x * x` and `|acc, x| acc + x`. `join_strings` folds a string slice with a separator without allocating an intermediate `Vec`. `group_by_key` maps each item to a key and folds into a `HashMap<K, Vec<T>>`.

Key patterns:
- `items.iter().map(mapper).fold(init, reducer)` — map then fold
- `*acc.entry(word).or_insert(0) += 1` — accumulating into a HashMap in fold
- Reusing `map_reduce` for multiple aggregations by varying the mapper/reducer pair

## OCaml Approach

OCaml's `List.map` and `List.fold_left` are the direct equivalents. A word count in OCaml uses `Hashtbl` or `Map` with `List.fold_left`. The pattern is idiomatic and concise, with no need for explicit type annotations in simple cases.

```ocaml
let word_count words =
  List.fold_left (fun tbl w ->
    let n = try Hashtbl.find tbl w with Not_found -> 0 in
    Hashtbl.replace tbl w (n + 1); tbl
  ) (Hashtbl.create 16) words
```

## Key Differences

1. **Parallelism path**: Rust's `map_reduce` can be parallelized by switching `iter()` to `par_iter()` from Rayon with no other change; OCaml's `List.fold_left` is inherently sequential.
2. **Ownership in fold**: Rust's `fold` takes ownership of the accumulator at each step (passing `V` by value); OCaml's fold passes the accumulator by value too but without ownership semantics — the GC handles sharing.
3. **HashMap vs Hashtbl**: Rust's `HashMap` is part of `std::collections`; OCaml's stdlib has `Hashtbl` (mutable) and `Map` (immutable functional maps) as separate choices.
4. **Type inference depth**: Rust requires explicit generic parameters on `map_reduce` signatures; OCaml's HM inference handles the same without annotations in most cases.

## Exercises

1. **Parallel map-reduce**: Replace `iter()` with `rayon::iter::IntoParallelRefIterator::par_iter()` in `map_reduce` and verify the word count produces the same result on a large word list.
2. **Max by key**: Implement `max_by_key<T, K: Ord, F: Fn(&T) -> K>(items: &[T], key: F) -> Option<&T>` using a single `fold` call without importing `Iterator::max_by_key`.
3. **Histogram**: Write a function that takes a `&[f64]` and returns a `Vec<usize>` representing a frequency histogram with `n` equal-width buckets between the min and max values.
