#![allow(clippy::all)]
/// # Map Module — Frequency Counter
///
/// Word frequency counting using HashMap — the Rust equivalent of OCaml's Map.Make.
use std::collections::HashMap;

/// Idiomatic Rust: split, lowercase, count with entry API.
/// The `entry().or_insert(0)` pattern is the standard way to update-or-insert.
pub fn word_freq(text: &str) -> HashMap<String, usize> {
    let mut freq = HashMap::new();
    for word in text.split_whitespace() {
        let lower = word.to_lowercase();
        // `entry` API: get mutable reference, inserting default if absent
        *freq.entry(lower).or_insert(0) += 1;
    }
    freq
}

/// Functional style using fold (iterator chain).
pub fn word_freq_functional(text: &str) -> HashMap<String, usize> {
    text.split_whitespace()
        .map(|w| w.to_lowercase())
        .fold(HashMap::new(), |mut acc, word| {
            *acc.entry(word).or_insert(0) += 1;
            acc
        })
}

/// Using BTreeMap for sorted output (like OCaml's Map which is ordered).
pub fn word_freq_sorted(text: &str) -> std::collections::BTreeMap<String, usize> {
    let mut freq = std::collections::BTreeMap::new();
    for word in text.split_whitespace() {
        *freq.entry(word.to_lowercase()).or_insert(0) += 1;
    }
    freq
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let freq = word_freq("the cat sat on the mat the cat");
        assert_eq!(freq["the"], 3);
        assert_eq!(freq["cat"], 2);
        assert_eq!(freq["sat"], 1);
    }

    #[test]
    fn test_empty() {
        let freq = word_freq("");
        assert!(freq.is_empty());
    }

    #[test]
    fn test_single_word() {
        let freq = word_freq("hello");
        assert_eq!(freq["hello"], 1);
    }

    #[test]
    fn test_case_insensitive() {
        let freq = word_freq("Hello hello HELLO");
        assert_eq!(freq["hello"], 3);
    }

    #[test]
    fn test_functional_version() {
        let freq = word_freq_functional("a b a c b a");
        assert_eq!(freq["a"], 3);
        assert_eq!(freq["b"], 2);
    }

    #[test]
    fn test_sorted() {
        let freq = word_freq_sorted("b a c a b");
        let keys: Vec<&String> = freq.keys().collect();
        assert_eq!(keys, vec!["a", "b", "c"]); // sorted!
    }
}
