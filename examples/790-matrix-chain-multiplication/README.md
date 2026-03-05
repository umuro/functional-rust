📖 **[View on hightechmind.io →](https://hightechmind.io/rust/790-matrix-chain-multiplication)**

---

# 790: Matrix Chain Multiplication

**Difficulty:** 4  **Level:** Advanced

Find the optimal parenthesization order to minimize scalar multiplications when chaining matrices together.

## The Problem This Solves

Matrix multiplication is associative: `(A × B) × C = A × (B × C)`. The order of parenthesization doesn't change the result — but it dramatically changes the cost. With the wrong order, multiplying a chain of matrices can require billions of operations; with the optimal order, the same chain might need only thousands.

This matters any time you compose linear transformations — 3D rendering pipelines, neural network weight matrices, compiler-generated loop transformations, and robotics kinematics chains all multiply sequences of matrices. In production, getting the order wrong by even one step can make a hot path 100× slower.

Matrix chain multiplication is a classic **interval DP** problem: you break the chain at every possible split point `k`, recursively solve both sub-chains, and pick the split that minimizes total cost. Bottom-up DP fills a triangular table in O(n³) time and O(n²) space, then backtracks through a `split` table to reconstruct the optimal parenthesization.

## The Intuition

Given matrices M₁, M₂, …, Mₙ with compatible dimensions, you want to find where to place the parentheses so the total number of scalar multiplications is minimized. Multiplying an `a×b` matrix by a `b×c` matrix costs `a*b*c` operations. The DP state `dp[i][j]` = minimum cost to multiply matrices `i` through `j`. Try every split point `k` between `i` and `j`, then `dp[i][j] = min over k of (dp[i][k] + dp[k+1][j] + dims[i]*dims[k+1]*dims[j+1])`. O(n³) time, O(n²) space.

## How It Works in Rust

```rust
fn matrix_chain(dims: &[usize]) -> (usize, Vec<Vec<usize>>) {
    let n = dims.len() - 1;  // number of matrices
    let mut dp    = vec![vec![0usize; n]; n];  // min cost
    let mut split = vec![vec![0usize; n]; n];  // optimal split point

    // Fill by chain length l = 2..=n
    for l in 2..=n {
        for i in 0..=(n - l) {
            let j = i + l - 1;
            dp[i][j] = usize::MAX;
            for k in i..j {
                // Cost = left sub-chain + right sub-chain + this multiplication
                let cost = dp[i][k]
                    .saturating_add(dp[k + 1][j])
                    .saturating_add(dims[i] * dims[k + 1] * dims[j + 1]);
                if cost < dp[i][j] {
                    dp[i][j] = cost;
                    split[i][j] = k;  // remember where to split
                }
            }
        }
    }
    (dp[0][n - 1], split)
}

// Reconstruct the optimal parenthesization from the split table
fn parenthesize(split: &Vec<Vec<usize>>, i: usize, j: usize) -> String {
    if i == j { format!("M{}", i + 1) }
    else {
        let k = split[i][j];
        format!("({} × {})", parenthesize(split, i, k), parenthesize(split, k + 1, j))
    }
}
```

Key Rust details: `saturating_add` prevents overflow when adding to `usize::MAX` (sentinel for "not yet computed"); the `split` table is a separate `Vec<Vec<usize>>` that mirrors the `dp` table structure; reconstruction is a natural recursive descent.

## What This Unlocks

- **Interval DP pattern** — the same "fill by sub-problem length" structure applies to optimal BST construction, polygon triangulation, and burst balloons problems.
- **Split-table reconstruction** — separating the "cost" DP from the "decision" DP lets you cheaply trace back the optimal solution after O(n³) work.
- **Saturating arithmetic for sentinels** — `usize::MAX` as infinity works safely with `saturating_add`, avoiding the classic "accidentally wrap to zero" bug.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| 2D mutable table | `Array.make_matrix n n 0` | `vec![vec![0usize; n]; n]` |
| Infinity sentinel | `max_int` | `usize::MAX` with `saturating_add` |
| Recursive reconstruction | Natural pattern matching | Explicit recursion with index tracking |
| Immutable sub-results | Functional style with `let` | Same — inner `for` builds toward `dp[i][j]` |
