//! Set Operations for Data Processing
//!
//! Demonstrates deduplication and membership testing using `HashSet`,
//! mirroring OCaml's `Set.Make` functor pattern.

use std::collections::HashSet;

/// Deduplicate a slice of strings, returning a `HashSet`.
/// OCaml: `StringSet.of_list words`
pub fn unique_words<'a>(words: &[&'a str]) -> HashSet<&'a str> {
    words.iter().copied().collect()
}

/// Remove stopwords from a set of words.
/// OCaml: `StringSet.diff unique stopwords`
pub fn remove_stopwords<'a>(
    words: &HashSet<&'a str>,
    stopwords: &HashSet<&'a str>,
) -> HashSet<&'a str> {
    words.difference(stopwords).copied().collect()
}

/// Check whether a word is a member of a set.
/// OCaml: `StringSet.mem word set`
pub fn is_member(set: &HashSet<&str>, word: &str) -> bool {
    set.contains(word)
}

/// Union of two word sets.
/// OCaml: `StringSet.union a b`
pub fn union<'a>(a: &HashSet<&'a str>, b: &HashSet<&'a str>) -> HashSet<&'a str> {
    a.union(b).copied().collect()
}

/// Intersection of two word sets.
/// OCaml: `StringSet.inter a b`
pub fn intersect<'a>(a: &HashSet<&'a str>, b: &HashSet<&'a str>) -> HashSet<&'a str> {
    a.intersection(b).copied().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unique_deduplicates() {
        let words = ["the", "cat", "sat", "on", "the", "mat", "the", "cat"];
        let result = unique_words(&words);
        assert_eq!(result.len(), 5);
        assert!(result.contains("the"));
        assert!(result.contains("cat"));
        assert!(result.contains("sat"));
        assert!(result.contains("on"));
        assert!(result.contains("mat"));
    }

    #[test]
    fn test_unique_empty() {
        let result = unique_words(&[]);
        assert!(result.is_empty());
    }

    #[test]
    fn test_remove_stopwords() {
        let words = ["the", "cat", "sat", "on", "the", "mat", "the", "cat"];
        let unique = unique_words(&words);
        let stopwords: HashSet<&str> = ["the", "on", "a", "an"].iter().copied().collect();
        let content = remove_stopwords(&unique, &stopwords);
        assert_eq!(content.len(), 3);
        assert!(content.contains("cat"));
        assert!(content.contains("sat"));
        assert!(content.contains("mat"));
        assert!(!content.contains("the"));
        assert!(!content.contains("on"));
    }

    #[test]
    fn test_remove_stopwords_no_overlap() {
        let words: HashSet<&str> = ["cat", "mat"].iter().copied().collect();
        let stopwords: HashSet<&str> = ["the", "on"].iter().copied().collect();
        let result = remove_stopwords(&words, &stopwords);
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn test_is_member() {
        let set: HashSet<&str> = ["cat", "mat", "sat"].iter().copied().collect();
        assert!(is_member(&set, "cat"));
        assert!(!is_member(&set, "the"));
    }

    #[test]
    fn test_union() {
        let a: HashSet<&str> = ["cat", "mat"].iter().copied().collect();
        let b: HashSet<&str> = ["mat", "rat"].iter().copied().collect();
        let result = union(&a, &b);
        assert_eq!(result.len(), 3);
        assert!(result.contains("cat"));
        assert!(result.contains("mat"));
        assert!(result.contains("rat"));
    }

    #[test]
    fn test_intersect() {
        let a: HashSet<&str> = ["cat", "mat", "sat"].iter().copied().collect();
        let b: HashSet<&str> = ["mat", "rat", "sat"].iter().copied().collect();
        let result = intersect(&a, &b);
        assert_eq!(result.len(), 2);
        assert!(result.contains("mat"));
        assert!(result.contains("sat"));
        assert!(!result.contains("cat"));
    }

    #[test]
    fn test_intersect_empty() {
        let a: HashSet<&str> = ["cat", "mat"].iter().copied().collect();
        let b: HashSet<&str> = ["the", "on"].iter().copied().collect();
        let result = intersect(&a, &b);
        assert!(result.is_empty());
    }
}
