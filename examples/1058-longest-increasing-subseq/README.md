📖 **[View on hightechmind.io →](https://hightechmind.io/rust/1058-longest-increasing-subseq)**

---

# 1058-longest-increasing-subseq — Longest Increasing Subsequence

## Problem Statement

The longest increasing subsequence (LIS) finds the longest subsequence of a sequence where elements are strictly increasing. It appears in scheduling (job sequencing by deadline), genomics (conserved gene segments), and card games (patience sorting, which inspired the O(n log n) algorithm).

The naive O(n²) DP can be improved to O(n log n) using patience sorting — the key insight being that you maintain a list of "piles" whose tops are sorted, enabling binary search.

## Learning Outcomes

- Implement the O(n²) LIS DP
- Implement the O(n log n) patience-sorting algorithm using binary search
- Use Rust's `binary_search` for the patience sort step
- Understand why patience sort works via the pigeonhole principle
- Connect LIS to the O(n log n) bounds on comparison-based sorting

## Rust Application

`src/lib.rs` implements `lis_dp` with the O(n²) approach: `dp[i]` is the LIS length ending at index `i`, computed by checking all previous elements. `lis_patience` uses a `tails` vector where `tails[k]` is the smallest tail element among all increasing subsequences of length `k+1`. Binary search finds the position to update on each element. The `tails` vector is always sorted, making binary search valid.

The patience sort connection: imagine sorting a hand of cards into piles following specific rules — the number of piles equals the LIS length (Dilworth's theorem).

## OCaml Approach

```ocaml
let lis_patience arr =
  let tails = Array.make (Array.length arr) 0 in
  let len = ref 0 in
  Array.iter (fun x ->
    (* Binary search for insertion position *)
    let lo = ref 0 and hi = ref !len in
    while !lo < !hi do
      let mid = (!lo + !hi) / 2 in
      if tails.(mid) < x then lo := mid + 1 else hi := mid
    done;
    tails.(!lo) <- x;
    if !lo = !len then incr len
  ) arr;
  !len
```

The algorithm is identical. The binary search for the lower bound is the same in both languages.

## Key Differences

1. **`binary_search` API**: Rust's `binary_search` returns `Ok(pos)` for exact matches and `Err(pos)` for insertion points; the patience algorithm needs `Err(pos)` which maps to a lower-bound bisection.
2. **In-place vs Vec growth**: Rust's `tails` grows dynamically with `push`; OCaml pre-allocates an array of the input length.
3. **Reconstruction**: Both require a separate `dp` array to reconstruct the actual LIS (not just its length); the `tails` array does not directly encode the subsequence.
4. **Stability**: The patience sort variant is stable and used in Python's `timsort` merge decision algorithm.

## Exercises

1. Modify `lis_patience` to also reconstruct the actual LIS elements, not just the length.
2. Implement the longest non-decreasing subsequence (≤ instead of <) by changing the binary search comparison.
3. Write `lis_count(arr: &[i32]) -> usize` that counts the number of distinct LIS sequences of maximum length.
