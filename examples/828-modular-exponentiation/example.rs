/// Fast Modular Exponentiation — binary (square-and-multiply) method.
/// Computes base^exp mod m in O(log exp) multiplications.

/// Iterative binary exponentiation. O(log exp).
fn pow_mod(mut base: u64, mut exp: u64, m: u64) -> u64 {
    if m == 1 { return 0; }
    let mut result = 1u64;
    base %= m;
    while exp > 0 {
        if exp & 1 == 1 {
            result = (result as u128 * base as u128 % m as u128) as u64;
        }
        base = (base as u128 * base as u128 % m as u128) as u64;
        exp >>= 1;
    }
    result
}

/// Recursive binary exponentiation — mirrors OCaml's natural style.
fn pow_mod_rec(base: u64, exp: u64, m: u64) -> u64 {
    if m == 1 { return 0; }
    match exp {
        0 => 1 % m,
        e if e & 1 == 1 => (base as u128 * pow_mod_rec(base, exp - 1, m) as u128 % m as u128) as u64,
        _ => {
            let half = pow_mod_rec(base, exp / 2, m);
            (half as u128 * half as u128 % m as u128) as u64
        }
    }
}

/// 2×2 matrix for matrix exponentiation (e.g., fast Fibonacci).
#[derive(Clone, Copy)]
struct Matrix {
    a: [[u64; 2]; 2],
}

impl Matrix {
    fn identity() -> Self {
        Matrix { a: [[1, 0], [0, 1]] }
    }

    fn mul(self, rhs: Self, m: u64) -> Self {
        let mut res = [[0u64; 2]; 2];
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    res[i][j] = (res[i][j] + self.a[i][k] as u128 * rhs.a[k][j] as u128 % m as u128) as u64 % m;
                }
            }
        }
        Matrix { a: res }
    }

    fn pow(self, mut exp: u64, m: u64) -> Self {
        let mut result = Self::identity();
        let mut base = self;
        while exp > 0 {
            if exp & 1 == 1 {
                result = result.mul(base, m);
            }
            base = base.mul(base, m);
            exp >>= 1;
        }
        result
    }
}

/// Fibonacci F(n) mod p in O(log n) using matrix exponentiation.
fn fib_mod(n: u64, p: u64) -> u64 {
    if n == 0 { return 0; }
    let m = Matrix { a: [[1, 1], [1, 0]] };
    m.pow(n - 1, p).a[0][0]
}

fn main() {
    println!("2^10 mod 1000 = {} (expected 24)", pow_mod(2, 10, 1000));
    println!("3^100 mod 1_000_000_007 = {}", pow_mod(3, 100, 1_000_000_007));
    println!("2^1000 mod 1009 = {}", pow_mod(2, 1000, 1009));
    println!("recursive: 2^10 mod 1000 = {}", pow_mod_rec(2, 10, 1000));

    println!("\nFibonacci via matrix exponentiation:");
    println!("fib(10) = {} (expected 55)", fib_mod(10, u64::MAX));
    println!("fib(10) mod 100 = {} (expected 55)", fib_mod(10, 100));
    println!("fib(100) mod 1_000_000_007 = {}", fib_mod(100, 1_000_000_007));
    println!("fib(1_000_000) mod 1_000_000_007 = {}", fib_mod(1_000_000, 1_000_000_007));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pow_mod_basic() {
        assert_eq!(pow_mod(2, 10, 1000), 24);     // 1024 mod 1000
        assert_eq!(pow_mod(2, 0, 7), 1);           // anything^0 = 1
        assert_eq!(pow_mod(0, 5, 7), 0);           // 0^anything = 0
    }

    #[test]
    fn test_pow_mod_prime_modulus() {
        // Fermat's little theorem: a^(p-1) ≡ 1 (mod p)
        let p = 1_000_000_007u64;
        assert_eq!(pow_mod(3, p - 1, p), 1);
        assert_eq!(pow_mod(7, p - 1, p), 1);
    }

    #[test]
    fn test_pow_mod_m1() {
        assert_eq!(pow_mod(100, 100, 1), 0);
    }

    #[test]
    fn test_recursive_matches_iterative() {
        for base in 0..10u64 {
            for exp in 0..15u64 {
                let m = 1009u64;
                assert_eq!(pow_mod(base, exp, m), pow_mod_rec(base, exp, m),
                    "mismatch at {base}^{exp} mod {m}");
            }
        }
    }

    #[test]
    fn test_fib_mod_basic() {
        // F(0)=0, F(1)=1, F(2)=1, F(10)=55
        assert_eq!(fib_mod(0, u64::MAX), 0);
        assert_eq!(fib_mod(1, u64::MAX), 1);
        assert_eq!(fib_mod(10, u64::MAX), 55);
    }

    #[test]
    fn test_fib_mod_large() {
        // Known: F(100) mod 10^9+7 = 687995182
        assert_eq!(fib_mod(100, 1_000_000_007), 687_995_182);
    }

    #[test]
    fn test_large_exponent() {
        // 2^62 mod (2^63-1) — stress test large values
        let result = pow_mod(2, 62, (1u64 << 63) - 1);
        assert_eq!(result, 1u64 << 62);
    }
}
