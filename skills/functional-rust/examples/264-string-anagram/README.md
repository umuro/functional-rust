# 264: String Anagram Check

**Difficulty:** 1  **Level:** Beginner

Two approaches to check if strings are anagrams — sort-and-compare (O(n log n)) and frequency counting (O(n)).

## The Problem This Solves

You're building a word game, a spell checker, or a search feature that suggests rearrangements. You need to determine whether two words use the exact same letters in different orders: "listen" and "silent" are anagrams; "listen" and "enlist" are too. "listen" and "listens" are not.

Without a built-in anagram check, you'd manually count character occurrences, handle case normalization, and compare frequency maps — all boilerplate that distracts from your actual problem. For filtering a list of candidates (the common real-world use case), you'd loop over each candidate and repeat the comparison logic.

The sort-based approach mirrors functional programming idioms: transform both strings the same way, compare results. The frequency-count approach is more efficient for large strings. Knowing both gives you the right tool for each situation.

## The Intuition

Two strings are anagrams if sorting their characters produces the same sequence — or equivalently, if they have identical character frequency maps.

## How It Works in Rust

```rust
// Sort-based: O(n log n) — mirrors OCaml approach
pub fn is_anagram_sort(s1: &str, s2: &str) -> bool {
    let sorted_chars = |s: &str| -> Vec<char> {
        let mut chars: Vec<char> = s.to_lowercase().chars().collect();
        chars.sort_unstable();  // in-place sort, faster than sort()
        chars
    };
    // Same word is not an anagram of itself
    s1.to_lowercase() != s2.to_lowercase() && sorted_chars(s1) == sorted_chars(s2)
}

// Frequency-based: O(n) — no sorting, HashMap counts each char
pub fn is_anagram_freq(s1: &str, s2: &str) -> bool {
    use std::collections::HashMap;
    let freq = |s: &str| -> HashMap<char, i32> {
        let mut map = HashMap::new();
        for c in s.to_lowercase().chars() {
            *map.entry(c).or_insert(0) += 1;
        }
        map
    };
    s1.to_lowercase() != s2.to_lowercase() && freq(s1) == freq(s2)
}

// Filter candidates: the classic use case
pub fn find_anagrams<'a>(word: &str, candidates: &[&'a str]) -> Vec<&'a str> {
    candidates.iter().copied()
        .filter(|c| is_anagram_sort(word, c))
        .collect()
}
// 'a lifetime: candidates outlive the returned Vec — borrows, not copies
```

## What This Unlocks

- **Word game engines:** Scrabble-style solvers that find all valid words from a rack of letters.
- **Duplicate detection:** Detecting near-duplicate records where fields are transposed or reordered.
- **Test data generation:** Generating all permutations of a string programmatically using the same char-frequency foundation.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| String to chars | `String.to_seq \|> List.of_seq` | `.chars().collect::<Vec<char>>()` |
| Sort | `List.sort Char.compare` (new list) | `.sort_unstable()` (in-place, faster) |
| Frequency map | Manual fold with `Map` | `HashMap` with `.entry().or_insert(0)` |
| Borrowing candidates | N/A (GC manages) | `'a` lifetime ties return to input slice |
| Pipeline | `\|>` operator | Method chaining `.filter().collect()` |
