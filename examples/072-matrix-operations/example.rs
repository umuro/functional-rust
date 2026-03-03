/// # Matrix Operations — Functional 2D
///
/// Matrix transpose and multiply using nested Vecs.
/// Demonstrates nested iteration patterns and borrowing with 2D structures.

pub type Matrix = Vec<Vec<i64>>;

/// Transpose: swap rows and columns.
/// Uses iterator-based column extraction.
pub fn transpose(matrix: &Matrix) -> Matrix {
    if matrix.is_empty() || matrix[0].is_empty() {
        return vec![];
    }
    let cols = matrix[0].len();
    (0..cols)
        .map(|col| matrix.iter().map(|row| row[col]).collect())
        .collect()
}

/// Dot product of two vectors.
pub fn dot(a: &[i64], b: &[i64]) -> i64 {
    a.iter().zip(b.iter()).map(|(x, y)| x * y).sum()
}

/// Matrix multiplication: A(m×n) × B(n×p) = C(m×p)
pub fn multiply(a: &Matrix, b: &Matrix) -> Matrix {
    let bt = transpose(b);
    a.iter()
        .map(|row| bt.iter().map(|col| dot(row, col)).collect())
        .collect()
}

/// Scalar multiplication
pub fn scale(matrix: &Matrix, scalar: i64) -> Matrix {
    matrix
        .iter()
        .map(|row| row.iter().map(|&x| x * scalar).collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transpose() {
        let m = vec![vec![1, 2, 3], vec![4, 5, 6]];
        assert_eq!(transpose(&m), vec![vec![1, 4], vec![2, 5], vec![3, 6]]);
    }

    #[test]
    fn test_transpose_empty() {
        let m: Matrix = vec![];
        assert_eq!(transpose(&m), vec![] as Matrix);
    }

    #[test]
    fn test_dot() {
        assert_eq!(dot(&[1, 2, 3], &[4, 5, 6]), 32);
    }

    #[test]
    fn test_multiply() {
        let a = vec![vec![1, 2, 3], vec![4, 5, 6]];
        let b = vec![vec![7, 8], vec![9, 10], vec![11, 12]];
        assert_eq!(multiply(&a, &b), vec![vec![58, 64], vec![139, 154]]);
    }

    #[test]
    fn test_scale() {
        let m = vec![vec![1, 2], vec![3, 4]];
        assert_eq!(scale(&m, 3), vec![vec![3, 6], vec![9, 12]]);
    }

    #[test]
    fn test_identity_multiply() {
        let a = vec![vec![1, 2], vec![3, 4]];
        let id = vec![vec![1, 0], vec![0, 1]];
        assert_eq!(multiply(&a, &id), a);
    }
}

fn main() {
    println!("{:?}", transpose(&m), vec![vec![1, 4], vec![2, 5], vec![3, 6]]);
    println!("{:?}", transpose(&m), vec![] as Matrix);
    println!("{:?}", dot(&[1, 2, 3], &[4, 5, 6]), 32);
}
