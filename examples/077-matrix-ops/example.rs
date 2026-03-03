/// Matrix operations using Vec<Vec<i64>>
///
/// Ownership insight: Matrices are heap-allocated Vec<Vec<i64>>.
/// Transpose and multiply consume or borrow the input matrices.

/// Transpose a matrix — takes ownership, returns new matrix
pub fn transpose(matrix: Vec<Vec<i64>>) -> Vec<Vec<i64>> {
    if matrix.is_empty() || matrix[0].is_empty() {
        return vec![];
    }
    let rows = matrix.len();
    let cols = matrix[0].len();
    (0..cols)
        .map(|c| (0..rows).map(|r| matrix[r][c]).collect())
        .collect()
}

/// Transpose by reference — borrows input
pub fn transpose_ref(matrix: &[Vec<i64>]) -> Vec<Vec<i64>> {
    if matrix.is_empty() || matrix[0].is_empty() {
        return vec![];
    }
    let cols = matrix[0].len();
    (0..cols)
        .map(|c| matrix.iter().map(|row| row[c]).collect())
        .collect()
}

/// Dot product of two slices
fn dot(a: &[i64], b: &[i64]) -> i64 {
    a.iter().zip(b.iter()).map(|(x, y)| x * y).sum()
}

/// Matrix multiplication
pub fn multiply(a: &[Vec<i64>], b: &[Vec<i64>]) -> Vec<Vec<i64>> {
    let bt = transpose_ref(b);
    a.iter()
        .map(|row| bt.iter().map(|col| dot(row, col)).collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transpose() {
        let m = vec![vec![1, 2, 3], vec![4, 5, 6]];
        assert_eq!(transpose(m), vec![vec![1, 4], vec![2, 5], vec![3, 6]]);
    }

    #[test]
    fn test_transpose_empty() {
        let m: Vec<Vec<i64>> = vec![];
        assert_eq!(transpose(m), Vec::<Vec<i64>>::new());
    }

    #[test]
    fn test_multiply() {
        let a = vec![vec![1, 2, 3], vec![4, 5, 6]];
        let b = vec![vec![7, 8], vec![9, 10], vec![11, 12]];
        assert_eq!(multiply(&a, &b), vec![vec![58, 64], vec![139, 154]]);
    }

    #[test]
    fn test_transpose_ref() {
        let m = vec![vec![1, 2], vec![3, 4]];
        assert_eq!(transpose_ref(&m), vec![vec![1, 3], vec![2, 4]]);
    }

    #[test]
    fn test_identity_multiply() {
        let a = vec![vec![1, 0], vec![0, 1]];
        let b = vec![vec![5, 6], vec![7, 8]];
        assert_eq!(multiply(&a, &b), b);
    }
}

fn main() {
    println!("{:?}", transpose(m), vec![vec![1, 4], vec![2, 5], vec![3, 6]]);
    println!("{:?}", transpose(m), Vec::<Vec<i64>>::new());
    println!("{:?}", multiply(&a, &b), vec![vec![58, 64], vec![139, 154]]);
}
