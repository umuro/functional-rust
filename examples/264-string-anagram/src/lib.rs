#![allow(clippy::all)]
//! String Anagram Check
//!
//! OCaml: sorts character lists and compares.
//! Rust: can use sorted Vec<char> or frequency counting with a HashMap.
//!
//! An anagram uses exactly the same letters in a different arrangement.
//! Two strings are anagrams if they have the same letter frequencies
//! but are not identical (case-insensitive).

//! Solution 1: Idiomatic Rust — sort characters and compare.
//! Mirrors the OCaml approach closely.
pub fn is_anagram_sort(s1: &str, s2: &str) -> bool {
    let normalize = |s: &str| -> String { s.to_lowercase() };
    let sorted_chars = |s: &str| -> Vec<char> {
        let mut chars: Vec<char> = s.to_lowercase().chars().collect();
        chars.sort_unstable();
        chars
    };
    normalize(s1) != normalize(s2) && sorted_chars(s1) == sorted_chars(s2)
}

/// Solution 2: Frequency counting — O(n) using a HashMap.
/// More efficient than sorting for long strings.
pub fn is_anagram_freq(s1: &str, s2: &str) -> bool {
    use std::collections::HashMap;

    let lower1 = s1.to_lowercase();
    let lower2 = s2.to_lowercase();
    if lower1 == lower2 {
        return false;
    }

    let freq = |s: &str| -> HashMap<char, i32> {
        let mut map = HashMap::new();
        for c in s.chars() {
            *map.entry(c).or_insert(0) += 1;
        }
        map
    };
    freq(&lower1) == freq(&lower2)
}

/// Finds all anagrams of `word` in a list of candidates.
/// OCaml: `let find_anagrams word candidates = List.filter (is_anagram word) candidates`
pub fn find_anagrams<'a>(word: &str, candidates: &[&'a str]) -> Vec<&'a str> {
    candidates
        .iter()
        .copied()
        .filter(|c| is_anagram_sort(word, c))
        .collect()
}

/// Functional/iterator approach: find anagrams using the freq method.
pub fn find_anagrams_freq<'a>(word: &str, candidates: &[&'a str]) -> Vec<&'a str> {
    candidates
        .iter()
        .copied()
        .filter(|c| is_anagram_freq(word, c))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_anagram() {
        assert!(is_anagram_sort("listen", "silent"));
        assert!(is_anagram_freq("listen", "silent"));
    }

    #[test]
    fn test_not_anagram() {
        assert!(!is_anagram_sort("hello", "world"));
        assert!(!is_anagram_freq("hello", "world"));
    }

    #[test]
    fn test_same_word_not_anagram() {
        // A word is not an anagram of itself
        assert!(!is_anagram_sort("listen", "listen"));
        assert!(!is_anagram_freq("listen", "listen"));
    }

    #[test]
    fn test_case_insensitive() {
        assert!(is_anagram_sort("Listen", "Silent"));
        assert!(is_anagram_freq("Listen", "Silent"));
    }

    #[test]
    fn test_different_lengths() {
        assert!(!is_anagram_sort("abc", "abcd"));
        assert!(!is_anagram_freq("abc", "abcd"));
    }

    #[test]
    fn test_find_anagrams() {
        let results = find_anagrams("listen", &["enlists", "google", "inlets", "silent"]);
        assert_eq!(results, vec!["inlets", "silent"]);
    }

    #[test]
    fn test_find_anagrams_freq() {
        let results = find_anagrams_freq("listen", &["enlists", "google", "inlets", "silent"]);
        assert_eq!(results, vec!["inlets", "silent"]);
    }

    #[test]
    fn test_empty_strings() {
        // Two empty strings are equal, not anagrams
        assert!(!is_anagram_sort("", ""));
        assert!(!is_anagram_freq("", ""));
    }
}
