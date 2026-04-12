[![hightechmind.io](https://img.shields.io/badge/hightechmind.io-functional--rust-blue)](https://hightechmind.io)

# 092 — Scan Accumulate
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Implement `running_sum` and `running_max` — prefix accumulations that produce a new sequence where each element is the accumulated result up to that position. Use Rust's `Iterator::scan` for `running_sum` and an explicit loop for `running_max`. Compare with OCaml's `scan_left` built on `List.fold_left`.

## Learning Outcomes

- Use `Iterator::scan(init, |state, x| { … ; Some(result) })` for stateful prefix computations
- Understand that `scan` is like `fold` but emits intermediate accumulator values
- Distinguish `scan` (emits each step) from `fold` (emits only the final result)
- Implement `running_max` with an explicit loop when `scan` becomes cumbersome
- Map Rust's `scan` to OCaml's `scan_left` built from `List.fold_left`
- Recognise prefix scan as a fundamental data-parallel primitive

## Rust Application

`running_sum` prepends a `0` sentinel, then extends with `v.iter().scan(0, |acc, &x| { *acc += x; Some(*acc) })`. The mutable `state` in `scan` is `*acc`, which is a mutable reference to the accumulator. Returning `Some(*acc)` emits the new running total; returning `None` would terminate the scan early. `running_max` uses an explicit `mut max_val` loop because `scan` with `max` requires careful state initialisation. Both produce a vector one element longer than the input (including the initial `0`/`v[0]` sentinel).

## OCaml Approach

OCaml's `scan_left f init lst` is implemented with `fold_left`: at each step, compute `next = f acc x`, accumulate `next :: res`, and reverse at the end. `running_sum = scan_left (+) 0` and `running_max = scan_left max x xs` with the first element as the initial value. The pattern is elegant but requires the `List.rev` at the end to restore order.

## Key Differences

| Aspect | Rust | OCaml |
|--------|------|-------|
| Scan primitive | `Iterator::scan(init, f)` | Custom `scan_left` via `fold_left` |
| State mutation | `*state += x` (mutable ref) | Immutable `next = f acc x` |
| Accumulator | Modified in place | New value each step |
| Running max | Explicit loop | `scan_left max x xs` |
| Order | No reversal needed | `List.rev result` needed |
| Standard library | `scan` built in | `scan_left` not in stdlib |

The scan operation is the prefix-sum primitive of functional programming. It generalises `fold` by keeping intermediate results — essential for computing things like running averages, cumulative distributions, and prefix XORs in competitive programming.

## Exercises

1. Implement `running_product(v: &[i64]) -> Vec<i64>` using `scan` with initial value `1`.
2. Write `running_avg(v: &[f64]) -> Vec<f64>` that computes the running arithmetic mean.
3. Implement `scan_right` (right-to-left scan) by reversing the input, scanning, then reversing the output.
4. Use `scan` to implement `enumerate`: produce `(0, v[0]), (1, v[1]), …` without calling `.enumerate()`.
5. In OCaml, implement `scan_left` using `Seq` to make it lazy — deferring computation until elements are demanded.
