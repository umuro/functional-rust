📖 **[View on hightechmind.io →](https://hightechmind.io/rust/290-iterator-unzip-partition)**

---

# 290: Advanced Splitting Patterns
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Real-world data classification often requires more than a binary split. Partitioning numbers into negative, zero, and positive; splitting strings by parse success while keeping both results; routing events to different queues — these require multi-way classification in a single pass. This example explores `unzip`, `partition`, and custom fold-based trisection patterns, demonstrating when to use each.

## Learning Outcomes

- Distinguish `unzip()` (split pairs by position) from `partition()` (split by predicate)
- Implement multi-way classification beyond binary using `fold()`
- Use `partition_map` patterns (via fold) for splitting while transforming
- Recognize that each additional split requires one more collection — and one more `fold` branch

## Rust Application

`partition` splits by boolean predicate; `unzip` splits by pair structure; `fold` enables arbitrary multi-way classification:

```rust
// Binary partition by sign
let (negatives, non_neg): (Vec<i32>, Vec<i32>) =
    vec![-1, 0, 1, -2, 2].into_iter().partition(|&x| x < 0);

// Unzip pairs
let (keys, values): (Vec<_>, Vec<_>) = pairs.into_iter().unzip();

// Trisect: three-way classification via fold
let (neg, zero, pos) = nums.into_iter().fold(
    (vec![], vec![], vec![]),
    |(mut n, mut z, mut p), x| {
        match x.cmp(&0) {
            Ordering::Less => n.push(x),
            Ordering::Equal => z.push(x),
            Ordering::Greater => p.push(x),
        }
        (n, z, p)
    }
);
```

## OCaml Approach

OCaml's `List.partition` handles binary splits. For multi-way classification, `List.fold_left` with a tuple accumulator is the standard approach — identical in structure to the Rust `fold` pattern:

```ocaml
let (neg, zero, pos) = List.fold_left (fun (n,z,p) x ->
  if x < 0 then (x::n, z, p)
  else if x = 0 then (n, x::z, p)
  else (n, z, x::p)
) ([], [], []) nums
```

## Key Differences

1. **Immutable accumulation**: OCaml's fold accumulator is passed by value and returned — `x::n` creates a new cons cell; Rust's `fold` with `Vec::push` mutates in place.
2. **Ordering**: OCaml's fold-based partition reverses order (prepend to list); Rust's `push` preserves insertion order.
3. **Ergonomics**: The `itertools` crate's `partition_map` and `partition_fold` provide cleaner APIs for common multi-way patterns.
4. **Real-world use**: Router dispatching, event classification, multi-bucket histogram construction.

## Exercises

1. Implement a four-way classifier that sorts strings into "short" (≤3), "medium" (4-7), "long" (8-12), and "very long" (>12) buckets.
2. Use fold to simultaneously partition results into successes and failures while counting each.
3. Build a pipeline that parses strings into `i32`, filters evens, and collects both valid evens and parse errors in a single fold.
