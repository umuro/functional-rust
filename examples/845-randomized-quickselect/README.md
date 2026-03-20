📖 **[View on hightechmind.io →](https://hightechmind.io/rust/845-randomized-quickselect)**

---

# Randomized Quickselect

## Problem Statement

Finding the kth smallest element in an unsorted array naively requires sorting: O(n log n). Quickselect finds it in O(n) expected time by using the partition step from quicksort but only recursing into one side. Choosing the pivot randomly avoids worst-case O(n^2) behavior from adversarial inputs. Practical uses: computing medians for statistics, percentile calculations in monitoring systems, streaming data analysis (p99 latency), and machine learning (median-based normalization). The deterministic variant (median-of-medians) guarantees O(n) worst-case but with higher constants.

## Learning Outcomes

- Implement Lomuto or Hoare partition around a pivot, placing smaller elements left
- Choose pivot randomly to achieve O(n) expected time (not amortized — each call is O(n) expected)
- Recurse only into the side containing the target rank, halving the problem each expected step
- Understand why expected O(n): geometric series 1 + 1/2 + 1/4 + ... = 2 (expected recurrence)
- Distinguish from sorting: quickselect doesn't sort the discarded side — it literally ignores it

## Rust Application

```rust
pub fn quickselect(arr: &mut [i32], k: usize) -> i32 {
    if arr.len() == 1 { return arr[0]; }
    let pivot_idx = rand_pivot(arr.len()); // random in [0, len)
    let pivot_pos = partition(arr, pivot_idx);
    match pivot_pos.cmp(&k) {
        std::cmp::Ordering::Equal => arr[pivot_pos],
        std::cmp::Ordering::Greater => quickselect(&mut arr[..pivot_pos], k),
        std::cmp::Ordering::Less => quickselect(&mut arr[pivot_pos+1..], k - pivot_pos - 1),
    }
}
```

The `match` on `pivot_pos.cmp(&k)` cleanly handles three cases: pivot is the answer, recurse left, or recurse right with adjusted rank. Rust's mutable slice references allow in-place partitioning without allocation. The pivot index adjustment in the `Less` case (`k - pivot_pos - 1`) converts the global rank to a local rank within the right subarray. Rust's `rand` crate provides the random pivot selection. The function modifies the array as a side effect — the elements are partitioned but not fully sorted.

## OCaml Approach

OCaml's quickselect uses `Array.swap` for the Lomuto partition. `Random.int n` provides the random pivot. The recursive call on a sub-array uses `Array.sub` (copying) or passes array bounds explicitly. OCaml's `compare` on the pivot position drives the three-case match. The functional style returns the kth element without modifying the array by working on a copy; the in-place version uses `Array.blit` for subarray passing.

## Key Differences

| Aspect | Rust | OCaml |
|---|---|---|
| In-place | `&mut [i32]` slice partitioned | `array` mutation or copy |
| Random pivot | `rand::thread_rng()` | `Random.int n` |
| Three-way match | `match pivot_pos.cmp(&k)` | `compare pivot_pos k \|> match` |
| Rank adjustment | `k - pivot_pos - 1` | Same arithmetic |
| Worst case | O(n^2) with bad pivots | Same |
| Deterministic | Median-of-medians variant | Same |

## Exercises

1. Implement median-of-medians pivot selection for guaranteed O(n) worst-case and compare speed with random pivot.
2. Find the top-k smallest elements (not just the kth) using quickselect plus partial sort on the left side.
3. Implement a streaming approximate median using the reservoir sampling or binning technique.
4. Measure the distribution of quickselect running time for n=10^6 across 1000 trials and compare with O(n) theory.
5. Implement the three-way partition (Dutch National Flag) to handle duplicate elements efficiently.
