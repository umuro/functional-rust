//! # Modular Arithmetic

pub fn mod_add(a: i64, b: i64, m: i64) -> i64 { ((a % m) + (b % m)) % m }
pub fn mod_sub(a: i64, b: i64, m: i64) -> i64 { ((a % m) - (b % m) + m) % m }
pub fn mod_mul(a: i64, b: i64, m: i64) -> i64 { ((a % m) * (b % m)) % m }
pub fn mod_pow(mut base: i64, mut exp: i64, m: i64) -> i64 {
    let mut result = 1;
    base %= m;
    while exp > 0 {
        if exp % 2 == 1 { result = result * base % m; }
        exp /= 2;
        base = base * base % m;
    }
    result
}
pub fn mod_inv(a: i64, m: i64) -> i64 { mod_pow(a, m - 2, m) } // Fermat's little theorem

#[cfg(test)]
mod tests {
    use super::*;
    const M: i64 = 1_000_000_007;
    #[test]
    fn test_ops() {
        assert_eq!(mod_add(5, 3, 7), 1);
        assert_eq!(mod_pow(2, 10, M), 1024);
    }
    #[test]
    fn test_inv() { let inv = mod_inv(3, M); assert_eq!(mod_mul(3, inv, M), 1); }
}
