📖 **[View on hightechmind.io →](https://hightechmind.io/rust/785-knapsack-01)**

---

# 785-knapsack-01 — 0/1 Knapsack Problem

## Problem Statement

The 0/1 knapsack problem is a foundational combinatorial optimization problem: given items with weights and values and a capacity-limited bag, select items to maximize total value without exceeding capacity. "0/1" means each item is either taken or not (no fractions). It appears in resource allocation, project portfolio selection, cargo loading, and is a building block for more complex optimization problems. The DP solution runs in O(n × W) time, making large instances tractable.

## Learning Outcomes

- Model the 0/1 knapsack as a 2D DP table `dp[i][w]` = max value using first i items with capacity w
- Implement the recurrence: `dp[i][w] = max(dp[i-1][w], dp[i-1][w - weight[i]] + value[i])`
- Optimize space from O(n × W) to O(W) using a 1D array with right-to-left iteration
- Understand why right-to-left iteration prevents using the same item twice
- Reconstruct the chosen items by backtracking through the DP table

## Rust Application

`knapsack(weights, values, capacity)` fills an `(n+1) × (capacity+1)` table. For each item and capacity, it takes the maximum of skipping or including the item. `knapsack_optimized` uses a single `Vec<usize>` of length `capacity+1`, iterating weights from right to left to avoid counting items twice. Tests cover the classic example `(weights=[1,2,3], values=[6,10,12], capacity=5)` and edge cases.

## OCaml Approach

OCaml's functional style uses `Array.init` for DP table construction and `Array.fold_left` for optimization. The functional approach with `Array.init (n+1) (fun i -> Array.init (cap+1) (fun w -> ...))` creates the full table lazily but requires a fix-point or explicit memoization. The imperative style with `for` loops and `Array.set` is idiomatic for DP in OCaml despite the functional aesthetic.

## Key Differences

1. **Mutability**: Rust's `vec![vec![0; capacity + 1]; n + 1]` with `dp[i][w] = ...` is imperative; OCaml's `for` loop equivalent is similar but uses `Array.set dp.(i).(w) val`.
2. **Space optimization**: Both languages implement the 1D right-to-left optimization identically — the algorithm is language-independent.
3. **Reconstruction**: Rust backtracks through `dp` by comparing `dp[i][w]` with `dp[i-1][w]`; OCaml uses the same technique.
4. **Bounds checking**: Rust's `Vec` bounds-checks by default; OCaml's `Array.unsafe_get` skips checks for performance in tight DP loops.

## Exercises

1. Add item reconstruction: modify `knapsack` to also return the indices of the chosen items by backtracking through the DP table.
2. Implement the unbounded knapsack variant where each item can be taken multiple times, modifying the recurrence and iteration direction.
3. Write a branch-and-bound solver for knapsack that prunes branches where the remaining capacity cannot improve the best-so-far solution, and compare it against DP for small instances.
