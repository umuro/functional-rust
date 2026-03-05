/// Modular Arithmetic: add, sub, mul, inverse, pow.
///
/// ModInt wraps a value in [0, modulus) and implements arithmetic operators.

use std::ops::{Add, Sub, Mul};

const MOD: u64 = 1_000_000_007;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct ModInt {
    v: u64,
    m: u64,
}

impl ModInt {
    fn new(v: i64, m: u64) -> Self {
        let v = ((v % m as i64) + m as i64) as u64 % m;
        ModInt { v, m }
    }

    /// Modular exponentiation: self^exp mod m. O(log exp).
    fn pow(self, mut exp: u64) -> Self {
        let mut base = self;
        let mut result = ModInt::new(1, self.m);
        while exp > 0 {
            if exp & 1 == 1 {
                result = result * base;
            }
            base = base * base;
            exp >>= 1;
        }
        result
    }

    /// Modular inverse via Fermat's little theorem (m must be prime).
    fn inv_fermat(self) -> Self {
        assert!(self.v != 0, "no inverse for 0");
        self.pow(self.m - 2)
    }

    /// Modular inverse via Extended Euclidean (works for any coprime v, m).
    fn inv(self) -> Option<Self> {
        let (g, x, _) = extended_gcd(self.v as i64, self.m as i64);
        if g != 1 {
            None
        } else {
            Some(ModInt::new(x, self.m))
        }
    }
}

impl Add for ModInt {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        ModInt { v: (self.v + rhs.v) % self.m, m: self.m }
    }
}

impl Sub for ModInt {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        ModInt { v: (self.v + self.m - rhs.v) % self.m, m: self.m }
    }
}

impl Mul for ModInt {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        // Widen to u128 to prevent overflow
        let v = (self.v as u128 * rhs.v as u128 % self.m as u128) as u64;
        ModInt { v, m: self.m }
    }
}

impl std::fmt::Display for ModInt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.v)
    }
}

/// Extended GCD: returns (gcd, x, y) where ax + by = gcd.
fn extended_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    if b == 0 {
        (a, 1, 0)
    } else {
        let (g, x, y) = extended_gcd(b, a % b);
        (g, y, x - (a / b) * y)
    }
}

/// Standalone modular inverse using Extended Euclidean.
fn mod_inv(a: u64, m: u64) -> Option<u64> {
    let (g, x, _) = extended_gcd(a as i64, m as i64);
    if g != 1 {
        None
    } else {
        Some(((x % m as i64 + m as i64) as u64) % m)
    }
}

fn main() {
    let a = ModInt::new(3, 7);
    let b = ModInt::new(5, 7);
    println!("3 + 5 mod 7 = {}", a + b);  // 1
    println!("3 - 5 mod 7 = {}", a - b);  // 5
    println!("3 * 5 mod 7 = {}", a * b);  // 1

    println!("\n--- MOD = {} ---", MOD);
    let x = ModInt::new(999_999_999, MOD);
    let y = ModInt::new(9, MOD);
    println!("999_999_999 + 9 mod MOD = {}", x + y); // 1

    let two = ModInt::new(2, MOD);
    println!("2^10 mod MOD = {}", two.pow(10)); // 1024

    // Modular inverse: 3 * inv(3) ≡ 1 mod 7
    let three = ModInt::new(3, 7);
    let inv3 = three.inv_fermat();
    println!("\ninv(3) mod 7 = {} (expected 5)", inv3);
    println!("3 * 5 mod 7 = {}", three * inv3); // 1

    // Extended Euclidean inverse (general)
    println!("mod_inv(3, 7) = {:?}", mod_inv(3, 7)); // Some(5)
    println!("mod_inv(2, 4) = {:?}", mod_inv(2, 4)); // None (gcd=2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_wrap() {
        let a = ModInt::new(6, 7);
        let b = ModInt::new(5, 7);
        assert_eq!((a + b).v, 4); // 11 % 7 = 4
    }

    #[test]
    fn test_sub_negative() {
        let a = ModInt::new(3, 10);
        let b = ModInt::new(7, 10);
        assert_eq!((a - b).v, 6); // 3-7=-4 → 6 mod 10
    }

    #[test]
    fn test_mul_overflow_safe() {
        // Large multiplication shouldn't overflow
        let a = ModInt::new(999_999_999, MOD);
        let b = ModInt::new(999_999_999, MOD);
        assert_eq!((a * b).v, (999_999_999u128 * 999_999_999 % MOD as u128) as u64);
    }

    #[test]
    fn test_pow_2_10() {
        assert_eq!(ModInt::new(2, MOD).pow(10).v, 1024);
    }

    #[test]
    fn test_inv_fermat() {
        let inv = ModInt::new(3, 7).inv_fermat();
        assert_eq!(inv.v, 5); // 3*5=15≡1 mod 7
    }

    #[test]
    fn test_inv_verify() {
        for a in 1u64..7 {
            let m = ModInt::new(a as i64, 7);
            let inv = m.inv_fermat();
            assert_eq!((m * inv).v, 1, "inv({a}) mod 7 failed");
        }
    }

    #[test]
    fn test_mod_inv_no_inverse() {
        assert_eq!(mod_inv(2, 4), None); // gcd(2,4)=2
        assert_eq!(mod_inv(6, 9), None); // gcd(6,9)=3
    }

    #[test]
    fn test_negative_input() {
        // Negative values should be normalised
        let a = ModInt::new(-1, 7);
        assert_eq!(a.v, 6);
    }
}
