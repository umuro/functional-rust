📖 **[View on hightechmind.io →](https://hightechmind.io/rust/275-iterator-min-max)**

---

# 275: Finding Extremes with min() and max()
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Finding the minimum or maximum of a collection is a fundamental algorithm — used in sorting, priority selection, range computation, and statistical analysis. The challenge is handling the empty-collection case safely. Rust's `min()` and `max()` return `Option<T>`, forcing callers to handle the empty case explicitly, while the `min_by_key()` variants enable comparison by a derived property without needing to implement `Ord` on the whole type.

## Learning Outcomes

- Understand that `min()` and `max()` return `Option<T>` — `None` for empty iterators
- Use `min_by_key()` and `max_by_key()` to find extremes by a derived attribute
- Recognize that these methods require `Ord` and compare lexicographically for compound types
- Use `min_by()` and `max_by()` for custom comparison functions (e.g., floating-point)

## Rust Application

`min()` returns `Some(&T)` for the minimum element, `None` for empty. For floating-point (which lacks `Ord`), use `min_by` with `partial_cmp`:

```rust
let v = [5i32, 3, 8, 1, 9, 2];
assert_eq!(v.iter().min(), Some(&1));
assert_eq!(v.iter().max(), Some(&9));

// min_by_key: find shortest word
let words = ["hello", "hi", "world"];
assert_eq!(words.iter().min_by_key(|w| w.len()), Some(&"hi"));

// Float min/max (no Ord, use partial_cmp)
let floats = [3.0f64, 1.0, 2.0];
let min = floats.iter().copied()
    .min_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
```

## OCaml Approach

OCaml uses `List.fold_left` with `min` (a built-in function for comparable types), or `List.sort` then `List.hd`. There is no standard `min`/`max` for lists that returns `Option`:

```ocaml
let minimum = function
  | [] -> None
  | xs -> Some (List.fold_left min (List.hd xs) (List.tl xs))

let min_by_key f lst =
  List.fold_left (fun acc x -> if f x < f acc then x else acc) (List.hd lst) lst
```

## Key Differences

1. **Option safety**: Rust returns `Option<T>` forcing empty-case handling; OCaml's standard `min` on lists panics on empty or requires manual wrapping.
2. **Trait bound**: Rust's `min()`/`max()` require `Ord`; floating-point types use `min_by` with `partial_cmp` due to NaN.
3. **Key vs comparator**: Rust provides both `min_by_key` (cheap key extraction) and `min_by` (custom comparison function) as separate methods.
4. **Stability**: `max()` returns the last of equal elements; `min()` returns the first — important when elements are equal by `Ord` but distinct.

## Exercises

1. Find the person with the highest score in a `Vec<(String, i32)>` using `max_by_key()`.
2. Find both the minimum and maximum in a single pass using `fold()` to compute a `(min, max)` tuple.
3. Implement a `median` function that uses `min()` and `max()` on sub-iterators — or uses sorting and indexing — and handles empty input.
