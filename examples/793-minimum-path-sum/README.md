📖 **[View on hightechmind.io →](https://hightechmind.io/rust/793-minimum-path-sum)**

---

# 793-minimum-path-sum — Minimum Path Sum

## Problem Statement

Finding the minimum-cost path through a grid from the top-left to the bottom-right (moving only right or down) is a fundamental 2D DP problem. It models route planning in robotics, game pathfinding with terrain costs, circuit board wiring, and is a building block for more complex grid DP problems. The constraint of only moving right or down makes it solvable in O(mn) with O(1) extra space (modifying the grid in place).

## Learning Outcomes

- Initialize boundary conditions: first row and column are prefix sums (no choice)
- Apply the 2D recurrence: `dp[i][j] = grid[i][j] + min(dp[i-1][j], dp[i][j-1])`
- Optimize space by modifying the grid in-place or using a rolling row
- Understand the difference from A* pathfinding (no heuristic needed here)
- Reconstruct the actual path by backtracking from `dp[m-1][n-1]`

## Rust Application

`min_path_sum(grid)` creates a `dp: Vec<Vec<i32>>` copy, initializes first row and column as prefix sums, then fills the rest with `grid[i][j] + min(dp[i-1][j], dp[i][j-1])`. Returns `dp[m-1][n-1]`. The classic test grid `[[1,3,1],[1,5,1],[4,2,1]]` has minimum path sum 7 (1→3→1→1→1). Tests cover the base cases: single cell, single row, single column.

## OCaml Approach

OCaml implements with `Array.make_matrix m n 0` and fills boundary conditions explicitly. The 2D iteration uses nested `for` loops. Functional style can use `Array.init m (fun i -> Array.init n (fun j -> ...))` but requires careful ordering for dependency correctness. The OCaml `min` function and array access patterns are idiomatic.

## Key Differences

1. **In-place optimization**: Rust can modify the input grid directly (if mutable) to achieve O(1) extra space; OCaml's immutable arrays require a copy.
2. **Rolling array**: Both languages support a rolling 1D array optimization reducing space from O(mn) to O(n).
3. **Path reconstruction**: Both backtrack from the bottom-right, comparing neighbors to trace the minimum path.
4. **Generalization**: This problem generalizes to arbitrary DAG shortest paths — the 2D grid DAG constraint makes it DP-solvable; general DAGs require Bellman-Ford.

## Exercises

1. Implement `min_path_sum_with_path(grid) -> (i32, Vec<(usize,usize)>)` that returns both the minimum sum and the actual path as a list of coordinates.
2. Modify to allow movement in all four directions (with a cycle-detection constraint) and compare the complexity with the restricted problem.
3. Implement the in-place variant that modifies `grid` directly and uses O(1) extra space. Note any aliasing concerns.
