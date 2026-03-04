# 796: Counting Paths in Grid with Obstacles

**Difficulty:** 3  **Level:** Advanced

Count all distinct paths from top-left to bottom-right of a grid, moving only right or down, while avoiding blocked cells.

## The Problem This Solves

How many ways can a robot navigate from the top-left to the bottom-right of a grid if some cells are blocked? This classic combinatorics-meets-DP problem appears in robotics (enumerate valid routes), coverage testing (how many execution paths exist through a flowchart), and combinatorics education (lattice path counting with constraints).

Without obstacles, the answer is the binomial coefficient `C(m+n-2, m-1)` — a closed-form formula. Obstacles break the symmetry and force DP. The key insight: the number of paths to `(i,j)` is the sum of paths to `(i-1,j)` (came from above) and `(i,j-1)` (came from the left), unless `(i,j)` itself is blocked (then zero). Obstacles propagate naturally: if a cell is blocked, no paths flow through it.

This is simpler than minimum path sum — no `min` comparison needed, just additive accumulation with a "wall" check. But it introduces an important subtlety: if the start or end cell is blocked, the answer is immediately zero.

## The Intuition

`dp[i][j]` = number of distinct paths from `(0,0)` to `(i,j)`. Base: `dp[0][0] = 1` (one way to be at the start). Blocked cell: `dp[i][j] = 0`. Otherwise: `dp[i][j] = dp[i-1][j] + dp[i][j-1]` (with boundary handling). First row and column can each only be reached in one way (if no obstacles block the way). The count can grow exponentially — for a 20×20 grid it's already in the billions — so `u64` is essential. O(m×n) time, O(m×n) space (or O(n) with row compression).

## How It Works in Rust

```rust
fn count_paths(grid: &[Vec<u8>]) -> u64 {
    let m = grid.len();
    let n = grid[0].len();
    // Early exit: start or end is blocked
    if grid[0][0] == 1 || grid[m-1][n-1] == 1 { return 0; }

    let mut dp = vec![vec![0u64; n]; m];
    dp[0][0] = 1;

    // First row: each cell reachable only from the left
    for j in 1..n {
        dp[0][j] = if grid[0][j] == 0 { dp[0][j-1] } else { 0 };
    }
    // First column: each cell reachable only from above
    for i in 1..m {
        dp[i][0] = if grid[i][0] == 0 { dp[i-1][0] } else { 0 };
    }
    // Interior: sum of above + left, blocked cells contribute 0
    for i in 1..m {
        for j in 1..n {
            dp[i][j] = if grid[i][j] == 1 { 0 }
                       else { dp[i-1][j] + dp[i][j-1] };
        }
    }
    dp[m-1][n-1]
}
```

The `u8` encoding (`0` = open, `1` = blocked) keeps the grid compact. Obstacles on the first row/column "cut off" all cells beyond them — propagating zeros cleanly. No special case needed: once a cell in the first row is zero, all subsequent cells in that row will also be zero.

## What This Unlocks

- **Grid DP template** — the structure (boundary init → interior fill) is identical across min path sum, edit distance, and longest common subsequence; mastering this template unlocks all grid-based DP.
- **Count vs. optimize** — counting paths uses addition while optimizing (min/max) uses comparison; the same DP structure supports both by changing the operator.
- **Obstacle propagation** — blocked cells naturally zero out all descendants; no special flood-fill or BFS needed; this is a key advantage of DP over search for counting problems.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Grid representation | `int array array` | `Vec<Vec<u8>>` — `u8` for compact obstacle flags |
| Zero propagation | `if grid.(i).(j)=1 then 0 else ...` | `if grid[i][j] == 1 { 0 } else { ... }` |
| Count overflow | OCaml `int` = 63-bit on 64-bit | `u64` explicit — required for large grids |
| Early return | `if ... then 0 else body` | `if ... { return 0; }` — imperative early exit |
