//! # Property-Based Testing
//!
//! std-only QuickCheck-style framework for property testing.

/// Simple deterministic PRNG (Linear Congruential Generator)
pub struct Lcg(u64);

impl Lcg {
    /// Create a new LCG with the given seed
    pub fn new(seed: u64) -> Self {
        Lcg(seed)
    }

    /// Generate the next random u64
    pub fn next_u64(&mut self) -> u64 {
        self.0 = self
            .0
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        self.0
    }

    /// Generate a random i32 in the range [lo, hi]
    pub fn next_i32_in(&mut self, lo: i32, hi: i32) -> i32 {
        let range = (hi - lo + 1) as u64;
        lo + (self.next_u64() % range) as i32
    }

    /// Generate a random usize in the range [lo, hi]
    pub fn next_usize_in(&mut self, lo: usize, hi: usize) -> usize {
        let range = (hi - lo + 1) as u64;
        lo + (self.next_u64() % range) as usize
    }
}

/// Trait for generating arbitrary test values with shrinking
pub trait Arbitrary: Sized + Clone + std::fmt::Debug {
    /// Generate an arbitrary value
    fn arbitrary(rng: &mut Lcg) -> Self;

    /// Shrink to simpler values for counterexample minimization
    fn shrink(&self) -> Vec<Self> {
        vec![]
    }
}

impl Arbitrary for i32 {
    fn arbitrary(rng: &mut Lcg) -> Self {
        rng.next_i32_in(-1000, 1000)
    }

    fn shrink(&self) -> Vec<i32> {
        if *self == 0 {
            return vec![];
        }
        vec![0, self / 2, self.abs() - 1]
            .into_iter()
            .filter(|&x| x.abs() < self.abs())
            .collect()
    }
}

impl Arbitrary for Vec<i32> {
    fn arbitrary(rng: &mut Lcg) -> Self {
        let len = rng.next_usize_in(0, 20);
        (0..len).map(|_| i32::arbitrary(rng)).collect()
    }

    fn shrink(&self) -> Vec<Vec<i32>> {
        let mut shrunk = vec![];
        if self.is_empty() {
            return shrunk;
        }
        shrunk.push(self[1..].to_vec());
        shrunk.push(self[..self.len() - 1].to_vec());
        shrunk.push(self[..self.len() / 2].to_vec());
        shrunk
    }
}

/// Run a property test with shrinking on failure
pub fn forall<T, F>(name: &str, tests: usize, mut prop: F) -> bool
where
    T: Arbitrary,
    F: FnMut(&T) -> bool,
{
    let mut rng = Lcg::new(42);
    for i in 0..tests {
        let input = T::arbitrary(&mut rng);
        if !prop(&input) {
            let mut minimal = input.clone();
            loop {
                let candidates = minimal.shrink();
                let smaller = candidates.into_iter().find(|c| !prop(c));
                match smaller {
                    Some(s) => minimal = s,
                    None => break,
                }
            }
            eprintln!(
                "✗ {} failed after {} tests. Counterexample: {:?}",
                name,
                i + 1,
                minimal
            );
            return false;
        }
    }
    true
}

/// Sort a vector (for testing)
pub fn my_sort(mut v: Vec<i32>) -> Vec<i32> {
    v.sort();
    v
}

/// Check if a slice is sorted
pub fn is_sorted(v: &[i32]) -> bool {
    v.windows(2).all(|w| w[0] <= w[1])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lcg_produces_different_values() {
        let mut rng = Lcg::new(1);
        let a = rng.next_u64();
        let b = rng.next_u64();
        assert_ne!(a, b);
    }

    #[test]
    fn test_property_sort_idempotent() {
        assert!(forall::<Vec<i32>, _>("sort idempotent", 500, |v| {
            my_sort(my_sort(v.clone())) == my_sort(v.clone())
        }));
    }

    #[test]
    fn test_property_sort_length_preserved() {
        assert!(forall::<Vec<i32>, _>("sort length", 500, |v| {
            my_sort(v.clone()).len() == v.len()
        }));
    }

    #[test]
    fn test_property_sort_ordered() {
        assert!(forall::<Vec<i32>, _>("sort ordered", 500, |v| {
            is_sorted(&my_sort(v.clone()))
        }));
    }

    #[test]
    fn test_i32_shrink_produces_smaller() {
        for x in [100i32, -50, 17] {
            for smaller in x.shrink() {
                assert!(
                    smaller.abs() < x.abs(),
                    "{} should shrink below {}",
                    smaller,
                    x
                );
            }
        }
    }

    #[test]
    fn test_vec_shrink_shorter() {
        let v = vec![1, 2, 3, 4, 5];
        let shrunk = v.shrink();
        assert!(!shrunk.is_empty());
        for s in shrunk {
            assert!(
                s.len() < v.len(),
                "shrunk {:?} should be shorter than {:?}",
                s,
                v
            );
        }
    }

    #[test]
    fn test_lcg_range() {
        let mut rng = Lcg::new(12345);
        for _ in 0..100 {
            let v = rng.next_i32_in(-10, 10);
            assert!((-10..=10).contains(&v));
        }
    }
}
