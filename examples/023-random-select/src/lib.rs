#![allow(clippy::all)]
// Select k elements uniformly at random without replacement (OCaml 99 Problems #23)
// via Fisher-Yates partial shuffle — O(k).
use rand::{Rng, RngExt};

pub fn random_select<T: Clone, R: Rng>(list: &[T], k: usize, rng: &mut R) -> Vec<T> {
    let k = k.min(list.len());
    let mut arr = list.to_vec();
    let len = arr.len();
    for i in 0..k {
        let j = rng.random_range(i..len);
        arr.swap(i, j);
    }
    arr[..k].to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand::rngs::StdRng;
    use std::collections::HashSet;

    #[test]
    fn test_selects_k_elements() {
        let mut rng = StdRng::seed_from_u64(42);
        let selected = random_select(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 4, &mut rng);
        assert_eq!(selected.len(), 4);
    }

    #[test]
    fn test_selected_elements_are_distinct_and_from_source() {
        let mut rng = StdRng::seed_from_u64(7);
        let source = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let selected = random_select(&source, 5, &mut rng);
        let unique: HashSet<_> = selected.iter().collect();
        assert_eq!(unique.len(), 5);
        for x in &selected {
            assert!(source.contains(x));
        }
    }

    #[test]
    fn test_k_zero_returns_empty() {
        let mut rng = StdRng::seed_from_u64(1);
        let empty: Vec<i32> = vec![];
        assert_eq!(random_select(&[1, 2, 3], 0, &mut rng), empty);
    }

    #[test]
    fn test_k_greater_than_len_returns_all_distinct() {
        let mut rng = StdRng::seed_from_u64(1);
        let selected = random_select(&[1, 2, 3], 10, &mut rng);
        assert_eq!(selected.len(), 3);
        let unique: HashSet<_> = selected.iter().collect();
        assert_eq!(unique.len(), 3);
    }

    #[test]
    fn test_same_seed_is_deterministic() {
        let mut rng1 = StdRng::seed_from_u64(99);
        let mut rng2 = StdRng::seed_from_u64(99);
        let a = random_select(&[1, 2, 3, 4, 5], 3, &mut rng1);
        let b = random_select(&[1, 2, 3, 4, 5], 3, &mut rng2);
        assert_eq!(a, b);
    }
}
