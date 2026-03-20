📖 **[View on hightechmind.io →](https://hightechmind.io/rust/1057-matrix-chain)**

---

# 1057-matrix-chain — Matrix Chain Multiplication

## Problem Statement

Multiplying a sequence of matrices is associative: `(AB)C = A(BC)`, but the computational cost varies dramatically with parenthesization. Multiplying a 10×30 matrix by a 30×5 matrix by a 5×60 matrix: `(AB)C` costs 10×30×5 + 10×5×60 = 4,500 + 3,000 = 7,500 operations; `A(BC)` costs 30×5×60 + 10×30×60 = 9,000 + 18,000 = 27,000. The optimal ordering can be 10–100× faster for large chains.

Matrix chain ordering is a classic interval DP problem and a fundamental optimization in scientific computing, neural network inference, and linear algebra libraries.

## Learning Outcomes

- Implement matrix chain DP with `dp[i][j]` = minimum cost for matrices i..j
- Understand interval DP: fill by increasing chain length
- Recover the optimal parenthesization using a split table
- Recognize that matrix multiplication associativity enables optimization
- Connect to BLAS/LAPACK and deep learning frameworks that optimize compute graphs

## Rust Application

`src/lib.rs` implements `matrix_chain_dp` with `dims: &[usize]` where matrix `k` has dimensions `dims[k] × dims[k+1]`. `dp[i][j]` stores the minimum cost; the outer loop increases the chain length from 2 to n. `matrix_chain_parens` adds a `split` table that records the optimal split point at each `dp[i][j]`, enabling reconstruction of the optimal parenthesization string.

TensorFlow, PyTorch, and XLA all implement variants of this DP for fusing matrix operations in compute graphs.

## OCaml Approach

```ocaml
let matrix_chain dims =
  let n = Array.length dims - 1 in
  let dp = Array.make_matrix n n 0 in
  for l = 2 to n do
    for i = 0 to n - l do
      let j = i + l - 1 in
      dp.(i).(j) <- max_int;
      for k = i to j - 1 do
        let cost = dp.(i).(k) + dp.(k+1).(j) + dims.(i) * dims.(k+1) * dims.(j+1) in
        if cost < dp.(i).(j) then dp.(i).(j) <- cost
      done
    done
  done;
  dp.(0).(n-1)
```

The algorithm is identical. Interval DP is a mathematical technique with a canonical implementation structure.

## Key Differences

1. **`usize::MAX` vs `max_int`**: Rust uses `usize::MAX` as infinity; OCaml uses `max_int`. Both risk overflow on addition — use careful comparison before adding.
2. **Interval filling order**: Both fill by increasing length `l` — the outer loop determines chain length before the inner loop tries split points.
3. **Reconstruction**: Both build a separate `split` table during DP and recursively decode it to produce a parenthesization string.
4. **Applications**: OCaml ML libraries use this optimization in tensor expression evaluation; Rust ML frameworks like `candle` apply similar optimizations.

## Exercises

1. Add memoized top-down implementation and verify it produces the same answer as the bottom-up version.
2. Implement the reconstruction function `format_chain(split: &Vec<Vec<usize>>, i: usize, j: usize, names: &[&str]) -> String` that produces a parenthesized expression like `"((A×B)×(C×D))"`.
3. Extend to weighted matrix chain where some multiplications have additional overhead (e.g., GPU memory transfer costs).
