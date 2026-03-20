📖 **[View on hightechmind.io →](https://hightechmind.io/rust/831-miller-rabin-primality)**

---

# Miller-Rabin Primality Test
**Difficulty:** ⭐  
**Category:** Functional Programming  


## Problem Statement

Deterministic primality testing via trial division is O(sqrt(n)), impractical for 64-bit numbers (up to 4 billion operations). Cryptographic applications need to test numbers with hundreds of digits for primality during key generation. Miller-Rabin is a probabilistic primality test that runs in O(k log^2 n) where k is the number of witness rounds — each round reduces error probability to 1/4. With k=25 rounds, the probability of a false positive is negligible. For 64-bit integers, a specific set of deterministic witnesses {2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37} makes Miller-Rabin deterministic — no false positives for any n < 3.3 * 10^24.

## Learning Outcomes

- Decompose n-1 as 2^r * d where d is odd, used in the Miller-Rabin witness test
- Implement the witness test: `a^d ≠ 1 (mod n)` AND `a^(2^j * d) ≠ -1 (mod n)` for all j means composite
- Understand strong pseudoprimes and why multiple witnesses reduce false positive rate
- Use the deterministic witness set for 64-bit integers to get an exact answer
- Compare with Fermat test: Miller-Rabin has no Carmichael number problem

## Rust Application

```rust
pub fn miller_rabin(n: u64) -> bool {
    if n < 4 { return n >= 2; }
    if n % 2 == 0 { return false; }
    let (mut r, mut d) = (0u32, n - 1);
    while d % 2 == 0 { d /= 2; r += 1; }
    // Witnesses that are deterministic for n < 3_317_044_064_679_887_385_961_981
    for &a in &[2u64, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37] {
        if !is_composite(n, a, d, r) { continue; }
        return false;
    }
    true
}
```

The witness set `{2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37}` covers all 64-bit integers deterministically (proven by exhaustive verification). The `is_composite` helper computes `a^d mod n` using `mod_pow` with `u128` intermediates, then checks the Miller-Rabin condition. Rust's exhaustive match or `for` over a fixed slice is idiomatic. Early return on `false` (composite found) is efficient. The function is pure and thread-safe with no global state.

## OCaml Approach

OCaml's Miller-Rabin decomposes `n-1` using a recursive/iterative bit-strip. The witness list is a `let witnesses = [2; 3; 5; 7; 11; 13; 17; 19; 23; 29; 31; 37]`. The test uses `List.for_all (fun a -> not (is_composite n a d r)) witnesses`. OCaml's `Int64` or `Zarith` handles modular multiplication for large n. The 63-bit native `int` suffices for n < 2^62; for full 64-bit range, `Int64` or `Zarith.mpz` is needed.

## Key Differences

| Aspect | Rust | OCaml |
|---|---|---|
| 64-bit modular mul | `as u128` intermediate | `Int64` or `Zarith` |
| Witness iteration | `for &a in &[...]` | `List.for_all` |
| Overflow safety | `u128` prevents overflow | `Zarith` arbitrary precision |
| Deterministic range | n < 3.3 * 10^24 | Same witnesses, same range |
| False positives | None with chosen witnesses | None |
| Probabilistic mode | Add random witnesses | `Random.int64` witnesses |

## Exercises

1. Implement the probabilistic version with k random witnesses and estimate false positive rate empirically.
2. Use Miller-Rabin to generate large primes: generate random odd n until Miller-Rabin returns true.
3. Compare Miller-Rabin with the Fermat primality test on Carmichael numbers (which fool Fermat but not Miller-Rabin).
4. Implement Baillie-PSW primality test (Miller-Rabin base 2 + strong Lucas) which has no known pseudoprimes.
5. Benchmark Miller-Rabin vs. trial division for n up to 10^12 and measure crossover point.
