📖 **[View on hightechmind.io →](https://hightechmind.io/rust/796-counting-paths-dp)**

---

# 796-counting-paths-dp — Counting Paths
**Difficulty:** ⭐  
**Category:** Functional Programming  


## Problem Statement

Counting the number of distinct paths in a grid from top-left to bottom-right (moving right or down) is a combinatorics problem solvable by DP. The answer without obstacles is `C(m+n-2, m-1)` — a binomial coefficient — but obstacles require DP. This problem models robot motion planning, network routing analysis, and combinatorial counting. The obstacle variant uses the same recurrence with zeroed cells.

## Learning Outcomes

- Implement `unique_paths(m, n)` filling `dp[i][j] = dp[i-1][j] + dp[i][j-1]`
- Handle the obstacle variant `unique_paths_obstacles` by zeroing blocked cells
- Understand that the answer without obstacles equals the binomial coefficient `C(m+n-2, m-1)`
- Optimize space from O(mn) to O(n) using a single rolling row
- See how this relates to Pascal's triangle

## Rust Application

`unique_paths(m, n)` initializes `dp[i][j] = 1` for i=0 or j=0 (boundary paths), then fills the rest. `unique_paths_obstacles` initializes carefully: obstacle cells are 0, boundary cells before an obstacle are 0 (blocked). Tests cover the standard `3×7` grid (28 paths), obstacle variants, and single-row/column grids.

## OCaml Approach

OCaml implements with `Array.make_matrix m n 0` and explicit boundary initialization. The obstacle check `grid.(i).(j) = 1` blocks paths. Functional style can express this as `let dp = Array.init m (fun i -> Array.init n (fun j -> if i=0 || j=0 then 1 else dp.(i-1).(j) + dp.(i).(j-1)))` but requires mutual reference tricks. Pascal's triangle connection: `dp[i][j]` equals `C(i+j, i)` in the obstacle-free case.

## Key Differences

1. **Pascal connection**: `dp[i][j] = dp[i-1][j] + dp[i][j-1]` is Pascal's recurrence rotated 45 degrees; both languages show this connection naturally.
2. **Obstacle handling**: Both languages require careful boundary initialization — an obstacle in row 0 or column 0 blocks all subsequent boundary cells.
3. **Overflow**: For large grids (100×100), path counts exceed u64 range; both languages need big integer support (BigInt/Zarith).
4. **Space optimization**: A single-row DP reduces space to O(n); `dp[j] += dp[j-1]` in a left-to-right pass works correctly.

## Exercises

1. Implement the space-optimized O(n) version using a single row, verifying it produces the same results as the 2D version.
2. Add support for a `start` and `end` position anywhere in the grid (not restricted to corners), modifying the initialization and boundary conditions.
3. Solve the 3D variant: counting paths in an m×n×p cube from `(0,0,0)` to `(m-1,n-1,p-1)` moving only right, down, or forward.
