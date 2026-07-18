#![allow(clippy::all)]
// Fisher-Yates shuffle: uniform random permutation of all elements, O(n)
use rand::{Rng, RngExt};
use std::collections::HashSet;

pub fn permutation<T: Clone, R: Rng>(list: &[T], rng: &mut R) -> Vec<T> {
    let mut v = list.to_vec();
    let n = v.len();
    if n > 1 {
        for i in (1..n).rev() {
            let j = rng.random_range(0..=i);
            v.swap(i, j);
        }
    }
    v
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand::rngs::StdRng;

    #[test]
    fn test_preserves_length() {
        let mut rng = StdRng::seed_from_u64(1);
        assert_eq!(permutation(&[1, 2, 3, 4, 5], &mut rng).len(), 5);
    }

    #[test]
    fn test_preserves_multiset_of_elements() {
        let mut rng = StdRng::seed_from_u64(2);
        let source = [1, 2, 3, 4, 5];
        let shuffled = permutation(&source, &mut rng);
        let orig: HashSet<_> = source.iter().collect();
        let new: HashSet<_> = shuffled.iter().collect();
        assert_eq!(orig, new);
    }

    #[test]
    fn test_same_seed_is_deterministic() {
        let mut rng1 = StdRng::seed_from_u64(42);
        let mut rng2 = StdRng::seed_from_u64(42);
        assert_eq!(permutation(&[1, 2, 3, 4, 5], &mut rng1), permutation(&[1, 2, 3, 4, 5], &mut rng2));
    }

    #[test]
    fn test_empty_and_single_element() {
        let mut rng = StdRng::seed_from_u64(3);
        let empty: Vec<i32> = vec![];
        assert_eq!(permutation(&empty, &mut rng), empty);
        assert_eq!(permutation(&[42], &mut rng), vec![42]);
    }
}
