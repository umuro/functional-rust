📖 **[View on hightechmind.io →](https://hightechmind.io/rust/798-kadane-max-subarray)**

---

# 798-kadane-max-subarray — Kadane's Algorithm
**Difficulty:** ⭐  
**Category:** Functional Programming  


## Problem Statement

Maximum subarray sum (Kadane's algorithm, 1984) finds the contiguous subarray with the largest sum in O(n) time. This is an astonishing result: intuitively it seems you must check all pairs, but a single pass suffices. It models maximum profit from a sequence of daily stock gains/losses, maximum signal amplitude in signal processing, and appears in image processing (maximum sum submatrix). It is one of the most elegant DP algorithms.

## Learning Outcomes

- Understand Kadane's insight: at each position, either extend the current subarray or start a new one
- Implement `max_subarray(arr) -> i32` with O(n) time and O(1) space
- Track start and end indices with `max_subarray_indices` for reconstruction
- Handle all-negative arrays (maximum subarray is the least-negative element)
- Extend to the 2D case (maximum sum submatrix) using Kadane's on column prefix sums

## Rust Application

`max_subarray` tracks `max_so_far` and `max_ending`. At each element: `max_ending = max(x, max_ending + x)` — start fresh if previous was dragging sum negative. `max_subarray_indices` additionally tracks `start`, `end`, and `s` (current start candidate). Classic test: `[-2,1,-3,4,-1,2,1,-5,4]` → sum=6 (subarray `[4,-1,2,1]`).

## OCaml Approach

OCaml implements Kadane's with a `fold_left` over the array, carrying `(max_so_far, max_ending)` as the accumulator: `Array.fold_left (fun (best, cur) x -> let cur' = max x (cur + x) in (max best cur', cur')) (arr.(0), arr.(0)) arr`. This functional one-liner is idiomatic OCaml. The index-tracking variant requires imperative state with `ref` cells.

## Key Differences

1. **Functional elegance**: OCaml's `fold_left` expresses Kadane's as a single combinator; Rust's explicit loop is more readable for the index-tracking variant.
2. **All-negative case**: Both languages return the maximum single element for all-negative arrays; the algorithm handles this naturally.
3. **2D extension**: Both languages implement the 2D variant by fixing left and right column boundaries and running Kadane's on the row-sum array.
4. **Applications**: The `max_subarray` pattern appears in financial (buy-low-sell-high when returns are daily differences), signal processing, and machine learning (max activation windows).

## Exercises

1. Implement `max_submatrix(matrix: &[Vec<i32>]) -> i32` using the column-compression + Kadane's technique to find the maximum sum submatrix in O(n² × m) time.
2. Implement `max_circular_subarray(arr: &[i32]) -> i32` for circular arrays using the observation that max(normal, total - min_subarray) handles the wrap-around case.
3. Implement `k_max_nonoverlapping_subarrays(arr, k)` that finds the top k non-overlapping maximum sum subarrays using a DP extension of Kadane's.
