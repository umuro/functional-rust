#![allow(clippy::all)]
// Example 091: Zip and Unzip
// OCaml List.combine/split → Rust zip/unzip

// === Approach 1: Basic zip/unzip ===

/// Pair two slices element-by-element (stops at shorter).
/// Mirrors OCaml's `List.combine`.
pub fn zip_vecs(a: &[i32], b: &[&str]) -> Vec<(i32, String)> {
    a.iter()
        .zip(b.iter())
        .map(|(&n, &s)| (n, s.to_string()))
        .collect()
}

/// Split a slice of pairs back into two Vecs.
/// Mirrors OCaml's `List.split`.
pub fn unzip_vecs(pairs: &[(i32, &str)]) -> (Vec<i32>, Vec<String>) {
    pairs.iter().map(|&(n, s)| (n, s.to_string())).unzip()
}

// === Approach 2: zip_with / map2 equivalent ===

/// Element-wise dot product — zip then fold.
pub fn dot_product(a: &[i32], b: &[i32]) -> i32 {
    a.iter().zip(b.iter()).map(|(x, y)| x * y).sum()
}

/// Element-wise maximum of two slices.
pub fn pairwise_max(a: &[i32], b: &[i32]) -> Vec<i32> {
    a.iter().zip(b.iter()).map(|(&x, &y)| x.max(y)).collect()
}

/// General element-wise operation (`zip_with` / `List.map2`).
pub fn pairwise_op<T, U>(a: &[T], b: &[T], f: impl Fn(&T, &T) -> U) -> Vec<U> {
    a.iter().zip(b.iter()).map(|(x, y)| f(x, y)).collect()
}

// === Approach 3: zip with index (enumerate) ===

/// Attach 0-based indices to each element.
/// Mirrors OCaml's `List.mapi (fun i x -> (i, x))`.
pub fn zip_with_index<T: Clone>(lst: &[T]) -> Vec<(usize, T)> {
    lst.iter()
        .enumerate()
        .map(|(i, x)| (i, x.clone()))
        .collect()
}

// === Approach 4: zip_longest — pad shorter sequence ===

/// Zip two slices, padding the shorter one with supplied defaults.
/// OCaml's `zip_longest` with `~default_a` / `~default_b`.
pub fn zip_longest<T: Clone>(a: &[T], b: &[T], default_a: T, default_b: T) -> Vec<(T, T)> {
    let len = a.len().max(b.len());
    (0..len)
        .map(|i| {
            let x = a.get(i).cloned().unwrap_or_else(|| default_a.clone());
            let y = b.get(i).cloned().unwrap_or_else(|| default_b.clone());
            (x, y)
        })
        .collect()
}

// === Approach 5: Unzip owned pairs (generic) ===

/// Unzip a Vec of owned pairs into two Vecs.
pub fn unzip_owned<A, B>(pairs: Vec<(A, B)>) -> (Vec<A>, Vec<B>) {
    pairs.into_iter().unzip()
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- zip_vecs / unzip_vecs ---

    #[test]
    fn test_zip_empty() {
        let result: Vec<(i32, String)> = zip_vecs(&[], &[]);
        assert_eq!(result, vec![]);
    }

    #[test]
    fn test_zip_equal_length() {
        let a = [1, 2, 3];
        let b = ["one", "two", "three"];
        let result = zip_vecs(&a, &b);
        assert_eq!(
            result,
            vec![
                (1, "one".to_string()),
                (2, "two".to_string()),
                (3, "three".to_string()),
            ]
        );
    }

    #[test]
    fn test_zip_truncates_at_shorter() {
        // b is shorter — Rust's zip stops at the shorter iterator
        let a = [1, 2, 3, 4];
        let b = ["a", "b"];
        let result = zip_vecs(&a, &b);
        assert_eq!(result, vec![(1, "a".to_string()), (2, "b".to_string())]);
    }

    #[test]
    fn test_unzip_roundtrip() {
        let pairs = [(10, "x"), (20, "y"), (30, "z")];
        let (nums, strs) = unzip_vecs(&pairs);
        assert_eq!(nums, vec![10, 20, 30]);
        assert_eq!(strs, vec!["x", "y", "z"]);
    }

    // --- dot_product ---

    #[test]
    fn test_dot_product_basic() {
        assert_eq!(dot_product(&[1, 2, 3], &[4, 5, 6]), 32); // 4+10+18
    }

    #[test]
    fn test_dot_product_zeros() {
        assert_eq!(dot_product(&[0, 0, 0], &[1, 2, 3]), 0);
    }

    // --- pairwise_max ---

    #[test]
    fn test_pairwise_max() {
        assert_eq!(pairwise_max(&[1, 5, 3], &[4, 2, 6]), vec![4, 5, 6]);
    }

    // --- pairwise_op ---

    #[test]
    fn test_pairwise_op_add() {
        let result = pairwise_op(&[1, 2, 3], &[10, 20, 30], |a, b| a + b);
        assert_eq!(result, vec![11, 22, 33]);
    }

    // --- zip_with_index ---

    #[test]
    fn test_zip_with_index_empty() {
        let result: Vec<(usize, i32)> = zip_with_index(&[]);
        assert_eq!(result, vec![]);
    }

    #[test]
    fn test_zip_with_index_basic() {
        let result = zip_with_index(&["a", "b", "c"]);
        assert_eq!(result, vec![(0, "a"), (1, "b"), (2, "c")]);
    }

    // --- zip_longest ---

    #[test]
    fn test_zip_longest_equal() {
        let result = zip_longest(&[1, 2], &[3, 4], 0, 0);
        assert_eq!(result, vec![(1, 3), (2, 4)]);
    }

    #[test]
    fn test_zip_longest_a_shorter() {
        let result = zip_longest(&[1], &[10, 20, 30], 0, 0);
        assert_eq!(result, vec![(1, 10), (0, 20), (0, 30)]);
    }

    #[test]
    fn test_zip_longest_b_shorter() {
        let result = zip_longest(&[1, 2, 3], &[9], 0, 0);
        assert_eq!(result, vec![(1, 9), (2, 0), (3, 0)]);
    }

    // --- unzip_owned ---

    #[test]
    fn test_unzip_owned() {
        let pairs = vec![(1, "a"), (2, "b"), (3, "c")];
        let (nums, letters): (Vec<i32>, Vec<&str>) = unzip_owned(pairs);
        assert_eq!(nums, vec![1, 2, 3]);
        assert_eq!(letters, vec!["a", "b", "c"]);
    }
}
