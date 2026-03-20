📖 **[View on hightechmind.io →](https://hightechmind.io/rust/825-prime-factorization)**

---

# Prime Factorization
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  


## Problem Statement

Decomposing an integer into its prime factors is fundamental to number theory: computing GCD, LCM, Euler's totient, cryptographic key generation, and combinatorial coefficient calculations all rely on factorization. For small numbers, trial division by primes up to sqrt(n) suffices. For large numbers (64-bit), Pollard's rho algorithm factorizes in O(n^(1/4)) expected time. Real-world cryptography relies on the hardness of factorizing large composites (RSA). Competitive programming uses factorization for: divisor enumeration, Mobius function, Euler's phi, and modular arithmetic.

## Learning Outcomes

- Implement trial division factorization in O(sqrt(n)) by testing primes up to sqrt(n)
- Represent the factorization as a map from prime to exponent: `HashMap<u64, u32>`
- Apply factorization to compute number of divisors, sum of divisors, Euler's phi
- Understand when to use sieve-based factorization (many numbers up to N) vs. per-number trial division
- Recognize the smallest prime factor sieve for batch factorization in O(log n) per number

## Rust Application

```rust
pub fn factorize(mut n: u64) -> HashMap<u64, u32> {
    let mut factors = HashMap::new();
    let mut d = 2u64;
    while d * d <= n {
        while n % d == 0 {
            *factors.entry(d).or_insert(0) += 1;
            n /= d;
        }
        d += 1;
    }
    if n > 1 { *factors.entry(n).or_insert(0) += 1; }
    factors
}
```

The algorithm consumes `n` by repeatedly dividing out each prime factor. After dividing out all factors of 2, then 3, etc., any remaining `n > 1` is itself prime. Rust's `HashMap::entry().or_insert(0)` pattern is idiomatic for accumulating counts. The optimization of checking `d * d <= n` (not `d <= sqrt(n)`) avoids floating-point. By incrementing `d` by 1 rather than using a precomputed prime list, the code is self-contained but does test composite divisors unnecessarily; using a precomputed prime sieve for d would be faster.

## OCaml Approach

OCaml uses a `Hashtbl` or returns a sorted `(int * int) list` of `(prime, exponent)` pairs. The recursive functional version divides out factors recursively, building the list with pattern matching. Trial division uses a `while` loop with mutable `n ref`. OCaml's `Map.Make(Int)` provides an immutable sorted factor map. The `let () = while !n mod d = 0 do ... done` pattern mirrors the Rust approach. For large numbers, OCaml's `Zarith` library handles arbitrary-precision factorization.

## Key Differences

| Aspect | Rust | OCaml |
|---|---|---|
| Factor map | `HashMap<u64, u32>` | `Hashtbl` or `(int * int) list` |
| Mutation | Takes `mut n: u64` by value | `let n = ref n` |
| Entry update | `.entry().or_insert(0) += 1` | `Hashtbl.replace` or match |
| Large numbers | `u64` (max ~1.8 * 10^19) | `Zarith` for arbitrary precision |
| Sieve integration | Can import SPF sieve function | `Array.get spf n` |
| Remaining prime | `if n > 1` check after loop | Same pattern |

## Exercises

1. Use factorization to compute the number of divisors of n: product of (exponent + 1) over all prime factors.
2. Implement Euler's totient phi(n) from the prime factorization: n * product of (1 - 1/p) for each prime p.
3. Compute LCM of a list of numbers using their prime factorizations (max exponent per prime).
4. Implement the smallest prime factor sieve and compare batch factorization speed vs. per-number trial division for 10^6 numbers.
5. Implement Pollard's rho algorithm for factorizing 64-bit numbers and measure speedup on large semiprimes.
