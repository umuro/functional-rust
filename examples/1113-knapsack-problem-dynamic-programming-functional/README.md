# Example 1113: 0/1 Knapsack with Dynamic Programming (Functional Style)
**Difficulty:** ⭐⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Given a set of items, each with a weight and a value, and a knapsack with a fixed capacity, find the maximum total value of items that fit without exceeding the capacity. Each item may be used at most once — this is the 0/1 constraint. The classic DP recurrence `dp[w] = max(dp[w], value[i] + dp[w - weight[i]])` applies, and the key implementation challenge is computing it correctly without a full 2D table.

## Learning Outcomes

- How a 2D DP table `dp[item][capacity]` collapses to a single row when you observe that each row only reads from the previous row — reducing space from O(n × W) to O(W)
- Why capacity must be iterated in **reverse** when reusing a single row: forward iteration would allow an item to be selected multiple times in the same pass, violating the 0/1 constraint
- How `(0..n).for_each(|i| ...)` and `(0..=capacity).rev().for_each(|w| ...)` replace imperative `for` loops with iterator combinators, expressing the same logic in a more compositional style
- How the OCaml and Rust implementations converge on the same algorithm even though the OCaml source uses explicit `for` loops and mutable array syntax
- The precise relationship between the bottom-up DP approach (Rust/OCaml) and the top-down recursive memoization approach: same recurrence, different traversal order

## OCaml Approach

The OCaml implementation uses a mutable `dp` array of length `capacity + 1` initialized to zero, then iterates items with a `for i = 0 to n - 1` loop and capacities with `for w = capacity downto 0`. The `downto` keyword is OCaml's built-in reverse-range iteration, directly expressing the requirement that each capacity is processed from large to small. The update `dp.(w) <- max dp.(w) (values.(i) + dp.(w - weights.(i)))` uses OCaml's array mutation syntax. Despite using mutable arrays and `for` loops, the OCaml version is structurally identical to the Rust version — both implement the same space-optimized bottom-up DP.

## Rust Application

`knapsack(weights, values, capacity)` allocates `dp` as a `Vec<usize>` of length `capacity + 1`. The outer loop `(0..n).for_each(|i| ...)` iterates items; the inner `(0..=capacity).rev().for_each(|w| ...)` iterates capacities in reverse, enforcing the 0/1 constraint. The update `dp[w] = dp[w].max(values[i] + dp[w - weights[i]])` is guarded by `if weights[i] <= w` to avoid underflow. Using `for_each` instead of `for` keeps the style composable — the range expression is an iterator that can be chained, filtered, or mapped without restructuring.

## Key Differences

1. **Traversal direction:** Rust uses `.rev()` on a range to iterate capacities from large to small; OCaml uses `downto` in the `for` loop — semantically identical, syntactically different
2. **Space complexity:** Both use a single O(capacity) row; a naïve 2D table would use O(n × capacity) but would make the subproblem structure (`dp[i][w]` depends on `dp[i-1][w-weight]`) visually explicit
3. **Loop style:** Rust expresses loops as iterator combinators (`for_each`); OCaml uses built-in `for`/`downto` syntax — both compile to the same flat iteration with no closure overhead
4. **Constraint enforcement:** The 0/1 constraint (each item used at most once) is enforced entirely by the reverse traversal direction; switching to forward order silently produces the unbounded knapsack result

## Exercises

1. Modify `knapsack` to also return the list of selected item indices — backtrack through the `dp` array after computing the maximum to reconstruct which items were chosen
2. Change the inner loop from `.rev()` to forward order and run the existing tests — identify which test cases fail and explain why forward order implements the unbounded knapsack instead
3. Implement the unbounded knapsack variant (each item may be used any number of times) and confirm it requires only the single traversal-direction change
4. Implement a top-down recursive version with memoization using a `HashMap<(usize, usize), usize>` keyed on `(item_index, remaining_capacity)` and verify it produces identical results
