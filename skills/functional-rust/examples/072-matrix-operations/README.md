# 072: Matrix Operations

**Difficulty:** ⭐⭐  **Level:** Foundations

Transpose and multiply matrices using nested iterators — and let the borrow checker guarantee you never accidentally mutate input data.

## The Problem This Solves

Matrices are everywhere: graphics transforms, machine learning weights, graph adjacency, image processing. At some point you need to transpose (flip rows and columns) or multiply two matrices. In most languages you write nested loops and hope you got the indices right.

The functional approach builds each result row/column as an iterator pipeline. The intent becomes visible in the code: transpose is "for each column index, collect the corresponding element from each row." Multiply is "for each row of A and each column of B, compute the dot product."

Rust's borrow checker gives you something extra: the input matrices are borrowed (`&Matrix`), so the compiler proves you're not accidentally modifying them while reading. No defensive copies needed.

## The Intuition

**Transpose:** Turn `[[1,2,3],[4,5,6]]` into `[[1,4],[2,5],[3,6]]`. Iterate over column indices (0, 1, 2), and for each one, collect that column from every row.

**Dot product:** Multiply corresponding elements and sum: `dot([1,2,3], [4,5,6]) = 1*4 + 2*5 + 3*6 = 32`. This is `zip` then `map` then `sum`.

**Matrix multiply:** For each row of A and each column of B, compute their dot product. The trick: transpose B first so columns become rows — then you can iterate over both with the same `dot` function.

## How It Works in Rust

```rust
pub type Matrix = Vec<Vec<i64>>;

// Transpose: for each column index, collect that element from every row
pub fn transpose(matrix: &Matrix) -> Matrix {
    if matrix.is_empty() || matrix[0].is_empty() { return vec![]; }
    let cols = matrix[0].len();
    (0..cols)
        .map(|col| matrix.iter().map(|row| row[col]).collect())
        .collect()
}

// Dot product: zip two slices, multiply pairs, sum
pub fn dot(a: &[i64], b: &[i64]) -> i64 {
    a.iter().zip(b.iter()).map(|(x, y)| x * y).sum()
}

// Multiply: A[i][j] = dot(row i of A, col j of B)
// Trick: transpose B so columns become rows
pub fn multiply(a: &Matrix, b: &Matrix) -> Matrix {
    let bt = transpose(b);
    a.iter()
        .map(|row| bt.iter().map(|col| dot(row, col)).collect())
        .collect()
}
```

## What This Unlocks

- **Linear algebra fundamentals** — the building blocks for 3D transforms, neural network layers, and graph algorithms
- **Iterator nesting patterns** — `outer.map(|row| inner.map(...).collect()).collect()` appears everywhere in 2D data processing
- **Safe borrowing** — `&Matrix` parameters let you work with large matrices without copying, guaranteed safe by the compiler

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Matrix type | `int list list` | `Vec<Vec<i64>>` (type alias: `Matrix`) |
| Transpose | `List.init cols (fun i -> List.map (fun row -> List.nth row i) matrix)` | `(0..cols).map(\|col\| matrix.iter().map(\|row\| row[col]).collect()).collect()` |
| Dot product | `List.fold_left2 (fun acc x y -> acc + x*y) 0 a b` | `a.iter().zip(b).map(\|(x,y)\| x*y).sum()` |
| Mutation safety | Immutable lists — safe by default | Borrow checker enforces `&Matrix` is read-only |
