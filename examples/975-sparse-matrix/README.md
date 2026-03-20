[sparse-matrix on hightechmind.io](https://hightechmind.io/posts/functional-rust/sparse-matrix)

---

## Problem Statement

Implement a sparse matrix using `HashMap<(usize, usize), f64>` to store only non-zero elements. Operations include get (returns 0.0 for absent entries), set (removes entry on zero assignment), matrix-vector multiply, and transpose. Compare with OCaml's `Hashtbl.Make` approach using a custom key module.

## Learning Outcomes

- Store sparse data in `HashMap<(usize, usize), f64>` — tuple keys are hashable and comparable
- Implement `set` that removes the entry when `v == 0.0` to maintain the sparse invariant
- Implement `get` returning `*data.get(&(r, c)).unwrap_or(&0.0)` for implicit zero
- Implement matrix-vector multiply `mv(vec: &[f64]) -> Vec<f64>` by iterating only non-zero entries
- Implement `transpose` by building a new `SparseMatrix` with swapped row/column indices

## Rust Application

```rust
pub struct SparseMatrix {
    rows: usize,
    cols: usize,
    data: HashMap<(usize, usize), f64>,
}

impl SparseMatrix {
    pub fn new(rows: usize, cols: usize) -> Self {
        SparseMatrix { rows, cols, data: HashMap::new() }
    }

    pub fn set(&mut self, r: usize, c: usize, v: f64) {
        assert!(r < self.rows && c < self.cols);
        if v == 0.0 { self.data.remove(&(r, c)); }
        else { self.data.insert((r, c), v); }
    }

    pub fn get(&self, r: usize, c: usize) -> f64 {
        *self.data.get(&(r, c)).unwrap_or(&0.0)
    }

    pub fn nnz(&self) -> usize { self.data.len() }

    pub fn mv(&self, vec: &[f64]) -> Vec<f64> {
        let mut result = vec![0.0; self.rows];
        for (&(r, c), &v) in &self.data {
            result[r] += v * vec[c];
        }
        result
    }

    pub fn transpose(&self) -> SparseMatrix {
        let mut result = SparseMatrix::new(self.cols, self.rows);
        for (&(r, c), &v) in &self.data {
            result.data.insert((c, r), v);
        }
        result
    }
}
```

`set` removes the entry when `v == 0.0` — maintaining the sparse invariant that absent entries implicitly equal zero. This keeps `nnz()` accurate and `mv` efficient (iterates only stored entries).

`mv` iterates only non-zero entries: for an `m×n` matrix with `k` non-zeros, `mv` runs in O(k + m) instead of O(m*n). This is the fundamental advantage of sparse representation.

## OCaml Approach

```ocaml
module IntPairKey = struct
  type t = int * int
  let compare = compare
  let hash (r, c) = Hashtbl.hash (r lsl 32 lor c)
  let equal a b = a = b
end

module SparseHashtbl = Hashtbl.Make(IntPairKey)

type sparse_matrix = {
  rows: int; cols: int;
  data: float SparseHashtbl.t;
}

let create rows cols =
  { rows; cols; data = SparseHashtbl.create 16 }

let set m r c v =
  if v = 0.0 then SparseHashtbl.remove m.data (r, c)
  else SparseHashtbl.replace m.data (r, c) v

let get m r c =
  try SparseHashtbl.find m.data (r, c) with Not_found -> 0.0

let mv m vec =
  let result = Array.make m.rows 0.0 in
  SparseHashtbl.iter (fun (r, c) v ->
    result.(r) <- result.(r) +. v *. vec.(c)
  ) m.data;
  result
```

OCaml's `Hashtbl.Make(Key)` requires defining a key module with `hash` and `equal`. Rust's `HashMap` automatically uses `Hash` and `Eq` trait implementations on `(usize, usize)` — no extra boilerplate.

## Key Differences

| Aspect | Rust | OCaml |
|--------|------|-------|
| Tuple as key | `(usize, usize)` — `Hash + Eq` auto-derived | Requires `Hashtbl.Make(IntPairKey)` module |
| Default value | `unwrap_or(&0.0)` | `try find ... with Not_found -> 0.0` |
| Sparse iterate | `for (&(r, c), &v) in &self.data` | `Hashtbl.iter (fun (r,c) v -> ...)` |
| Floating-point == | `v == 0.0` (exact zero) | `v = 0.0` (same) |

Comparing floating-point values with `== 0.0` is deliberate here — we only want to remove entries that were explicitly set to zero, not entries that are approximately zero. For numerical stability, use a threshold in production code.

## Exercises

1. Implement `add_matrices(a, b) -> SparseMatrix` that merges two sparse matrices entry-by-entry.
2. Implement `multiply(a: &SparseMatrix, b: &SparseMatrix) -> SparseMatrix` — only compute non-zero products.
3. Add `density(&self) -> f64` returning `nnz / (rows * cols)` — the fraction of non-zero entries.
4. Implement `to_dense(&self) -> Vec<Vec<f64>>` that materializes the full matrix.
5. Implement a CSR (Compressed Sparse Row) format alongside the `HashMap` format and benchmark `mv` for a 1000×1000 matrix with 1% fill.
