# Cache-Friendly Iteration
**Difficulty:** ⭐  
**Category:** Functional Programming  


> **Functional Rust** · [hightechmind.io](https://hightechmind.io)

## Problem Statement

A CPU cache operates on 64-byte cache lines. When a program accesses memory
sequentially (stride-1 access), the hardware prefetcher loads ahead and nearly every
access hits cache. When a program accesses memory with large strides—skipping N bytes
between each element—the prefetcher cannot keep up and every access is a cache miss
costing 100–300 cycles on modern hardware.

Matrix transposition is the canonical example: iterating row-major over a row-major
matrix is fast; iterating column-major is slow because each column element is separated
by `cols * sizeof(element)` bytes. For a 1024×1024 float matrix, column-major access
produces ~1M cache misses vs ~16K for row-major—a 60× difference. Tiled (blocked)
transposition trades the miss pattern by fitting tiles into L1/L2 cache, enabling
sequential reads and writes within each tile.

## Learning Outcomes

- Explain the relationship between access stride, cache line size, and miss rate
- Implement row-major vs column-major matrix iteration and measure the gap
- Apply loop tiling/blocking to fit working sets into cache
- Use `cargo bench` / `criterion` to attribute performance differences to memory access
- Reason about the interaction between auto-vectorization and access patterns

## Rust Application

```rust
const N: usize = 512;

struct Matrix {
    data: Vec<f64>,
    rows: usize,
    cols: usize,
}

impl Matrix {
    fn get(&self, r: usize, c: usize) -> f64 {
        self.data[r * self.cols + c]   // row-major storage
    }

    // Fast: sequential reads, sequential writes
    fn sum_row_major(&self) -> f64 {
        self.data.iter().sum()
    }

    // Slow: stride = cols between reads, 1 between writes
    fn transpose_naive(&self) -> Matrix {
        let mut out = vec![0.0; self.rows * self.cols];
        for r in 0..self.rows {
            for c in 0..self.cols {
                out[c * self.rows + r] = self.get(r, c);
            }
        }
        Matrix { data: out, rows: self.cols, cols: self.rows }
    }

    // Fast: tiled — each tile fits in L1 cache
    fn transpose_tiled(&self, tile: usize) -> Matrix {
        let mut out = vec![0.0; self.rows * self.cols];
        for r in (0..self.rows).step_by(tile) {
            for c in (0..self.cols).step_by(tile) {
                for i in r..(r + tile).min(self.rows) {
                    for j in c..(c + tile).min(self.cols) {
                        out[j * self.rows + i] = self.get(i, j);
                    }
                }
            }
        }
        Matrix { data: out, rows: self.cols, cols: self.rows }
    }
}
```

The tiled version keeps a `tile × tile` sub-matrix in L1 cache during the inner loop,
converting random column writes into sequential tile writes.

## OCaml Approach

OCaml arrays are flat and unboxed for `float array`, so the access pattern issues are
identical. The standard library offers `Bigarray` for C-layout (row-major) or
Fortran-layout (column-major) control:

```ocaml
open Bigarray

let make_matrix rows cols =
  Array2.create float64 c_layout rows cols

(* Row-major: inner loop over columns — cache-friendly *)
let sum_row_major mat =
  let rows = Array2.dim1 mat and cols = Array2.dim2 mat in
  let acc = ref 0.0 in
  for r = 0 to rows - 1 do
    for c = 0 to cols - 1 do
      acc := !acc +. mat.{r, c}
    done
  done;
  !acc
```

OCaml's optimizer does not auto-vectorize as aggressively as LLVM/Clang; SIMD
benefits require manual `Bigarray` with `Lacaml` or `Owl`.

## Key Differences

| Aspect | Rust | OCaml |
|---|---|---|
| Float array layout | Unboxed `Vec<f64>`, row-major | Flat `float array` or `Bigarray` |
| Auto-vectorization | LLVM vectorizes stride-1 loops | Limited; needs `Bigarray` + hints |
| Tiling strategy | Nested loops with `step_by` | Same approach, no stdlib support |
| Cache profiling | `perf stat -e cache-misses` | Same external tools |
| Abstraction cost | Zero-cost iterators | Closure calls add minor overhead |

Both languages expose identical cache-miss pathologies. Rust's LLVM backend is more
likely to auto-vectorize the tiled inner loop.

## Exercises

1. Benchmark naive vs tiled transposition for 256×256, 512×512, and 1024×1024 matrices
   with `criterion`. Plot cache-miss counts using `perf stat`.
2. Experiment with tile sizes (8, 16, 32, 64) and find the optimal size for your CPU's
   L1 cache (check `getconf LEVEL1_DCACHE_SIZE`).
3. Implement a generic `TiledIter<T>` that yields `(row_start, col_start, tile_data)`
   for any 2D slice, abstracting the tiling logic.
4. Apply tiling to sparse matrix-vector multiplication: given a CSR matrix, tile the
   row-range loop to keep the `x` vector in L2 cache.
5. Compare row-major vs column-major `sum` on a matrix stored as `Vec<Vec<f64>>` vs
   flat `Vec<f64>` to quantify the cost of pointer indirection in addition to stride.
