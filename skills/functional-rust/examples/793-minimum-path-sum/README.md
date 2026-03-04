# 793: Minimum Path Sum in Grid

**Difficulty:** 3  **Level:** Advanced

Find the path from top-left to bottom-right of a grid (moving only right or down) that minimizes the sum of cell values.

## The Problem This Solves

Given an `m×n` grid of non-negative integers, find the path from `(0,0)` to `(m-1, n-1)` that minimizes the total cost. You can only move right or down. This problem appears directly in robotics path planning (minimize energy over a terrain grid), image seam carving (Photoshop's content-aware scaling removes "minimum cost seams"), game map traversal, and any optimization over a DAG that has a natural grid structure.

The DP approach is elegant: since you can only move right or down, each cell `(i,j)` can only be reached from `(i-1,j)` or `(i,j-1)`. The minimum cost to reach `(i,j)` is therefore `grid[i][j] + min(cost_to_reach(i-1,j), cost_to_reach(i,j-1))`. This gives an O(m×n) solution that overwrites the grid in-place — O(1) extra space.

This is one of the simplest DP patterns on a 2D grid, but it introduces the key ideas: memoization via overwriting, boundary initialization, and path reconstruction from the final DP table.

## The Intuition

Initialize the first row (only rightward moves possible) and first column (only downward moves). Then fill the rest: `dp[i][j] = grid[i][j] + min(dp[i-1][j], dp[i][j-1])`. The answer is `dp[m-1][n-1]`. To reconstruct the path, backtrack from the bottom-right corner: at each step, move toward whichever neighbor has the smaller accumulated cost. O(m×n) time, O(1) space (in-place variant).

## How It Works in Rust

```rust
fn min_path_sum(grid: &[Vec<u64>]) -> u64 {
    let m = grid.len();
    let n = grid[0].len();
    let mut g: Vec<Vec<u64>> = grid.to_vec();  // working copy

    // Initialize edges: only one way to reach cells on first row/column
    for j in 1..n { g[0][j] += g[0][j - 1]; }   // top row: sum left-to-right
    for i in 1..m { g[i][0] += g[i - 1][0]; }   // left col: sum top-to-bottom

    // Fill interior: come from whichever neighbor is cheaper
    for i in 1..m {
        for j in 1..n {
            g[i][j] += g[i - 1][j].min(g[i][j - 1]);
        }
    }
    g[m - 1][n - 1]
}

// Backtrack from (m-1, n-1) to reconstruct the path
let mut path = Vec::new();
let (mut i, mut j) = (m - 1, n - 1);
loop {
    path.push((i, j));
    if i == 0 && j == 0 { break; }
    if i == 0 { j -= 1; }          // stuck on top row: can only go left
    else if j == 0 { i -= 1; }     // stuck on left col: can only go up
    else if g[i-1][j] < g[i][j-1] { i -= 1; }  // came from above
    else { j -= 1; }               // came from the left
}
path.reverse();  // collected bottom-up, need top-down order
```

The in-place trick (`grid.to_vec()` then overwrite) avoids a separate DP array. The reconstruction handles edge cases (being on a boundary row/column) before the general case.

## What This Unlocks

- **In-place DP** — overwriting the input grid (or a copy) is the standard O(1) extra space technique for 2D grid DP; the same trick applies to unique paths, edit distance row-by-row, and triangle DP.
- **Boundary initialization** — always initializing the first row and column separately is the pattern that prevents off-by-one errors in grid DP.
- **Backtracking from cost table** — the reconstructed path comes "for free" once you have the DP table; no extra tracking array needed if the recurrence is reversible.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| 2D array copy | `Array.copy` / `Array.map Array.copy` | `grid.to_vec()` (deep clone of `Vec<Vec<T>>`) |
| `min` of two values | `min a b` | `a.min(b)` — method syntax on numeric types |
| Mutable 2D indexing | `g.(i).(j) <- v` | `g[i][j] = v` |
| Path reconstruction | Usually functional with `List.rev` | `Vec::push` then `reverse()` — same pattern |
