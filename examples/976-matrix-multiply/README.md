[matrix-multiply on hightechmind.io](https://hightechmind.io/posts/functional-rust/matrix-multiply)

---

## Problem Statement

Implement matrix multiplication in two styles: a naive O(n³) triple-loop version and a cache-friendly dot-product style using transpose. Also implement a 2×2 Strassen demo. Compare row-major access patterns and their effect on cache performance, and contrast with OCaml's list-of-lists functional approach.

## Learning Outcomes

- Implement naive `mat_multiply(a, b) -> Vec<Vec<f64>>` with triple nested loops
- Implement cache-friendly multiply via `transpose(b)` then row-dot-row access
- Understand why transposing `b` improves cache performance: sequential row access in both operands
- Implement `transpose` with pre-allocated output matrix and index-swap
- Recognize that O(n³) is the practical complexity for dense matrices; Strassen achieves O(n^2.807) theoretically

## Rust Application

```rust
pub fn mat_multiply(a: &[Vec<f64>], b: &[Vec<f64>]) -> Vec<Vec<f64>> {
    let (n, m, k) = (a.len(), b[0].len(), b.len());
    assert_eq!(a[0].len(), k);
    let mut result = vec![vec![0.0f64; m]; n];
    for i in 0..n {
        for j in 0..m {
            for l in 0..k {
                result[i][j] += a[i][l] * b[l][j];  // cache-unfriendly b column access
            }
        }
    }
    result
}

pub fn transpose(m: &[Vec<f64>]) -> Vec<Vec<f64>> {
    if m.is_empty() { return vec![]; }
    let (rows, cols) = (m.len(), m[0].len());
    let mut t = vec![vec![0.0f64; rows]; cols];
    for i in 0..rows {
        for j in 0..cols {
            t[j][i] = m[i][j];
        }
    }
    t
}

// Cache-friendly: transpose b first, then row-dot-row
pub fn mat_multiply_dotprod(a: &[Vec<f64>], b: &[Vec<f64>]) -> Vec<Vec<f64>> {
    let bt = transpose(b);
    a.iter()
        .map(|row_a| {
            bt.iter()
                .map(|row_bt| row_a.iter().zip(row_bt).map(|(x, y)| x * y).sum())
                .collect()
        })
        .collect()
}
```

In the naive version, `b[l][j]` accesses column `j` of `b` — jumping by `m * sizeof(f64)` bytes per step. This is cache-unfriendly for large matrices. After transposing `b`, `bt[j][l]` is a row access — sequential memory, cache-friendly.

The dot-product style uses nested `map` + `zip` + `sum()` — idiomatic functional style that the compiler can potentially vectorize.

## OCaml Approach

```ocaml
let mat_multiply a b =
  let bt = transpose b in
  List.map (fun row_a ->
    List.map (fun row_bt ->
      List.fold_left2 (fun acc x y -> acc +. x *. y) 0.0 row_a row_bt
    ) bt
  ) a
```

OCaml's list-of-lists approach is clean but cache-inefficient — linked lists have poor spatial locality. For performance-critical matrix work, OCaml uses `Bigarray` (C-backed arrays) or the `owl` library.

## Key Differences

| Aspect | Rust | OCaml |
|--------|------|-------|
| Matrix type | `Vec<Vec<f64>>` — heap rows | `float list list` — linked lists |
| Cache behavior | Row access O(1), column access jumps | All access O(n) pointer chasing |
| Functional style | `map/zip/sum` pipeline | `List.map (List.fold_left2 ...)` |
| Performance | `ndarray` for production | `Bigarray` / `owl` for production |

For production matrix operations, use `nalgebra` or `ndarray` crates (Rust) or `owl` / `Bigarray` (OCaml). The `Vec<Vec<f64>>` approach here demonstrates the algorithm, not the optimal data layout.

## Exercises

1. Implement Strassen's algorithm for 2×2 matrices using 7 multiplications instead of 8.
2. Implement `identity_matrix(n: usize) -> Vec<Vec<f64>>` and verify `multiply(A, I) == A`.
3. Add a `mat_add(a, b) -> Vec<Vec<f64>>` function and verify distributivity.
4. Implement block matrix multiplication: split matrices into 2×2 blocks and multiply recursively.
5. Benchmark naive vs transpose-based multiply for 256×256 and 512×512 matrices.
