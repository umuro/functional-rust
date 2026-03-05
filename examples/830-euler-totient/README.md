📖 **[View on hightechmind.io →](https://hightechmind.io/rust/830-euler-totient)**

---

# 830: Euler's Totient Function φ(n)

**Difficulty:** 4  **Level:** Advanced

Count integers in [1, n] coprime to n — and compute it for all values up to N in O(N log log N) with a sieve.

## The Problem This Solves

Euler's totient function φ(n) counts how many integers in [1, n] share no common factor with n. For a prime p, φ(p) = p-1 (all integers below p are coprime to it). For n = p^k, φ(n) = p^(k-1)(p-1). For general n, φ is multiplicative: φ(mn) = φ(m)φ(n) when gcd(m,n) = 1.

φ(n) appears everywhere in number theory and cryptography. RSA key generation relies on φ(n) where n = p·q — specifically, the fact that a^φ(n) ≡ 1 (mod n) for all a coprime to n (Euler's theorem). Diffie-Hellman and ElGamal work in groups whose order is φ(p) = p-1 for prime p. Primitive roots and discrete logarithms are defined relative to φ.

This example implements both a single-value O(√n) computation and an O(N log log N) sieve for computing φ(k) for all k up to N simultaneously.

## The Intuition

**Single value**: factor n into prime powers using trial division. Apply the formula: φ(n) = n · ∏(1 - 1/p) for each distinct prime factor p of n. In integer arithmetic: multiply by (p-1) and divide by p for each prime factor.

**Sieve**: analogous to the Sieve of Eratosthenes. Start with `phi[i] = i`. For each prime p (those with `phi[p] == p` still, unmarked), apply the totient factor to all multiples: `phi[k] = phi[k] / p * (p - 1)`. This is O(N log log N), same as the prime sieve.

The sieve approach is essential when you need φ for many values — computing each independently would cost O(N √N).

## How It Works in Rust

```rust
// Single value: O(√n)
fn euler_totient(mut n: u64) -> u64 {
    let mut result = n;
    let mut p = 2u64;
    while p * p <= n {
        if n % p == 0 {
            // p is a prime factor — apply (1 - 1/p)
            while n % p == 0 { n /= p; }
            result -= result / p; // result = result * (p-1) / p
        }
        p += 1;
    }
    if n > 1 {
        // Remaining factor is prime
        result -= result / n;
    }
    result
}

// Sieve for all values up to limit: O(N log log N)
fn totient_sieve(limit: usize) -> Vec<u64> {
    let mut phi: Vec<u64> = (0..=limit as u64).collect(); // phi[i] = i initially

    for p in 2..=limit {
        if phi[p] as usize == p {
            // p is prime (not yet modified by any smaller prime)
            let mut k = p;
            while k <= limit {
                phi[k] -= phi[k] / p as u64; // phi[k] *= (p-1)/p
                k += p;
            }
        }
    }
    phi
}
```

The key insight in `euler_totient`: `result -= result / p` implements `result *= (p-1)/p` in integer arithmetic without fractions — divide first, then subtract, to maintain exactness. This works because `p | result` at that point (since `p | n` originally).

In the sieve: `phi[p] as usize == p` detects primes exactly like the Sieve of Eratosthenes detects composites — but by checking if the initial value was preserved.

For RSA: `phi[n] = (p-1) * (q-1)` directly, but understanding the general formula matters for understanding why RSA works with composite moduli.

## What This Unlocks

- **RSA key generation**: compute the private exponent `d ≡ e⁻¹ (mod φ(n))` — requires knowing φ(n) = (p-1)(q-1).
- **Primitive root finding**: a generator of (Z/nZ)* exists iff φ(n) = 1, 2, or φ(p^k) for prime p — totient structure determines group structure.
- **Euler's theorem applications**: for any a coprime to n, a^φ(n) ≡ 1 (mod n) — used to reduce large modular exponentiations.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Integer division exactness | `/` truncates (same as Rust) | `/` truncates — `result -= result / p` works identically |
| Prime detection in sieve | `phi.(p) = p` comparison | `phi[p] as usize == p` — same logic, explicit cast |
| Mutable sieve array | `Array.make (n+1) 0` with loop | `(0..=limit as u64).collect()` initializes with identity values |
| Trial division loop | `while p * p <= n` with `ref n` | `while p * p <= n` with `mut n` — shadowing the parameter |
| u64 vs int | `Int64` or native int (63-bit) | `u64` — explicit width, no overflow surprises |
