#![allow(clippy::all)]
// 976: Matrix Multiplication
// Naive O(n³) and Strassen 2x2 demo
// OCaml: list-of-lists (functional) + arrays; Rust: Vec<Vec<f64>>

// Approach 1: Vec<Vec<f64>> naive multiply
pub fn mat_multiply(a: &[Vec<f64>], b: &[Vec<f64>]) -> Vec<Vec<f64>> {
    let n = a.len();
    let m = b[0].len();
    let k = b.len();
    assert_eq!(a[0].len(), k, "dimension mismatch");

    let mut result = vec![vec![0.0f64; m]; n];
    for i in 0..n {
        for j in 0..m {
            for l in 0..k {
                result[i][j] += a[i][l] * b[l][j];
            }
        }
    }
    result
}

// Transpose a matrix
pub fn transpose(m: &[Vec<f64>]) -> Vec<Vec<f64>> {
    if m.is_empty() {
        return vec![];
    }
    let rows = m.len();
    let cols = m[0].len();
    let mut t = vec![vec![0.0f64; rows]; cols];
    for i in 0..rows {
        for j in 0..cols {
            t[j][i] = m[i][j];
        }
    }
    t
}

// Approach 2: Dot-product style (cache-friendly via transpose)
pub fn mat_multiply_transposed(a: &[Vec<f64>], b: &[Vec<f64>]) -> Vec<Vec<f64>> {
    let n = a.len();
    let m = b[0].len();
    let bt = transpose(b);

    let mut result = vec![vec![0.0f64; m]; n];
    for i in 0..n {
        for j in 0..m {
            result[i][j] = a[i].iter().zip(&bt[j]).map(|(x, y)| x * y).sum();
        }
    }
    result
}

// Approach 3: Strassen 2x2 (demonstrates the 7-multiply algorithm)
// Real Strassen: recursively split into n/2 x n/2 blocks
pub fn strassen_2x2(a: &[[f64; 2]; 2], b: &[[f64; 2]; 2]) -> [[f64; 2]; 2] {
    let (a11, a12, a21, a22) = (a[0][0], a[0][1], a[1][0], a[1][1]);
    let (b11, b12, b21, b22) = (b[0][0], b[0][1], b[1][0], b[1][1]);

    let m1 = (a11 + a22) * (b11 + b22);
    let m2 = (a21 + a22) * b11;
    let m3 = a11 * (b12 - b22);
    let m4 = a22 * (b21 - b11);
    let m5 = (a11 + a12) * b22;
    let m6 = (a21 - a11) * (b11 + b12);
    let m7 = (a12 - a22) * (b21 + b22);

    [[m1 + m4 - m5 + m7, m3 + m5], [m2 + m4, m1 - m2 + m3 + m6]]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2x2_multiply() {
        let a = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
        let b = vec![vec![5.0, 6.0], vec![7.0, 8.0]];
        let c = mat_multiply(&a, &b);
        assert_eq!(c[0][0], 19.0);
        assert_eq!(c[0][1], 22.0);
        assert_eq!(c[1][0], 43.0);
        assert_eq!(c[1][1], 50.0);
    }

    #[test]
    fn test_non_square() {
        let m23 = vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]];
        let m32 = vec![vec![7.0, 8.0], vec![9.0, 10.0], vec![11.0, 12.0]];
        let result = mat_multiply(&m23, &m32);
        assert_eq!(result[0][0], 58.0);
        assert_eq!(result[0][1], 64.0);
        assert_eq!(result[1][0], 139.0);
        assert_eq!(result[1][1], 154.0);
    }

    #[test]
    fn test_transposed_matches_naive() {
        let a = vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]];
        let b = vec![vec![7.0, 8.0], vec![9.0, 10.0], vec![11.0, 12.0]];
        let naive = mat_multiply(&a, &b);
        let transposed = mat_multiply_transposed(&a, &b);
        assert_eq!(naive, transposed);
    }

    #[test]
    fn test_strassen_2x2() {
        let a = [[1.0, 2.0], [3.0, 4.0]];
        let b = [[5.0, 6.0], [7.0, 8.0]];
        let c = strassen_2x2(&a, &b);
        assert_eq!(c[0][0], 19.0);
        assert_eq!(c[0][1], 22.0);
        assert_eq!(c[1][0], 43.0);
        assert_eq!(c[1][1], 50.0);
    }

    #[test]
    fn test_identity() {
        let a = vec![vec![3.0, 4.0], vec![5.0, 6.0]];
        let identity = vec![vec![1.0, 0.0], vec![0.0, 1.0]];
        let result = mat_multiply(&a, &identity);
        assert_eq!(result, a);
    }
}
