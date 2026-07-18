#![allow(clippy::all)]
// Generate all k-element combinations of a list (OCaml 99 Problems #26),
// via the classic include/exclude backtracking recursion.
pub fn combinations<T: Clone>(k: usize, list: &[T]) -> Vec<Vec<T>> {
    if k == 0 {
        return vec![vec![]];
    }
    match list.split_first() {
        None => vec![],
        Some((head, tail)) => {
            let mut with_head: Vec<Vec<T>> = combinations(k - 1, tail)
                .into_iter()
                .map(|mut c| {
                    c.insert(0, head.clone());
                    c
                })
                .collect();
            with_head.extend(combinations(k, tail));
            with_head
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_combinations_of_2_from_3() {
        assert_eq!(
            combinations(2, &[1, 2, 3]),
            vec![vec![1, 2], vec![1, 3], vec![2, 3]]
        );
    }

    #[test]
    fn test_k_zero_yields_one_empty_combination() {
        assert_eq!(combinations(0, &[1, 2, 3]), vec![Vec::<i32>::new()]);
    }

    #[test]
    fn test_k_greater_than_len_yields_none() {
        let empty: Vec<Vec<i32>> = vec![];
        assert_eq!(combinations(5, &[1, 2, 3]), empty);
    }

    #[test]
    fn test_k_equals_len_yields_one_combination() {
        assert_eq!(combinations(3, &[1, 2, 3]), vec![vec![1, 2, 3]]);
    }

    #[test]
    fn test_count_matches_binomial_coefficient() {
        // C(5, 2) = 10
        assert_eq!(combinations(2, &[1, 2, 3, 4, 5]).len(), 10);
    }
}
