📖 **[View on hightechmind.io →](https://hightechmind.io/rust/1059-rod-cutting)**

---

# 1059-rod-cutting — Rod Cutting

## Problem Statement

Rod cutting asks: given a rod of length n and a price table for each possible length, how should you cut the rod to maximize revenue? Cutting a rod of length 7 into pieces of length 3 and 4 might be worth more than selling it whole. This is an unbounded knapsack variant where the rod lengths are the "item sizes" and prices are the "values."

The problem appears in metal fabrication, lumber pricing, fiber optic cable installation, and any scenario where raw material can be sold at different rates by size.

## Learning Outcomes

- Implement rod cutting using bottom-up DP
- Understand the unbounded knapsack structure (each piece can be cut repeatedly)
- Implement the memoized top-down variant
- Reconstruct the optimal cut sequence
- Recognize the difference from 0/1 knapsack (unbounded reuse)

## Rust Application

`src/lib.rs` implements `rod_cut_dp` with `dp[i]` = maximum revenue from a rod of length `i`. The inner loop tries all cut lengths from 1 to min(i, prices.len()). `rod_cut_memo` uses a `HashMap` for top-down memoization. `rod_cut_with_cuts` tracks the optimal first cut at each length, enabling reconstruction of the full cut sequence.

Rod cutting is the canonical example of unbounded knapsack in most algorithms textbooks (CLRS uses it as a running example in the DP chapter).

## OCaml Approach

```ocaml
let rod_cut prices n =
  let dp = Array.make (n + 1) 0 in
  for i = 1 to n do
    for j = 1 to min i (Array.length prices) do
      dp.(i) <- max dp.(i) (prices.(j-1) + dp.(i - j))
    done
  done;
  dp.(n)
```

Identical structure to Rust. Both iterate over rod lengths in the outer loop and cut sizes in the inner loop, taking the maximum revenue at each step.

## Key Differences

1. **`prices.len()` vs `Array.length prices`**: Rust's `prices.len()` and OCaml's `Array.length prices` are equivalent but syntactically different.
2. **1-indexed prices**: Both use `prices[j-1]` for a 0-indexed array accessed with 1-based cut length; the off-by-one is a direct consequence of the domain (length 1 cut costs `prices[0]`).
3. **Reconstruction**: Both use a `cuts` array recording the optimal first cut, then loop to peel off the cuts one by one.
4. **CLRS connection**: This is CLRS Chapter 15's first DP example — the algorithms textbook formulation matches both implementations exactly.

## Exercises

1. Add a `fixed_cost_per_cut: i64` parameter representing the cost of each cut operation. Modify the DP to subtract this cost each time a cut is made.
2. Implement `rod_cut_bounded(prices: &[i64], max_pieces: usize) -> i64` where the rod can be cut into at most `max_pieces` pieces.
3. Write a test that verifies the rod cutting solution against the brute-force solution for all rods of length 1–10.
