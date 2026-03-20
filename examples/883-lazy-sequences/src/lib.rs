#![allow(clippy::all)]
// Example 089: Lazy Sequences
// OCaml Seq → Rust Iterator + take

// === Approach 1: Infinite iterators with closures ===
fn naturals() -> impl Iterator<Item = u64> {
    0u64..
}

fn squares() -> impl Iterator<Item = u64> {
    naturals().map(|n| n * n)
}

fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    let mut d = 2;
    while d * d <= n {
        if n % d == 0 {
            return false;
        }
        d += 1;
    }
    true
}

fn primes() -> impl Iterator<Item = u64> {
    naturals().filter(|&n| is_prime(n))
}

// === Approach 2: Custom lazy generators ===
fn powers_of(base: u64) -> impl Iterator<Item = u64> {
    std::iter::successors(Some(1u64), move |&prev| prev.checked_mul(base))
}

fn triangle_numbers() -> impl Iterator<Item = u64> {
    naturals().skip(1).scan(0u64, |acc, n| {
        *acc += n;
        Some(*acc)
    })
}

// === Approach 3: take_while / skip_while ===
fn primes_below(limit: u64) -> Vec<u64> {
    primes().take_while(|&p| p < limit).collect()
}

fn first_prime_over(threshold: u64) -> Option<u64> {
    primes().find(|&p| p > threshold)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_naturals() {
        let v: Vec<u64> = naturals().take(5).collect();
        assert_eq!(v, vec![0, 1, 2, 3, 4]);
    }

    #[test]
    fn test_squares() {
        let v: Vec<u64> = squares().take(5).collect();
        assert_eq!(v, vec![0, 1, 4, 9, 16]);
    }

    #[test]
    fn test_primes() {
        let v: Vec<u64> = primes().take(5).collect();
        assert_eq!(v, vec![2, 3, 5, 7, 11]);
    }

    #[test]
    fn test_powers_of_2() {
        let v: Vec<u64> = powers_of(2).take(5).collect();
        assert_eq!(v, vec![1, 2, 4, 8, 16]);
    }

    #[test]
    fn test_powers_of_3() {
        let v: Vec<u64> = powers_of(3).take(4).collect();
        assert_eq!(v, vec![1, 3, 9, 27]);
    }

    #[test]
    fn test_triangle_numbers() {
        let v: Vec<u64> = triangle_numbers().take(5).collect();
        assert_eq!(v, vec![1, 3, 6, 10, 15]);
    }

    #[test]
    fn test_primes_below() {
        assert_eq!(primes_below(20), vec![2, 3, 5, 7, 11, 13, 17, 19]);
    }

    #[test]
    fn test_first_prime_over() {
        assert_eq!(first_prime_over(100), Some(101));
    }

    #[test]
    fn test_lazy_composition() {
        let count = naturals().filter(|n| n % 2 == 0).take(100).count();
        assert_eq!(count, 100);
    }
}
