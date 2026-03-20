📖 **[View on hightechmind.io →](https://hightechmind.io/rust/104-scan-left)**

---

# 104-scan-left — Scan Left (Running Accumulation)

## Problem Statement

`fold` reduces a sequence to one value. `scan` (also called `scanl` in Haskell or `scan_left` in OCaml) returns all intermediate accumulator values — a prefix sum, running maximum, or cumulative product. This is the essential operation for financial running totals, cumulative distribution functions, and online statistics algorithms.

Rust's `Iterator::scan` implements this lazily, making it composable with the full iterator adapter chain.

## Learning Outcomes

- Use `Iterator::scan` to produce running totals and running extremes
- Implement a custom `scan_left` mirroring OCaml's `List.scan_left`
- Understand that `scan` is `fold` that emits intermediate results
- Apply scan to prefix sums, running maximum, and cumulative products
- Chain `scan` with other iterator adapters for complex streaming computations

## Rust Application

`src/lib.rs` implements `running_sum` using `scan` with a mutable state variable, and `running_max` using `scan` with `i32::MIN` as the initial state. The `scan_left<T, U>` generic function mirrors OCaml's signature exactly: it takes an accumulator function, an initial value, and a slice.

`running_sum` produces the sequence `[0, a0, a0+a1, ...]` — the prefix sums including the initial zero. This is the standard algorithm for efficient range-sum queries: `sum(i..=j) = prefix[j+1] - prefix[i]`.

## OCaml Approach

```ocaml
let scan_left f init xs =
  List.fold_left (fun (acc, result) x ->
    let new_acc = f acc x in
    (new_acc, result @ [new_acc])
  ) (init, [init]) xs
  |> snd
```

`Base.List.folding_map` provides a cleaner one-pass implementation. OCaml's lazy `Seq.scan` would be the equivalent of Rust's lazy `Iterator::scan`.

## Key Differences

1. **Laziness**: Rust's `Iterator::scan` is lazy — values are computed only when consumed; OCaml's `List`-based scan is strict.
2. **State mutation**: Rust's `scan` closure receives `&mut state` for mutation; OCaml's fold passes state by value (immutable by default).
3. **Initial value inclusion**: Rust's scan excludes the initial value by default; the `running_sum` function uses `once(0).chain(...)` to include it — matching OCaml's `scan_left` which includes the initial accumulator.
4. **`scan_left` name**: OCaml uses `List.scan_left` (or `fold_left` with accumulation); Rust uses `Iterator::scan`.

## Exercises

1. Implement `running_product(xs: &[i64]) -> Vec<i64>` using `scan`, stopping accumulation at the first zero.
2. Write a `cumulative_average(xs: &[f64]) -> Vec<f64>` using `scan` that tracks both the running sum and count.
3. Use prefix sums to implement `range_sum(prefix: &[i64], i: usize, j: usize) -> i64` with O(1) query time after O(n) preprocessing.
