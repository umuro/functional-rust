# Matrix Multiply — Comparison

## Core Insight
Matrix multiplication is O(n³) naive, O(n^2.807) Strassen. The triple loop `for i, j, k: C[i][j] += A[i][k] * B[k][j]` is identical in both languages. OCaml's functional list-of-lists approach is readable but slow; the array approach matches Rust's performance. Transposing B before multiplication improves cache locality (column access becomes row access).

## OCaml Approach
- `[[1.0;2.0];[3.0;4.0]]` — list of float lists (functional, poor cache)
- `List.init cols (fun c -> List.map (fun row -> List.nth row c) m)` — transpose
- `List.fold_left2` for dot product
- `Array.make_matrix n m 0.0` for imperative approach
- `for i = 0 to n-1 do ... done` — triple nested imperative loop
- `!s +. a.(i).(l) *. b.(l).(j)` — float arithmetic with `.` suffix

## Rust Approach
- `Vec<Vec<f64>>` — row-major, similar memory layout to 2D array
- `vec![vec![0.0f64; m]; n]` — initialize result matrix
- `result[i][j] += a[i][l] * b[l][j]` — clean triple loop
- Transpose via double loop (same algorithm, no magic)
- `.iter().zip(&bt[j]).map(|(x,y)| x*y).sum()` — functional dot product
- `[[f64; 2]; 2]` for fixed-size Strassen (stack-allocated, no allocation)

## Comparison Table

| Aspect | OCaml | Rust |
|--------|-------|------|
| Functional matrix | `float list list` | `Vec<Vec<f64>>` |
| Init result | `Array.make_matrix n m 0.0` | `vec![vec![0.0; m]; n]` |
| Element access | `a.(i).(l)` | `a[i][l]` |
| Float arithmetic | `+.`, `*.` (explicit) | `+`, `*` (same operators) |
| Dot product | `List.fold_left2` | `.zip().map().sum()` |
| Transpose | `List.init cols (fun c -> List.map ...)` | Double loop |
| Fixed 2x2 | `[| [| ... |] |]` | `[[f64; 2]; 2]` (stack-allocated) |
| Strassen | 7 muls, same formula | 7 muls, same formula |
