# OCaml vs Rust: Cache-Friendly Iteration and Data Access Patterns

## Side-by-Side Code

### OCaml — flat matrix, row-major sum

```ocaml
type matrix = { data: float array; rows: int; cols: int }

let make_matrix rows cols init =
  { data = Array.init (rows * cols) init; rows; cols }

let get m r c = m.data.(r * m.cols + c)

(* Row-major: sequential — cache-friendly *)
let sum_row_major m = Array.fold_left (+.) 0.0 m.data

(* Column-major: stride = cols — cache-unfriendly *)
let sum_col_major m =
  let acc = ref 0.0 in
  for c = 0 to m.cols - 1 do
    for r = 0 to m.rows - 1 do
      acc := !acc +. get m r c
    done
  done;
  !acc
```

### Rust — flat matrix, row-major sum (idiomatic)

```rust
pub struct Matrix {
    data: Vec<f32>,
    pub rows: usize,
    pub cols: usize,
}

impl Matrix {
    // Row-major sum: iterates the backing Vec sequentially — optimal
    pub fn sum_row_major(&self) -> f32 {
        self.data.iter().sum()
    }

    // Column-major: stride = cols * sizeof(f32) bytes — cache-unfriendly
    pub fn sum_col_major(&self) -> f32 {
        (0..self.cols)
            .flat_map(|c| (0..self.rows).map(move |r| (r, c)))
            .map(|(r, c)| self.get(r, c))
            .sum()
    }
}
```

### Rust — tiled transpose (cache-friendly write path)

```rust
pub fn transpose_tiled(&self, tile: usize) -> Self {
    let mut out = Self::new(self.cols, self.rows, 0.0);
    for r_base in (0..self.rows).step_by(tile) {
        for c_base in (0..self.cols).step_by(tile) {
            let r_end = (r_base + tile).min(self.rows);
            let c_end = (c_base + tile).min(self.cols);
            for r in r_base..r_end {
                for c in c_base..c_end {
                    out.set(c, r, self.get(r, c));
                }
            }
        }
    }
    out
}
```

### Rust — Structure of Arrays (SoA)

```rust
pub struct Vec3SoA {
    pub xs: Vec<f32>,
    pub ys: Vec<f32>,
    pub zs: Vec<f32>,
}

impl Vec3SoA {
    // Only xs are loaded into cache — ys and zs never touched
    pub fn sum_x(&self) -> f32 { self.xs.iter().sum() }

    // Three sequential passes, each over one contiguous slice
    pub fn magnitudes_sq(&self) -> Vec<f32> {
        self.xs.iter().zip(self.ys.iter()).zip(self.zs.iter())
            .map(|((x, y), z)| x * x + y * y + z * z)
            .collect()
    }
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Flat matrix | `{ data: float array; rows: int; cols: int }` | `struct Matrix { data: Vec<f32>, rows: usize, cols: usize }` |
| Element access | `m.data.(r * m.cols + c)` | `self.data[r * self.cols + c]` |
| Row-major sum | `Array.fold_left (+.) 0.0 m.data` | `self.data.iter().sum()` |
| Prefix scan | `Array.fold_left` with accumulator | `Iterator::scan` |
| Stride type | `int` (unboxed int) | `usize` (pointer-sized) |

## Key Insights

1. **Memory layout is identical**: OCaml `float array` and Rust `Vec<f32>` both store elements contiguously in a single heap allocation. The row-major index formula `r * cols + c` is the same in both languages.

2. **Cache line arithmetic is language-agnostic**: A cache line is 64 bytes = 16 `f32`s regardless of language. Row-major iteration is fast in both OCaml and Rust because it is sequential — the hardware prefetcher loads the next cache line speculatively. Column-major access jumps `cols * 4` bytes between elements, causing a cache miss on each access once the matrix exceeds L1 capacity (~32 KB).

3. **`float array array` vs flat `Vec`**: OCaml's nested array `float array array` is an array of *pointers* to individual row allocations — two pointer dereferences per access and potentially non-contiguous rows. Rust's `Vec<Vec<f32>>` has the same problem. The flat `Vec<f32>` with manual index arithmetic eliminates the extra indirection entirely.

4. **Tiling (blocking) beats algorithmic cleverness**: A naïve transpose iterates input rows sequentially but writes output columns — cache-unfriendly on writes. Tiling `TILE × TILE` sub-blocks ensures both the read tile and the write tile fit in L1 cache simultaneously, maintaining ~L1 bandwidth for both sides. The optimal tile size is `sqrt(L1_size / sizeof(element))`.

5. **Structure of Arrays (SoA) vs Array of Structures (AoS)**: When only one field of a record is needed in a hot loop, SoA layouts load only the relevant `Vec` into cache. AoS forces the CPU to load all fields even if only one is used, wasting cache bandwidth by a factor of `struct_size / field_size`.

6. **`Iterator::scan` is the idiomatic prefix-sum**: OCaml uses `Array.fold_left` with a `ref` accumulator; Rust's `scan` threads the state through the iterator chain without mutation at the call site. Both produce sequential memory access — the access pattern is what matters for cache performance, not the syntactic form.

7. **Auto-vectorisation requires sequential layout**: The LLVM backend can emit SIMD instructions (AVX2 processes 8 `f32`s per cycle) only when the data is contiguous and the access stride is 1. `data.iter().sum()` meets this condition; `sum_col_major` does not, so it cannot be vectorised.

## When to Use Each Style

**Use row-major iteration** when processing all elements or all elements of a row — this is the fast path and should be the default for any flat matrix.

**Use tiled transposition** when you must transpose a large matrix and performance matters; the tile size should be tuned to the target machine's L1 cache size (typically 4–16 elements per dimension for `f32`).

**Use SoA layout** when hot loops operate on a subset of fields from a collection of records (e.g., physics simulation updating only positions, or a renderer reading only normals).

**Use gather/scatter sparingly** — random-access patterns defeat the hardware prefetcher. Where possible, sort work by memory address or use sequential layouts instead.
