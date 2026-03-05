//! # GCD and LCM
//! Euclidean algorithm

pub fn gcd(a: u64, b: u64) -> u64 { if b == 0 { a } else { gcd(b, a % b) } }
pub fn gcd_iter(mut a: u64, mut b: u64) -> u64 { while b != 0 { let t = b; b = a % b; a = t; } a }
pub fn lcm(a: u64, b: u64) -> u64 { a / gcd(a, b) * b }
pub fn extended_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    if b == 0 { (a, 1, 0) }
    else { let (g, x, y) = extended_gcd(b, a % b); (g, y, x - (a / b) * y) }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_gcd() { assert_eq!(gcd(48, 18), 6); assert_eq!(gcd_iter(48, 18), 6); }
    #[test]
    fn test_lcm() { assert_eq!(lcm(4, 6), 12); }
    #[test]
    fn test_ext_gcd() { let (g, x, y) = extended_gcd(35, 15); assert_eq!(g, 5); assert_eq!(35*x + 15*y, 5); }
}
