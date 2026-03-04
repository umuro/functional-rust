# 824: Sieve of Eratosthenes

**Difficulty:** 3  **Level:** Intermediate

Generate all primes up to N in O(n log log n) by iteratively marking composites — the oldest and most cache-friendly batch prime algorithm.

## The Problem This Solves

When you need many primes — RSA key generation, primality sieves in cryptographic libraries, competitive programming precomputation, prime-counting functions — you need a batch algorithm, not per-number testing. Trial division checks one number in O(√n); the sieve amortizes this across all numbers up to N and achieves O(n log log n) total, which in practice is nearly linear.

The sieve is also the canonical example of a simple, cache-friendly computation that benefits enormously from modern CPU memory hierarchies. The inner marking loop is a stride-p write over a contiguous boolean array — exactly the access pattern that hardware prefetchers handle well. Using `Vec<bool>` (or a bit-vector) over a `HashSet` can be 10-50× faster in practice because of this.

The segmented sieve variant (also shown) processes the range `[lo, hi]` using only O(√hi) base primes — essential when N is too large for a single array (e.g., primes up to 10^12 in a problem).

## The Intuition

Mark every multiple of p starting from p² as composite. Start from p² because all smaller multiples of p have already been marked by smaller primes. Only sieve up to √N because any composite ≤ N has a prime factor ≤ √N. The inner loop runs `N/p` times per prime p; summing over all primes gives Σ N/p ≈ N × log log N by Mertens' theorem — the source of the O(n log log n) complexity.

OCaml uses `Array.make n true` with a for loop; Rust's `.step_by(p)` range is identical in behavior, slightly more idiomatic.

## How It Works in Rust

```rust
fn sieve(limit: usize) -> Vec<usize> {
    if limit < 2 { return vec![]; }
    let mut is_prime = vec![true; limit + 1];
    is_prime[0] = false;
    is_prime[1] = false;

    let sqrt_limit = (limit as f64).sqrt() as usize;
    for p in 2..=sqrt_limit {
        if is_prime[p] {
            // Start at p² — all smaller multiples already marked
            // step_by(p): stride-p access is cache-prefetcher-friendly
            for j in (p * p..=limit).step_by(p) {
                is_prime[j] = false;  // O(1) per mark, N/p total
            }
        }
    }

    // Collect: enumerate gives (index, value) — filter_map keeps only primes
    is_prime.iter().enumerate()
        .filter_map(|(i, &b)| if b { Some(i) } else { None })
        .collect()
}

// Returns the sieve itself for O(1) primality queries
fn prime_sieve(limit: usize) -> Vec<bool> {
    // Same construction; returns the boolean array
    // O(1) per query after O(n log log n) build — the key practical advantage
    // ...
}
```

For memory-critical applications: replace `Vec<bool>` with a `Vec<u64>` bit-packed manually or the `bit-vec` crate — reduces memory by 8×, improving L1/L2 cache utilization dramatically for large N.

## What This Unlocks

- **Cryptography precomputation**: Generate primes for RSA/DSA parameter selection; sieve to 10^7 takes ~40ms and fits in L2 cache.
- **Competitive programming**: The segmented sieve finds primes in `[lo, hi]` using O(√hi) memory — essential for problems where N > 10^9.
- **Prime-counting and analytic number theory**: `π(n)` (number of primes ≤ n) is computed by counting `true` values after sieving; used in cryptanalysis and theoretical CS.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Boolean array | `Array.make (n+1) true` | `vec![true; n + 1]` |
| Mark multiples | `for j = p*p to n step p do` | `(p*p..=n).step_by(p)` — lazy range |
| Integer sqrt | `int_of_float (sqrt (float_of_int n))` | `(n as f64).sqrt() as usize` |
| Collect primes | `Array.to_seqi |> Seq.filter_map` | `.enumerate().filter_map(...)` |
| Bit-packing | External library needed | Manual `Vec<u64>` or `bit-vec` crate |
