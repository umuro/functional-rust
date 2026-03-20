📖 **[View on hightechmind.io →](https://hightechmind.io/rust/824-sieve-of-eratosthenes)**

---

# Sieve of Eratosthenes
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  


## Problem Statement

Generating all prime numbers up to N is a fundamental operation in number theory with applications in cryptography (RSA key generation requires large primes), competitive programming, and mathematical software. Trial division for each number takes O(sqrt(n)) per number, giving O(n*sqrt(n)) total. The Sieve of Eratosthenes achieves O(n log log n) by marking multiples of each prime in bulk: once 2's multiples are marked, 3's multiples, 5's multiples, etc. Every composite number gets eliminated exactly once. For N up to 10^7, the sieve runs in milliseconds and uses O(n) memory. Segmented variants extend this to arbitrary ranges without holding all of 0..N in memory.

## Learning Outcomes

- Implement the classic sieve: initialize all to true, mark multiples of each prime starting from p^2
- Understand why we start marking from p^2: smaller multiples were already marked by earlier primes
- Recognize the time complexity: each composite is marked exactly once by its smallest prime factor
- Learn the segmented sieve for memory-efficient prime generation for very large N
- Apply the sieve to number-theoretic computations: Euler's totient, smallest prime factor table

## Rust Application

```rust
pub fn sieve(n: usize) -> Vec<bool> {
    let mut is_prime = vec![true; n + 1];
    is_prime[0] = false;
    if n >= 1 { is_prime[1] = false; }
    let mut p = 2;
    while p * p <= n {
        if is_prime[p] {
            let mut m = p * p;
            while m <= n { is_prime[m] = false; m += p; }
        }
        p += 1;
    }
    is_prime
}
```

Rust's `Vec<bool>` stores one boolean per byte; for large N, a `BitVec` (from the `bitvec` crate) gives 8x memory reduction. The `while p * p <= n` loop condition avoids floating-point with `sqrt`. Starting inner loop at `p * p` (not `2*p`) saves marking already-marked composites. The function returns the sieve array rather than the prime list, letting callers enumerate primes lazily with `enumerate().filter()`. This is more flexible than returning a `Vec<usize>` — callers can ask for count, nth prime, or primality test without rebuilding.

## OCaml Approach

OCaml implements the sieve with `Array.make (n+1) true` and two nested loops using `for` and `while`. The functional equivalent uses `Array.iteri` for the outer loop, though mutable array mutation is idiomatic here. OCaml's `Array.to_seqi |> Seq.filter_map (fun (i, b) -> if b then Some i else None)` generates prime sequences lazily. The `Bigarray` module provides compact bit arrays for memory efficiency. OCaml's `Printf.printf` easily prints prime counts for verification.

## Key Differences

| Aspect | Rust | OCaml |
|---|---|---|
| Array init | `vec![true; n+1]` | `Array.make (n+1) true` |
| Outer loop | `while p * p <= n` | `for p = 2 to isqrt n` |
| Inner loop | `while m <= n { m += p }` | `let m = ref (p*p); while !m <= n` |
| Memory (bool) | 1 byte/bool (8x waste) | 1 word/bool (worse) |
| Result type | `Vec<bool>` (sieve) or `Vec<usize>` | `bool array` or `int list` |
| Segmented variant | Manual chunk iteration | Same approach |

## Exercises

1. Implement a segmented sieve that generates primes in the range [L, R] using only O(sqrt(R)) memory.
2. Build the smallest prime factor table: `spf[i]` = smallest prime dividing i, useful for fast factorization.
3. Compute Euler's totient function for all n ≤ N using a sieve-like approach.
4. Count twin primes (pairs p, p+2 both prime) up to 10^7 and compare with pi(n) estimates.
5. Implement a `BitVec`-backed sieve and measure memory and speed improvement vs `Vec<bool>`.
