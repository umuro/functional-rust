use std::collections::BTreeMap;
/// Word Count with Map
///
/// Building a word-frequency map from text. Demonstrates string
/// normalization, splitting, and folding into a map.
use std::collections::HashMap;

/// Tokenize: lowercase and extract alphanumeric words.
pub fn tokenize(s: &str) -> Vec<String> {
    let s = s.to_lowercase();
    let mut words = Vec::new();
    let mut buf = String::new();

    for c in s.chars() {
        if c.is_alphanumeric() {
            buf.push(c);
        } else if !buf.is_empty() {
            words.push(buf.clone());
            buf.clear();
        }
    }
    if !buf.is_empty() {
        words.push(buf);
    }
    words
}

/// Word count using HashMap — O(1) average lookup.
pub fn word_count(sentence: &str) -> HashMap<String, usize> {
    let mut map = HashMap::new();
    for word in tokenize(sentence) {
        *map.entry(word).or_insert(0) += 1;
    }
    map
}

/// Word count using iterator fold — more functional style.
pub fn word_count_fold(sentence: &str) -> HashMap<String, usize> {
    tokenize(sentence)
        .into_iter()
        .fold(HashMap::new(), |mut map, word| {
            *map.entry(word).or_insert(0) += 1;
            map
        })
}

/// Ordered word count using BTreeMap (like OCaml's Map).
pub fn word_count_ordered(sentence: &str) -> BTreeMap<String, usize> {
    let mut map = BTreeMap::new();
    for word in tokenize(sentence) {
        *map.entry(word).or_insert(0) += 1;
    }
    map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let m = word_count("the cat sat on the mat");
        assert_eq!(m.get("the"), Some(&2));
        assert_eq!(m.get("cat"), Some(&1));
    }

    #[test]
    fn test_punctuation() {
        let m = word_count("the cat sat on the mat, the cat sat");
        assert_eq!(m.get("the"), Some(&3));
        assert_eq!(m.get("cat"), Some(&2));
        assert_eq!(m.get("sat"), Some(&2));
    }

    #[test]
    fn test_case_insensitive() {
        let m = word_count("The THE the");
        assert_eq!(m.get("the"), Some(&3));
    }

    #[test]
    fn test_empty() {
        let m = word_count("");
        assert!(m.is_empty());
    }

    #[test]
    fn test_single_word() {
        let m = word_count("hello");
        assert_eq!(m.get("hello"), Some(&1));
        assert_eq!(m.len(), 1);
    }

    #[test]
    fn test_fold_matches() {
        let s = "the cat sat on the mat";
        assert_eq!(word_count(s), word_count_fold(s));
    }

    #[test]
    fn test_ordered() {
        let m = word_count_ordered("banana apple cherry apple");
        let keys: Vec<_> = m.keys().collect();
        assert_eq!(keys, vec!["apple", "banana", "cherry"]);
    }
}
