#![allow(dead_code)]
//! Generating All Subsets of a List Recursively
//! See example.ml for OCaml reference
//!
//! The power set of a list with n elements contains 2ⁿ subsets.
//! The recursive insight: for each element, either include it or exclude it.

/// Idiomatic Rust: generate all subsets of a slice.
/// Mirrors OCaml's `let rec powerset = function | [] -> [[]] | x :: rest -> ...`
///
/// Requires `T: Clone` because each element is copied into subset Vecs.
pub fn subsets<T: Clone>(items: &[T]) -> Vec<Vec<T>> {
    match items.split_first() {
        None => {
            // Empty slice: one subset, the empty set.
            vec![vec![]]
        }
        Some((first, rest)) => {
            let without = subsets(rest);
            // Subsets containing `first`: prepend it to each subset from `without`.
            let with_first: Vec<Vec<T>> = without
                .iter()
                .map(|s| {
                    let mut new_s = vec![first.clone()];
                    new_s.extend_from_slice(s);
                    new_s
                })
                .collect();
            // Combine: subsets without first + subsets with first.
            let mut result = without;
            result.extend(with_first);
            result
        }
    }
}

/// Functional recursive: exactly mirrors OCaml `powerset` structure.
/// `without @ (List.map (fun s -> x :: s) without)` becomes:
/// `[without..., (first prepended to each)...]`
pub fn subsets_recursive<T: Clone>(items: &[T]) -> Vec<Vec<T>> {
    if items.is_empty() {
        return vec![vec![]];
    }
    let first = &items[0];
    let rest = &items[1..];
    let ps = subsets_recursive(rest);
    // Map: prepend `first` to each existing subset.
    let with_first: Vec<Vec<T>> = ps
        .iter()
        .map(|s| {
            let mut sub = vec![first.clone()];
            sub.extend_from_slice(s);
            sub
        })
        .collect();
    let mut result = ps;
    result.extend(with_first);
    result
}

/// Generate all subsets of a given size k (combinations C(n, k)).
pub fn subsets_of_size<T: Clone>(items: &[T], k: usize) -> Vec<Vec<T>> {
    subsets(items)
        .into_iter()
        .filter(|s| s.len() == k)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subsets_empty() {
        // The power set of {} has exactly 1 element: the empty set.
        let result = subsets::<i32>(&[]);
        assert_eq!(result, vec![vec![]]);
    }

    #[test]
    fn test_subsets_single_element() {
        let result = subsets(&[1_i32]);
        // {}, {1}
        assert_eq!(result.len(), 2);
        assert!(result.contains(&vec![]));
        assert!(result.contains(&vec![1]));
    }

    #[test]
    fn test_subsets_three_elements_count() {
        let result = subsets(&[1_i32, 2, 3]);
        // 2^3 = 8 subsets
        assert_eq!(result.len(), 8);
    }

    #[test]
    fn test_subsets_four_elements_count() {
        let result = subsets(&[1_i32, 2, 3, 4]);
        assert_eq!(result.len(), 16);
    }

    #[test]
    fn test_subsets_recursive_matches_idiomatic() {
        let items = [1_i32, 2, 3];
        let mut a = subsets(&items);
        let mut b = subsets_recursive(&items);
        // Sort to compare regardless of order.
        for s in a.iter_mut() {
            s.sort();
        }
        for s in b.iter_mut() {
            s.sort();
        }
        a.sort();
        b.sort();
        assert_eq!(a, b);
    }

    #[test]
    fn test_subsets_of_size_two() {
        let result = subsets_of_size(&[1_i32, 2, 3, 4], 2);
        // C(4,2) = 6 pairs.
        assert_eq!(result.len(), 6);
        for s in &result {
            assert_eq!(s.len(), 2);
        }
    }

    #[test]
    fn test_subsets_contains_empty_and_full() {
        let items = [1_i32, 2, 3];
        let result = subsets(&items);
        // Must contain the empty subset and the full set.
        assert!(result.contains(&vec![]));
        assert!(result.contains(&vec![1, 2, 3]));
    }

    #[test]
    fn test_subsets_five_elements() {
        assert_eq!(subsets(&[1_i32, 2, 3, 4, 5]).len(), 32);
    }
}
