📖 **[View on hightechmind.io →](https://hightechmind.io/rust/830-euler-totient)**

---

# Euler's Totient Function
**Difficulty:** ⭐  
**Category:** Functional Programming  



## Problem Statement

Euler's totient function phi(n) counts the integers from 1 to n that are coprime to n. It is fundamental to RSA key generation: the private key d satisfies `e * d ≡ 1 (mod phi(n))` where n = p*q. By Euler's theorem, `a^phi(n) ≡ 1 (mod n)` for any a coprime to n, generalizing Fermat's little theorem. phi(n) also counts primitive roots, answers "how many fractions in lowest terms have denominator n," and appears in the analysis of the Stern-Brocot tree. For prime p, phi(p) = p-1; for prime power p^k, phi(p^k) = p^k - p^(k-1); these combine multiplicatively for general n.

## Learning Outcomes

- Compute phi(n) from prime factorization: phi(n) = n * product of (1 - 1/p) for each prime factor p
- Implement the sieve-based batch computation of phi(1..n) in O(n log log n)
- Apply Euler's theorem: a^phi(n) ≡ 1 (mod n) for gcd(a,n) = 1
- Use phi for RSA private key derivation: d = e^(-1) mod phi(n)
- Understand multiplicativity: phi(a*b) = phi(a)*phi(b) when gcd(a,b) = 1

## Rust Application

```rust
pub fn euler_totient(n: u64) -> u64 {
    let mut result = n;
    let mut n = n;
    let mut d = 2u64;
    while d * d <= n {
        if n % d == 0 {
            while n % d == 0 { n /= d; }
            result -= result / d;
        }
        d += 1;
    }
    if n > 1 { result -= result / n; }
    result
}
```

The formula `result -= result / d` applies the multiplicative factor `(1 - 1/p)` as `result * (p-1) / p` without floating point: `result - result/p` where the integer division is exact because result is always divisible by p at that point. This works because we divide out all factors of p before moving on. The remaining `n > 1` case handles the last prime factor. Rust's `u64` is sufficient for n up to ~1.8 * 10^19.

## OCaml Approach

OCaml's `euler_totient n` mirrors the Rust approach using mutable `let r = ref n` and `let n = ref n`. The sieve version initializes `phi.(i) = i` and iterates: for each i, if `phi.(i) = i` (i is prime), for each multiple j: `phi.(j) <- phi.(j) / i * (i-1)`. OCaml's `Array.init n (fun i -> i)` creates the initial array. The product formula `phi.(n) = n * fold primes (fun acc p -> acc * (p-1) / p)` is idiomatic in functional style.

## Key Differences

| Aspect | Rust | OCaml |
|---|---|---|
| Per-n computation | Trial division O(sqrt n) | Same approach |
| Batch 1..n | Sieve with `Vec<u64>` | `Array.init` + sieve loop |
| Factor application | `result -= result / d` | `r := !r - !r / d` |
| Multiplicativity | Explicit in algorithm | Same |
| Standard library | No built-in | No built-in |
| RSA integration | Used in key generation tests | Same theoretical role |

## Exercises

1. Compute the sum of phi(k) for k=1..n and verify it equals n*(n+1)/2 for prime n.
2. Implement the sieve for phi(1..n) in O(n log log n) and use it to answer sum(phi(k), k=1..n) queries.
3. Verify Euler's theorem: compute a^phi(n) mod n = 1 for several coprime (a,n) pairs.
4. Use phi to count the number of primitive roots modulo a prime p (answer: phi(p-1)).
5. Implement RSA key generation: choose primes p,q; compute phi=phi(p*q)=(p-1)*(q-1); find e, d=e^(-1) mod phi.
