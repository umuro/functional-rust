# 096: ExactSizeIterator

**Difficulty:** 3  **Level:** Intermediate

Iterators that know their remaining length — enabling O(1) `.len()`, perfect pre-allocation, and progress tracking.

## The Problem This Solves

When building a `Vec` from an iterator, Rust needs to know how much memory to allocate. If the iterator can report its exact size upfront, a single allocation suffices. If it can't, the runtime resizes the buffer repeatedly — wasting allocations.

Progress bars, batch processors, and capacity-constrained buffers all benefit from knowing "how many are left" without consuming the iterator. `ExactSizeIterator` is the trait that exposes this: `.len()` returns the exact remaining count in O(1).

Without it, your only option is `size_hint()` — which gives a fuzzy lower/upper bound, not a guarantee.

## The Intuition

Most concrete iterators you use daily implement `ExactSizeIterator`: slices, ranges, `Vec::iter()`, the output of `.map()`, `.zip()`, `.enumerate()`. 

The catch: some adapters *lose* exact size information. `.filter()` can't know how many elements will pass. `.chain()` loses it if either side doesn't have it. `.take(n)` keeps it (min of n and remaining). Knowing which adapters preserve it helps you design pipelines that stay allocation-friendly.

OCaml doesn't have this concept — `List.length` is O(n), arrays have O(1) length but you can't query it mid-pipeline.

## How It Works in Rust

```rust
// .len() on ExactSizeIterator — O(1), doesn't consume
fn show_progress(data: &[i32]) {
    let iter = data.iter();
    println!("About to process {} items", iter.len()); // works because slice iter is ExactSize
}

// Pre-allocate exactly: Vec::with_capacity from .len()
fn map_preallocated<T, U>(data: &[T], f: impl Fn(&T) -> U) -> Vec<U> {
    let mut result = Vec::with_capacity(data.len()); // no reallocations
    for item in data { result.push(f(item)); }
    result
}

// map() preserves ExactSize — safe to call .len() on mapped iterator
fn demonstrate() {
    let data = vec![1, 2, 3, 4, 5];
    let mapped = data.iter().map(|x| x * 2);
    println!("{}", mapped.len()); // ✓ works

    // filter() does NOT implement ExactSizeIterator — unknown how many pass
    // let filtered = data.iter().filter(|&&x| x > 2);
    // filtered.len(); // ✗ compile error
}

// Custom iterator: implement ExactSizeIterator manually
struct Counter { current: usize, end: usize }

impl Iterator for Counter {
    type Item = usize;
    fn next(&mut self) -> Option<usize> {
        if self.current < self.end { let v = self.current; self.current += 1; Some(v) }
        else { None }
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.end - self.current;
        (remaining, Some(remaining)) // exact lower and upper bound
    }
}
// ExactSizeIterator has no required methods — size_hint must be exact
impl ExactSizeIterator for Counter {}
```

## What This Unlocks

- **Zero-realloc pipelines**: `Vec::with_capacity(iter.len())` allocates once, no growth.
- **Progress tracking**: display `n/total` without a separate counter variable.
- **Split and stride**: split a known-size iterator into equal halves with `.len() / 2`.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| O(1) length | `Array.length` (not mid-pipeline) | `.len()` on `ExactSizeIterator` |
| Pre-allocation | `Array.make n` | `Vec::with_capacity(iter.len())` |
| map preserves | N/A | Yes |
| filter preserves | N/A | No |
| Custom impl | N/A | `impl ExactSizeIterator` (no methods required) |
