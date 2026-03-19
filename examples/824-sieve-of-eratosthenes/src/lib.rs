//! # Sieve of Eratosthenes
pub fn sieve(n: usize) -> Vec<bool> {
    let mut is_prime = vec![true; n + 1];
    is_prime[0] = false;
    if n >= 1 {
        is_prime[1] = false;
    }
    for i in 2..=((n as f64).sqrt() as usize) {
        if is_prime[i] {
            for j in (i * i..=n).step_by(i) {
                is_prime[j] = false;
            }
        }
    }
    is_prime
}

pub fn primes_up_to(n: usize) -> Vec<usize> {
    sieve(n)
        .iter()
        .enumerate()
        .filter_map(|(i, &b)| if b { Some(i) } else { None })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sieve() {
        assert_eq!(primes_up_to(20), vec![2, 3, 5, 7, 11, 13, 17, 19]);
    }
}
