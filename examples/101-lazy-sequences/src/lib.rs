#![allow(clippy::all)]
//! # Lazy Sequences
//!
//! OCaml's `Seq` module provides lazy sequences. Rust's iterators are
//! lazy by default — they only compute values when consumed.

// ---------------------------------------------------------------------------
// Approach A: Iterator adaptors (idiomatic Rust)
// ---------------------------------------------------------------------------

/// Infinite natural numbers starting from n
pub fn naturals(start: u64) -> impl Iterator<Item = u64> {
    start..
}

/// Fibonacci sequence as an iterator
pub fn fibs() -> impl Iterator<Item = u64> {
    std::iter::successors(Some((0u64, 1u64)), |&(a, b)| {
        a.checked_add(b).map(|s| (b, s))
    })
    .map(|(a, _)| a)
}

/// Infinite prime number iterator
pub fn primes() -> impl Iterator<Item = u64> {
    (2..).filter(|&n| is_prime(n))
}

fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    (2..).take_while(|&i| i * i <= n).all(|i| n % i != 0)
}

// ---------------------------------------------------------------------------
// Approach B: Custom unfold (mirrors OCaml's Seq.unfold)
// ---------------------------------------------------------------------------

pub fn unfold<S, T>(seed: S, f: impl Fn(&S) -> Option<(T, S)>) -> impl Iterator<Item = T> {
    let mut state = Some(seed);
    std::iter::from_fn(move || {
        let s = state.take()?;
        let (value, next) = f(&s)?;
        state = Some(next);
        Some(value)
    })
}

pub fn fibs_unfold() -> impl Iterator<Item = u64> {
    unfold((0u64, 1u64), |&(a, b)| Some((a, (b, a + b))))
}

// ---------------------------------------------------------------------------
// Approach C: Successors (std::iter::successors)
// ---------------------------------------------------------------------------

pub fn naturals_succ(start: u64) -> impl Iterator<Item = u64> {
    std::iter::successors(Some(start), |&n| Some(n + 1))
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
    fn test_unfold_fibs() {
        let first10: Vec<u64> = fibs_unfold().take(10).collect();
        assert_eq!(first10, vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34]);
    }

    #[test]
    fn test_laziness() {
        // This doesn't hang because iterators are lazy
        let _infinite = naturals(0);
        let first = naturals(0).take(1).collect::<Vec<_>>();
        assert_eq!(first, vec![0]);
    }
}
