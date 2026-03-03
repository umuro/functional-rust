//! # Map Module — Frequency Counter
//!
//! Count word frequencies using a map. OCaml's `Map.Make(String)` creates
//! an immutable tree map; Rust's `HashMap` is the standard mutable map.

use std::collections::HashMap;
use std::collections::BTreeMap;

// ---------------------------------------------------------------------------
// Approach A: HashMap (idiomatic Rust)
// ---------------------------------------------------------------------------

pub fn word_freq_hashmap(text: &str) -> HashMap<String, usize> {
    let mut freq = HashMap::new();
    for word in text.split_whitespace() {
        let w = word.to_lowercase();
        *freq.entry(w).or_insert(0) += 1;
    }
    freq
}

// ---------------------------------------------------------------------------
// Approach B: BTreeMap (sorted, like OCaml's Map)
// ---------------------------------------------------------------------------

pub fn word_freq_btree(text: &str) -> BTreeMap<String, usize> {
    text.split_whitespace()
        .map(|w| w.to_lowercase())
        .fold(BTreeMap::new(), |mut acc, w| {
            *acc.entry(w).or_insert(0) += 1;
            acc
        })
}

// ---------------------------------------------------------------------------
// Approach C: Functional — collect into counts
// ---------------------------------------------------------------------------

pub fn word_freq_functional(text: &str) -> BTreeMap<String, usize> {
    let words: Vec<String> = text
        .split_whitespace()
        .map(|w| w.to_lowercase())
        .collect();

    let mut sorted = words.clone();
    sorted.sort();

    let mut result = BTreeMap::new();
    let mut i = 0;
    while i < sorted.len() {
        let word = &sorted[i];
        let count = sorted[i..].iter().take_while(|w| *w == word).count();
        result.insert(word.clone(), count);
        i += count;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_freq() {
        let freq = word_freq_hashmap("the cat sat on the mat the cat");
        assert_eq!(freq["the"], 3);
        assert_eq!(freq["cat"], 2);
        assert_eq!(freq["sat"], 1);
    }

    #[test]
    fn test_btree_sorted() {
        let freq = word_freq_btree("the cat sat on the mat the cat");
        let keys: Vec<&String> = freq.keys().collect();
        assert_eq!(keys, vec!["cat", "mat", "on", "sat", "the"]);
    }

    #[test]
    fn test_empty() {
        let freq = word_freq_hashmap("");
        assert!(freq.is_empty());
    }

    #[test]
    fn test_case_insensitive() {
        let freq = word_freq_hashmap("The THE the");
        assert_eq!(freq["the"], 3);
    }

    #[test]
    fn test_functional_matches() {
        let a = word_freq_btree("hello world hello");
        let b = word_freq_functional("hello world hello");
        assert_eq!(a, b);
    }
}
