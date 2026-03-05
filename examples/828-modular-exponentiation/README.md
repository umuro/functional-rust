# 828: Fast Modular Exponentiation

**Difficulty:** 3  **Level:** Intermediate

Compute a^b mod m in O(log b) by repeated squaring — the primitive operation behind RSA, Diffie-Hellman, Miller-Rabin, and Fermat's little theorem.

## The Problem This Solves

Naïve exponentiation multiplies `a` by itself `b` times: O(b) multiplications. For cryptographic exponents (b ≈ 2^2048 in RSA), this is completely infeasible. Fast exponentiation reduces this to O(log b) multiplications by exploiting the binary representation of the exponent: square when the current bit is 0, square-and-multiply when it's 1.

This single algorithm appears everywhere in cryptography: RSA encryption is `m^e mod n`, RSA decryption is `c^d mod n`, Diffie-Hellman key exchange is `g^a mod p`, Miller-Rabin primality tests compute `a^d mod n`, and Fermat's little theorem gives modular inverses as `a^(p-2) mod p`. If you understand one algorithm that enables modern cryptography, this is it.

The matrix variant extends the idea: `matrix^n mod m` in O(log n) matrix multiplications computes Fibonacci numbers, linear recurrences, and graph path counts in logarithmic time.

## The Intuition

Binary representation of b: 10 in binary is 1010. Reading bits right-to-left: always square, multiply when bit is 1. `a^10 = a^(8+2) = a^8 × a^2`. Each step: `result = result * base` if the low bit is 1, then `base = base * base`, then `b >>= 1`. Total: ⌊log₂ b⌋ squarings and at most as many multiplications.

Overflow: with 64-bit modulus, intermediate products need 128 bits. `(a as u128 * b as u128) % m as u128` is the idiomatic Rust pattern — hardware-supported on x86-64, no external library needed.

## How It Works in Rust

```rust
// Iterative: O(log exp), O(1) space
fn pow_mod(mut base: u64, mut exp: u64, m: u64) -> u64 {
    if m == 1 { return 0; }
    let mut result = 1u64;
    base %= m;                          // Reduce base immediately
    while exp > 0 {
        if exp & 1 == 1 {              // Low bit set: multiply into result
            result = (result as u128 * base as u128 % m as u128) as u64;
        }
        base = (base as u128 * base as u128 % m as u128) as u64;  // Square
        exp >>= 1;                      // Move to next bit
    }
    result
}

// Recursive: mirrors OCaml's natural style — easier to verify correct
fn pow_mod_rec(base: u64, exp: u64, m: u64) -> u64 {
    if m == 1 { return 0; }
    match exp {
        0 => 1 % m,
        e if e & 1 == 1 =>             // Odd: a^n = a × a^(n-1)
            (base as u128 * pow_mod_rec(base, exp - 1, m) as u128 % m as u128) as u64,
        _ => {                          // Even: a^n = (a^(n/2))²
            let half = pow_mod_rec(base, exp / 2, m);
            (half as u128 * half as u128 % m as u128) as u64
        }
    }
}

// Matrix exponentiation: O(k³ log n) for k×k matrices
// Fast Fibonacci: [F(n+1), F(n)] = [[1,1],[1,0]]^n × [1, 0]
fn fib_fast(n: u64, m: u64) -> u64 {
    // [[1,1],[1,0]]^n gives Fibonacci numbers in O(log n) multiplications
    // See the Matrix struct in the source for full implementation
}
```

The `match` on `exp & 1` is idiomatic Rust pattern matching on a computed value — cleaner than a chain of `if/else`, and the compiler generates identical code.

## What This Unlocks

- **RSA encryption/decryption**: Encrypt: `c = m^e mod n`; Decrypt: `m = c^d mod n` — both are single `pow_mod` calls with 2048-bit exponents.
- **Miller-Rabin primality**: Every witness check is `a^d mod n` and `a^(2^r × d) mod n` — fast exponentiation makes testing a 64-bit prime take microseconds.
- **Linear recurrences in O(log n)**: Matrix exponentiation computes the nth Fibonacci number, nth term of any linear recurrence, or number of paths of length n in a graph — in O(log n) time.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Iterative version | Explicit `ref` variables | `let mut` — cleaner syntax |
| Recursive version | `let rec pow_mod a e m = match e with` | `fn pow_mod_rec(...) { match exp { ... } }` |
| Bit test | `e land 1 = 1` | `exp & 1 == 1` |
| Right shift | `e lsr 1` | `exp >>= 1` |
| 128-bit multiply | `Int64` widening or Zarith | `(a as u128 * b as u128) % m as u128` — native |
