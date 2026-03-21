**Difficulty:** ⭐⭐  
**Category:** Functional Programming  

**Difficulty:** ⭐⭐  

[scan-left on hightechmind.io](https://hightechmind.io/posts/functional-rust/scan-left)

---

## Problem Statement

Implement `scan_left`, which produces all intermediate accumulator values while folding over a sequence. Unlike `fold` (which returns only the final value), `scan_left` returns the initial value followed by every partial result, making it possible to observe the fold's evolution step by step. Build running sum and running max as concrete applications, then compare with Rust's built-in `Iterator::scan` adapter.

## Learning Outcomes

- Understand the relationship between `fold` (reduces to one value) and `scan` (keeps every intermediate value)
- Implement `scan_left<T, A, F>(init, items, f) -> Vec<A>` using a generic accumulator
- Apply `scan_left` to compute running sum, running max, and other prefix aggregations
- Use Rust's `Iterator::scan` with mutable state (`&mut state`) and `Some`/`None` control
- Recognize why `scan_left` produces `n + 1` values for an `n`-element input (includes the initial value)

## Rust Application

```rust
pub fn scan_left<T, A, F>(init: A, items: &[T], f: F) -> Vec<A>
where
    A: Clone,
    F: Fn(&A, &T) -> A,
{
    let mut result = vec![init.clone()];
    let mut acc = init;
    for item in items {
        acc = f(&acc, item);
        result.push(acc.clone());
    }
    result
}

pub fn running_sum(nums: &[i64]) -> Vec<i64> {
    scan_left(0i64, nums, |acc, x| acc + x)
}

// Idiomatic: use Iterator::scan with mutable state
pub fn running_sum_idiomatic(nums: &[i64]) -> Vec<i64> {
    let mut result = vec![0i64];
    result.extend(nums.iter().scan(0i64, |state, &x| {
        *state += x;
        Some(*state)
    }));
    result
}
```

`scan_left` returns `n + 1` values: the initial accumulator plus one new value per element. `running_sum(&[1,2,3,4,5])` returns `[0, 1, 3, 6, 10, 15]`.

The built-in `Iterator::scan` takes a mutable state reference. Returning `None` from the closure terminates the scan early — a capability the custom `scan_left` above lacks. This makes `Iterator::scan` suitable for early-termination patterns such as stopping when a balance goes negative.

`running_max` uses `i64::MIN` as the seed so that the first element always replaces it: `scan_left(i64::MIN, nums, |acc, x| *acc.max(x))`.

## OCaml Approach

OCaml's standard library lacks a built-in `scan_left` but it is trivially defined:

```ocaml
let scan_left f init xs =
  let acc = ref init in
  init :: List.map (fun x ->
    acc := f !acc x;
    !acc
  ) xs

(* purely functional version *)
let scan_left_pure f init xs =
  List.fold_left
    (fun (acc, sofar) x ->
      let acc' = f acc x in
      (acc', sofar @ [acc']))
    (init, [init]) xs
  |> snd
```

OCaml's `Seq` module supports lazy scan over infinite sequences without materializing a list:

```ocaml
let seq_scan f init s =
  let open Seq in
  let rec go acc s () = match s () with
    | Nil -> Nil
    | Cons (x, rest) ->
        let acc' = f acc x in
        Cons (acc', go acc' rest)
  in
  Cons (init, go init s)
```

## Key Differences

| Aspect | Rust | OCaml |
|--------|------|-------|
| Built-in scan | `Iterator::scan` (mutable state, `Option` control) | None in stdlib; `Seq`-based for laziness |
| Result size | `n + 1` values (includes seed) | Typically same |
| Early termination | Return `None` from closure | Pattern match on `Seq` |
| Laziness | Eager with `Vec`; lazy with `Iterator::scan` chained | `Seq` is lazy by default |
| State mutation | `&mut state` in closure | `ref`/`!` in imperative version; accumulator in pure version |

Rust's `Iterator::scan` is the idiomatic choice for streaming prefix aggregation. The key insight is that `scan` is to `fold` what `map` is to `for_each`: it produces values along the way rather than consuming them silently.

## Exercises

1. Implement `running_product` using `scan_left` and verify it matches `Iterator::scan`.
2. Build a `running_balance` that tracks a bank account balance through a list of transactions (positive = deposit, negative = withdrawal).
3. Use `Iterator::scan` with `None` to implement `scan_until_negative`: stop scanning as soon as the running sum goes below zero.
4. Implement `max_prefix_sum`: use `scan_left` to find the maximum running sum over the entire array — useful in the Kadane's algorithm family.
5. Write a lazy `scan_seq` that works over Rust iterators without collecting into a `Vec`, using a custom struct that implements `Iterator`.
