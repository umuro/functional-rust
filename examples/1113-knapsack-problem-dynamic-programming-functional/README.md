# Example 1113: 0/1 Knapsack with Dynamic Programming (Functional Style)

## Problem Statement
Given items with weights and values and a fixed capacity, find the maximum total value that fits without exceeding the capacity. Each item may be used at most once (0/1 constraint).

## Learning Outcomes
- How space-optimized DP reduces a 2D table to a single row by iterating capacities in reverse
- Using `for_each` with iterator combinators instead of imperative `for` loops to express DP transitions functionally
- Why reverse traversal of the capacity dimension is essential for the 0/1 constraint

## Rust Application
`knapsack(weights, values, capacity)` maintains a single `dp` row of length `capacity+1`, iterating items with `(0..n).for_each(|i| ...)` and capacities in reverse with `(0..=capacity).rev().for_each(|w| ...)`, updating `dp[w] = dp[w].max(values[i] + dp[w - weights[i]])` when the item fits.

## OCaml Approach
The OCaml reference uses a recursive function with a 2D memoization table, pattern matching on the item count and remaining capacity to express the same recurrence as top-down recursive case splits rather than bottom-up iteration.

## Key Differences
1. **Traversal direction:** Rust iterates capacities in reverse (`rev()`) to enforce the 0/1 constraint in-place; OCaml memoization naturally avoids reuse by indexing a fresh cell `(i, w)` per item
2. **Space complexity:** Rust uses O(capacity) space with a single row; OCaml's 2D table uses O(n × capacity) but makes the subproblem structure explicit
3. **Control flow style:** Rust expresses loops as iterator combinators (`for_each`); OCaml expresses recursion as pattern-matched function calls, both avoiding mutable loop variables

## Exercises
1. Modify the Rust implementation to also return the list of selected item indices, not just the maximum value
2. Change the inner loop from `rev()` to forward order and observe how the result changes — explain why this breaks the 0/1 constraint
3. Implement an unbounded knapsack variant (each item may be used multiple times) and identify the single traversal-direction change required
