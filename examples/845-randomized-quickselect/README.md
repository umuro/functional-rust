📖 **[View on hightechmind.io →](https://hightechmind.io/rust/845-randomized-quickselect)**

---

# 845: Randomized Quickselect

**Difficulty:** 3  **Level:** Intermediate

Find the k-th smallest element in O(n) average time without sorting — the linear-time order statistic algorithm that powers percentile computation, median finding, and partial sorts.

## The Problem This Solves

Finding the minimum or maximum is O(n). Finding the k-th smallest (a general order statistic) by sorting is O(n log n). Quickselect cuts this to O(n) average by partitioning around a pivot — like quicksort, but only recursing into the partition that contains the k-th element, discarding the other half.

Real-world use: finding the median (k = n/2) for statistical algorithms, computing the 99th percentile latency for SLO monitoring, partial sort (return the top-k elements), database ORDER BY LIMIT k without full sort, and the median-of-medians algorithm (which gives O(n) worst case).

Without randomization, quickselect degrades to O(n²) on adversarially sorted input (always picking the smallest or largest element as pivot). Randomized pivot selection — choosing the pivot uniformly at random — makes worst case negligibly unlikely in practice and reduces expected case to O(n).

## The Intuition

Partition the array around a pivot: all elements less than the pivot go left, all greater go right. The pivot lands at some index `p`. If `p == k`, done. If `p > k`, the k-th element is in the left partition — recurse there only. If `p < k`, it's in the right partition — recurse there only. Each step discards at least one element (the pivot). On average, the partition roughly halves the search space: T(n) = T(n/2) + O(n) → O(n).

Lomuto partition scheme: swap a random pivot to the end, sweep left-to-right moving elements smaller than pivot to the left partition, swap pivot to its final position. Rust's `&mut [T]` slice allows in-place swaps without pointer arithmetic.

## How It Works in Rust

```rust
// XorShift PRNG: fast, no external crates, good enough for randomized pivot
struct Rng(u64);
impl Rng {
    fn new(seed: u64) -> Self { Rng(seed) }
    fn next_usize(&mut self, n: usize) -> usize {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        (self.0 as usize) % n
    }
}

// Lomuto partition: rearrange arr[lo..=hi] around a random pivot
// Returns the pivot's final position
fn partition<T: Ord>(arr: &mut [T], lo: usize, hi: usize, rng: &mut Rng) -> usize {
    let pivot_idx = lo + rng.next_usize(hi - lo + 1);  // Random pivot
    arr.swap(pivot_idx, hi);   // Move pivot to end
    let mut store = lo;
    for i in lo..hi {
        if arr[i] < arr[hi] {  // arr[hi] is the pivot
            arr.swap(store, i);
            store += 1;
        }
    }
    arr.swap(store, hi);       // Place pivot at its final position
    store
}

// Quickselect: find the k-th smallest (0-indexed) in O(n) average
pub fn quickselect<T: Ord>(arr: &mut [T], mut lo: usize, mut hi: usize, k: usize) -> &T {
    let mut rng = Rng::new(0xdeadbeef);
    loop {
        if lo == hi { return &arr[lo]; }
        let p = partition(arr, lo, hi, &mut rng);
        match p.cmp(&k) {
            std::cmp::Ordering::Equal   => return &arr[p],   // Found it
            std::cmp::Ordering::Greater => hi = p - 1,       // k is in left partition
            std::cmp::Ordering::Less    => lo = p + 1,       // k is in right partition
        }
    }
}

// 1-indexed wrapper that doesn't modify the original
pub fn kth_smallest<T: Ord + Clone>(arr: &[T], k: usize) -> T {
    assert!(k >= 1 && k <= arr.len());
    let mut copy = arr.to_vec();
    let n = copy.len();
    quickselect(&mut copy, 0, n - 1, k - 1).clone()
}
```

Rust's `&mut [T]` slice is the natural representation for in-place algorithms: `arr.swap(i, j)` is safe (bounds-checked in debug, elided in release) and communicates intent clearly.

## What This Unlocks

- **Median-of-medians**: Replace the random pivot with the median of 5-element groups to get O(n) *worst-case* quickselect — used when adversarial inputs are possible (e.g., user-controlled data).
- **Percentile metrics in observability**: Computing p50/p95/p99 latency from a large sample uses k-th order statistics; quickselect avoids sorting the entire sample.
- **Partial sort and top-k**: After quickselect places the k-th element, all elements ≤ k are in the left partition — sort that partition for top-k in O(n + k log k).

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Random pivot | `Random.int (hi - lo + 1) + lo` | XorShift PRNG — no external crates |
| In-place partition | `Array.blit` or `Array.swap` | `arr.swap(i, j)` — bounds-checked |
| Recursive vs iterative | Natural tail-recursive style | Iterative `loop` — no stack frames |
| Return by reference | Not idiomatic | `&arr[p]` — zero-copy, borrow-checked |
| Copy for non-destructive | `Array.copy arr` | `.to_vec()` then `quickselect` on copy |
