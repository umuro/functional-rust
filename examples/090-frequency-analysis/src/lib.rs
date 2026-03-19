#![allow(clippy::all)]
use std::collections::{BTreeMap, HashMap};

/// Solution 1: Idiomatic Rust — HashMap with fold
/// Uses entry API for efficient in-place counting
pub fn frequency(s: &str) -> HashMap<char, usize> {
    s.chars()
        .filter(|c| c.is_ascii_alphabetic())
        .map(|c| c.to_ascii_lowercase())
        .fold(HashMap::new(), |mut map, c| {
            *map.entry(c).or_insert(0) += 1;
            map
        })
}

/// Solution 2: BTreeMap — mirrors OCaml's Map.Make(Char), keys stay sorted
/// Equivalent to OCaml's `CMap.update c (function None -> Some 1 | Some n -> Some (n+1)) m`
pub fn frequency_btree(s: &str) -> BTreeMap<char, usize> {
    s.chars()
        .filter(|c| c.is_ascii_alphabetic())
        .map(|c| c.to_ascii_lowercase())
        .fold(BTreeMap::new(), |mut map, c| {
            *map.entry(c).or_insert(0) += 1;
            map
        })
}

/// Solution 3: Sorted by frequency descending, ties broken alphabetically
/// Mirrors OCaml's `List.sort (fun (_, a) (_, b) -> compare b a)`
pub fn sorted_freq(s: &str) -> Vec<(char, usize)> {
    let mut pairs: Vec<(char, usize)> = frequency(s).into_iter().collect();
    pairs.sort_by(|(c1, n1), (c2, n2)| n2.cmp(n1).then(c1.cmp(c2)));
    pairs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_string() {
        assert_eq!(frequency(""), HashMap::new());
        assert!(sorted_freq("").is_empty());
    }

    #[test]
    fn test_single_character() {
        let freq = frequency("a");
        assert_eq!(freq[&'a'], 1);
        assert_eq!(freq.len(), 1);
    }

    #[test]
    fn test_case_insensitive() {
        let freq = frequency("AaAa");
        assert_eq!(freq[&'a'], 4);
        assert_eq!(freq.len(), 1);
    }

    #[test]
    fn test_non_alpha_ignored() {
        let freq = frequency("a1b2c! a");
        assert_eq!(freq[&'a'], 2);
        assert_eq!(freq[&'b'], 1);
        assert_eq!(freq[&'c'], 1);
        assert_eq!(freq.len(), 3);
    }

    #[test]
    fn test_pangram_all_letters_present() {
        let text = "The quick brown fox jumps over the lazy dog";
        let freq = frequency(text);
        // All 26 letters appear at least once in a pangram
        assert_eq!(freq.len(), 26);
        // 'e' and 'o' appear most in this pangram
        assert_eq!(freq[&'e'], 3);
        assert_eq!(freq[&'o'], 4);
    }

    #[test]
    fn test_btree_sorted_by_key() {
        let freq = frequency_btree("bac");
        let keys: Vec<char> = freq.keys().copied().collect();
        assert_eq!(keys, vec!['a', 'b', 'c']);
    }

    #[test]
    fn test_sorted_freq_descending() {
        let result = sorted_freq("aaabbc");
        assert_eq!(result[0], ('a', 3));
        assert_eq!(result[1], ('b', 2));
        assert_eq!(result[2], ('c', 1));
    }

    #[test]
    fn test_sorted_freq_ties_broken_alphabetically() {
        // 'a' and 'b' both appear twice — 'a' should come first
        let result = sorted_freq("aabb");
        assert_eq!(result[0], ('a', 2));
        assert_eq!(result[1], ('b', 2));
    }
}
