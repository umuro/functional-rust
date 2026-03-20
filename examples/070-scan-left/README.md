📖 **[View on hightechmind.io →](https://hightechmind.io/rust/070-scan-left)**

---

# 070 — Scan Left (Running Accumulation)

## Problem Statement

`scan_left` (Haskell: `scanl`, OCaml: `List.fold_left` with output collection) produces all intermediate values of a fold, not just the final result. `scan_left(+, 0, [1,2,3,4])` gives `[0, 1, 3, 6, 10]` — each prefix sum. This is the "running total" operation.

Running totals appear in financial ledgers, cumulative statistics, prefix sum arrays (for range sum queries in O(1)), exponential moving averages, and sliding-window algorithms. Prefix sum arrays are a fundamental data structure in competitive programming and algorithmic interviews.

## Learning Outcomes

- Use Rust's `.scan(init, |acc, &x| { *acc += x; Some(*acc) })` for running accumulation
- Implement a generic `scan_left<T, F>` function that mirrors OCaml's `List.fold_left` with output
- Distinguish scan (keeps all intermediates) from fold (keeps only final result)
- Use prefix sums for O(1) range sum queries
- Implement running max, running min, and running product with the same pattern

## Rust Application

`running_sum` uses `.scan(0, |state, &x| { *state += x; Some(*state) })` — the `state` is mutated each step, and each call emits the current state. `scan_left<T, F>` is a generic version that works for any accumulator and combining function. `running_product` uses `scan_left(1, v, |a, b| a * b)`. `running_max` maintains the current maximum.

## OCaml Approach

OCaml's `List.fold_left` computes only the final result. For scan: `let scan_left f init lst = let rec aux acc list result = match list with [] -> List.rev result | x :: t -> let new_acc = f acc x in aux new_acc t (new_acc :: result) in init :: aux init lst []`. OCaml 4.11 adds `List.fold_left_map` for producing both a result and a transformed list, which partially addresses this.

## Key Differences

1. **`.scan()` in stdlib**: Rust's `Iterator::scan` is built in — providing `(0..n).scan(0, |acc, x| { *acc += x; Some(*acc) })`. OCaml has no stdlib `scan`; it must be manually implemented.
2. **State mutation**: Rust's scan closure receives `&mut state` and can modify it. OCaml's functional approach passes the new accumulator as a recursive argument.
3. **Including initial value**: The standard `scan_left(f, init, list)` includes `init` as the first element. Rust's `.scan()` does NOT include the initial value — it starts with the result of applying `f` to the first element. Account for this difference.
4. **Prefix sum array**: `(0..=n).scan(0i32, |s, x| { *s += x; Some(*s) }).collect::<Vec<_>>()` builds a prefix sum array for O(1) range queries: `sum(i..j) = prefix[j] - prefix[i]`.

## Exercises

1. **Range sum query**: Build a prefix sum array from `[3, 1, 4, 1, 5, 9, 2, 6]`. Write `range_sum(prefix: &[i32], i: usize, j: usize) -> i32` that computes the sum of elements from index i to j in O(1).
2. **Running statistics**: Write `running_mean(v: &[f64]) -> Vec<f64>` where `result[i]` is the mean of `v[0..=i]`. Use `scan` with a `(sum, count)` state tuple.
3. **Sliding window max**: Using a running max, implement a sliding window maximum of size k: for each position i, find the maximum in `v[i-k+1..=i]`. This requires a deque, not just scan — research the monotone deque approach.
