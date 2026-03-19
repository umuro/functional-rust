/// # Seq Module — Lazy Sequences
///
/// Rust's iterator trait is inherently lazy — no need for a separate Seq module.
/// This demonstrates Rust's `std::iter` as the equivalent of OCaml's `Seq`.

/// Infinite naturals starting from n
pub fn naturals(start: u64) -> impl Iterator<Item = u64> {
    start..
}

/// Infinite Fibonacci sequence using `std::iter::successors`
pub fn fibs() -> impl Iterator<Item = u64> {
    std::iter::successors(Some((0u64, 1u64)), |&(a, b)| Some((b, a + b))).map(|(a, _)| a)
}

/// Infinite primes using trial division (lazy — only computes as needed)
pub fn primes() -> impl Iterator<Item = u64> {
    (2u64..).filter(|&n| {
        let limit = (n as f64).sqrt() as u64;
        (2..=limit).all(|i| n % i != 0)
    })
}

/// `unfold` — Rust's equivalent is `std::iter::successors` or `std::iter::from_fn`
pub fn unfold<T, S>(seed: S, f: impl Fn(S) -> Option<(T, S)>) -> Vec<T>
where
    S: Clone,
{
    let mut result = Vec::new();
    let mut state = seed;
    while let Some((value, next)) = f(state.clone()) {
        result.push(value);
        state = next;
    }
    result
}

/// Collatz sequence using unfold
pub fn collatz(n: u64) -> Vec<u64> {
    unfold(n, |x| {
        if x == 0 {
            None
        } else if x == 1 {
            Some((1, 0))
        } else if x % 2 == 0 {
            Some((x, x / 2))
        } else {
            Some((x, 3 * x + 1))
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_naturals() {
        let first5: Vec<u64> = naturals(0).take(5).collect();
        assert_eq!(first5, vec![0, 1, 2, 3, 4]);
    }

    #[test]
    fn test_fibs() {
        let first10: Vec<u64> = fibs().take(10).collect();
        assert_eq!(first10, vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34]);
    }

    #[test]
    fn test_primes() {
        let first10: Vec<u64> = primes().take(10).collect();
        assert_eq!(first10, vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29]);
    }

    #[test]
    fn test_collatz() {
        assert_eq!(collatz(6), vec![6, 3, 10, 5, 16, 8, 4, 2, 1]);
    }

    #[test]
    fn test_lazy_computation() {
        // This doesn't compute all naturals — only the ones we take
        let sum: u64 = naturals(1).take(100).sum();
        assert_eq!(sum, 5050);
    }
}
