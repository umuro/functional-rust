//! # Number Theory Overview

pub fn gcd(a: u64, b: u64) -> u64 { if b == 0 { a } else { gcd(b, a % b) } }
pub fn lcm(a: u64, b: u64) -> u64 { a / gcd(a, b) * b }
pub fn is_prime(n: u64) -> bool {
    if n < 2 { return false; }
    if n == 2 { return true; }
    if n % 2 == 0 { return false; }
    let mut i = 3;
    while i * i <= n { if n % i == 0 { return false; } i += 2; }
    true
}
pub fn mod_pow(mut base: u64, mut exp: u64, modulo: u64) -> u64 {
    let mut result = 1;
    base %= modulo;
    while exp > 0 {
        if exp % 2 == 1 { result = result * base % modulo; }
        exp /= 2;
        base = base * base % modulo;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_gcd() { assert_eq!(gcd(48, 18), 6); }
    #[test]
    fn test_prime() { assert!(is_prime(17)); assert!(!is_prime(15)); }
    #[test]
    fn test_mod_pow() { assert_eq!(mod_pow(2, 10, 1000), 24); }
}
