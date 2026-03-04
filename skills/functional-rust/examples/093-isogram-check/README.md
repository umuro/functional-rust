# 093: Isogram Check

**Difficulty:** 1  **Level:** Beginner

Check whether a word contains no repeated letters (ignoring case, hyphens, and spaces).

## The Problem This Solves

"Lumberjacks" is an isogram — every letter appears exactly once. "Eleven" is not — `e` appears three times. Isogram checking is a clean exercise in duplicate detection: the same pattern appears in finding duplicate IDs, validating unique usernames, checking for repeated elements in configuration.

Three approaches with different trade-offs: sort-and-dedup (mirrors OCaml's `List.sort_uniq`, extra memory), hash set with early exit (most practical), bitset (fastest for ASCII alphabets).

## The Intuition

The set approach: as you scan letters, add each to a set. If a letter is already in the set, you found a duplicate — it's not an isogram. If you reach the end without duplicates, it is an isogram.

`HashSet::insert` in Rust returns `false` when the element was already present. Combined with `.all()`, this gives early exit on the first duplicate without any explicit `if` check.

The bitset approach: same idea but with a `u32`. If the bit for a letter is already set when you encounter it, you found a duplicate. Otherwise, set the bit and continue.

## How It Works in Rust

```rust
use std::collections::HashSet;

// HashSet with early exit — idiomatic, practical
pub fn is_isogram_hashset(s: &str) -> bool {
    let mut seen = HashSet::new();
    s.chars()
        .filter(|c| c.is_ascii_alphabetic())  // ignore hyphens, spaces
        .all(|c| seen.insert(c.to_ascii_lowercase()))
        // insert returns false if already present → .all() short-circuits
}

// Sort + dedup — mirrors OCaml's List.sort_uniq
pub fn is_isogram_sort(s: &str) -> bool {
    let mut chars: Vec<char> = s.chars()
        .filter_map(|c| c.is_ascii_alphabetic().then_some(c.to_ascii_lowercase()))
        .collect();
    let total = chars.len();
    chars.sort_unstable();
    chars.dedup();          // remove consecutive duplicates (works after sort)
    chars.len() == total    // if dedup removed anything, there were duplicates
}

// Bitset — fastest for ASCII letters
pub fn is_isogram_bitset(s: &str) -> bool {
    let mut bits: u32 = 0;
    for c in s.chars().filter(|c| c.is_ascii_alphabetic()) {
        let mask = 1 << (c.to_ascii_lowercase() as u32 - 'a' as u32);
        if bits & mask != 0 { return false; }  // bit already set = duplicate
        bits |= mask;
    }
    true
}
```

The `HashSet::insert` returning `bool` trick is the most idiomatic Rust: it makes `.all()` the natural way to check "no duplicates found while inserting."

## What This Unlocks

- **Duplicate detection pattern** — `HashSet::insert` returning `false` on duplicate is reusable anywhere you need "insert and check for duplicate."
- **`then_some()` shortcut** — `condition.then_some(value)` is cleaner than `if condition { Some(value) } else { None }` in `filter_map` chains.
- **Bitset for letter problems** — the same 26-bit pattern from pangram check, but checking for collision instead of completion.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Sort + dedup | `List.sort_uniq Char.compare chars` (one call) | `sort_unstable()` then `dedup()` (two calls) |
| HashSet duplicate check | `not (List.mem c seen)` + add | `seen.insert(c)` returns `false` on duplicate |
| Early exit | `List.exists` / exception | `.all(\|c\| seen.insert(c))` short-circuits naturally |
| Letter filter | `Char.code c >= 97 && ...` | `.is_ascii_alphabetic()` |
| Bitset | Manual bit ops | `u32` with `&` and `\|=` |
