#![allow(clippy::all)]
// Example 126: Const Generics
// Rust's const generics encode array sizes and matrix dimensions in the type,
// turning dimension mismatches into compile errors rather than runtime panics.

// ── Approach 1: Fixed-size array wrapper ─────────────────────────────────────
// The const generic N is part of the type, so `FixedArray<f64, 3>` and
// `FixedArray<f64, 4>` are distinct, incompatible types.

#[derive(Debug, Clone, PartialEq)]
pub struct FixedArray<T, const N: usize> {
    data: [T; N],
}

impl<T: Default + Copy, const N: usize> FixedArray<T, N> {
    pub fn new() -> Self {
        FixedArray {
            data: [T::default(); N],
        }
    }

    pub fn from_array(data: [T; N]) -> Self {
        FixedArray { data }
    }

    pub fn len(&self) -> usize {
        N
    }

    pub fn is_empty(&self) -> bool {
        N == 0
    }

    pub fn get(&self, i: usize) -> Option<&T> {
        self.data.get(i)
    }

    pub fn set(&mut self, i: usize, val: T) {
        if i < N {
            self.data[i] = val;
        }
    }

    pub fn map<U: Default + Copy, F: Fn(T) -> U>(&self, f: F) -> FixedArray<U, N> {
        let mut result = [U::default(); N];
        for (dst, &src) in result.iter_mut().zip(self.data.iter()) {
            *dst = f(src);
        }
        FixedArray { data: result }
    }
}

impl<T: Default + Copy, const N: usize> Default for FixedArray<T, N> {
    fn default() -> Self {
        Self::new()
    }
}

// Dot product — only defined when both arrays have the same N.
// Passing arrays of different sizes is a compile error.
pub fn dot<const N: usize>(a: &FixedArray<f64, N>, b: &FixedArray<f64, N>) -> f64 {
    a.data.iter().zip(b.data.iter()).map(|(x, y)| x * y).sum()
}

// ── Approach 2: Matrix with compile-time dimensions ───────────────────────────
// Matrix<ROWS, COLS> stores data in row-major order.
// Multiplication Matrix<M,K> × Matrix<K,N> → Matrix<M,N> is type-checked:
// the shared inner dimension K must match.

#[derive(Debug, Clone, PartialEq)]
pub struct Matrix<const ROWS: usize, const COLS: usize> {
    data: [[f64; COLS]; ROWS],
}

impl<const ROWS: usize, const COLS: usize> Matrix<ROWS, COLS> {
    pub fn zeros() -> Self {
        Matrix {
            data: [[0.0; COLS]; ROWS],
        }
    }

    pub fn from_array(data: [[f64; COLS]; ROWS]) -> Self {
        Matrix { data }
    }

    pub fn get(&self, row: usize, col: usize) -> f64 {
        self.data[row][col]
    }

    pub fn set(&mut self, row: usize, col: usize, val: f64) {
        self.data[row][col] = val;
    }

    pub fn rows(&self) -> usize {
        ROWS
    }

    pub fn cols(&self) -> usize {
        COLS
    }

    // Transpose: Matrix<ROWS,COLS> → Matrix<COLS,ROWS>
    pub fn transpose(&self) -> Matrix<COLS, ROWS> {
        let mut result = Matrix::<COLS, ROWS>::zeros();
        for r in 0..ROWS {
            for c in 0..COLS {
                result.data[c][r] = self.data[r][c];
            }
        }
        result
    }
}

// Matrix multiplication: (M×K) × (K×N) → (M×N)
// The shared dimension K is the same type variable — enforced at compile time.
pub fn matmul<const M: usize, const K: usize, const N: usize>(
    a: &Matrix<M, K>,
    b: &Matrix<K, N>,
) -> Matrix<M, N> {
    let mut result = Matrix::<M, N>::zeros();
    for i in 0..M {
        for j in 0..N {
            result.data[i][j] = (0..K).map(|k| a.data[i][k] * b.data[k][j]).sum();
        }
    }
    result
}

// ── Approach 3: Const generic in a function ───────────────────────────────────
// A stack-allocated chunk function — size is a compile-time constant.
pub fn chunks_fixed<T: Copy + Default, const CHUNK: usize>(slice: &[T]) -> Vec<[T; CHUNK]> {
    slice
        .chunks(CHUNK)
        .filter(|c| c.len() == CHUNK)
        .map(|c| {
            let mut arr = [T::default(); CHUNK];
            arr.copy_from_slice(c);
            arr
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── FixedArray tests ──────────────────────────────────────────────────────

    #[test]
    fn test_fixed_array_new_and_len() {
        let arr = FixedArray::<i32, 5>::new();
        assert_eq!(arr.len(), 5);
        assert!(!arr.is_empty());
    }

    #[test]
    fn test_fixed_array_get_set() {
        let mut arr = FixedArray::<i32, 3>::new();
        arr.set(1, 42);
        assert_eq!(arr.get(0), Some(&0));
        assert_eq!(arr.get(1), Some(&42));
        assert_eq!(arr.get(3), None); // out of bounds
    }

    #[test]
    fn test_fixed_array_from_array() {
        let arr = FixedArray::from_array([1, 2, 3]);
        assert_eq!(arr.get(0), Some(&1));
        assert_eq!(arr.get(2), Some(&3));
    }

    #[test]
    fn test_fixed_array_map() {
        let arr = FixedArray::from_array([1.0_f64, 2.0, 3.0]);
        let doubled = arr.map(|x| x * 2.0);
        assert_eq!(doubled, FixedArray::from_array([2.0, 4.0, 6.0]));
    }

    // ── Dot product tests ─────────────────────────────────────────────────────

    #[test]
    fn test_dot_product_basic() {
        let a = FixedArray::from_array([1.0_f64, 2.0, 3.0]);
        let b = FixedArray::from_array([4.0_f64, 5.0, 6.0]);
        // 1*4 + 2*5 + 3*6 = 4 + 10 + 18 = 32
        assert!((dot(&a, &b) - 32.0).abs() < 1e-10);
    }

    #[test]
    fn test_dot_product_zero_vector() {
        let a = FixedArray::from_array([0.0_f64, 0.0, 0.0]);
        let b = FixedArray::from_array([1.0_f64, 2.0, 3.0]);
        assert!((dot(&a, &b)).abs() < 1e-10);
    }

    #[test]
    fn test_dot_product_unit_vectors() {
        let a = FixedArray::from_array([1.0_f64, 0.0]);
        let b = FixedArray::from_array([0.0_f64, 1.0]);
        assert!((dot(&a, &b)).abs() < 1e-10); // orthogonal
    }

    // ── Matrix tests ──────────────────────────────────────────────────────────

    #[test]
    fn test_matrix_zeros() {
        let m = Matrix::<2, 3>::zeros();
        assert_eq!(m.rows(), 2);
        assert_eq!(m.cols(), 3);
        assert_eq!(m.get(0, 0), 0.0);
    }

    #[test]
    fn test_matrix_set_get() {
        let mut m = Matrix::<2, 2>::zeros();
        m.set(0, 1, 7.0);
        assert_eq!(m.get(0, 1), 7.0);
        assert_eq!(m.get(1, 0), 0.0);
    }

    #[test]
    fn test_matrix_transpose() {
        let m = Matrix::from_array([[1.0, 2.0, 3.0], [4.0, 5.0, 6.0]]);
        let t = m.transpose();
        assert_eq!(t.rows(), 3);
        assert_eq!(t.cols(), 2);
        assert_eq!(t.get(0, 0), 1.0);
        assert_eq!(t.get(2, 1), 6.0);
    }

    #[test]
    fn test_matmul_identity() {
        // Identity 2×2
        let id = Matrix::from_array([[1.0, 0.0], [0.0, 1.0]]);
        let m = Matrix::from_array([[3.0, 4.0], [5.0, 6.0]]);
        let result = matmul(&id, &m);
        assert_eq!(result.get(0, 0), 3.0);
        assert_eq!(result.get(1, 1), 6.0);
    }

    #[test]
    fn test_matmul_2x3_times_3x2() {
        // (2×3) × (3×2) → (2×2)
        let a = Matrix::from_array([[1.0, 2.0, 3.0], [4.0, 5.0, 6.0]]);
        let b = Matrix::from_array([[7.0, 8.0], [9.0, 10.0], [11.0, 12.0]]);
        let c = matmul(&a, &b);
        // Row 0: [1*7+2*9+3*11, 1*8+2*10+3*12] = [58, 64]
        // Row 1: [4*7+5*9+6*11, 4*8+5*10+6*12] = [139, 154]
        assert!((c.get(0, 0) - 58.0).abs() < 1e-10);
        assert!((c.get(0, 1) - 64.0).abs() < 1e-10);
        assert!((c.get(1, 0) - 139.0).abs() < 1e-10);
        assert!((c.get(1, 1) - 154.0).abs() < 1e-10);
    }

    // ── chunks_fixed tests ────────────────────────────────────────────────────

    #[test]
    fn test_chunks_fixed_even() {
        let data = [1, 2, 3, 4, 5, 6];
        let result = chunks_fixed::<i32, 2>(&data);
        assert_eq!(result, vec![[1, 2], [3, 4], [5, 6]]);
    }

    #[test]
    fn test_chunks_fixed_trailing_drop() {
        // last chunk is incomplete — dropped
        let data = [1, 2, 3, 4, 5];
        let result = chunks_fixed::<i32, 2>(&data);
        assert_eq!(result, vec![[1, 2], [3, 4]]);
    }

    #[test]
    fn test_chunks_fixed_empty() {
        let data: [i32; 0] = [];
        let result = chunks_fixed::<i32, 3>(&data);
        assert!(result.is_empty());
    }
}
