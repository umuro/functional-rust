/// Creates a 1D vector of `n` elements, each initialized to `val`.
/// Equivalent to OCaml's `Array.make n val`.
///
/// Uses the `vec!` macro which is the idiomatic Rust approach.
pub fn make<T: Clone>(n: usize, val: T) -> Vec<T> {
    vec![val; n]
}

/// Creates a 1D vector using iterator chaining — functional style.
/// `std::iter::repeat_n` repeats `val` exactly `n` times, equivalent to
/// the classic `repeat(val).take(n)` but clearer in intent.
pub fn make_iter<T: Clone>(n: usize, val: T) -> Vec<T> {
    std::iter::repeat_n(val, n).collect()
}

/// Creates a 2D matrix of `rows × cols`, each cell initialized to `val`.
/// Equivalent to OCaml's `Array.make_matrix rows cols val`.
///
/// Each row is an independent `Vec<T>` — mutation to one row does not affect others,
/// matching OCaml's guarantee that `Array.make_matrix` allocates independent rows.
pub fn make_matrix<T: Clone>(rows: usize, cols: usize, val: T) -> Vec<Vec<T>> {
    vec![vec![val; cols]; rows]
}

/// Creates a 2D matrix using explicit iterator mapping — functional style.
/// Each row is built via `(0..cols).map(|_| val.clone()).collect()`,
/// making the per-cell cloning visible rather than implicit in `vec!`.
pub fn make_matrix_iter<T: Clone>(rows: usize, cols: usize, val: T) -> Vec<Vec<T>> {
    (0..rows)
        .map(|_| (0..cols).map(|_| val.clone()).collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- make / make_iter ---

    #[test]
    fn test_make_empty() {
        let v: Vec<i32> = make(0, 0);
        assert_eq!(v, Vec::<i32>::new());
    }

    #[test]
    fn test_make_single() {
        assert_eq!(make(1, 42), vec![42]);
    }

    #[test]
    fn test_make_multiple() {
        assert_eq!(make(5, 0), vec![0, 0, 0, 0, 0]);
    }

    #[test]
    fn test_make_string() {
        assert_eq!(make(3, "hi"), vec!["hi", "hi", "hi"]);
    }

    #[test]
    fn test_make_iter_matches_make() {
        assert_eq!(make_iter(5, 0), make(5, 0));
        assert_eq!(make_iter(0, 99), make(0, 99));
        assert_eq!(make_iter(1, 7), make(1, 7));
    }

    #[test]
    fn test_make_rows_are_independent() {
        // Modifying one element must not affect others — vec! allocates fresh storage.
        let mut v = make(3, 0);
        v[1] = 99;
        assert_eq!(v, vec![0, 99, 0]);
    }

    // --- make_matrix / make_matrix_iter ---

    #[test]
    fn test_make_matrix_zero_rows() {
        let m: Vec<Vec<f64>> = make_matrix(0, 4, 0.0);
        assert!(m.is_empty());
    }

    #[test]
    fn test_make_matrix_zero_cols() {
        let m: Vec<Vec<f64>> = make_matrix(3, 0, 0.0);
        assert_eq!(m.len(), 3);
        assert!(m.iter().all(|row| row.is_empty()));
    }

    #[test]
    fn test_make_matrix_dimensions() {
        let m = make_matrix(3, 4, 0.0_f64);
        assert_eq!(m.len(), 3);
        assert!(m.iter().all(|row| row.len() == 4));
    }

    #[test]
    fn test_make_matrix_all_zeros() {
        let m = make_matrix(3, 4, 0.0_f64);
        assert!(m.iter().all(|row| row.iter().all(|&x| x == 0.0)));
    }

    #[test]
    fn test_make_matrix_mutation_independent_rows() {
        // Matches OCaml: Array.make_matrix allocates independent rows.
        // Mutation of one row must not bleed into another.
        let mut m = make_matrix(3, 4, 0.0_f64);
        m[1][2] = 42.0;
        assert_eq!(m[0][2], 0.0);
        assert_eq!(m[1][2], 42.0);
        assert_eq!(m[2][2], 0.0);
    }

    #[test]
    fn test_make_matrix_iter_matches_make_matrix() {
        assert_eq!(make_matrix_iter(3, 4, 0.0_f64), make_matrix(3, 4, 0.0_f64));
        assert_eq!(make_matrix_iter(0, 4, 0), make_matrix(0, 4, 0));
        assert_eq!(make_matrix_iter(2, 0, 0), make_matrix(2, 0, 0));
    }

    #[test]
    fn test_make_matrix_iter_mutation_independent_rows() {
        let mut m = make_matrix_iter(3, 4, 0.0_f64);
        m[1][2] = 99.0;
        assert_eq!(m[0][2], 0.0);
        assert_eq!(m[1][2], 99.0);
        assert_eq!(m[2][2], 0.0);
    }
}
