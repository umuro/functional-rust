# 823: Polynomial Rolling Hash

**Difficulty:** 3  **Level:** Intermediate

Hash any substring in O(1) after O(n) preprocessing — enabling fast string matching, deduplication, and similarity detection.

## The Problem This Solves

Computing and comparing substrings naïvely is O(length) per comparison. Polynomial rolling hashes precompute prefix hash values so that the hash of any substring `s[l..r]` can be computed in O(1) using a simple formula — enabling O(n) or O(n log n) algorithms where the naïve approach would be O(n²).

Rolling hashes power the Rabin-Karp string search algorithm (find all occurrences of a pattern in O(n+m) expected time), duplicate substring detection (longest repeated substring in O(n log n)), and similarity detection in plagiarism checkers. They're also used in rolling checksums for rsync-style file synchronisation and in competitive programming for string problems that would otherwise require suffix arrays.

This example implements double hashing (two independent hash functions) to reduce collision probability, and exposes `substring_hash(l, r)` as the primary API.

## The Intuition

Hash a string by treating it as a polynomial: `h(s) = s[0]·B^(n-1) + s[1]·B^(n-2) + ... + s[n-1]` (mod P). Precompute prefix hashes `H[i] = h(s[0..i])`. Then: `h(s[l..r]) = (H[r] - H[l] · B^(r-l)) mod P`.

This is O(1) per query after O(n) setup. The base `B` and modulus `P` are chosen to make collisions unlikely. A single hash has ~1/P collision probability; double hashing reduces this to ~1/(P₁·P₂) ≈ 10⁻¹⁸ with 64-bit moduli.

In OCaml, you'd use `Int64` or `Zarith` for modular arithmetic. In Rust, `u64` wrapping arithmetic with explicit `% MOD` is clean and fast — no bignum overhead, and `wrapping_mul` avoids overflow panics in debug mode.

## How It Works in Rust

```rust
const B1: u64 = 131;
const M1: u64 = 1_000_000_007;
const B2: u64 = 137;
const M2: u64 = 998_244_353;

struct RollingHash {
    h1: Vec<u64>, // prefix hashes mod M1
    h2: Vec<u64>, // prefix hashes mod M2
    p1: Vec<u64>, // B1^i mod M1
    p2: Vec<u64>, // B2^i mod M2
}

impl RollingHash {
    fn new(s: &str) -> Self {
        let n = s.len();
        let bytes: Vec<u8> = s.bytes().collect();

        let mut h1 = vec![0u64; n + 1];
        let mut h2 = vec![0u64; n + 1];
        let mut p1 = vec![1u64; n + 1];
        let mut p2 = vec![1u64; n + 1];

        for i in 0..n {
            // Shift left by one base position and add new character
            h1[i+1] = (h1[i] * B1 + bytes[i] as u64) % M1;
            h2[i+1] = (h2[i] * B2 + bytes[i] as u64) % M2;
            p1[i+1] = p1[i] * B1 % M1;
            p2[i+1] = p2[i] * B2 % M2;
        }
        RollingHash { h1, h2, p1, p2 }
    }

    // Hash of s[l..r] (inclusive l, exclusive r)
    fn get(&self, l: usize, r: usize) -> (u64, u64) {
        let len = r - l;
        // Subtract the contribution of the prefix s[0..l]
        let v1 = (self.h1[r] + M1 - self.h1[l] * self.p1[len] % M1) % M1;
        let v2 = (self.h2[r] + M2 - self.h2[l] * self.p2[len] % M2) % M2;
        (v1, v2)
    }

    // Check if s[l1..r1] == s[l2..r2] probabilistically
    fn equal(&self, l1: usize, r1: usize, l2: usize, r2: usize) -> bool {
        r1 - l1 == r2 - l2 && self.get(l1, r1) == self.get(l2, r2)
    }
}
```

The `+ M - x % M` idiom avoids underflow when subtracting modular values: adding M before the subtraction ensures the result stays positive before taking `% M`. This is standard in competitive programming and necessary because Rust's `u64` doesn't wrap on subtraction in debug mode.

Choosing coprime primes for M1 and M2 maximises the collision resistance of the double hash pair.

## What This Unlocks

- **Rabin-Karp pattern search**: hash the pattern, slide a window of the same length over the text, compare hashes in O(1) — O(n+m) expected total with O(n·m) worst case on hash collision.
- **Longest repeated substring**: binary search on length + hash-based deduplication gives O(n log n) — versus O(n²) naïve.
- **Plagiarism detection / document fingerprinting**: MinHash over rolling window hashes identifies similar documents in large corpora.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| 64-bit modular arithmetic | `Int64` operators or `Zarith` | `u64` with explicit `% MOD` — native, no overhead |
| Underflow guard | `(a - b + m) mod m` | `(h[r] + M - h[l] * p[len] % M) % M` — same pattern |
| Byte access | `Char.code s.[i]` | `s.bytes().collect::<Vec<u8>>()` then index |
| Double hash | Two separate functions or tuples | `(u64, u64)` pair — struct fields, compared as tuple |
| Prefix array init | `Array.make (n+1) 0L` | `vec![0u64; n + 1]` — same concept |
