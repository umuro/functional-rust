//! # Const Type Arithmetic
//!
//! Type-level arithmetic using const generics.

/// Type-level addition result
pub struct Add<const A: usize, const B: usize>;

impl<const A: usize, const B: usize> Add<A, B> {
    pub const VALUE: usize = A + B;
}

/// Type-level multiplication result
pub struct Mul<const A: usize, const B: usize>;

impl<const A: usize, const B: usize> Mul<A, B> {
    pub const VALUE: usize = A * B;
}

/// Vector with compile-time length
#[derive(Debug)]
pub struct Vec3<const N: usize>([f64; N]);

impl<const N: usize> Vec3<N> {
    pub fn new(data: [f64; N]) -> Self {
        Vec3(data)
    }

    pub const fn len(&self) -> usize {
        N
    }

    pub fn get(&self, idx: usize) -> Option<f64> {
        self.0.get(idx).copied()
    }

    pub fn as_slice(&self) -> &[f64] {
        &self.0
    }
}

/// Concatenate two vectors — returns a Vec since { A + B } requires nightly.
pub fn concat_vec<const A: usize, const B: usize>(
    a: &Vec3<A>,
    b: &Vec3<B>,
) -> Vec<f64> {
    let mut result = Vec::with_capacity(A + B);
    result.extend_from_slice(a.as_slice());
    result.extend_from_slice(b.as_slice());
    result
}

/// Matrix dimensions at type level
#[derive(Debug)]
pub struct Matrix<const ROWS: usize, const COLS: usize> {
    data: [[f64; COLS]; ROWS],
}

impl<const ROWS: usize, const COLS: usize> Matrix<ROWS, COLS> {
    pub fn new() -> Self {
        Matrix {
            data: [[0.0; COLS]; ROWS],
        }
    }

    pub fn from_array(data: [[f64; COLS]; ROWS]) -> Self {
        Matrix { data }
    }

    pub const fn rows(&self) -> usize {
        ROWS
    }

    pub const fn cols(&self) -> usize {
        COLS
    }

    pub fn get(&self, row: usize, col: usize) -> Option<f64> {
        self.data.get(row).and_then(|r| r.get(col)).copied()
    }

    pub fn set(&mut self, row: usize, col: usize, val: f64) {
        if row < ROWS && col < COLS {
            self.data[row][col] = val;
        }
    }
}

impl<const ROWS: usize, const COLS: usize> Default for Matrix<ROWS, COLS> {
    fn default() -> Self {
        Self::new()
    }
}

/// Matrix multiplication with dimension checking
pub fn matmul<const M: usize, const N: usize, const P: usize>(
    a: &Matrix<M, N>,
    b: &Matrix<N, P>,
) -> Matrix<M, P> {
    let mut result = Matrix::<M, P>::new();
    for i in 0..M {
        for j in 0..P {
            let mut sum = 0.0;
            for k in 0..N {
                sum += a.data[i][k] * b.data[k][j];
            }
            result.data[i][j] = sum;
        }
    }
    result
}

/// Transpose with dimension swap
pub fn transpose<const M: usize, const N: usize>(a: &Matrix<M, N>) -> Matrix<N, M> {
    let mut result = Matrix::<N, M>::new();
    for i in 0..M {
        for j in 0..N {
            result.data[j][i] = a.data[i][j];
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_type() {
        assert_eq!(Add::<3, 4>::VALUE, 7);
        assert_eq!(Add::<10, 20>::VALUE, 30);
    }

    #[test]
    fn test_mul_type() {
        assert_eq!(Mul::<3, 4>::VALUE, 12);
        assert_eq!(Mul::<5, 6>::VALUE, 30);
    }

    #[test]
    fn test_vec_concat() {
        let a = Vec3::new([1.0, 2.0]);
        let b = Vec3::new([3.0, 4.0, 5.0]);
        let c = concat_vec(&a, &b);
        assert_eq!(c.len(), 5);
        assert_eq!(c[0], 1.0);
        assert_eq!(c[4], 5.0);
    }

    #[test]
    fn test_matrix_dimensions() {
        let m: Matrix<3, 4> = Matrix::new();
        assert_eq!(m.rows(), 3);
        assert_eq!(m.cols(), 4);
    }

    #[test]
    fn test_matmul_dimensions() {
        let a: Matrix<2, 3> = Matrix::new();
        let b: Matrix<3, 4> = Matrix::new();
        let c = matmul(&a, &b);
        assert_eq!(c.rows(), 2);
        assert_eq!(c.cols(), 4);
    }

    #[test]
    fn test_transpose() {
        let mut m: Matrix<2, 3> = Matrix::new();
        m.set(0, 1, 5.0);
        let t = transpose(&m);
        assert_eq!(t.rows(), 3);
        assert_eq!(t.cols(), 2);
        assert_eq!(t.get(1, 0), Some(5.0));
    }

    // Compile-time dimension check
    const _: () = assert!(Add::<3, 4>::VALUE == 7);
    const _: () = assert!(Mul::<3, 4>::VALUE == 12);
}
