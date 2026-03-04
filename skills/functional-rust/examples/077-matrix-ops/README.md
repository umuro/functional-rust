# 077: Matrix Operations — Functional 2D

**Difficulty:** 3  **Level:** Intermediate

Transpose and multiply matrices using `Vec<Vec<T>>` — nested iterators, ownership choices, and cache-friendly layout.

## The Problem This Solves

Matrix operations are everywhere: graphics (rotation, projection), machine learning (weight matrices), simulation (state transitions), signal processing (convolution). Understanding how to implement them correctly in Rust — especially the ownership tradeoffs — teaches the difference between consuming and borrowing nested structures.

Transpose and multiply are the two foundational operations. Everything else (determinant, inverse, LU decomposition) builds on them.

## The Intuition

**Transpose**: swap rows and columns. Element at `[r][c]` goes to `[c][r]`. If the original is `m×n`, the transpose is `n×m`.

**Multiply**: `A × B` where A is `m×n` and B is `n×p` produces an `m×p` matrix. Each output element `[i][j]` is the dot product of row `i` from A with column `j` from B.

The multiply trick: transposing B first makes "column j of B" into "row j of B-transposed." Then matrix multiply becomes: for each row of A, for each row of B-transposed, compute dot product. Rows are contiguous in memory — cache-friendly.

## How It Works in Rust

```rust
/// Transpose by reference — borrows input, returns new matrix
pub fn transpose_ref(matrix: &[Vec<i64>]) -> Vec<Vec<i64>> {
    if matrix.is_empty() || matrix[0].is_empty() { return vec![]; }
    let cols = matrix[0].len();
    (0..cols)
        .map(|c| matrix.iter().map(|row| row[c]).collect())
        //         ^^^^ for each column index,
        //               ^^^^ gather that column from every row
        .collect()
}

/// Transpose consuming — takes ownership, useful when you won't need the original
pub fn transpose(matrix: Vec<Vec<i64>>) -> Vec<Vec<i64>> {
    if matrix.is_empty() || matrix[0].is_empty() { return vec![]; }
    let rows = matrix.len();
    let cols = matrix[0].len();
    (0..cols)
        .map(|c| (0..rows).map(|r| matrix[r][c]).collect())
        .collect()
}

/// Dot product of two equal-length slices
fn dot(a: &[i64], b: &[i64]) -> i64 {
    a.iter().zip(b.iter()).map(|(x, y)| x * y).sum()
}

/// Matrix multiply: (m×n) × (n×p) → (m×p)
pub fn multiply(a: &[Vec<i64>], b: &[Vec<i64>]) -> Vec<Vec<i64>> {
    let bt = transpose_ref(b); // transpose B so we iterate over rows
    a.iter()
        .map(|row| bt.iter().map(|col| dot(row, col)).collect())
        //           ^^^^ each "column of B" is now a "row of B-transposed"
        .collect()
}
```

The nested iterator chain in `transpose_ref` reads: for each column index `c`, collect the value `matrix[row][c]` across all rows. This is idiomatic Rust — no explicit loop counters.

Note: `Vec<Vec<T>>` is not a flat contiguous array. For serious numeric work, use `ndarray` or a flat `Vec<T>` with manual indexing (`matrix[r * cols + c]`).

## What This Unlocks

- **Linear algebra foundations**: all higher-level operations (rotation, projection, Gaussian elimination) build on transpose and multiply.
- **Functional 2D patterns**: the `(0..n).map(|i| row.iter().map(...).collect()).collect()` idiom generalizes to any 2D transformation.
- **Ownership design**: the consuming vs borrowing transpose shows how to choose signatures for library APIs.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Matrix type | `int list list` | `Vec<Vec<i64>>` |
| Transpose access | `List.nth` (O(n)) | `matrix[r][c]` (O(1)) |
| Column iteration | Manual recursion | `(0..cols).map(\|c\| matrix.iter().map(\|row\| row[c]))` |
| Dot product | `List.fold_left2` | `.zip().map().sum()` |
| Consuming vs borrowing | GC handles both | `Vec<Vec<T>>` vs `&[Vec<T>]` — explicit choice |
