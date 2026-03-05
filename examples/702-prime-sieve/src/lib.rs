//! # Prime Sieve
//! Sieve of Eratosthenes O(n log log n)

pub fn sieve(n: usize) -> Vec<bool> {
    let mut is_prime = vec![true; n + 1];
    is_prime[0] = false;
    if n >= 1 { is_prime[1] = false; }
    
    let mut i = 2;
    while i * i <= n {
        if is_prime[i] { for j in (i * i..=n).step_by(i) { is_prime[j] = false; } }
        i += 1;
    }
    is_prime
}

pub fn primes_up_to(n: usize) -> Vec<usize> {
    sieve(n).iter().enumerate().filter(|(_, &p)| p).map(|(i, _)| i).collect()
}

pub fn count_primes(n: usize) -> usize { sieve(n).iter().filter(|&&p| p).count() }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sieve() { assert_eq!(primes_up_to(20), vec![2, 3, 5, 7, 11, 13, 17, 19]); }
    #[test]
    fn test_count() { assert_eq!(count_primes(100), 25); }
}
