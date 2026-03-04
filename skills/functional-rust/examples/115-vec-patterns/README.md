# 115: Vec Operations Functionally

**Difficulty:** 2  **Level:** Intermediate

Rust's iterator adapters (`map`, `filter`, `fold`, `flat_map`) translate OCaml's list operations directly — with lazy evaluation and zero intermediate allocation.

## The Problem This Solves

Imperative loops work, but they obscure intent. A `for` loop that filters some items, transforms others, and accumulates a result puts the *how* (loop, counter, conditional append) in the way of the *what* (select items matching X, transform by Y, reduce to Z). Bugs hide in the bookkeeping.

Python has list comprehensions and `map`/`filter`, but they're eager — every intermediate result is a full list in memory. Chain three operations and you have three lists allocated. For large datasets, this matters.

Java's streams are lazy but verbose. And languages without an ownership model have to decide: does `map` copy the collection or share it? Does the caller own the transformed result?

Rust's iterator system is lazy and zero-cost. `vec.iter().map(f).filter(g)` allocates nothing until you call `.collect()` or consume the iterator. Each element flows through the entire pipeline in one pass. The resulting code reads like a data pipeline description — what you're doing, not how you're doing it — and compiles to code as efficient as a hand-written loop.

## The Intuition

Rust's iterator adapters are lazy transformations that chain without allocating intermediate collections — call `.collect()` at the end to materialize the result exactly once.

## How It Works in Rust

```rust
let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

// OCaml: List.map (fun x -> x * x) lst
let squares: Vec<i32> = numbers.iter()
    .map(|&x| x * x)
    .collect();

// OCaml: List.filter (fun x -> x mod 2 = 0) lst
let evens: Vec<i32> = numbers.iter()
    .filter(|&&x| x % 2 == 0)
    .copied()
    .collect();

// OCaml: List.fold_left (+) 0 lst
let sum: i32 = numbers.iter().sum();
let sum2: i32 = numbers.iter().fold(0, |acc, &x| acc + x);

// OCaml: List.filter_map
let parsed: Vec<i32> = vec!["1", "two", "3", "four", "5"]
    .iter()
    .filter_map(|s| s.parse().ok())
    .collect();
// [1, 3, 5]

// OCaml: List.flatten (List.map f lst)
let flat: Vec<i32> = vec![vec![1, 2], vec![3, 4], vec![5]]
    .into_iter()
    .flatten()
    .collect();

// flat_map = map + flatten
let words = vec!["hello world", "foo bar"];
let all_words: Vec<&str> = words.iter()
    .flat_map(|s| s.split_whitespace())
    .collect();

// Chaining multiple operations — one pass, zero intermediate allocations
let result: Vec<String> = numbers.iter()
    .filter(|&&x| x % 2 == 0)   // keep evens
    .map(|&x| x * x)             // square them
    .filter(|&x| x > 10)         // keep > 10
    .map(|x| format!("{}!", x))  // format as strings
    .collect();
// ["16!", "36!", "64!", "100!"]

// zip, enumerate, take, skip
let paired: Vec<(i32, i32)> = numbers.iter()
    .copied()
    .zip(numbers.iter().copied().rev())
    .take(3)
    .collect();
// [(1, 10), (2, 9), (3, 8)]

// into_iter() consumes the Vec; iter() borrows
let owned_result: Vec<i32> = numbers.into_iter()
    .map(|x| x * 2)
    .collect();
// numbers is moved — can't use it again
```

## What This Unlocks

- **One-pass laziness** — chained iterators process each element once through the entire pipeline; no intermediate `Vec` allocations regardless of chain length.
- **Composable pipelines** — each adapter (`map`, `filter`, `flat_map`) is a pure transformation; the pipeline describes intent, not implementation.
- **Ownership clarity** — `.iter()` borrows, `.iter_mut()` borrows mutably, `.into_iter()` consumes; the choice is explicit and the compiler enforces it.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Map | `List.map f lst` | `vec.iter().map(f).collect()` |
| Filter | `List.filter pred lst` | `vec.iter().filter(pred).collect()` |
| Fold | `List.fold_left f init lst` | `vec.iter().fold(init, f)` |
| Flat map | `List.concat_map f lst` | `vec.iter().flat_map(f).collect()` |
| Laziness | Eager (lists) / Lazy (Seq) | Always lazy — evaluated on `.collect()` or consumption |
| Intermediate allocations | One per operation | Zero — single pass through the pipeline |
