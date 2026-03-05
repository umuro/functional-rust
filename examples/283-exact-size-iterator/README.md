📖 **[View on hightechmind.io →](https://hightechmind.io/rust/283-exact-size-iterator)**

---

# 283: ExactSizeIterator

**Difficulty:** 3  **Level:** Advanced

Provide O(1) element count on your custom iterator to enable pre-allocation, size assertions, and optimizer hints.

## The Problem This Solves

When you `collect()` an iterator into a `Vec`, Rust checks `size_hint()` to pre-allocate the right amount of memory. If the hint is wrong or absent, `Vec` starts small and reallocates multiple times as it grows — potentially 2-3x the necessary work for large collections.

`ExactSizeIterator` is the trait that says "I know my remaining count precisely." Implementing it lets consumers call `.len()` on your iterator — just like calling `.len()` on a `Vec` — and allocate exactly the right capacity upfront. The standard library uses this internally: `Vec::from_iter` for ranges is O(1) allocation because ranges implement `ExactSizeIterator`.

You implement it by providing accurate `size_hint()` bounds (both lower and upper matching exactly) and optionally overriding `len()`. Once you implement the trait, callers can use `Vec::with_capacity(iter.len())` + `extend()` for single-allocation collection.

## The Intuition

Declare that your iterator knows exactly how many elements remain, unlocking O(1) length queries and pre-allocation optimizations.

## How It Works in Rust

```rust
struct FixedRange { current: usize, end: usize }

impl Iterator for FixedRange {
    type Item = usize;
    fn next(&mut self) -> Option<usize> { /* ... */ }

    // Required: size_hint must be exact (lo == hi)
    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.end.saturating_sub(self.current);
        (remaining, Some(remaining))  // both bounds equal = exact
    }
}

// Declare: this iterator knows its exact remaining count
impl ExactSizeIterator for FixedRange {
    fn len(&self) -> usize {
        self.end.saturating_sub(self.current)
    }
}

// Now callers can:
let fr = FixedRange::new(3, 8);
println!("{}", fr.len());  // 5 — O(1), no traversal

// Pre-allocate exactly right
let source = vec![1i32, 2, 3, 4, 5];
let mut dest = Vec::with_capacity(source.iter().len());  // single allocation
dest.extend(source.iter().map(|&x| x * 2));
// dest == [2, 4, 6, 8, 10], allocated once

// Built-in ExactSizeIterators
let arr = [1i32, 2, 3, 4, 5];
let mut it = arr.iter();
it.len();     // 5
it.next();
it.len();     // 4 — tracks remaining count

(0i32..10).len();  // 10 — ranges know their size
```

## What This Unlocks

- **Single-allocation collect:** Pre-allocate Vec with `with_capacity(iter.len())` + `extend()` — no reallocation, especially valuable in hot paths.
- **API contracts:** Functions that require exactly N elements can `assert_eq!(iter.len(), n)` instead of collecting and checking `len()` after.
- **Optimizer hints:** The standard library uses `ExactSizeIterator` bounds to eliminate per-element capacity checks inside `collect()`.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Length of sequence | `Array.length` (arrays only) | `ExactSizeIterator::len()` on any iterator |
| Lists | `List.length` — O(n) traversal | No direct equivalent; iterators can be O(1) |
| Pre-allocation | Manual size tracking | `Vec::with_capacity(iter.len())` |
| Safety | Arrays are fixed-size | `len()` must match actual remaining elements |
| Standard iterators | N/A | Slices, ranges, `zip` of ESI all implement it |
