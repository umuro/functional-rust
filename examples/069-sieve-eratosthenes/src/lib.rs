/// Sieve of Eratosthenes (Functional)
///
/// A purely functional prime sieve using recursive filtering.
/// OCaml's version recursively filters a list. Rust's idiomatic
/// version uses a boolean array (imperative sieve) but we show
/// both functional and imperative approaches.

/// Functional sieve — recursive filter, mirrors the OCaml version.
/// Not efficient (O(n * primes) due to repeated filtering) but elegant.
pub fn sieve_functional(candidates: Vec<u64>) -> Vec<u64> {
    match candidates.as_slice() {
        [] => vec![],
        [p, ..] => {
            let p = *p;
            let rest: Vec<u64> = candidates[1..]
                .iter()
                .filter(|&&n| n % p != 0)
                .copied()
                .collect();
            let mut result = vec![p];
            result.extend(sieve_functional(rest));
            result
        }
    }
}

pub fn primes_up_to_functional(n: u64) -> Vec<u64> {
    if n < 2 {
        return vec![];
    }
    let candidates: Vec<u64> = (2..=n).collect();
    sieve_functional(candidates)
}

/// Imperative sieve — idiomatic Rust, O(n log log n).
pub fn primes_up_to(n: usize) -> Vec<usize> {
    if n < 2 {
        return vec![];
    }
    let mut is_prime = vec![true; n + 1];
    is_prime[0] = false;
    is_prime[1] = false;

    let mut i = 2;
    while i * i <= n {
        if is_prime[i] {
            let mut j = i * i;
            while j <= n {
                is_prime[j] = false;
                j += i;
            }
        }
        i += 1;
    }

    (0..=n).filter(|&i| is_prime[i]).collect()
}

/// Iterator-based: generate primes lazily using trial division.
pub fn nth_prime(n: usize) -> u64 {
    let mut primes = Vec::new();
    let mut candidate = 2u64;
    while primes.len() < n {
        if primes.iter().all(|&p| candidate % p != 0) {
            primes.push(candidate);
        }
        candidate += 1;
    }
    *primes.last().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primes_up_to_50() {
        let expected = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47];
        assert_eq!(primes_up_to(50), expected);
    }

    #[test]
    fn test_functional_sieve() {
        let expected: Vec<u64> = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47];
        assert_eq!(primes_up_to_functional(50), expected);
    }

    #[test]
    fn test_count_up_to_100() {
        assert_eq!(primes_up_to(100).len(), 25);
    }

    #[test]
    fn test_nth_prime() {
        assert_eq!(nth_prime(10), 29);
    }

    #[test]
    fn test_edge_cases() {
        assert_eq!(primes_up_to(0), vec![]);
        assert_eq!(primes_up_to(1), vec![]);
        assert_eq!(primes_up_to(2), vec![2]);
    }

    #[test]
    fn test_small() {
        assert_eq!(primes_up_to(10), vec![2, 3, 5, 7]);
    }
}
