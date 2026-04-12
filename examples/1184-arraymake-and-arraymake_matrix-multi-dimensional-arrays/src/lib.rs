#![allow(dead_code)]
//! Array.make and Array.make_matrix — Multi-Dimensional Arrays
//! See example.ml for OCaml reference
//!
//! OCaml's `Array.make n x` creates a 1D array of `n` copies of `x`.
//! OCaml's `Array.make_matrix rows cols x` creates a 2D array (array of arrays).
//! Rust uses `vec![x; n]` and `Vec<Vec<T>>` respectively.

/// Create a 1D vector of `n` copies of `value`.
/// Mirrors OCaml: `Array.make n value`
pub fn make<T: Clone>(n: usize, value: T) -> Vec<T> {
    vec![value; n]
}

/// Create a 2D matrix of `rows` × `cols` with every cell set to `value`.
/// Mirrors OCaml: `Array.make_matrix rows cols value`
/// Each row is an independent Vec — mutating one row does not affect others.
pub fn make_matrix<T: Clone>(rows: usize, cols: usize, value: T) -> Vec<Vec<T>> {
    vec![vec![value; cols]; rows]
}

/// Set a cell in a 2D matrix. Returns `None` if coordinates are out of bounds.
/// Mirrors OCaml: `matrix.(row).(col) <- new_value`
pub fn matrix_set<T: Clone>(matrix: &mut [Vec<T>], row: usize, col: usize, value: T) -> bool {
    if let Some(r) = matrix.get_mut(row) {
        if let Some(cell) = r.get_mut(col) {
            *cell = value;
            return true;
        }
    }
    false
}

/// Get a cell from a 2D matrix. Returns `None` if out of bounds.
pub fn matrix_get<T>(matrix: &[Vec<T>], row: usize, col: usize) -> Option<&T> {
    matrix.get(row).and_then(|r| r.get(col))
}

/// Create an identity matrix (1s on diagonal, 0s elsewhere).
pub fn identity_matrix(n: usize) -> Vec<Vec<f64>> {
    let mut m = make_matrix(n, n, 0.0_f64);
    for (i, row) in m.iter_mut().enumerate() {
        row[i] = 1.0;
    }
    m
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_zeros() {
        assert_eq!(make(5, 0_i32), vec![0, 0, 0, 0, 0]);
    }

    #[test]
    fn test_make_single_element() {
        assert_eq!(make(1, 42_i32), vec![42]);
    }

    #[test]
    fn test_make_empty() {
        let v: Vec<i32> = make(0, 0);
        assert!(v.is_empty());
    }

    #[test]
    fn test_make_matrix_dimensions() {
        let m = make_matrix(3, 4, 0.0_f64);
        assert_eq!(m.len(), 3);
        for row in &m {
            assert_eq!(row.len(), 4);
        }
    }

    #[test]
    fn test_make_matrix_set_and_get() {
        let mut m = make_matrix(3, 4, 0.0_f64);
        assert!(matrix_set(&mut m, 1, 2, 42.0));
        assert_eq!(matrix_get(&m, 1, 2), Some(&42.0));
        // Other cells still zero.
        assert_eq!(matrix_get(&m, 0, 0), Some(&0.0));
    }

    #[test]
    fn test_matrix_set_out_of_bounds() {
        let mut m = make_matrix(3, 4, 0.0_f64);
        assert!(!matrix_set(&mut m, 5, 0, 99.0));
        assert!(!matrix_set(&mut m, 0, 10, 99.0));
    }

    #[test]
    fn test_rows_are_independent() {
        // Modifying one row must not affect other rows (independent Vecs).
        let mut m = make_matrix(3, 3, 0_i32);
        m[0][0] = 99;
        assert_eq!(m[1][0], 0);
        assert_eq!(m[2][0], 0);
    }

    #[test]
    fn test_identity_matrix() {
        let id = identity_matrix(3);
        for i in 0..3 {
            for j in 0..3 {
                let expected = if i == j { 1.0 } else { 0.0 };
                assert_eq!(id[i][j], expected);
            }
        }
    }
}
