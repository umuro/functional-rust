/// 748: Property-Based Testing — std-only QuickCheck-style framework

// ── Simple deterministic PRNG (LCG) ───────────────────────────────────────────

struct Lcg(u64);

impl Lcg {
    fn new(seed: u64) -> Self { Lcg(seed) }

    fn next_u64(&mut self) -> u64 {
        // LCG: Numerical Recipes constants
        self.0 = self.0.wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        self.0
    }

    fn next_i32_in(&mut self, lo: i32, hi: i32) -> i32 {
        let range = (hi - lo + 1) as u64;
        lo + (self.next_u64() % range) as i32
    }

    fn next_usize_in(&mut self, lo: usize, hi: usize) -> usize {
        let range = (hi - lo + 1) as u64;
        lo + (self.next_u64() % range) as usize
    }
}

// ── Arbitrary trait ───────────────────────────────────────────────────────────

trait Arbitrary: Sized + Clone + std::fmt::Debug {
    fn arbitrary(rng: &mut Lcg) -> Self;
    /// Shrink to simpler values (for counterexample minimisation)
    fn shrink(&self) -> Vec<Self> { vec![] }
}

impl Arbitrary for i32 {
    fn arbitrary(rng: &mut Lcg) -> Self {
        rng.next_i32_in(-1000, 1000)
    }
    fn shrink(&self) -> Vec<i32> {
        if *self == 0 { return vec![]; }
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
        if self.is_empty() { return shrunk; }
        // Remove first element
        shrunk.push(self[1..].to_vec());
        // Remove last element
        shrunk.push(self[..self.len()-1].to_vec());
        // Halve
        shrunk.push(self[..self.len()/2].to_vec());
        shrunk
    }
}

// ── forall: property checker with shrinking ────────────────────────────────────

fn forall<T, F>(name: &str, tests: usize, mut prop: F) -> bool
where
    T: Arbitrary,
    F: FnMut(&T) -> bool,
{
    let mut rng = Lcg::new(42);
    for i in 0..tests {
        let input = T::arbitrary(&mut rng);
        if !prop(&input) {
            // Try to shrink the counterexample
            let mut minimal = input.clone();
            loop {
                let candidates = minimal.shrink();
                let smaller = candidates.into_iter().find(|c| !prop(c));
                match smaller {
                    Some(s) => minimal = s,
                    None    => break,
                }
            }
            eprintln!("✗ {} failed after {} tests. Counterexample: {:?}", name, i+1, minimal);
            return false;
        }
    }
    println!("✓ {} — {} tests passed", name, tests);
    true
}

// ── Functions under test ───────────────────────────────────────────────────────

fn my_sort(mut v: Vec<i32>) -> Vec<i32> {
    v.sort();
    v
}

fn is_sorted(v: &[i32]) -> bool {
    v.windows(2).all(|w| w[0] <= w[1])
}

fn sum_commutative(a: i32, b: i32) -> bool {
    a.wrapping_add(b) == b.wrapping_add(a)
}

fn main() {
    // Properties of sort
    forall::<Vec<i32>, _>("sort is idempotent",
        1000, |v| my_sort(my_sort(v.clone())) == my_sort(v.clone()));

    forall::<Vec<i32>, _>("sort preserves length",
        1000, |v| my_sort(v.clone()).len() == v.len());

    forall::<Vec<i32>, _>("sort result is ordered",
        1000, |v| is_sorted(&my_sort(v.clone())));

    // Property of addition
    let mut rng = Lcg::new(99);
    let mut ok = true;
    for _ in 0..1000 {
        let a = i32::arbitrary(&mut rng);
        let b = i32::arbitrary(&mut rng);
        ok &= sum_commutative(a, b);
    }
    println!("{} addition is commutative — 1000 tests passed", if ok { "✓" } else { "✗" });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lcg_produces_different_values() {
        let mut rng = Lcg::new(1);
        let a = rng.next_u64();
        let b = rng.next_u64();
        assert_ne!(a, b);
    }

    #[test]
    fn property_sort_idempotent() {
        assert!(forall::<Vec<i32>, _>("sort idempotent",
            500, |v| my_sort(my_sort(v.clone())) == my_sort(v.clone())));
    }

    #[test]
    fn property_sort_length_preserved() {
        assert!(forall::<Vec<i32>, _>("sort length",
            500, |v| my_sort(v.clone()).len() == v.len()));
    }

    #[test]
    fn property_sort_ordered() {
        assert!(forall::<Vec<i32>, _>("sort ordered",
            500, |v| is_sorted(&my_sort(v.clone()))));
    }

    #[test]
    fn i32_shrink_produces_smaller() {
        for x in [100i32, -50, 17] {
            for smaller in x.shrink() {
                assert!(smaller.abs() < x.abs(),
                    "{} should shrink below {}", smaller, x);
            }
        }
    }

    #[test]
    fn vec_shrink_shorter() {
        let v = vec![1, 2, 3, 4, 5];
        let shrunk = v.shrink();
        assert!(!shrunk.is_empty());
        for s in shrunk {
            assert!(s.len() < v.len(), "shrunk {:?} should be shorter than {:?}", s, v);
        }
    }
}
