# 840: Divide and Conquer — Generic Recursive Framework

**Difficulty:** 3  **Level:** Intermediate

Split, recurse, combine: the algorithmic pattern behind merge sort, binary search, FFT, and closest-pair — parameterize it with closures to make it reusable.

## The Problem This Solves

Divide and conquer is the algorithmic pattern that turns O(n²) or O(n³) brute-force problems into O(n log n) or O(n log² n) solutions. Merge sort, FFT, Strassen matrix multiplication, closest pair of points, and many geometric algorithms all decompose into: split the problem, solve each half recursively, combine. Recognizing this pattern is the difference between a working solution and an optimal one.

The generic D&C framework — parameterized by `is_base_case`, `split`, `solve_base`, `recurse`, and `combine` — makes the pattern explicit and reusable. In practice you implement each algorithm concretely (merge sort, binary search), but understanding the abstract structure helps you apply D&C to new problems and reason about complexity with the master theorem.

The master theorem gives you the recurrence T(n) = a×T(n/b) + f(n): merge sort (a=2, b=2, f=O(n)) → O(n log n). Binary search (a=1, b=2, f=O(1)) → O(log n). This is the formula you apply when designing a new D&C algorithm.

## The Intuition

Every D&C algorithm has the same skeleton: split the input at a midpoint (or pivot), recurse on each piece, then merge the results. The interesting work happens in the split or the merge. For merge sort, the split is trivial (cut in half), the merge is the O(n) work. For binary search, the merge is trivial (just the result from one half), the split is O(1). For FFT, both split and combine are O(n) but the constant factors are carefully chosen to cancel imaginary parts.

In Rust, slices make D&C natural: `&xs[..mid]` and `&xs[mid..]` give the two halves with zero copying, and the borrow checker ensures they don't alias.

## How It Works in Rust

```rust
// Merge sort: O(n log n) — trivial split, O(n) merge
fn merge_sort<T: Ord + Clone>(xs: &[T]) -> Vec<T> {
    if xs.len() <= 1 { return xs.to_vec(); }
    let mid = xs.len() / 2;
    let left  = merge_sort(&xs[..mid]);   // Recurse left half
    let right = merge_sort(&xs[mid..]);   // Recurse right half
    merge(left, right)                    // Combine: O(n)
}

fn merge<T: Ord>(a: Vec<T>, b: Vec<T>) -> Vec<T> {
    let (mut i, mut j) = (0, 0);
    let mut result = Vec::with_capacity(a.len() + b.len());
    while i < a.len() && j < b.len() {
        if a[i] <= b[j] { result.push(a[i].clone()); i += 1; }
        else             { result.push(b[j].clone()); j += 1; }
    }
    result.extend_from_slice(&a[i..]);
    result.extend_from_slice(&b[j..]);
    result
}

// Binary search: O(log n) — O(n/2) split, trivial merge
fn binary_search<T: Ord>(arr: &[T], target: &T) -> Option<usize> {
    let (mut lo, mut hi) = (0usize, arr.len());
    while lo < hi {
        let mid = lo + (hi - lo) / 2;  // Avoids overflow vs (lo + hi) / 2
        match arr[mid].cmp(target) {
            std::cmp::Ordering::Equal   => return Some(mid),
            std::cmp::Ordering::Less    => lo = mid + 1,
            std::cmp::Ordering::Greater => hi = mid,
        }
    }
    None
}

// Master theorem quick reference:
// T(n) = 2T(n/2) + O(n)   → O(n log n)   [merge sort]
// T(n) = 1T(n/2) + O(1)   → O(log n)     [binary search]
// T(n) = 2T(n/2) + O(n²)  → O(n²)        [f dominates: Case 3]
```

`lo + (hi - lo) / 2` is the standard overflow-safe midpoint — `(lo + hi) / 2` overflows for large indices in many languages including Rust (though Rust will panic in debug mode, making the bug visible).

## What This Unlocks

- **FFT and polynomial multiplication**: The FFT is D&C on complex exponentials — splits into even/odd indices, recurses, combines with butterfly operations — giving O(n log n) polynomial multiplication.
- **Closest pair of points**: Split by x-coordinate, find closest pair in each half, then merge by checking only the O(n) strip near the split line — O(n log n) vs O(n²) naïve.
- **External merge sort**: The same merge pattern scales to sorting files larger than RAM — split into sorted runs on disk, merge them with k-way merge.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Slice splitting | `Array.sub arr 0 mid` (copies) | `&arr[..mid]` — zero-copy borrow |
| Generic type | `'a array` with `compare` | `<T: Ord + Clone>` — explicit bounds |
| Higher-order D&C | Functions as arguments | `Fn(...)` trait objects or generics |
| Merge sort result | Returns new list/array | Returns `Vec<T>` — always allocates |
| Overflow-safe mid | `lo + (hi - lo) / 2` | Same — explicit, not compiler-magic |
