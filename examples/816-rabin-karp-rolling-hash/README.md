📖 **[View on hightechmind.io →](https://hightechmind.io/rust/816-rabin-karp-rolling-hash)**

---

# 816: Rabin-Karp Rolling Hash Search

**Difficulty:** 4  **Level:** Advanced

Hash-based string search: slide a window in O(1) per step by subtracting the old character and adding the new one — enabling efficient multi-pattern search.

## The Problem This Solves

KMP and BMH excel at single-pattern search. But what if you need to search for thousands of patterns simultaneously — virus signatures in a file, a dictionary of forbidden words, or plagiarism detection across documents? Running KMP once per pattern costs O(patterns × text), which is unacceptable.

Rabin-Karp solves this by reducing each pattern to a hash value. Computing a hash for every m-length window in the text naïvely is O(n×m), but with a *rolling hash* — where you subtract the outgoing character's contribution and add the incoming one — each slide costs O(1). This gives O(n + m) expected time for single-pattern and O(n + k×m) for k patterns (or O(n + k) with a hash set).

The practical application is document fingerprinting: Rabin fingerprinting (a variant) powers Google's copy detection, plagiarism checkers, and the rsync rolling checksum for efficient file synchronization.

## The Intuition

The hash is a polynomial: `H = s[0]×base^(m-1) + s[1]×base^(m-2) + … + s[m-1]`. To slide one position right: subtract `s[i]×base^(m-1)` (the leftmost character's contribution), multiply by `base` to shift everything left, then add `s[i+m]`. All operations are mod a large prime to keep values in u64 range. When hashes match, verify with a direct comparison to handle collisions. Expected O(n + m) because collisions are rare with a good prime.

OCaml uses the same polynomial; Rust adds explicit `u64` wrapping and `+ PRIME` to avoid underflow in subtraction before taking mod.

## How It Works in Rust

```rust
const BASE:  u64 = 256;
const PRIME: u64 = 1_000_000_007;

fn rabin_karp(text: &str, pattern: &str) -> Vec<usize> {
    let (t, p) = (text.as_bytes(), pattern.as_bytes());
    let (n, m) = (t.len(), p.len());
    if m == 0 || m > n { return vec![]; }

    // Precompute base^(m-1) mod PRIME — the weight of the leftmost character
    let mut pow = 1u64;
    for _ in 0..m - 1 { pow = pow * BASE % PRIME; }

    // Initial hashes for pattern and first window
    let (mut hash_p, mut hash_t) = (0u64, 0u64);
    for i in 0..m {
        hash_p = (hash_p * BASE + p[i] as u64) % PRIME;
        hash_t = (hash_t * BASE + t[i] as u64) % PRIME;
    }

    let mut matches = Vec::new();
    for i in 0..=n - m {
        // Hash match → verify to eliminate collisions
        if hash_t == hash_p && &t[i..i + m] == p {
            matches.push(i);
        }
        if i < n - m {
            // Rolling update: remove t[i], add t[i+m]
            // +PRIME before subtracting prevents underflow in modular arithmetic
            hash_t = (hash_t + PRIME - t[i] as u64 * pow % PRIME) % PRIME;
            hash_t = (hash_t * BASE + t[i + m] as u64) % PRIME;
        }
    }
    matches
}
```

The `+ PRIME` before subtraction is the idiomatic Rust pattern for modular subtraction with unsigned integers — avoids wrapping panics in debug mode.

## What This Unlocks

- **Multi-pattern search**: Store pattern hashes in a `HashSet`; one pass finds all patterns — the basis of Aho-Corasick alternatives for short patterns.
- **Document fingerprinting and plagiarism detection**: Rabin-Karp fingerprinting powers Moss, Google's duplicate detection, and rsync's rolling checksum.
- **Substring matching in competitive programming**: When you need overlapping match counts or 2D pattern matching, rolling hashes extend naturally to 2D grids.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Polynomial hash | `let h = h * base + c mod p` | Same; explicit `u64` types |
| Modular subtraction | `(h - old*pow + prime) mod prime` | `(h + PRIME - old * pow % PRIME) % PRIME` |
| Integer overflow | `Int64` for 64-bit safety | `u64` arithmetic; `u128` not needed here |
| Sliding window | Manual loop with ref vars | Idiomatic `for i in 0..=n-m` |
| Collision check | `String.sub` comparison | `&t[i..i+m] == p` slice equality |
