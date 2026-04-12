[![hightechmind.io](https://img.shields.io/badge/hightechmind.io-functional--rust-blue)](https://hightechmind.io)

# 092 — Pangram Check
**Difficulty:** ⭐  
**Category:** Functional Programming  



## Problem Statement

Determine whether a string contains every letter of the alphabet at least once. Implement three approaches: `HashSet`-based (collect lowercase letters, check `.len() == 26`), bitset-based (OR 26 bits, check all set), and functional (check `('a'..='z').all(|c| lower.contains(c))`). Compare with OCaml's `Set.Make(Char)` and subset check.

## Learning Outcomes

- Collect characters into a `HashSet<char>` and check cardinality
- Use a `u32` bitset with bit-shifting to track 26 booleans in one integer
- Apply `('a'..='z').all(|c| s.contains(c))` for a readable functional check
- Use `filter_map` to normalise case and filter non-alpha in one pass
- Map Rust's three approaches to OCaml's `CS.subset alphabet chars`
- Choose the right approach based on readability vs performance requirements

## Rust Application

The `HashSet` approach normalises each character to lowercase, filters non-alphabetic characters with `filter_map`, collects into a `HashSet<char>`, and checks `.len() == 26`. The bitset approach maintains a `u32` where bit `i` represents letter `i`; checking `bits == (1 << 26) - 1` verifies all 26 are set. The functional `all` approach is most readable: `('a'..='z').all(|c| lower.contains(c))` — but `String::contains` is O(n), making this O(26n) overall.

## OCaml Approach

OCaml builds the `alphabet` set once as a `CS.of_list` of 26 chars. `is_pangram` filters the input string to lowercase letters via `Seq.filter`, builds a `CS.of_seq`, then checks `CS.subset alphabet chars`. This is clean and declarative. OCaml's `Set.Make(Char)` is a balanced BST, so operations are O(log n); Rust's `HashSet` is O(1) average.

## Key Differences

| Aspect | Rust HashSet | Rust Bitset | OCaml |
|--------|-------------|-------------|-------|
| Data structure | `HashSet<char>` | `u32` | `Set.Make(Char)` |
| Check | `.len() == 26` | `bits == (1<<26)-1` | `CS.subset` |
| Memory | ~26 entries | 4 bytes | ~26 balanced nodes |
| Performance | O(n) avg | O(n) optimal | O(n log 26) |
| Readability | High | Low | High |
| Case normalise | `to_ascii_lowercase` | Same | `String.lowercase_ascii` |

For a pangram check, the bitset is the fastest (single integer comparison) but least readable. The `all` approach is most readable but least efficient. In practice, the bitset version is preferred when performance matters; the `HashSet` version when generality does.

## Exercises

1. Add a `missing_letters(s: &str) -> Vec<char>` function that returns letters not present in the string.
2. Implement a `pangram_score(s: &str) -> usize` that counts how many distinct letters appear.
3. Write a property test: `is_pangram_hashset(s) == is_pangram_bitset(s) == is_pangram_all(s)` for any `s`.
4. Extend to Unicode: check whether a string contains all letters from a given alphabet set (not just ASCII).
5. In OCaml, benchmark `CS.subset`-based vs bitset-based pangram check on a 1MB string with all letters.
