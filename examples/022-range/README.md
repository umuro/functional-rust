📖 **[View on hightechmind.io →](https://hightechmind.io/rust/022-range)**

---

# 022 — Create a List Containing All Integers in a Range

## Problem Statement

Generating a range of integers (OCaml 99 Problems #22) — `range(4, 9)` → `[4, 5, 6, 7, 8, 9]` — is one of the simplest list-generating operations and a gateway to understanding lazy vs eager sequence generation. In Python it is `range(4, 9)`, in Haskell `[4..9]`, in OCaml (with batteries) `4 -- 9`. The key insight is that a range can be represented lazily (an iterator) or eagerly (a list/vector).

This distinction matters enormously in practice: Rust's `(4..=9)` is a lazy `Range<i32>` that allocates nothing until consumed. Generating `(0..1_000_000)` as a range object is O(1); collecting it into a `Vec` is O(n). Understanding this is essential for writing efficient pipelines.

## Learning Outcomes

- Use Rust's built-in range syntax `(a..=b)` for lazy range generation
- Collect a range to `Vec<i32>` with `.collect()`
- Implement an explicit recursive range generator to understand the structure
- Handle the case `a > b` (empty range or descending range)
- Understand lazy (iterator) vs eager (Vec) range representations

## Rust Application

The idiomatic Rust approach is `(a..=b).collect::<Vec<i32>>()` — this uses the built-in `Range` type which is a lazy iterator. For descending ranges: `(b..=a).rev().collect()`. The recursive version builds `[a, a+1, ..., b]` by prepending `a` and recurring on `a+1`. An `unfold`-based version: `std::iter::successors(Some(a), |&x| if x < b { Some(x + 1) } else { None }).collect()`.

## OCaml Approach

OCaml's version: `let range a b = let rec aux acc n = if n < a then acc else aux (n :: acc) (n - 1) in aux [] b`. The recursion counts down from `b` to `a`, building the result in forward order by prepending. `List.init (b - a + 1) (fun i -> a + i)` is a more concise version. With `Seq`: `Seq.unfold (fun i -> if i > b then None else Some (i, i + 1)) a |> List.of_seq`.

## Key Differences

1. **Built-in range**: Rust's `(a..=b)` syntax generates a lazy `RangeInclusive<i32>` at zero cost. OCaml has no built-in range syntax; you construct it manually or use Batteries/Core.
2. **Lazy vs eager**: Rust's range is lazy — `(0..1_000_000).take(5)` never computes element 5. OCaml's `range a b` from the 99 Problems is eager — it builds the full list.
3. **Descending**: Rust has no lazy descending range; use `(a..=b).rev()`. OCaml's recursive version naturally builds descending if you count up.
4. **Step**: Rust's `(0..20).step_by(3)` produces 0, 3, 6, ... OCaml requires manual arithmetic: `List.init n (fun i -> a + i * step)`.

## Exercises

1. **Floating-point range**: Write `frange(a: f64, b: f64, step: f64) -> Vec<f64>` that generates a floating-point range. Handle floating-point accumulation errors by computing `a + i * step` rather than adding `step` repeatedly.
2. **Sparse range**: Write `range_except(a: i32, b: i32, exclude: &[i32]) -> Vec<i32>` that generates `[a..=b]` minus the excluded values. Use `filter`.
3. **Infinite counter**: Implement `counter(start: i32, step: i32) -> impl Iterator<Item=i32>` as an infinite iterator that can be `take(n)` limited. Compare with `std::iter::successors`.
