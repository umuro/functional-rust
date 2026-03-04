# 092: Pangram Check

**Difficulty:** 1  **Level:** Beginner

Determine whether a string contains every letter of the English alphabet at least once.

## The Problem This Solves

"The quick brown fox jumps over the lazy dog" is a famous pangram — it contains all 26 letters. Pangram detection is a clean string-processing exercise that exercises character classification, set operations, and bit manipulation.

In production, this pattern generalises to "does this input contain all required elements?" — checking required fields, mandatory keywords, complete alphabets, all required keys in a config. The algorithms here scale from 26 letters to arbitrary required sets.

## The Intuition

The naive approach: build a set of all lowercase letters in the string, then check if the set has 26 elements. That's the `HashSet` approach — intuitive, O(n) time, O(26) space.

A faster approach for fixed alphabets: use 26 bits of a `u32`. Bit `i` represents letter `i`. Set bit `i` when letter `i` appears. When all 26 bits are set, the number equals `(1 << 26) - 1` — you have a pangram. No hash table, just integer operations.

The most readable approach: `('a'..='z').all(|c| s.contains(c))`. Correct, O(26n) time, but the intent is crystal clear.

## How It Works in Rust

```rust
use std::collections::HashSet;

// HashSet approach — mirrors OCaml's Set.Make(Char)
pub fn is_pangram_hashset(s: &str) -> bool {
    let chars: HashSet<char> = s.chars()
        .filter_map(|c| {
            let lc = c.to_ascii_lowercase();
            if lc.is_ascii_lowercase() { Some(lc) } else { None }
        })
        .collect();
    chars.len() == 26  // exactly 26 distinct letters = pangram
}

// Bitset approach — 26 bits, one per letter
pub fn is_pangram_bitset(s: &str) -> bool {
    let mut bits: u32 = 0;
    for c in s.chars() {
        let lc = c.to_ascii_lowercase();
        if lc.is_ascii_lowercase() {
            bits |= 1 << (lc as u32 - 'a' as u32);  // set bit for this letter
        }
    }
    bits == (1 << 26) - 1  // all 26 bits set?
}

// Declarative approach — reads like the definition
pub fn is_pangram_all(s: &str) -> bool {
    let lower = s.to_ascii_lowercase();
    ('a'..='z').all(|c| lower.contains(c))
}
```

The bitset approach is the fastest in practice: integer comparison instead of hash lookups, cache-friendly, branchless bit operations.

## What This Unlocks

- **Bitset technique for small alphabets** — any fixed-size membership problem with ≤64 elements can use a `u64` bitset instead of a hash set.
- **`filter_map` pattern** — transform and filter in one pass; here, lowercase and filter non-letters simultaneously.
- **`all()` for universal checks** — "do all required elements appear?" is `.all()` on the requirements.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Char set | `module CharSet = Set.Make(Char)` — functor required | `HashSet<char>` — works out of the box |
| Bitset | Manual `Int32` manipulation | `u32` with `\|=` and `<<` |
| Lowercase | `Char.lowercase_ascii` | `.to_ascii_lowercase()` |
| Letter check | `Char.code c - Char.code 'a'` | `c as u32 - 'a' as u32` |
| All-letters check | `CharSet.cardinal set = 26` | `chars.len() == 26` or `bits == (1<<26)-1` |
