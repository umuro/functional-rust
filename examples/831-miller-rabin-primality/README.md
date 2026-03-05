📖 **[View on hightechmind.io →](https://hightechmind.io/rust/831-miller-rabin-primality)**

---

# 831: Miller-Rabin Probabilistic Primality Test

**Difficulty:** 4  **Level:** Advanced

Deterministic primality for all 64-bit integers using 12 fixed witnesses — the algorithm inside RSA key generation and every serious cryptographic library.

## The Problem This Solves

Trial division checks primality in O(√n): for a 64-bit prime near 2^63, that's ~3 billion divisions — unacceptably slow. The sieve is fast but requires O(n) memory — impractical for numbers near 2^63. Miller-Rabin tests primality in O(k log² n) where k is the number of witnesses, regardless of the size of n.

For probabilistic Miller-Rabin, each witness independently has a ≤ 1/4 chance of being fooled by a composite. With 12 witnesses, a composite passing all tests has probability ≤ (1/4)^12 ≈ 6 × 10^-8. But more powerfully: with the specific deterministic witness set `{2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37}`, the test is *provably correct* for all n < 3.3 × 10^24 — covering every 64-bit integer with certainty. This is what production RSA implementations use.

Understanding Miller-Rabin also illuminates *why* RSA's random prime generation works: generate a random odd number, run Miller-Rabin with a few witnesses, repeat until prime. The prime number theorem guarantees roughly 1 in ln(n) candidates near n is prime — for 2048-bit primes, you need ~1400 trials on average.

## The Intuition

Factor `n - 1 = 2^s × d` where d is odd. By Fermat's little theorem, if n is prime, then `a^(n-1) ≡ 1 (mod n)` for any a. More precisely: the sequence `a^d, a^(2d), a^(4d), …, a^(2^s × d)` mod n must either start at 1, or hit -1 (= n-1) somewhere before the end. Any composite n fails this condition for at least 3/4 of all bases a. Checking multiple witnesses gives exponential confidence.

`trailing_zeros()` is the idiomatic way to factor out powers of 2 from `n-1` — replaces the loop that tests `n % 2 == 0` repeatedly.

## How It Works in Rust

```rust
fn mulmod(a: u64, b: u64, m: u64) -> u64 {
    (a as u128 * b as u128 % m as u128) as u64  // u128 prevents overflow
}

fn pow_mod(mut base: u64, mut exp: u64, m: u64) -> u64 {
    let mut result = 1u64;
    base %= m;
    while exp > 0 {
        if exp & 1 == 1 { result = mulmod(result, base, m); }
        base = mulmod(base, base, m);
        exp >>= 1;
    }
    result
}

fn miller_witness(n: u64, d: u64, s: u32, a: u64) -> bool {
    let mut x = pow_mod(a, d, n);
    if x == 1 || x == n - 1 { return true; }   // Passed trivially
    for _ in 1..s {
        x = mulmod(x, x, n);
        if x == n - 1 { return true; }           // Hit -1 in the sequence
    }
    false  // Failed: n is composite (or a is an exceptional witness)
}

pub fn is_prime(n: u64) -> bool {
    match n {
        0 | 1 => false,
        2 | 3 | 5 | 7 => true,
        _ if n % 2 == 0 || n % 3 == 0 => false,
        _ => {
            // Factor n-1 = 2^s * d with d odd
            let mut d = n - 1;
            let s = d.trailing_zeros();    // Count factors of 2
            d >>= s;

            // 12 witnesses: deterministic for all n < 3.3 × 10^24
            const WITNESSES: &[u64] = &[2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37];
            WITNESSES.iter().all(|&a| a >= n || miller_witness(n, d, s, a))
        }
    }
}
```

The `a >= n` guard handles the edge case where a witness is larger than n itself (e.g., testing primality of n=5 with witness a=7).

## What This Unlocks

- **RSA key generation**: Generate random odd numbers, test with Miller-Rabin, repeat until prime — this is exactly how OpenSSL generates RSA primes.
- **Large prime discovery**: Primality testing for Mersenne prime candidates, safe primes for Diffie-Hellman, and Sophie Germain primes all use Miller-Rabin or its deterministic refinements.
- **Factoring integration**: Miller-Rabin combines with Pollard's rho (#825) to build a complete integer factorizer: recursively split n if composite, stop when n is prime.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| 128-bit multiply | `Int64` widening or Zarith | `(a as u128 * b as u128) % m as u128` |
| Factor 2s from n-1 | Manual loop dividing by 2 | `d.trailing_zeros()` — hardware instruction |
| Witness iteration | `List.for_all (fun a -> ...)` | `WITNESSES.iter().all(|&a| ...)` |
| Early exit on failure | Exception or bool accumulator | `all()` short-circuits on `false` |
| Guard for small witness | Explicit `if a < n` check | `a >= n \|\|` in the closure |
