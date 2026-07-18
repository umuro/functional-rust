#![allow(clippy::all)]
// Lotto draw: n distinct numbers from 1..=m, composing range (022) + random_select (023)
use rand::{Rng, RngExt};

pub fn lotto_select<R: Rng>(n: usize, m: u32, rng: &mut R) -> Vec<u32> {
    let mut pool: Vec<u32> = (1..=m).collect();
    let n = n.min(pool.len());
    let len = pool.len();
    for i in 0..n {
        let j = rng.random_range(i..len);
        pool.swap(i, j);
    }
    let mut result = pool[..n].to_vec();
    result.sort();
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand::rngs::StdRng;
    use std::collections::HashSet;

    #[test]
    fn test_draws_n_numbers() {
        let mut rng = StdRng::seed_from_u64(42);
        let draw = lotto_select(6, 49, &mut rng);
        assert_eq!(draw.len(), 6);
    }

    #[test]
    fn test_draw_is_sorted_and_in_range() {
        let mut rng = StdRng::seed_from_u64(7);
        let draw = lotto_select(6, 49, &mut rng);
        let mut sorted = draw.clone();
        sorted.sort();
        assert_eq!(draw, sorted);
        for x in &draw {
            assert!(*x >= 1 && *x <= 49);
        }
    }

    #[test]
    fn test_draw_has_distinct_numbers() {
        let mut rng = StdRng::seed_from_u64(3);
        let draw = lotto_select(6, 49, &mut rng);
        let unique: HashSet<_> = draw.iter().collect();
        assert_eq!(unique.len(), 6);
    }

    #[test]
    fn test_n_greater_than_m_returns_all() {
        let mut rng = StdRng::seed_from_u64(1);
        let draw = lotto_select(10, 5, &mut rng);
        assert_eq!(draw, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_same_seed_is_deterministic() {
        let mut rng1 = StdRng::seed_from_u64(123);
        let mut rng2 = StdRng::seed_from_u64(123);
        assert_eq!(lotto_select(6, 49, &mut rng1), lotto_select(6, 49, &mut rng2));
    }
}
