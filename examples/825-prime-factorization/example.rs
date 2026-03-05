/// Prime Factorization: trial division + Pollard's rho concept.
/// Trial division handles small factors; Pollard's rho concept for large n.

/// GCD (Euclidean) — mirrors OCaml's elegant one-liner.
fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 { a } else { gcd(b, a % b) }
}

/// Trial division: returns sorted list of (prime, exponent) pairs.
fn factorize(mut n: u64) -> Vec<(u64, u32)> {
    let mut factors = Vec::new();
    // Factor out 2
    if n % 2 == 0 {
        let mut exp = 0u32;
        while n % 2 == 0 { exp += 1; n /= 2; }
        factors.push((2, exp));
    }
    // Odd factors from 3
    let mut d = 3u64;
    while d * d <= n {
        if n % d == 0 {
            let mut exp = 0u32;
            while n % d == 0 { exp += 1; n /= d; }
            factors.push((d, exp));
        }
        d += 2;
    }
    if n > 1 {
        factors.push((n, 1));
    }
    factors
}

/// Pollard's rho: find a non-trivial factor of n (n composite, n > 1).
/// Uses Floyd's cycle detection with f(x) = (x² + c) mod n.
fn pollard_rho(n: u64) -> u64 {
    if n % 2 == 0 {
        return 2;
    }
    let mut c = 1u64;
    loop {
        let mut x = 2u64;
        let mut y = 2u64;
        let mut d = 1u64;
        while d == 1 {
            x = (x.wrapping_mul(x).wrapping_add(c)) % n;
            y = (y.wrapping_mul(y).wrapping_add(c)) % n;
            y = (y.wrapping_mul(y).wrapping_add(c)) % n;
            d = gcd(x.abs_diff(y), n);
        }
        if d != n {
            return d;
        }
        c += 1; // Retry
    }
}

/// Miller-Rabin primality (simple version for testing in Pollard context).
fn is_prime_trial(n: u64) -> bool {
    if n < 2 { return false; }
    if n == 2 { return true; }
    if n % 2 == 0 { return false; }
    let mut d = 3u64;
    while d * d <= n {
        if n % d == 0 { return false; }
        d += 2;
    }
    true
}

/// Full factorization using Pollard's rho for large primes.
fn factorize_full(n: u64) -> Vec<u64> {
    if n <= 1 { return vec![]; }
    if is_prime_trial(n) { return vec![n]; }
    let d = pollard_rho(n);
    let mut a = factorize_full(d);
    let mut b = factorize_full(n / d);
    a.append(&mut b);
    a.sort();
    a
}

fn format_factors(factors: &[(u64, u32)]) -> String {
    factors
        .iter()
        .map(|&(p, e)| if e == 1 { format!("{p}") } else { format!("{p}^{e}") })
        .collect::<Vec<_>>()
        .join(" × ")
}

fn main() {
    let tests = [12u64, 100, 360, 97, 1_234_567_890, 720_720];
    for &n in &tests {
        let f = factorize(n);
        println!("factorize({n}) = {}", format_factors(&f));
    }

    println!("\nPollard rho examples:");
    for &n in &[15u64, 35, 77, 8051, 1_000_003 * 1_000_033] {
        let factors = factorize_full(n);
        println!("  {n} = {:?}", factors);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factorize_12() {
        assert_eq!(factorize(12), vec![(2, 2), (3, 1)]);
    }

    #[test]
    fn test_factorize_100() {
        assert_eq!(factorize(100), vec![(2, 2), (5, 2)]);
    }

    #[test]
    fn test_factorize_prime() {
        assert_eq!(factorize(97), vec![(97, 1)]);
    }

    #[test]
    fn test_factorize_1() {
        assert_eq!(factorize(1), vec![]);
    }

    #[test]
    fn test_factorize_360() {
        // 360 = 2³ × 3² × 5
        assert_eq!(factorize(360), vec![(2, 3), (3, 2), (5, 1)]);
    }

    #[test]
    fn test_product_check() {
        // Verify factors multiply back to original
        for &n in &[12u64, 100, 360, 720720] {
            let factors = factorize(n);
            let product: u64 = factors.iter().map(|&(p, e)| p.pow(e)).product();
            assert_eq!(product, n, "product mismatch for {n}");
        }
    }

    #[test]
    fn test_pollard_finds_factor() {
        // 35 = 5 × 7
        let d = pollard_rho(35);
        assert!(d == 5 || d == 7);
    }

    #[test]
    fn test_is_prime_trial() {
        assert!(is_prime_trial(97));
        assert!(!is_prime_trial(100));
        assert!(is_prime_trial(2));
    }
}
