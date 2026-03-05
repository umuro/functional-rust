/// Euler's Totient Function φ(n).
///
/// φ(n) = count of k in [1,n] with gcd(k,n) = 1.
/// Formula: φ(n) = n × ∏_{p|n} (1 - 1/p).

/// Single value: O(√n).
fn totient(mut n: u64) -> u64 {
    let mut result = n;
    let mut p = 2u64;
    while p * p <= n {
        if n % p == 0 {
            while n % p == 0 {
                n /= p;
            }
            result -= result / p; // result *= (p-1)/p
        }
        p += 1;
    }
    if n > 1 {
        result -= result / n; // remaining prime factor
    }
    result
}

/// Totient sieve: compute φ(i) for all i in [0, limit]. O(n log log n).
fn totient_sieve(limit: usize) -> Vec<u64> {
    let mut phi: Vec<u64> = (0..=limit as u64).collect();
    for i in 2..=limit {
        if phi[i] == i as u64 {
            // i is prime — update all multiples
            let mut j = i;
            while j <= limit {
                phi[j] -= phi[j] / i as u64;
                j += i;
            }
        }
    }
    phi
}

/// Property: sum of φ(d) over all divisors d of n equals n.
fn sum_divisor_totients(n: u64) -> u64 {
    let phi = totient_sieve(n as usize);
    (1..=n).filter(|&d| n % d == 0).map(|d| phi[d as usize]).sum()
}

fn main() {
    let values = [1u64, 2, 6, 9, 10, 12, 36, 100];
    for &n in &values {
        println!("φ({n}) = {}", totient(n));
    }

    println!("\nTotient sieve [1..12]:");
    let phi = totient_sieve(12);
    for (i, &v) in phi.iter().enumerate().skip(1) {
        println!("  φ({i}) = {v}");
    }

    println!("\nProperty ∑_{{d|n}} φ(d) = n:");
    for n in [6u64, 12, 30] {
        let s = sum_divisor_totients(n);
        println!("  n={n}: sum={s}, match={}", s == n);
    }

    // φ(p) = p-1 for primes
    println!("\nPrime totients:");
    for p in [2u64, 3, 5, 7, 11, 13] {
        println!("  φ({p}) = {} (= p-1 = {})", totient(p), p - 1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_totient_known() {
        assert_eq!(totient(1), 1);
        assert_eq!(totient(2), 1);
        assert_eq!(totient(6), 2);   // coprime: 1, 5
        assert_eq!(totient(9), 6);   // coprime: 1,2,4,5,7,8
        assert_eq!(totient(10), 4);  // coprime: 1,3,7,9
        assert_eq!(totient(12), 4);  // coprime: 1,5,7,11
    }

    #[test]
    fn test_prime_totient() {
        for p in [2u64, 3, 5, 7, 11, 13, 97] {
            assert_eq!(totient(p), p - 1, "φ({p}) should be p-1");
        }
    }

    #[test]
    fn test_prime_power() {
        // φ(p^k) = p^k - p^(k-1) = p^(k-1) * (p-1)
        assert_eq!(totient(8), 4);   // φ(2³) = 4
        assert_eq!(totient(9), 6);   // φ(3²) = 6
        assert_eq!(totient(25), 20); // φ(5²) = 20
    }

    #[test]
    fn test_sieve_matches_direct() {
        let phi = totient_sieve(100);
        for n in 1..=100u64 {
            assert_eq!(phi[n as usize], totient(n),
                "sieve[{n}] = {} but totient({n}) = {}", phi[n as usize], totient(n));
        }
    }

    #[test]
    fn test_divisor_sum_property() {
        for n in [6u64, 12, 30, 100] {
            assert_eq!(sum_divisor_totients(n), n, "sum_divisor_totients({n}) != {n}");
        }
    }

    #[test]
    fn test_multiplicative() {
        // φ is multiplicative: φ(mn) = φ(m)φ(n) when gcd(m,n)=1
        assert_eq!(totient(3 * 5), totient(3) * totient(5));
        assert_eq!(totient(4 * 9), totient(4) * totient(9));
    }
}
