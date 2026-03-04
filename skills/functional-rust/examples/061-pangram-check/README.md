# 061: Pangram Check

**Difficulty:** ⭐  **Level:** Foundations

Check whether a sentence uses every letter of the alphabet at least once — with iterators and a HashSet.

## The Problem This Solves

Every programmer encounters string validation tasks: "does this input contain the right characters?" Pangram detection is the clearest version of that problem. Validate a sentence for a typing game, check a font specimen covers all letters, or write a quick smoke-test for your keyboard — the pattern is the same.

The naive approach is a double loop: for each of the 26 letters, scan the entire string. It works, but it's O(26n) and reads like a TODO list. Python coders reach for `set(sentence.lower()) >= set('abcdefghijklmnopqrstuvwxyz')`. JavaScript developers write something similar with `Set`.

Rust gives you the exact same idea, but the iterator pipeline is explicit, efficient, and reads like a description of what you're doing: filter to letters, lowercase them, collect into a set, count.

## The Intuition

Think of it as a checklist of 26 boxes. For each character in the sentence, if it's a letter, check off that box. At the end, are all 26 checked?

A `HashSet<char>` is that checklist. You add characters and duplicates disappear automatically. At the end, `len() == 26` means every letter appeared.

The bitflag variant is the same idea with a `u32` — 26 bits for 26 letters, no heap allocation. Useful when you're checking millions of strings.

## How It Works in Rust

```rust
use std::collections::HashSet;

pub fn is_pangram(sentence: &str) -> bool {
    let unique_letters: HashSet<char> = sentence
        .chars()                          // iterate over Unicode chars
        .filter(|c| c.is_ascii_alphabetic()) // keep only a-z, A-Z
        .map(|c| c.to_ascii_lowercase())  // normalize to lowercase
        .collect();                        // HashSet auto-deduplicates
    unique_letters.len() == 26
}
```

Zero-allocation bitflag version (same logic, no heap):

```rust
pub fn is_pangram_bitflag(sentence: &str) -> bool {
    let mut seen: u32 = 0;
    for c in sentence.chars() {
        if c.is_ascii_alphabetic() {
            let idx = c.to_ascii_lowercase() as u32 - 'a' as u32;
            seen |= 1 << idx;  // set bit for this letter
        }
    }
    seen == (1 << 26) - 1  // all 26 bits set?
}
```

## What This Unlocks

- **Input validation** — check that a string contains required character classes (digits, symbols, uppercase, etc.)
- **Font/keyboard testing** — verify a sample text exercises all characters you care about
- **Set intersection patterns** — the same `.filter().map().collect::<HashSet<_>>()` idiom applies to any "does A contain all of B?" question

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Set type | `module CharSet = Set.Make(Char)` | `HashSet<char>` from std |
| Build set | `List.fold_left CharSet.add CharSet.empty chars` | `.collect()` directly |
| Count | `CharSet.cardinal set` | `set.len()` |
| Bitflag | Manual with `lor`/`land` | `|=` / `&` on `u32` |
| No-alloc check | Requires explicit recursion | `bitflag` with plain `u32` |
