📖 **[View on hightechmind.io →](https://hightechmind.io/rust/264-string-anagram)**

---

# Example 264: String Anagram Check

**Difficulty:** ⭐  
**Category:** String Processing  
**OCaml Source:** https://exercism.org/tracks/ocaml/exercises/anagram

## Problem Statement

Check if two strings are anagrams — they use exactly the same letters in a different arrangement. Also find all anagrams of a word from a list of candidates.

## Learning Outcomes

- String transformation with `to_lowercase()` and character iteration
- Two approaches: sorting-based (O(n log n)) and frequency-counting (O(n))
- Using closures as local helper functions
- Filtering with iterator adapters and lifetime annotations

## OCaml Approach

OCaml converts strings to character lists via `String.to_seq |> List.of_seq`, sorts them with `List.sort Char.compare`, and compares. The pipeline operator `|>` chains the transformations cleanly.

## Rust Approach

Rust offers two idiomatic approaches: sorting a `Vec<char>` (mirrors OCaml) or counting character frequencies with a `HashMap`. The iterator-based `filter` with `find_anagrams` parallels OCaml's `List.filter`.

## Key Differences

1. **String to chars:** OCaml uses `String.to_seq |> List.of_seq`; Rust uses `.chars().collect::<Vec<_>>()`
2. **Sorting:** OCaml sorts a linked list (O(n log n)); Rust sorts a Vec in-place with `sort_unstable`
3. **Frequency counting:** Not idiomatic in OCaml stdlib; Rust's HashMap makes O(n) solution natural
4. **Lifetimes:** Rust's `find_anagrams` needs `'a` lifetime to borrow candidate strings from the input slice

## Exercises

1. Generalize `is_anagram` to work on any `Iterator<Item=char>` rather than `&str`, enabling anagram detection over character streams.
2. Implement `group_anagrams` that takes a list of words and groups them into buckets where each bucket contains words that are anagrams of each other.
3. Build a fast anagram index: preprocess a large word list into a `HashMap<String, Vec<String>>` mapping sorted-letter keys to matching words, then look up all anagrams of a query word in O(k log k) time (k = word length).
