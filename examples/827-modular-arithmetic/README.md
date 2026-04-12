📖 **[View on hightechmind.io →](https://hightechmind.io/rust/827-modular-arithmetic)**

---

# Modular Arithmetic
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  



## Problem Statement

When computing large combinatorial values (n! mod p, binomial coefficients, number of paths), intermediate results overflow 64-bit integers long before the final answer. Modular arithmetic performs all operations in a finite field Z/pZ, keeping values bounded. Computing `(a + b) % m`, `(a * b) % m`, `pow(a, b, m)`, and `inv(a, m)` are the building blocks of competitive programming, cryptography, and number theory. The key insight: `(a * b) % m = ((a % m) * (b % m)) % m`. Modular exponentiation (fast power) runs in O(log b) and is essential for RSA, Diffie-Hellman, and primality testing.

## Learning Outcomes

- Implement modular addition, subtraction, and multiplication with proper overflow prevention
- Implement fast modular exponentiation via repeated squaring in O(log exp) time
- Understand modular inverse and when it exists (only when gcd(a, m) = 1)
- Compute factorial and binomial coefficients modulo a prime efficiently
- Recognize when to use Fermat's little theorem vs. extended Euclidean for modular inverse

## Rust Application

```rust
pub fn mod_add(a: u64, b: u64, m: u64) -> u64 { (a + b) % m }
pub fn mod_mul(a: u64, b: u64, m: u64) -> u64 { (a % m) * (b % m) % m }
pub fn mod_pow(mut base: u64, mut exp: u64, m: u64) -> u64 {
    let mut result = 1u64;
    base %= m;
    while exp > 0 {
        if exp & 1 == 1 { result = result * base % m; }
        base = base * base % m;
        exp >>= 1;
    }
    result
}
```

Rust's `u64` multiplication can overflow for moduli above 2^32, requiring `u128` for intermediate products: `((a as u128 * b as u128) % m as u128) as u64`. The `wrapping_mul` method is not appropriate here — we want the exact mathematical result mod m. `mod_pow` uses the binary exponentiation algorithm: at each bit of `exp`, square the base and optionally multiply into result. The bit-test `exp & 1 == 1` and right-shift `exp >>= 1` are cache-friendly and branch-predictable.

## OCaml Approach

OCaml's `int` is 63-bit on 64-bit systems, allowing products up to about 4.6 * 10^18. For moduli up to 2^30, `a * b mod m` stays in bounds. For larger moduli, OCaml uses `Int64` or `Zarith`. Modular exponentiation is a clean tail-recursive function: `let rec pow_mod base exp m = if exp = 0 then 1 else let half = pow_mod (base * base mod m) (exp / 2) m in if exp mod 2 = 0 then half else half * base mod m`. OCaml's `Fun.protect` ensures cleanup in modular operations with side effects.

## Key Differences

| Aspect | Rust | OCaml |
|---|---|---|
| Overflow risk | `u64 * u64` overflows above 2^32 mod | `int * int` overflows above 2^30 mod |
| Large modulus | Use `u128` intermediate | Use `Int64` or `Zarith` |
| Mod exponentiation | Iterative with bit manipulation | Recursive or iterative |
| Modular inverse | `mod_pow(a, m-2, m)` for prime m | Same via Fermat |
| Binomial coefficients | Precompute factorial array mod p | Same approach |
| Newtype safety | `struct Mod<const M: u64>(u64)` | `type modint = int` (no enforcement) |

## Exercises

1. Implement a `ModInt<const M: u64>` newtype that overloads arithmetic operators to auto-apply modular reduction.
2. Precompute factorial and inverse factorial tables mod p to answer n-choose-k queries in O(1).
3. Implement Lucas' theorem for computing binomial coefficients mod a prime p for very large n, k.
4. Use modular arithmetic to verify matrix multiplication: `(A * B) % m = ((A % m) * (B % m)) % m`.
5. Benchmark `u64` vs. `u128` intermediate for modular multiplication and measure the throughput difference.
