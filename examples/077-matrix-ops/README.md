📖 **[View on hightechmind.io →](https://hightechmind.io/rust/077-matrix-ops)**

---

# 077 — Matrix Operations

## Problem Statement

Matrix operations — transpose and multiplication — are the foundation of linear algebra, which underpins graphics (rotation/scaling transforms), machine learning (neural network layers are matrix multiplications), physics simulation (finite element methods), and signal processing (FFT as matrix multiplication). Understanding the data layout and functional implementation of these operations is prerequisite for any numerical computing work.

The functional approach: transpose by column extraction with iterators; multiply by dot products of rows and columns. The ownership model for matrices in Rust — take ownership for consuming operations, borrow for non-consuming ones — illustrates the practical impact of ownership on API design.

## Learning Outcomes

- Transpose a matrix using iterator-based column extraction
- Multiply matrices using dot products of rows and transposed columns
- Understand why transpose is taken before multiplication (column access)
- Use `(0..cols).map(|c| (0..rows).map(|r| matrix[r][c]).collect())` for functional transpose
- Design APIs that take ownership vs borrow based on whether they consume the matrix

## Rust Application

`transpose(matrix: Vec<Vec<i64>>) -> Vec<Vec<i64>>` takes ownership and returns a new matrix. `transpose_ref(matrix: &[Vec<i64>]) -> Vec<Vec<i64>>` borrows and clones into the result. `multiply(a, b)` computes `bt = transpose_ref(b)` then `a.iter().map(|row| bt.iter().map(|col| dot(row, col)).collect()).collect()`. `dot` uses `zip + map + sum`.

## OCaml Approach

OCaml's transpose: `let transpose m = let rows = List.length m and cols = List.length (List.hd m) in List.init cols (fun c -> List.init rows (fun r -> List.nth (List.nth m r) c))`. `List.nth` is O(n) — for performance, use `Array.make_matrix`. Matrix multiply: `let multiply a b = let bt = transpose b in List.map (fun row -> List.map (fun col -> List.fold_left2 (+) 0 row col) bt) a`.

## Key Differences

1. **`List.nth` is O(n)**: OCaml's list-based matrix has O(n·m) access patterns due to `List.nth`. Use `Array.make_matrix` for O(1) random access. Rust's `Vec<Vec<i64>>` has O(1) indexed access.
2. **Ownership in API**: `transpose(matrix: Vec<Vec<i64>>)` moves the matrix — useful when you do not need the original. `transpose_ref(&[Vec<i64>])` borrows — useful when the original is still needed.
3. **Cache efficiency**: `Vec<Vec<i64>>` is not contiguous in memory (each row is separately allocated). For high-performance matrix operations, use a flat `Vec<i64>` with manual indexing or `ndarray` crate.
4. **`zip + sum` for dot product**: `a.iter().zip(b.iter()).map(|(x, y)| x * y).sum()` is the idiomatic Rust dot product. OCaml's `List.fold_left2 (+) 0 row col` is equivalent.

## Exercises

1. **Strassen's algorithm**: Implement Strassen's O(n^2.807) matrix multiplication for 2x2 matrices and verify it produces the same result as naive multiplication.
2. **Identity check**: Write `is_identity(m: &[Vec<i64>]) -> bool` that checks if a matrix is the identity matrix. What are the conditions?
3. **Matrix power**: Write `mat_pow(m: &[Vec<i64>], n: u32) -> Vec<Vec<i64>>` using fast exponentiation (squaring). Verify `mat_pow([[1,1],[1,0]], n)` gives Fibonacci numbers.
