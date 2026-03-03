/// Map.Make Functor — String→Int Dictionary
///
/// OCaml's `Map.Make(String)` creates a specialized balanced BST map.
/// Rust's `std::collections::BTreeMap` and `HashMap` serve the same role.
/// The key difference: OCaml uses functors (module-level functions) to
/// parameterize the map by key type; Rust uses generics with trait bounds.

use std::collections::BTreeMap;

/// Build a word-length map using BTreeMap (ordered, like OCaml's Map).
pub fn word_lengths(words: &[&str]) -> BTreeMap<String, usize> {
    words
        .iter()
        .map(|w| (w.to_string(), w.len()))
        .collect()
}

/// Filter entries by a predicate on values.
pub fn filter_by_value<K: Ord + Clone, V: Clone>(
    map: &BTreeMap<K, V>,
    pred: impl Fn(&V) -> bool,
) -> BTreeMap<K, V> {
    map.iter()
        .filter(|(_, v)| pred(v))
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect()
}

/// Map over values, producing a new map.
pub fn map_values<K: Ord + Clone, V, U>(
    map: &BTreeMap<K, V>,
    f: impl Fn(&V) -> U,
) -> BTreeMap<K, U> {
    map.iter()
        .map(|(k, v)| (k.clone(), f(v)))
        .collect()
}

/// Using HashMap for O(1) average lookup (unordered).
use std::collections::HashMap;

pub fn word_lengths_hash(words: &[&str]) -> HashMap<String, usize> {
    words.iter().map(|w| (w.to_string(), w.len())).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_word_lengths() {
        let words = vec!["ocaml", "rust", "haskell", "erlang", "go"];
        let m = word_lengths(&words);
        assert_eq!(m.get("rust"), Some(&4));
        assert_eq!(m.get("haskell"), Some(&7));
        assert_eq!(m.get("missing"), None);
    }

    #[test]
    fn test_filter() {
        let words = vec!["ocaml", "rust", "haskell", "erlang", "go"];
        let m = word_lengths(&words);
        let long = filter_by_value(&m, |v| *v > 4);
        assert_eq!(long.len(), 3); // ocaml(5), haskell(7), erlang(6)
        assert!(long.contains_key("haskell"));
        assert!(!long.contains_key("go"));
    }

    #[test]
    fn test_map_values() {
        let words = vec!["rust", "go"];
        let m = word_lengths(&words);
        let doubled = map_values(&m, |v| v * 2);
        assert_eq!(doubled.get("rust"), Some(&8));
        assert_eq!(doubled.get("go"), Some(&4));
    }

    #[test]
    fn test_empty() {
        let m = word_lengths(&[]);
        assert!(m.is_empty());
    }

    #[test]
    fn test_hash_map() {
        let words = vec!["rust", "go"];
        let m = word_lengths_hash(&words);
        assert_eq!(m.get("rust"), Some(&4));
    }

    #[test]
    fn test_btree_ordering() {
        let words = vec!["zebra", "apple", "mango"];
        let m = word_lengths(&words);
        let keys: Vec<_> = m.keys().collect();
        assert_eq!(keys, vec!["apple", "mango", "zebra"]); // sorted
    }
}

fn main() {
    println!("{:?}", m.get("rust"), Some(&4));
    println!("{:?}", m.get("haskell"), Some(&7));
    println!("{:?}", m.get("missing"), None);
}
