**Difficulty:** ⭐  
**Category:** Functional Programming  

**Difficulty:** ⭐  

[matrix-operations on hightechmind.io](https://hightechmind.io/posts/functional-rust/matrix-operations)

---

## Problem Statement

Implement functional matrix operations — transpose, dot product, matrix multiplication, and scalar scaling — using nested iterators. The goal is to represent matrices as `Vec<Vec<i64>>` and express all operations as iterator pipelines, mirroring OCaml's style of computing on immutable nested lists. Avoid explicit index mutation; derive matrix multiply from transpose and dot product.

## Learning Outcomes

- Represent a 2D matrix as `Vec<Vec<i64>>` (type alias `Matrix`) and iterate over rows and columns
- Implement `transpose` by column extraction: `(0..cols).map(|col| matrix.iter().map(|row| row[col]).collect())`
- Implement `dot` product using `.zip().map().sum()` — no manual index loops
- Derive `multiply(A, B)` as `A.rows × B^T.rows` via nested `map` + `dot`
- Implement scalar `scale` with nested `map`

## Rust Application

```rust
pub type Matrix = Vec<Vec<i64>>;

pub fn transpose(matrix: &Matrix) -> Matrix {
    if matrix.is_empty() || matrix[0].is_empty() {
        return vec![];
    }
    let cols = matrix[0].len();
    (0..cols)
        .map(|col| matrix.iter().map(|row| row[col]).collect())
        .collect()
}

pub fn dot(a: &[i64], b: &[i64]) -> i64 {
    a.iter().zip(b.iter()).map(|(x, y)| x * y).sum()
}

pub fn multiply(a: &Matrix, b: &Matrix) -> Matrix {
    let bt = transpose(b);
    a.iter()
        .map(|row| bt.iter().map(|col| dot(row, col)).collect())
        .collect()
}

pub fn scale(matrix: &Matrix, scalar: i64) -> Matrix {
    matrix
        .iter()
        .map(|row| row.iter().map(|&x| x * scalar).collect())
        .collect()
}
```

The key insight is that `multiply(A, B)` row `i`, column `j` equals `dot(A[i], B^T[j])`. By transposing `B` first, matrix multiplication becomes a double `map` over row pairs — no triple nested loops.

`transpose` uses index-based column extraction: for each column index `col`, map over all rows to collect the elements at that column. This is safe because all rows are assumed to have equal length.

Each operation is pure and returns a fresh `Matrix`. No in-place mutation occurs, directly mirroring the functional style.

## OCaml Approach

```ocaml
let transpose = function
  | [] -> []
  | [] :: _ -> []
  | rows ->
    let ncols = List.length (List.hd rows) in
    List.init ncols (fun col ->
      List.map (fun row -> List.nth row col) rows)

let dot a b =
  List.fold_left2 (fun acc x y -> acc + x * y) 0 a b

let multiply a b =
  let bt = transpose b in
  List.map (fun row ->
    List.map (fun col -> dot row col) bt
  ) a

let scale s m =
  List.map (List.map (( * ) s)) m
```

OCaml's `List.map (List.map ...)` idiom is extremely concise for nested operations. The `scale` function using `(( * ) s)` is a partial application of multiplication — even more compact than Rust's closure syntax.

## Key Differences

| Aspect | Rust | OCaml |
|--------|------|-------|
| Matrix type | `Vec<Vec<i64>>` — heap-allocated, O(1) index access | `int list list` — linked lists, O(n) `List.nth` |
| Column extraction | `row[col]` — O(1) | `List.nth row col` — O(n) |
| Nested map | `.map(|row| .map(|col| ...))` | `List.map (List.map ...)` |
| Partial application | Closure required: `move |x| x * s` | `(( * ) s)` — natural currying |
| Performance | Cache-friendly for row-major access | Poor cache behavior due to linked list pointers |

For performance-critical matrix work, `ndarray` or `nalgebra` crates are preferred over `Vec<Vec<T>>`. The functional style shown here prioritizes clarity and correctness over performance.

## Exercises

1. Add `add_matrices(a, b)` that element-wise sums two matrices of equal dimensions.
2. Implement `identity_matrix(n)` that generates an n×n identity matrix using `(0..n).map(...)`.
3. Write a matrix power function `pow(m, n)` using repeated multiplication.
4. Implement `trace(m)` — the sum of diagonal elements — using `enumerate`.
5. Verify that `multiply(A, identity(n)) == A` and `transpose(transpose(A)) == A` in tests.
