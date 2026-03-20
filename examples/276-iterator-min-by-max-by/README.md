📖 **[View on hightechmind.io →](https://hightechmind.io/rust/276-iterator-min-by-max-by)**

---

# 276: Custom Comparison with min_by() and max_by()
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  


## Problem Statement

Many types either cannot implement `Ord` (floating-point numbers have NaN) or require a non-standard ordering (reverse order, case-insensitive comparison, comparison by a computed property). The `min_by()` and `max_by()` methods accept a custom comparator function `Fn(&A, &A) -> Ordering`, enabling arbitrary orderings without modifying the type's own comparison. This is essential for types like `f64` and for domain-specific orderings.

## Learning Outcomes

- Understand `min_by(cmp)` and `max_by(cmp)` as taking explicit `Fn(&A, &A) -> Ordering` comparators
- Use `partial_cmp` for floating-point types that implement `PartialOrd` but not `Ord`
- Use `min_by` with a reversed comparator to implement max-as-min (useful for min-heaps)
- Distinguish `min_by` (comparator function) from `min_by_key` (key extraction function)

## Rust Application

`min_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal))` handles floats safely. The comparator receives references to elements:

```rust
use std::cmp::Ordering;

let floats = [3.0f64, 1.0, 2.0];
let min = floats.iter().copied()
    .min_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));
// Some(1.0)

// Reverse comparator finds max via min_by
let nums = [1i32, 5, 3, 2, 4];
let max_via_min = nums.iter().min_by(|a, b| b.cmp(a));
// Some(&5) — reversed comparison finds the largest
```

## OCaml Approach

OCaml's `List.fold_left` with a custom comparison function serves the same purpose. The `compare` parameter accepts any `'a -> 'a -> int`-typed function:

```ocaml
let min_by cmp lst =
  List.fold_left (fun acc x -> if cmp x acc < 0 then x else acc) (List.hd lst) lst

let min_float = min_by Float.compare [3.0; 1.0; 2.0]  (* 1.0 *)
```

OCaml's `compare` function is polymorphic by default, handling floats (including NaN) with structural comparison.

## Key Differences

1. **Float handling**: Rust explicitly lacks `Ord` for `f64` — `min_by` with `partial_cmp` is the required pattern; OCaml's polymorphic `compare` handles floats but NaN behavior differs.
2. **Comparator type**: Rust receives `&A` references in the comparator; OCaml's functions typically take values by value.
3. **vs min_by_key**: `min_by_key(f)` is `min_by(|a,b| f(a).cmp(&f(b)))` — prefer `min_by_key` when the key is cheap to compute.
4. **Stability**: On ties, `min_by` returns the first encountered minimum; this is consistent with Rust's general iterator stability.

## Exercises

1. Find the string with the minimum length using `min_by(|a, b| a.len().cmp(&b.len()))` and verify it gives the same result as `min_by_key(|s| s.len())`.
2. Find the floating-point value closest to zero in a slice using `min_by` with `|a, b| a.abs().partial_cmp(&b.abs())`.
3. Implement a `min_by_key_or_first` function that returns the first element if the slice is empty or returns the minimum-by-key otherwise.
