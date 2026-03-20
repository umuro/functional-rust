#![allow(clippy::all)]
/// Generate the next row of Pascal's triangle from the current row.
///
/// Uses the "zip-with-add" trick: prepend 0 to the row, append 0 to the row,
/// then add corresponding elements pairwise.
///
/// OCaml: `List.map2 (+) (0 :: row) (row @ [0])`
/// Rust:  zip two iterators with offset and sum
pub fn next_row(row: &[u64]) -> Vec<u64> {
    // [0, a, b, c] zipped with [a, b, c, 0] → [a, a+b, b+c, c]
    std::iter::once(&0)
        .chain(row.iter())
        .zip(row.iter().chain(std::iter::once(&0)))
        .map(|(a, b)| a + b)
        .collect()
}

/// Generate `n` rows of Pascal's triangle.
///
/// # Idiomatic Rust — iterative with successors
pub fn pascal(n: usize) -> Vec<Vec<u64>> {
    std::iter::successors(Some(vec![1u64]), |prev| Some(next_row(prev)))
        .take(n)
        .collect()
}

/// Recursive version — mirrors OCaml's `let rec go`.
pub fn pascal_recursive(n: usize) -> Vec<Vec<u64>> {
    fn go(row: Vec<u64>, i: usize, n: usize) -> Vec<Vec<u64>> {
        if i > n {
            return Vec::new();
        }
        let next = next_row(&row);
        let mut result = vec![row];
        result.extend(go(next, i + 1, n));
        result
    }
    if n == 0 {
        return Vec::new();
    }
    go(vec![1], 1, n)
}

/// Fold-based version — accumulates rows by folding over a range.
pub fn pascal_fold(n: usize) -> Vec<Vec<u64>> {
    if n == 0 {
        return Vec::new();
    }
    let (rows, _) = (1..n).fold((vec![vec![1u64]], vec![1u64]), |(mut rows, prev), _| {
        let next = next_row(&prev);
        rows.push(next.clone());
        (rows, next)
    });
    rows
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        assert!(pascal(0).is_empty());
    }

    #[test]
    fn test_single_row() {
        assert_eq!(pascal(1), vec![vec![1]]);
    }

    #[test]
    fn test_five_rows() {
        let expected = vec![
            vec![1],
            vec![1, 1],
            vec![1, 2, 1],
            vec![1, 3, 3, 1],
            vec![1, 4, 6, 4, 1],
        ];
        assert_eq!(pascal(5), expected);
    }

    #[test]
    fn test_eight_rows() {
        let rows = pascal(8);
        assert_eq!(rows.len(), 8);
        // Row 7 (0-indexed) should be [1, 7, 21, 35, 35, 21, 7, 1]
        assert_eq!(rows[7], vec![1, 7, 21, 35, 35, 21, 7, 1]);
    }

    #[test]
    fn test_row_symmetry() {
        let rows = pascal(10);
        for row in &rows {
            let reversed: Vec<u64> = row.iter().rev().copied().collect();
            assert_eq!(row, &reversed);
        }
    }

    #[test]
    fn test_next_row() {
        assert_eq!(next_row(&[1]), vec![1, 1]);
        assert_eq!(next_row(&[1, 1]), vec![1, 2, 1]);
        assert_eq!(next_row(&[1, 2, 1]), vec![1, 3, 3, 1]);
    }

    #[test]
    fn test_recursive_matches_iterative() {
        for n in 0..10 {
            assert_eq!(pascal(n), pascal_recursive(n), "Mismatch at n={}", n);
        }
    }

    #[test]
    fn test_fold_matches_iterative() {
        for n in 0..10 {
            assert_eq!(pascal(n), pascal_fold(n), "Fold mismatch at n={}", n);
        }
    }
}
