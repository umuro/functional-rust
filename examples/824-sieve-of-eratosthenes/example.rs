/// Sieve of Eratosthenes — generate all primes up to N in O(n log log n).

/// Returns all primes ≤ limit.
fn sieve(limit: usize) -> Vec<usize> {
    if limit < 2 {
        return vec![];
    }
    let mut is_prime = vec![true; limit + 1];
    is_prime[0] = false;
    is_prime[1] = false;
    let sqrt_limit = (limit as f64).sqrt() as usize;
    for p in 2..=sqrt_limit {
        if is_prime[p] {
            // Mark multiples of p starting from p²
            for j in (p * p..=limit).step_by(p) {
                is_prime[j] = false;
            }
        }
    }
    is_prime
        .iter()
        .enumerate()
        .filter_map(|(i, &b)| if b { Some(i) } else { None })
        .collect()
}

/// Returns a boolean sieve (useful for O(1) primality queries after O(n) build).
fn prime_sieve(limit: usize) -> Vec<bool> {
    if limit < 2 {
        return vec![false; limit + 1];
    }
    let mut is_prime = vec![true; limit + 1];
    is_prime[0] = false;
    is_prime[1] = false;
    let sqrt_limit = (limit as f64).sqrt() as usize;
    for p in 2..=sqrt_limit {
        if is_prime[p] {
            for j in (p * p..=limit).step_by(p) {
                is_prime[j] = false;
            }
        }
    }
    is_prime
}

/// Prime-counting function π(n): number of primes ≤ n.
fn prime_pi(n: usize) -> usize {
    prime_sieve(n).iter().filter(|&&b| b).count()
}

/// Segmented sieve: all primes in [lo, hi].
fn segmented_sieve(lo: usize, hi: usize) -> Vec<usize> {
    let base_primes = sieve((hi as f64).sqrt() as usize + 1);
    let size = hi - lo + 1;
    let mut is_prime = vec![true; size];
    // 0 and 1 are not prime
    if lo == 0 { is_prime[0] = false; }
    if lo <= 1 && 1 <= hi { is_prime[1 - lo] = false; }

    for p in base_primes {
        // First multiple of p in [lo, hi]
        let start = (lo + p - 1) / p * p;
        let start = if start == p { p + p } else { start };
        if start > hi { continue; }
        for j in (start..=hi).step_by(p) {
            is_prime[j - lo] = false;
        }
    }
    is_prime
        .iter()
        .enumerate()
        .filter_map(|(i, &b)| if b { Some(lo + i) } else { None })
        .collect()
}

fn main() {
    let primes = sieve(50);
    println!("Primes ≤ 50: {:?}", primes);
    println!("π(100) = {} (expected 25)", prime_pi(100));
    println!("π(1000) = {} (expected 168)", prime_pi(1000));

    let seg = segmented_sieve(10, 50);
    println!("Primes in [10,50]: {:?}", seg);

    // Quick primality check using sieve
    let sieve_100 = prime_sieve(100);
    println!("Is 97 prime? {}", sieve_100[97]);
    println!("Is 100 prime? {}", sieve_100[100]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primes_to_30() {
        assert_eq!(sieve(30), vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29]);
    }

    #[test]
    fn test_prime_pi_25() {
        assert_eq!(prime_pi(100), 25);
    }

    #[test]
    fn test_prime_pi_168() {
        assert_eq!(prime_pi(1000), 168);
    }

    #[test]
    fn test_no_primes_below_2() {
        assert_eq!(sieve(1), vec![]);
        assert_eq!(sieve(0), vec![]);
    }

    #[test]
    fn test_sieve_2() {
        assert_eq!(sieve(2), vec![2]);
    }

    #[test]
    fn test_segmented_matches_basic() {
        let full = sieve(100);
        let seg: Vec<usize> = full.into_iter().filter(|&p| p >= 10).collect();
        assert_eq!(segmented_sieve(10, 100), seg);
    }

    #[test]
    fn test_97_is_prime() {
        let s = prime_sieve(100);
        assert!(s[97]);
        assert!(!s[98]);
        assert!(!s[99]);
        assert!(!s[100]);
    }

    #[test]
    fn test_twin_primes() {
        // 11,13 — 17,19 — 29,31 all twin primes ≤ 32
        let s = prime_sieve(32);
        assert!(s[11] && s[13]);
        assert!(s[17] && s[19]);
        assert!(s[29] && s[31]);
    }
}
