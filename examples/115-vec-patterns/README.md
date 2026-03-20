📖 **[View on hightechmind.io →](https://hightechmind.io/rust/115-vec-patterns)**

---

# 115-vec-patterns — Vec Iterator Patterns
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  


## Problem Statement

The vector iterator pattern — `filter`, `map`, `fold`, `zip`, `flat_map`, `scan` — is the functional programming toolkit applied to Rust's `Vec<T>`. These adapters are lazy, composable, and zero-overhead: the compiler fuses them into a single loop with no intermediate allocations.

This example demonstrates the full map-filter-fold pipeline, showing how OCaml's `List.map`, `List.filter`, `List.fold_left`, and `List.concat_map` translate to Rust's iterator chain.

## Learning Outcomes

- Chain `filter`, `map`, and `sum` in a single lazy pass
- Use `zip` to pair two iterators element-wise
- Use `flat_map` to expand-and-flatten (OCaml's `List.concat_map`)
- Build a histogram with `fold`
- Compute prefix sums with `scan`

## Rust Application

`src/lib.rs` implements five patterns. `sum_of_doubled_evens` chains filter + map + sum without intermediate allocations. `zip_names_ages` pairs two slices into tuples. `partition_by_age` splits pairs using `partition`. `expand_range` uses `flat_map` to expand each element into a range. `histogram` builds a frequency map with `fold`.

All these operations are lazy: the iterator chain is fused by the compiler into a single loop. The `collect()` call triggers evaluation. Calling `.count()` or `.sum()` directly avoids the allocation entirely.

## OCaml Approach

```ocaml
(* sum_of_doubled_evens *)
let sum_doubled_evens data =
  data
  |> List.filter (fun x -> x mod 2 = 0)
  |> List.map (fun x -> x * 2)
  |> List.fold_left (+) 0

(* flat_map *)
let expand_range data =
  List.concat_map (fun x -> List.init x Fun.id) data
```

OCaml's `|>` pipeline is strict — each step allocates a new list. Rust's `.filter().map().sum()` chain is lazy and allocation-free until `.collect()`.

## Key Differences

1. **Laziness**: Rust's iterator chain is lazy (no intermediate collections); OCaml's `List.map |> List.filter` creates intermediate lists at each step.
2. **`partition`**: Rust's `Iterator::partition` produces two `Vec`s in one pass; OCaml's `List.partition` is also one pass.
3. **`flat_map` vs `concat_map`**: Rust's `flat_map(|x| 0..x)` flattens ranges; OCaml uses `List.concat_map` which flattens lists.
4. **`scan` vs running fold**: Rust's `scan` is a first-class iterator adapter; OCaml has no direct equivalent in stdlib (use `List.fold_left` with accumulation).

## Exercises

1. Write `word_count_pipeline(text: &str) -> HashMap<String, usize>` using `split_whitespace().map().fold()` in a single chain.
2. Implement `sliding_window_averages(data: &[f64], k: usize) -> Vec<f64>` using `windows(k)` and `map`.
3. Write `group_consecutive_equal(data: &[i32]) -> Vec<Vec<i32>>` using `fold` to collect equal consecutive elements into groups.
