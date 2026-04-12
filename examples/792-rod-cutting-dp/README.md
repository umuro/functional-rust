📖 **[View on hightechmind.io →](https://hightechmind.io/rust/792-rod-cutting-dp)**

---

# 792-rod-cutting-dp — Rod Cutting Problem
**Difficulty:** ⭐⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Rod cutting is a classic unbounded knapsack variant: given a rod of length n and prices for each length, maximize revenue by cutting the rod into pieces. Unlike 0/1 knapsack, pieces of the same length can be used multiple times. It was popularized by Cormen et al. "Introduction to Algorithms" and models real cutting-stock problems in manufacturing, timber processing, and cable management.

## Learning Outcomes

- Model rod cutting as `dp[i]` = max revenue for rod of length i
- Apply the unbounded knapsack recurrence: for each length i, try all cut positions j from 1 to i
- Understand why this is O(n²) time and O(n) space
- Recognize the connection to the coin change problem (same recurrence structure)
- Reconstruct the optimal cuts taken

## Rust Application

`rod_cutting(prices, n)` initializes `dp = [0; n+1]`. For each length `i`, tries all cut lengths `j` from 1 to `min(i, prices.len())`. Updates `dp[i] = max(dp[i], prices[j-1] + dp[i-j])`. The classic example: prices `[1,5,8,9,10,17,17,20]` for length 8 yields revenue 22 (cutting into pieces of 2+6 or similar). Tests cover the textbook examples.

## OCaml Approach

OCaml implements the same recurrence with `Array.make (n+1) 0` and nested `for` loops. The reconstruction uses a `cut: int array` tracking which cut was made at each length. Functional style can use `Array.fold_left` over cut lengths. OCaml's `List.init` generates the price list naturally. The rod cutting problem is typically presented alongside the coin change problem in OCaml algorithm courses.

## Key Differences

1. **Recurrence**: Identical to coin change but indexed from 1 (rod lengths) instead of the coin denominations; the algorithms are structurally the same.
2. **Reconstruction**: Both use an auxiliary `cut` array; Rust's `Option<usize>` per cell is cleaner than OCaml's `-1` sentinel.
3. **Manufacturing**: Real cutting-stock problems add kerf (saw blade waste) and minimum piece length; both languages model these as additional constraints.
4. **Optimization**: The price array may have `None` (unsaleable lengths); handling this requires wrapping prices in `Option`.

## Exercises

1. Implement `rod_cutting_reconstruct(prices, n) -> Vec<usize>` that returns the lengths of cuts made (in any order), not just the max revenue.
2. Add a fixed cost per cut (blade setup cost) to the problem: the revenue for k cuts is `max_revenue - k * cut_cost`, and find the optimal number of cuts.
3. Implement the 2D variant: a rectangular sheet can be cut horizontally or vertically, and pieces have prices by (width × height). Model as a 2D DP.
