# 276: Iterator min_by() and max_by()

**Difficulty:** 2  **Level:** Intermediate

Find the minimum or maximum element using a custom comparator closure that returns `Ordering` — essential for floats, complex structs, and multi-key sorts.

## The Problem This Solves

`min()` and `max()` require `Ord` — a total ordering. But `f64` doesn't implement `Ord` because `NaN != NaN`. And sometimes you want to compare structs by multiple fields: "shortest string, then alphabetical as tiebreaker." You can't express that with `min_by_key` alone.

`min_by` and `max_by` give you a full comparator closure: `Fn(&A, &A) -> Ordering`. This unlocks any comparison logic you can express: float comparisons with NaN handling, multi-field sorting, distance calculations, reversed ordering (finding the max *via* `min_by` with a reversed comparator).

The `Ordering` type (`Less`, `Equal`, `Greater`) combined with `.then_with()` enables clean multi-key comparisons without custom `Ord` implementations on your types.

## The Intuition

Like `min()` and `max()`, but instead of requiring `Ord`, you provide the comparison function yourself — full control over what "less than" means.

## How It Works in Rust

```rust
use std::cmp::Ordering;

// f64 min/max — handle NaN by falling back to Equal
let floats = [3.14f64, 1.41, 2.71, 0.57];
let min_f = floats.iter().copied()
    .min_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));
// → Some(0.57)

// Multi-key: shortest first, then alphabetical as tiebreaker
let words = ["banana", "apple", "fig", "kiwi"];
let min_word = words.iter().min_by(|a, b| {
    a.len().cmp(&b.len())         // primary: length
     .then_with(|| a.cmp(b))      // tiebreaker: alphabetical
});
// → Some("fig")

// Closest point to origin — computed key, not a simple field
struct Point { x: f64, y: f64 }
let closest = points.iter().min_by(|a, b| {
    let da = (a.x*a.x + a.y*a.y).sqrt();
    let db = (b.x*b.x + b.y*b.y).sqrt();
    da.partial_cmp(&db).unwrap_or(Ordering::Equal)
});

// Trick: find max using min_by with reversed comparator
let max_via_min = nums.iter().min_by(|a, b| b.cmp(a));
```

## What This Unlocks

- **Float extremes:** `f64` comparisons that handle NaN gracefully instead of panicking or producing wrong results.
- **Multi-field ordering:** Compose comparisons with `.then_with()` — primary key, secondary key, tertiary key — without implementing `Ord` on your struct.
- **Geometric queries:** Find the point closest to a reference, the element with maximum weighted score, or any extreme defined by a formula.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Custom min | `List.fold_left` with custom compare | `.min_by(\|a, b\| ...)` |
| Comparator signature | `'a -> 'a -> int` (-1/0/1) | `Fn(&A, &A) -> Ordering` |
| Float comparison | `Float.compare` (works) | `partial_cmp().unwrap_or(Equal)` |
| Multi-key | Manual chaining with `if` | `.then_with(\|\| ...)` on Ordering |
| Reverse order | Swap args in comparator | Swap args in comparator (`b.cmp(a)`) |
