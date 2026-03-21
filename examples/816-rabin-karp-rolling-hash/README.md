📖 **[View on hightechmind.io →](https://hightechmind.io/rust/816-rabin-karp-rolling-hash)**

---

# Rabin-Karp Rolling Hash
**Difficulty:** ⭐  
**Category:** Functional Programming  



## Problem Statement

Finding multiple patterns in a text, or detecting plagiarism across documents, requires more than single-pattern search. Rabin-Karp solves this using polynomial hashing: compute a hash for each window of text equal in size to the pattern, and compare hashes rather than characters. The rolling hash property means each step takes O(1) — subtract the contribution of the outgoing character and add the incoming one. This enables searching for k patterns simultaneously in O(n + k*m) expected time. Real-world uses: duplicate code detection, plagiarism detection, DNA fingerprinting, and detecting repeated substrings in large files.

## Learning Outcomes

- Understand polynomial rolling hash: `hash = (c0*b^(m-1) + c1*b^(m-2) + ... + c(m-1)) mod p`
- Implement O(1) hash update by rolling: subtract old char contribution, multiply, add new char
- Handle hash collisions with character-level verification (spurious hits)
- Recognize Rabin-Karp's advantage for multi-pattern search vs single-pattern algorithms
- Learn the importance of choosing prime modulus and base to minimize collision probability

## Rust Application

```rust
pub fn rabin_karp(text: &str, pattern: &str) -> Vec<usize> {
    let base: u64 = 31;
    let modulus: u64 = 1_000_000_007;
    // Compute pattern hash and initial window hash
    // Roll hash: hash = (hash - old_char * pow_base) * base + new_char
}
```

Rust's `u64` arithmetic with explicit modulus prevents overflow that would occur with `usize`. The `wrapping_mul` or `% modulus` pattern keeps values in range. Collecting bytes via `.as_bytes()` makes the ASCII-level computation straightforward. The rolling update subtracts the outgoing byte's contribution (scaled by `base^(m-1) mod p`), multiplies by `base`, and adds the incoming byte — three operations per slide step. On collision, a full character comparison confirms the match, maintaining correctness at the cost of rare extra work.

## OCaml Approach

OCaml implements rolling hash with `int` arithmetic, relying on its 63-bit native integers to avoid overflow on most platforms. The `Char.code` function extracts byte values. The rolling hash state threads through a tail-recursive function rather than a loop, maintaining immutability. OCaml's `List.rev` collects match positions in reverse and reverses at the end. For multi-pattern Rabin-Karp, a `Hashtbl` maps hash values to pattern lists, checking all patterns with matching hashes.

## Key Differences

| Aspect | Rust | OCaml |
|---|---|---|
| Arithmetic | `u64` with explicit `% modulus` | `int` (63-bit) with `mod` |
| Byte access | `.as_bytes()[i]` | `Char.code (String.get s i)` |
| Hash state | Mutable variable in loop | Threaded through recursion |
| Multi-pattern | `HashMap<u64, Vec<Pattern>>` | `Hashtbl` keyed by hash |
| Overflow risk | None with `u64 % modulus` | Rare on 64-bit, possible on 32-bit |
| False positives | Verified with `==` comparison | Same character-level check |

## Exercises

1. Implement 2D Rabin-Karp to find a pattern matrix inside a larger text matrix.
2. Add multi-pattern support: search for all patterns in a set simultaneously in a single pass.
3. Measure false positive rate empirically with random text and various base/modulus choices.
4. Compare performance with Boyer-Moore on long text with many short patterns of equal length.
5. Handle Unicode by hashing on code points rather than bytes and verify correctness.
