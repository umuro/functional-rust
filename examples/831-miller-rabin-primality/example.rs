/// Miller-Rabin Probabilistic Primality Test.
///
/// Deterministic for all u64 using witness set {2,3,5,7,11,13,17,19,23,29,31,37}.

fn mulmod(a: u64, b: u64, m: u64) -> u64 {
    (a as u128 * b as u128 % m as u128) as u64
}

fn pow_mod(mut base: u64, mut exp: u64, m: u64) -> u64 {
    let mut result = 1u64;
    base %= m;
    while exp > 0 {
        if exp & 1 == 1 { result = mulmod(result, base, m); }
        base = mulmod(base, base, m);
        exp >>= 1;
    }
    result
}

/// Test if `a` is a Miller-Rabin witness for composite n.
/// Returns true if n is probably prime with respect to witness a.
fn miller_witness(n: u64, d: u64, s: u32, a: u64) -> bool {
    let mut x = pow_mod(a, d, n);
    if x == 1 || x == n - 1 {
        return true;
    }
    for _ in 1..s {
        x = mulmod(x, x, n);
        if x == n - 1 {
            return true;
        }
    }
    false
}

/// Deterministic Miller-Rabin primality test for all u64.
pub fn is_prime(n: u64) -> bool {
    match n {
        0 | 1 => false,
        2 | 3 | 5 | 7 => true,
        _ if n % 2 == 0 || n % 3 == 0 => false,
        _ => {
            // Factor n-1 = 2^s * d where d is odd
            let mut d = n - 1;
            let s = d.trailing_zeros();
            d >>= s;

            // Deterministic witness set for all n < 3.3 × 10^24
            const WITNESSES: &[u64] = &[2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37];
            WITNESSES.iter().all(|&a| {
                a >= n || miller_witness(n, d, s, a)
            })
        }
    }
}

/// Generate primes up to limit using Miller-Rabin (educational comparison).
fn primes_up_to(limit: u64) -> Vec<u64> {
    (2..=limit).filter(|&n| is_prime(n)).collect()
}

fn main() {
    let primes    = [2u64, 3, 5, 17, 97, 997, 1_000_003, 999_999_937];
    let composites = [4u64, 9, 15, 100, 1001, 1_000_000];

    for &n in &primes {
        println!("is_prime({n}) = {} (expected true)", is_prime(n));
    }
    for &n in &composites {
        println!("is_prime({n}) = {} (expected false)", is_prime(n));
    }

    // Carmichael number 561 = 3×11×17 — fools Fermat but not Miller-Rabin
    println!("is_prime(561) = {} (Carmichael, expected false)", is_prime(561));

    // Large semiprime
    let p: u64 = 1_000_000_007;
    let q: u64 = 1_000_000_009;
    println!("is_prime(p={p}) = {}", is_prime(p));
    println!("is_prime(p*q={}) = {}", is_prime(p * q));

    let small_primes = primes_up_to(50);
    println!("Primes ≤ 50: {small_primes:?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_small_primes() {
        let expected = [2u64, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47];
        for &p in &expected {
            assert!(is_prime(p), "{p} should be prime");
        }
    }

    #[test]
    fn test_composites() {
        for &n in &[0u64, 1, 4, 6, 8, 9, 10, 15, 25, 100] {
            assert!(!is_prime(n), "{n} should be composite");
        }
    }

    #[test]
    fn test_carmichael() {
        // 561, 1105, 1729 are Carmichael numbers
        assert!(!is_prime(561));
        assert!(!is_prime(1105));
        assert!(!is_prime(1729));
    }

    #[test]
    fn test_large_prime() {
        assert!(is_prime(999_999_937));
        assert!(is_prime(1_000_000_007));
        assert!(is_prime(1_000_000_009));
    }

    #[test]
    fn test_semiprime_not_prime() {
        assert!(!is_prime(1_000_000_007 * 3));
    }

    #[test]
    fn test_matches_sieve() {
        // Verify Miller-Rabin matches sieve for n ≤ 1000
        let mut sieve = vec![true; 1001];
        sieve[0] = false;
        sieve[1] = false;
        for i in 2..=31 {
            if sieve[i] {
                let mut j = i * i;
                while j <= 1000 { sieve[j] = false; j += i; }
            }
        }
        for n in 0..=1000u64 {
            assert_eq!(is_prime(n), sieve[n as usize],
                "mismatch at n={n}");
        }
    }
}
