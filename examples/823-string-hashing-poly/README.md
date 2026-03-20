📖 **[View on hightechmind.io →](https://hightechmind.io/rust/823-string-hashing-poly)**

---

# Polynomial String Hashing
**Difficulty:** ⭐  
**Category:** Functional Programming  


## Problem Statement

Comparing strings character by character takes O(m) time. When you need to compare many substrings — checking if a string is a rotation of another, finding duplicate substrings, comparing all n^2 pairs of substrings — naive comparison becomes O(n^2 * m). Polynomial hashing reduces substring comparison to O(1) with O(n) preprocessing: compute prefix hashes, then any substring hash is `(prefix[r] - prefix[l] * base^(r-l)) % mod`. This enables O(n log n) string sorting, O(n) duplicate detection, and O(n log n) LCP (longest common prefix) binary search. It's the hash function behind rolling hash algorithms, string fingerprinting, and near-duplicate document detection.

## Learning Outcomes

- Build prefix hash array `h[i] = h[i-1]*base + s[i]` for O(1) substring hash queries
- Precompute powers of base for the rolling subtraction formula
- Handle hash collisions using double hashing (two independent hash functions)
- Understand the polynomial hash formula and why prime moduli reduce collision probability
- Apply hashing to O(n log n) LCP computation via binary search + hash comparison

## Rust Application

```rust
pub struct PolyHash {
    h: Vec<u64>,
    pw: Vec<u64>,
    base: u64,
    modulus: u64,
}
impl PolyHash {
    pub fn get(&self, l: usize, r: usize) -> u64 {
        // (h[r] - h[l] * pw[r - l]) % modulus
        (self.h[r] + self.modulus * self.modulus - self.h[l] * self.pw[r - l] % self.modulus) % self.modulus
    }
}
```

Rust's `u64` arithmetic prevents overflow from intermediate products when `modulus < 2^32`. The subtraction formula adds `modulus * modulus` before subtracting to avoid underflow — a common trick in modular arithmetic. The `pw` array precomputes `base^i mod modulus` so `get(l, r)` is O(1). Double hashing uses two `PolyHash` instances with different `(base, modulus)` pairs, reducing collision probability to `1/(mod1 * mod2)`. Rust's struct ensures base, modulus, and arrays stay coupled, preventing mismatch bugs.

## OCaml Approach

OCaml uses `Int64` or native `int` (63-bit on 64-bit systems) for modular arithmetic. The prefix hash array is `Array.make (n+1) 0` and powers array is `Array.make (n+1) 1`. OCaml's `lsl`, `land`, and `mod` operators handle modular arithmetic. The `get` function is a simple pure function over the arrays. For double hashing, a record `{ h1: int array; h2: int array; pw1: int array; pw2: int array }` keeps both hash functions bundled. OCaml's polymorphic comparison avoids the need for custom hash combination.

## Key Differences

| Aspect | Rust | OCaml |
|---|---|---|
| Arithmetic type | `u64` (64-bit unsigned) | `int` (63-bit signed) or `Int64` |
| Underflow prevention | Add `modulus^2` before subtract | Use absolute value or unsigned |
| Double hashing | Two `PolyHash` structs | Record with two hash arrays |
| Power precomputation | `Vec<u64>` with loop | `Array.init` or loop |
| Collision rate | `1/mod` per hash | Same; `1/(mod1*mod2)` for double |
| String access | `.as_bytes()[i]` | `Char.code (String.get s i)` |

## Exercises

1. Implement O(n log n) LCP array computation using binary search + polynomial hashing.
2. Use double hashing to find the longest duplicate substring with high confidence and no false positives.
3. Detect all anagram windows: substrings of length k that are permutations of a query string.
4. Implement string sorting using hash-based radix sort on hashed prefixes.
5. Measure empirical collision rate for single vs. double hashing on a large random string corpus.
