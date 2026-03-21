📖 **[View on hightechmind.io →](https://hightechmind.io/rust/1054-knapsack-01)**

---

# 1054-knapsack-01 — 0/1 Knapsack
**Difficulty:** ⭐⭐⭐  
**Category:** Functional Programming  



## Problem Statement

The 0/1 knapsack problem is the quintessential combinatorial optimization problem: given items with weights and values, and a weight capacity, select items to maximize total value without exceeding capacity. Each item can be included at most once (0 or 1 times). It has direct applications in resource allocation, financial portfolio optimization, and load balancing.

The DP solution runs in O(n × capacity) time — pseudo-polynomial because it depends on the numeric value of capacity, not just the number of items.

## Learning Outcomes

- Implement the 0/1 knapsack with a 2D DP table
- Optimize to a 1D rolling array using reverse iteration
- Understand why the reverse iteration is necessary for 0/1 (vs unbounded)
- Implement the top-down memoized variant for comparison
- Reconstruct which items were selected using backtracking over the DP table

## Rust Application

`src/lib.rs` provides three approaches. `knapsack_2d` uses a full `n × capacity` table where `dp[i][w]` is the best value using the first `i` items with capacity `w`. `knapsack_1d` optimizes to a single row by iterating capacity in reverse — critical for correctness (ensures each item is counted at most once). `knapsack_memo` uses a `HashMap<(usize, usize), i64>` for top-down memoization.

The reverse iteration in `knapsack_1d` is the key insight: iterating `w` from `capacity` down to `weights[i]` ensures that when computing `dp[w]`, the value `dp[w - weights[i]]` still reflects the state before item `i` was considered.

## OCaml Approach

```ocaml
let knapsack weights values capacity =
  let n = Array.length weights in
  let dp = Array.make (capacity + 1) 0 in
  for i = 0 to n - 1 do
    for w = capacity downto weights.(i) do
      dp.(w) <- max dp.(w) (dp.(w - weights.(i)) + values.(i))
    done
  done;
  dp.(capacity)
```

The `downto` keyword makes the reverse iteration clear in OCaml. The structure is identical to Rust's approach.

## Key Differences

1. **`downto` syntax**: OCaml's `for w = capacity downto weights.(i)` expresses reverse iteration clearly; Rust uses `.rev()` on a range.
2. **2D vs 1D**: Both languages support both approaches; the 1D is preferred for space efficiency when item reconstruction is not needed.
3. **Index vs slice**: OCaml's `weights.(i)` is direct array access; Rust's `weights[i]` is equivalent with bounds checking.
4. **Reconstruction**: Backtracking over the 2D table to find which items were selected is language-independent — both scan from `dp[n][capacity]` backward.

## Exercises

1. Add item selection reconstruction: return `Vec<usize>` of selected item indices alongside the maximum value.
2. Implement the unbounded knapsack (each item can be used multiple times) and compare the recurrence change (no reverse iteration needed).
3. Solve the fractional knapsack (items can be split) using a greedy algorithm and compare it with the DP solution.
