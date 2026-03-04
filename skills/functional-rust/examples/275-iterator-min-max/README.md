# 275: Iterator min() and max()

**Difficulty:** 1  **Level:** Beginner

Consume an iterator to find its minimum or maximum element, returning `Option<T>` to handle the empty case.

## The Problem This Solves

You want the smallest or largest value in a collection. Without `min()` and `max()`, you'd write a fold with a running minimum, initialize it to `i32::MAX`, and hope you remembered to handle the empty case. Or you'd sort and take the first element — O(n log n) when O(n) is all you need.

The more interesting problem is finding the element with the *minimum of a derived property* — the shortest string, the student with the highest score, the point closest to the origin. `.min_by_key(|x| x.score)` handles this without implementing `Ord` on your struct or creating intermediate (value, key) tuples.

One important constraint: `min()` and `max()` require `Ord`. That means `f64` doesn't work directly — floats don't have a total ordering because of `NaN`. The example shows the `reduce(f64::min)` workaround for floats.

## The Intuition

Walk the entire iterator, keep track of the extreme value seen so far, return it wrapped in `Some` — or `None` if the iterator was empty.

## How It Works in Rust

```rust
let nums = [3i32, 1, 4, 1, 5, 9, 2, 6];
println!("{:?}", nums.iter().min());  // Some(1)
println!("{:?}", nums.iter().max());  // Some(9)

// Empty iterator → None (not a crash, not a sentinel value)
let empty: Vec<i32> = vec![];
println!("{:?}", empty.iter().min());  // None

// min/max by a derived key — no need to implement Ord
let words = ["banana", "apple", "fig", "cherry"];
words.iter().min_by_key(|w| w.len());  // Some("fig")
words.iter().max_by_key(|w| w.len());  // Some("banana")

// Works on any type with a comparable field
struct Student { name: &'static str, score: u32 }
let students = [/* ... */];
students.iter().max_by_key(|s| s.score);  // top scorer

// f64 doesn't implement Ord — use reduce instead
let floats = [3.14f64, 1.41, 2.71];
let min_f = floats.iter().cloned().reduce(f64::min);  // Some(1.41)
```

## What This Unlocks

- **Struct extremes without Ord:** `.max_by_key(|u| u.age)` finds the oldest user; `.min_by_key(|p| p.price)` finds the cheapest product — no trait impl required.
- **Pipeline composition:** `iter.filter(pred).min()` finds the minimum among matching elements in one readable expression.
- **Safe extreme extraction:** `Option<T>` return forces you to handle empty inputs — no panics, no magic sentinel values like `i32::MAX`.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Min of list | `List.fold_left min max_int lst` | `iter.min()` |
| Empty list | Manual guard or exception | Returns `None` automatically |
| Float min | Works (but NaN is UB) | Must use `reduce(f64::min)` or `min_by` |
| By key | Manual: `List.sort` + `List.hd` | `.min_by_key(f)` / `.max_by_key(f)` |
| Return type | `'a` (unwrapped) | `Option<T>` |
