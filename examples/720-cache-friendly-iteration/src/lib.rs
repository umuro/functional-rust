//! 720: Cache-Friendly Iteration and Data Access Patterns
//!
//! Demonstrates row-major vs column-major access, tiled transposition,
//! and how sequential iterator chaining produces cache-friendly, auto-vectorisable code.
//!
//! Key insight: a CPU cache line is 64 bytes = 16 f32s. Sequential access lets
//! the hardware prefetcher load the next cache line before you need it.
//! Column-major access on a row-major matrix jumps `cols * 4` bytes between
//! elements — each access is a potential cache miss.

// ── Flat row-major matrix ────────────────────────────────────────────────────

/// A flat, row-major 2-D matrix backed by a single `Vec<f32>`.
///
/// Element `(r, c)` lives at `data[r * cols + c]`, so iterating over a
/// row is sequential — identical to iterating over a slice.
pub struct Matrix {
    data: Vec<f32>,
    pub rows: usize,
    pub cols: usize,
}

impl Matrix {
    /// Allocate a matrix filled with `init`.
    pub fn new(rows: usize, cols: usize, init: f32) -> Self {
        Self {
            data: vec![init; rows * cols],
            rows,
            cols,
        }
    }

    /// Allocate a matrix where element `(r, c)` is computed by `f(r, c)`.
    pub fn from_fn(rows: usize, cols: usize, f: impl Fn(usize, usize) -> f32) -> Self {
        // Sequential nested push: identical memory access order to the flat_map
        // version, but avoids closure ownership issues when `f` is not `Copy`.
        let mut data = Vec::with_capacity(rows * cols);
        for r in 0..rows {
            for c in 0..cols {
                data.push(f(r, c));
            }
        }
        Self { data, rows, cols }
    }

    #[inline]
    pub fn get(&self, r: usize, c: usize) -> f32 {
        self.data[r * self.cols + c]
    }

    #[inline]
    pub fn set(&mut self, r: usize, c: usize, v: f32) {
        self.data[r * self.cols + c] = v;
    }

    /// Sum all elements: iterates the backing `Vec` sequentially — optimal.
    ///
    /// The compiler can auto-vectorise this into SIMD because the memory
    /// layout is contiguous and the access pattern is fully predictable.
    pub fn sum_row_major(&self) -> f32 {
        self.data.iter().sum()
    }

    /// Sum all elements column-by-column (cache-unfriendly reference path).
    ///
    /// Between two successive accesses the stride is `cols * sizeof(f32)` bytes.
    /// On a 1024-column matrix that is 4 096 bytes — 64 cache lines skipped.
    /// Every access is likely a cache miss once the matrix exceeds L1 capacity.
    pub fn sum_col_major(&self) -> f32 {
        (0..self.cols)
            .flat_map(|c| (0..self.rows).map(move |r| (r, c)))
            .map(|(r, c)| self.get(r, c))
            .sum()
    }

    /// Row iterator: yields `&[f32]` slices — zero overhead, sequential.
    pub fn rows_iter(&self) -> impl Iterator<Item = &[f32]> {
        self.data.chunks(self.cols)
    }

    /// Transpose using tiled (blocked) access.
    ///
    /// A naïve transpose writes column-by-column into the output (cache-unfriendly
    /// on writes). Tiling processes `TILE × TILE` sub-blocks that fit in L1 cache,
    /// so both reads and writes stay warm.
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

    /// Naïve (non-tiled) transpose for comparison.
    pub fn transpose_naive(&self) -> Self {
        Self::from_fn(self.cols, self.rows, |r, c| self.get(c, r))
    }
}

// ── Structure of Arrays (SoA) vs Array of Structures (AoS) ──────────────────

/// Array of Structures — common OOP layout, cache-unfriendly for field-only ops.
///
/// If you only need `x` values, you still load `y` and `z` into cache.
pub struct Vec3Aos {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

/// Structure of Arrays — all `x` values are contiguous.
///
/// A loop over `xs` touches only the `x` cache lines; `ys` and `zs` are never
/// loaded. SIMD auto-vectorisation is straightforward because the data is already
/// in the right layout for `_mm256_add_ps` etc.
pub struct Vec3SoA {
    pub xs: Vec<f32>,
    pub ys: Vec<f32>,
    pub zs: Vec<f32>,
}

impl Vec3SoA {
    pub fn new(n: usize) -> Self {
        Self {
            xs: vec![0.0; n],
            ys: vec![0.0; n],
            zs: vec![0.0; n],
        }
    }

    /// Sum only the x-components: touches one contiguous Vec — minimal cache pressure.
    pub fn sum_x(&self) -> f32 {
        self.xs.iter().sum()
    }

    /// Compute squared magnitudes for all vectors.
    ///
    /// Three sequential passes, each over a single contiguous slice.
    /// The compiler can vectorise each `.zip` independently.
    pub fn magnitudes_sq(&self) -> Vec<f32> {
        self.xs
            .iter()
            .zip(self.ys.iter())
            .zip(self.zs.iter())
            .map(|((x, y), z)| x * x + y * y + z * z)
            .collect()
    }
}

// ── Prefetch-friendly sequential scan helpers ────────────────────────────────

/// Running sum over a slice — the canonical sequential scan.
///
/// Produces a prefix-sum array. Sequential read + sequential write:
/// the prefetcher handles both streams perfectly.
pub fn prefix_sum(data: &[f32]) -> Vec<f32> {
    data.iter()
        .scan(0.0_f32, |acc, &x| {
            *acc += x;
            Some(*acc)
        })
        .collect()
}

/// Gather operation (random read): indices may jump anywhere in `src`.
///
/// Classic cache-unfriendly pattern — each `indices[i]` may point to
/// a different cache line in `src`.
pub fn gather(src: &[f32], indices: &[usize]) -> Vec<f32> {
    indices.iter().map(|&i| src[i]).collect()
}

/// Scatter operation (random write): inverse cache problem on the write side.
pub fn scatter(dst: &mut [f32], indices: &[usize], values: &[f32]) {
    indices
        .iter()
        .zip(values.iter())
        .for_each(|(&i, &v)| dst[i] = v);
}

// ─────────────────────────────────────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    // ── Matrix correctness ──────────────────────────────────────────────────

    #[test]
    fn row_and_col_sums_are_equal() {
        // A 4×4 matrix filled with 1.0 — both access orders must give 16.0.
        let m = Matrix::new(4, 4, 1.0);
        assert_eq!(m.sum_row_major(), 16.0);
        assert_eq!(m.sum_col_major(), 16.0);
    }

    #[test]
    fn from_fn_and_get_round_trip() {
        let m = Matrix::from_fn(3, 3, |r, c| (r * 3 + c) as f32);
        assert_eq!(m.get(0, 0), 0.0);
        assert_eq!(m.get(1, 1), 4.0);
        assert_eq!(m.get(2, 2), 8.0);
    }

    #[test]
    fn transpose_tiled_matches_naive() {
        let m = Matrix::from_fn(8, 12, |r, c| (r * 12 + c) as f32);
        let t_naive = m.transpose_naive();
        let t_tiled = m.transpose_tiled(4);

        assert_eq!(t_tiled.rows, 12);
        assert_eq!(t_tiled.cols, 8);

        for r in 0..t_naive.rows {
            for c in 0..t_naive.cols {
                assert_eq!(
                    t_naive.get(r, c),
                    t_tiled.get(r, c),
                    "mismatch at ({r},{c})"
                );
            }
        }
    }

    #[test]
    fn rows_iter_yields_correct_slices() {
        let m = Matrix::from_fn(3, 4, |r, c| (r * 4 + c) as f32);
        let rows: Vec<&[f32]> = m.rows_iter().collect();
        assert_eq!(rows.len(), 3);
        assert_eq!(rows[0], &[0.0, 1.0, 2.0, 3.0]);
        assert_eq!(rows[1], &[4.0, 5.0, 6.0, 7.0]);
        assert_eq!(rows[2], &[8.0, 9.0, 10.0, 11.0]);
    }

    // ── SoA correctness ─────────────────────────────────────────────────────

    #[test]
    fn soa_sum_x_is_correct() {
        let mut soa = Vec3SoA::new(4);
        soa.xs = vec![1.0, 2.0, 3.0, 4.0];
        assert_eq!(soa.sum_x(), 10.0);
    }

    #[test]
    fn soa_magnitudes_sq_unit_vectors() {
        let mut soa = Vec3SoA::new(2);
        soa.xs = vec![1.0, 0.0];
        soa.ys = vec![0.0, 1.0];
        soa.zs = vec![0.0, 0.0];
        let mags = soa.magnitudes_sq();
        assert_eq!(mags, vec![1.0, 1.0]);
    }

    // ── Sequential helpers ──────────────────────────────────────────────────

    #[test]
    fn prefix_sum_basic() {
        let data = vec![1.0, 2.0, 3.0, 4.0];
        let ps = prefix_sum(&data);
        assert_eq!(ps, vec![1.0, 3.0, 6.0, 10.0]);
    }

    #[test]
    fn prefix_sum_empty() {
        assert_eq!(prefix_sum(&[]), Vec::<f32>::new());
    }

    #[test]
    fn gather_and_scatter_round_trip() {
        let src = vec![10.0_f32, 20.0, 30.0, 40.0];
        let indices = vec![3, 1, 0, 2];
        let gathered = gather(&src, &indices);
        assert_eq!(gathered, vec![40.0, 20.0, 10.0, 30.0]);

        let mut dst = vec![0.0_f32; 4];
        scatter(&mut dst, &indices, &gathered);
        // scatter[3]=40, scatter[1]=20, scatter[0]=10, scatter[2]=30
        assert_eq!(dst, vec![10.0, 20.0, 30.0, 40.0]);
    }

    // ── Large-matrix sum consistency (catches index formula bugs) ───────────

    #[test]
    fn large_matrix_row_col_sum_agree() {
        let m = Matrix::from_fn(32, 64, |r, c| (r + c) as f32);
        let row = m.sum_row_major();
        let col = m.sum_col_major();
        assert!(
            (row - col).abs() < 1e-3,
            "row={row} col={col} differ by more than epsilon"
        );
    }
}
