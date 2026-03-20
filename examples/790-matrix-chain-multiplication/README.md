📖 **[View on hightechmind.io →](https://hightechmind.io/rust/790-matrix-chain-multiplication)**

---

# 790-matrix-chain-multiplication — Matrix Chain Multiplication
**Difficulty:** ⭐⭐⭐  
**Category:** Functional Programming  


## Problem Statement

The matrix chain multiplication problem asks: in what order should a sequence of matrices be multiplied to minimize the total number of scalar multiplications? Different association orders can differ by orders of magnitude: `(A × B) × C` might cost 1000 operations while `A × (B × C)` costs 100. The O(n³) DP solution, from Godbole (1973), is a textbook example of interval DP and is used in query optimization in databases (join reordering) and deep learning compilers (operation fusion).

## Learning Outcomes

- Model matrix chain as `dims: &[usize]` where `dims[i] × dims[i+1]` is the size of matrix i
- Fill a 2D DP table where `dp[i][j]` = min cost to multiply matrices i through j
- Understand interval DP: building solutions to larger intervals from smaller ones
- Implement the split-point recurrence: try all `k` in `i..j` as split points
- Use the optimal ordering for actual computation (requires parenthesization reconstruction)

## Rust Application

`matrix_chain(dims)` initializes `dp[i][i] = 0` (single matrix = no cost) and fills by chain length from 2 to n. For each interval `[i, j]` and split point `k`, computes `dp[i][k] + dp[k+1][j] + dims[i]*dims[k+1]*dims[j+1]` and takes the minimum. The classic test case `dims=[10,20,30,40,30]` should produce cost 30000.

## OCaml Approach

OCaml implements interval DP with `Array.make_matrix n n 0` and nested `for` loops. The outer loop is over chain length, not indices directly, to ensure smaller subproblems are solved before larger ones. OCaml's `min` function and imperative array updates make this direct. The reconstruction uses a separate `split: int array array` table storing optimal split points.

## Key Differences

1. **Interval DP pattern**: Both languages use the same outer-loop-over-length approach to ensure subproblem ordering; the code structure is nearly identical.
2. **Initialization**: Rust uses `dp[i][j] = usize::MAX` as infinity; OCaml uses `max_int` — same concept.
3. **Real-world**: Database query optimizers use the same interval DP for join ordering; Rust-based databases like TiKV use this internally.
4. **Space**: O(n²) space is required; no known space-efficient algorithm exists for this problem.

## Exercises

1. Add a `matrix_chain_order(dims)` function that returns the optimal parenthesization as a string like `"((A(BC))D)"`.
2. Implement the memoized top-down version and verify it produces the same results as the bottom-up tabulation.
3. Apply matrix chain optimization to a sequence of 10 randomly-sized matrices and measure how much the optimal ordering reduces the operation count vs. left-to-right.
