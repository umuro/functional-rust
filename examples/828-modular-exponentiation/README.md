📖 **[View on hightechmind.io →](https://hightechmind.io/rust/828-modular-exponentiation)**

---

# Modular Exponentiation
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  


## Problem Statement

Computing a^b mod m naively multiplies a together b times, requiring O(b) operations — impractical for b = 10^18. Fast modular exponentiation (binary exponentiation / repeated squaring) achieves O(log b) by decomposing the exponent in binary: if b is even, `a^b = (a^(b/2))^2`; if odd, `a^b = a * a^(b-1)`. This is the core operation in RSA encryption/decryption (a^e mod n, a^d mod n), Diffie-Hellman key exchange, primality testing (Fermat, Miller-Rabin), and computing large Fibonacci numbers via matrix exponentiation. Without it, public-key cryptography would be computationally infeasible.

## Learning Outcomes

- Implement binary exponentiation iteratively: scan bits of exponent from LSB to MSB
- Understand why O(log b) squarings suffice: the exponent halves at each step
- Extend to matrix exponentiation: same algorithm works with matrix multiplication replacing integer multiplication
- Apply Fermat's little theorem: `a^(p-1) ≡ 1 (mod p)` for prime p implies `a^(-1) ≡ a^(p-2) (mod p)`
- Recognize the connection: fast power → modular inverse → modular division

## Rust Application

```rust
pub fn mod_pow(mut base: u64, mut exp: u64, modulus: u64) -> u64 {
    let mut result = 1u64;
    base %= modulus;
    while exp > 0 {
        if exp % 2 == 1 {
            result = (result as u128 * base as u128 % modulus as u128) as u64;
        }
        exp /= 2;
        base = (base as u128 * base as u128 % modulus as u128) as u64;
    }
    result
}
```

Using `u128` for intermediate products prevents overflow when `modulus` is up to `2^63 - 1`. The iterative approach scans the exponent from the least significant bit: at each step, if the current bit is 1, multiply result by the current base; always square the base and halve the exponent. Rust's `as u128` cast is explicit and clear about the widening intent. The function handles the edge case `exp = 0` naturally (returns 1 immediately). The variable name `modulus` is clearer than `m` in library code.

## OCaml Approach

OCaml's `mod_pow` is naturally recursive: `if exp = 0 then 1 else if exp mod 2 = 0 then let h = mod_pow base (exp/2) m in h * h mod m else base * mod_pow base (exp-1) m mod m`. For tail-recursive efficiency, the accumulator version threads the result: `let rec go acc base exp m`. OCaml's `Int64.mul` handles 64-bit products; for moduli requiring 128-bit, `Zarith.mpz` is used. Matrix exponentiation extends naturally with OCaml's algebraic types for `2x2` or `NxN` matrices.

## Key Differences

| Aspect | Rust | OCaml |
|---|---|---|
| Overflow handling | Explicit `as u128` cast | `Int64` or `Zarith` |
| Style | Iterative while loop | Recursive or iterative |
| Edge cases | `exp = 0` → returns 1 naturally | Same |
| Matrix variant | Generic `fn mat_pow<T: Mul>` | Functor over ring type |
| Modular inverse | `mod_pow(a, p-2, p)` for prime p | Same |
| Largest safe modulus | ~9.2 * 10^18 (u64) | ~4.6 * 10^18 (63-bit int) |

## Exercises

1. Implement matrix exponentiation to compute the nth Fibonacci number in O(log n) using 2×2 matrix multiplication.
2. Use `mod_pow(a, p-2, p)` to compute modular inverse and verify `a * a^(p-2) ≡ 1 (mod p)` for several values.
3. Implement the Fermat primality test using `mod_pow`: test if `a^(n-1) ≡ 1 (mod n)` for multiple bases a.
4. Compute `2^(10^18) mod (10^9 + 7)` and verify the result using the property `2^p-1 ≡ 1 (mod p)` for Mersenne primes.
5. Generalize to a `Ring` trait and implement `mod_pow` generically over any ring (integers, matrices, polynomials).
