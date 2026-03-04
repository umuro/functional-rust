# 825: Prime Factorization

**Difficulty:** 4  **Level:** Advanced

Factor any 64-bit integer: trial division for small factors, Pollard's rho for large semi-primes — combining O(√n) and O(n^(1/4)) into a practical factorizer.

## The Problem This Solves

Cryptography depends on the hardness of factoring large numbers: RSA's security relies on the fact that given n = p × q where p, q are large primes, finding p and q is computationally infeasible. Implementing a factorizer teaches you *why* RSA is hard — and exactly where its security boundary lies.

For competitive programming, factorizing numbers up to 10^18 appears constantly in problems involving Euler's totient function, divisor counts, and multiplicative functions. Trial division alone fails for large primes and semi-primes. Pollard's rho is the practical algorithm: it finds a factor of n in O(n^(1/4)) expected time using a pseudo-random walk and Floyd's cycle detection.

The combined approach (trial division for small factors, Pollard's rho for the residue, Miller-Rabin to confirm primality) is how real-world factorizers work — including Mathematica's `FactorInteger` for moderate-sized inputs.

## The Intuition

Trial division: divide by 2, then odd numbers up to √n. Any remaining factor > √n must be prime. This handles all numbers efficiently when the smallest prime factor is small (which it often is — about 30% of random numbers are divisible by 2, 3, or 5).

Pollard's rho: the sequence `x_{n+1} = (x_n² + c) mod n` eventually cycles (Birthday Paradox — after ~O(n^(1/4)) steps, two sequence values share a factor with n). Floyd's tortoise-and-hare detects the cycle. The GCD of the difference with n reveals the factor. Uses O(1) space beyond the sequence variables.

In Rust: `u64` wrapping arithmetic handles the pseudo-random walk; `u64::abs_diff` avoids signed overflow.

## How It Works in Rust

```rust
fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 { a } else { gcd(b, a % b) }
}

// Trial division: O(√n), handles small factors efficiently
fn factorize(mut n: u64) -> Vec<(u64, u32)> {
    let mut factors = Vec::new();
    if n % 2 == 0 {
        let mut exp = 0u32;
        while n % 2 == 0 { exp += 1; n /= 2; }
        factors.push((2, exp));
    }
    let mut d = 3u64;
    while d * d <= n {
        if n % d == 0 {
            let mut exp = 0u32;
            while n % d == 0 { exp += 1; n /= d; }
            factors.push((d, exp));
        }
        d += 2;
    }
    if n > 1 { factors.push((n, 1)); }  // Remaining factor is prime
    factors
}

// Pollard's rho: O(n^(1/4)) expected, for large semi-primes
fn pollard_rho(n: u64) -> u64 {
    if n % 2 == 0 { return 2; }
    let mut c = 1u64;
    loop {
        let (mut x, mut y, mut d) = (2u64, 2u64, 1u64);
        while d == 1 {
            // Pseudo-random walk: f(x) = (x² + c) mod n
            x = (x.wrapping_mul(x).wrapping_add(c)) % n;
            y = (y.wrapping_mul(y).wrapping_add(c)) % n;  // Tortoise step
            y = (y.wrapping_mul(y).wrapping_add(c)) % n;  // Hare double-steps
            d = gcd(x.abs_diff(y), n);                    // GCD reveals factor
        }
        if d != n { return d; }
        c += 1;  // Retry with different constant
    }
}
```

`abs_diff` (stable since Rust 1.60) cleanly handles unsigned subtraction without overflow — replaces the `(a as i64 - b as i64).unsigned_abs()` pattern from earlier Rust.

## What This Unlocks

- **RSA and cryptography**: Factoring is the hard problem RSA relies on; understanding Pollard's rho tells you why 512-bit RSA is broken but 2048-bit is not.
- **Euler's totient and multiplicative functions**: `φ(n)`, `σ(n)`, `τ(n)` all require the prime factorization; competitive problems use them constantly.
- **Primality certificates**: Combined with Miller-Rabin, factorization enables Pratt certificates — proofs that a number is prime.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| GCD one-liner | `let rec gcd a b = if b=0 then a else gcd b (a mod b)` | `fn gcd(a:u64,b:u64)->u64{if b==0{a}else{gcd(b,a%b)}}` |
| 64-bit arithmetic | `Int64` or `nativeint` | `u64` natively; no wrapper needed |
| Unsigned subtract | `abs (a - b)` with care | `a.abs_diff(b)` — safe on `u64` |
| Wrapping mul | `Int64.rem (Int64.mul x x) n` | `x.wrapping_mul(x) % n` |
| Recursive style | Natural tail recursion | Iterative loop to avoid stack |
