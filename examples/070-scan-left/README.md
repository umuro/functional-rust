📖 **[View on hightechmind.io →](https://hightechmind.io/rust/070-scan-left)**

---

# 070 — Scan Left (Running Accumulation)

## Problem Statement

`scan_left` (Haskell: `scanl`, OCaml: `List.fold_left` with output collection) produces all intermediate values of a fold, not just the final result. Where `fold([1,2,3,4], +, 0)` gives `10`, `scan_left([1,2,3,4], +, 0)` gives `[0, 1, 3, 6, 10]` — every prefix sum including the initial accumulator. This is the "running total" operation.

Running totals appear throughout practical computing. Financial ledgers track a running balance alongside each transaction, enabling audit trails. Prefix sum arrays — the canonical scan_left application — allow O(1) range sum queries: `sum(i..j) = prefix[j] - prefix[i]`, making them a standard data structure in competitive programming and database query engines. Exponential moving averages in signal processing and real-time analytics compute as a scan over time-series data. Compilers accumulate struct field offset tables by scanning over field sizes. Any computation that needs to observe the fold at every step, not just at the end, uses scan.

## Learning Outcomes

- Use Rust's `.scan(init, |acc, &x| { *acc = ...; Some(*acc) })` for running accumulation with mutable state
- Implement a generic `scan_left<T: Clone, F>(init: T, v: &[T], f: F) -> Vec<T>` that mirrors OCaml's pattern
- Distinguish scan (keeps all intermediates) from fold (keeps only the final result)
- Build prefix sum arrays for O(1) range sum queries
- Implement running max, running min, and running product with the same generic combinator
- Recognize that scan is fold with intermediate-result observation at every step

## Rust Application

`running_sum` uses Rust's built-in `.scan()` iterator method:
- Mutable state `*state` is updated in place each step
- Each invocation emits the current state via `Some(*state)`
- The initial `0` is prepended to include the starting accumulator

The generic `scan_left<T: Clone, F>(init: T, v: &[T], f: F) -> Vec<T>` starts with `vec![init.clone()]` and loops, pushing `f(&acc, x)` at each step. The `Clone` bound is required because every intermediate value must be owned and stored. `running_product` reuses `scan_left(1, v, |a, b| a * b)`, showing the same combinator works for any associative operation.

## OCaml Approach

OCaml's standard library `List.fold_left` computes only the final result. For scan, you define it manually:

```ocaml
let scan_left f init lst =
  let rec aux acc lst result =
    match lst with
    | [] -> List.rev result
    | x :: t ->
      let acc' = f acc x in
      aux acc' t (acc' :: result)
  in
  init :: aux init lst []
```

This is tail-recursive via the accumulator `result` list, reversed at the end. OCaml 4.11 added `List.fold_left_map` which produces a transformed list alongside the accumulator, partially overlapping with scan for element-wise transforms but not providing running intermediates directly.

## Key Differences

1. **`.scan()` in stdlib**: Rust's `Iterator::scan` is built in — providing `(0..n).scan(0, |acc, x| { *acc += x; Some(*acc) })`. OCaml has no stdlib `scan`; it must be manually implemented.
2. **State mutation**: Rust's scan closure receives `&mut state` and can modify it. OCaml's functional approach passes the new accumulator as a recursive argument.
3. **Including initial value**: The standard `scan_left(f, init, list)` includes `init` as the first element. Rust's `.scan()` does NOT include the initial value — it starts with the result of applying `f` to the first element. Account for this difference.
4. **Prefix sum array**: `(0..=n).scan(0i32, |s, x| { *s += x; Some(*s) }).collect::<Vec<_>>()` builds a prefix sum array for O(1) range queries: `sum(i..j) = prefix[j] - prefix[i]`.

## Exercises

1. **Range sum query**: Build a prefix sum array from `[3, 1, 4, 1, 5, 9, 2, 6]`. Write `range_sum(prefix: &[i32], i: usize, j: usize) -> i32` that computes the sum of elements from index i to j in O(1).
2. **Running statistics**: Write `running_mean(v: &[f64]) -> Vec<f64>` where `result[i]` is the mean of `v[0..=i]`. Use `scan` with a `(sum, count)` state tuple.
3. **Sliding window max**: Using a running max, implement a sliding window maximum of size k: for each position i, find the maximum in `v[i-k+1..=i]`. This requires a deque, not just scan — research the monotone deque approach.
