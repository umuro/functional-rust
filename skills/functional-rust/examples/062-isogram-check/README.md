# 062: Isogram Check

**Difficulty:** ⭐  **Level:** Foundations

Detect whether a word has any repeating letters — a classic interview problem solved cleanly with a HashSet.

## The Problem This Solves

You want to check for duplicate characters in a string. Maybe you're validating a crossword answer, building a word puzzle, or writing a coding challenge checker. The constraint is simple: every letter must appear exactly once.

The brute-force way sorts the string and compares adjacent characters, or uses a nested loop. Both are clunky. In Python you'd write `len(set(word)) == len(word)`. JavaScript does similar with `Set`.

Rust gives you two clean approaches: compare the length of a `HashSet` to the original count (simple), or use `HashSet::insert()` which returns `false` on duplicates and exit immediately (fast). The second approach is the more idiomatic Rust — it stops at the first problem rather than scanning everything.

## The Intuition

`HashSet::insert()` returns a boolean: `true` if the value was new, `false` if it was already there. That one detail makes duplicate detection elegant — you don't need to check after the fact, you learn about duplicates the moment they happen.

Python's `set` doesn't tell you whether insertion was new. You have to compare lengths afterward. Rust's API gives you the information exactly when you need it.

## How It Works in Rust

Simple approach — compare set size to character count:

```rust
pub fn is_isogram(word: &str) -> bool {
    let letters: Vec<char> = word
        .chars()
        .filter(|c| c.is_ascii_alphabetic())  // ignore hyphens, spaces
        .map(|c| c.to_ascii_lowercase())
        .collect();
    let unique: HashSet<char> = letters.iter().copied().collect();
    letters.len() == unique.len()  // equal means no duplicates
}
```

Early-exit approach — stop at the first duplicate:

```rust
pub fn is_isogram_early_exit(word: &str) -> bool {
    let mut seen = HashSet::new();
    for c in word.chars() {
        if c.is_ascii_alphabetic() {
            // insert() returns false if already present
            if !seen.insert(c.to_ascii_lowercase()) {
                return false;
            }
        }
    }
    true
}
```

## What This Unlocks

- **Duplicate detection** — the same `insert() → false` pattern works for any type in any collection
- **Uniqueness constraints** — validate that IDs, usernames, or configuration keys don't repeat
- **Interview problems** — "find first repeated character", "check all unique" — all use this exact pattern

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Set insert | `Set.add` (always succeeds, returns new set) | `HashSet::insert()` returns `bool` |
| Early exit | `exception` or manual recursion | `return false` inside a loop |
| Immutability | OCaml sets are immutable values | `HashSet` is mutable, must declare `mut` |
| Length | `Set.cardinal` | `.len()` |
