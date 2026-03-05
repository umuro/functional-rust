use std::collections::BTreeSet;

/// Reverse a string by reversing its Unicode characters.
/// Uses an iterator chain rather than indexing.
pub fn string_rev(s: &str) -> String {
    s.chars().rev().collect()
}

/// Find all anadrome pairs from a set of words.
///
/// An anadrome is a pair (w, rev(w)) where both words appear in the set.
/// Each pair is returned once: the lexicographically smaller word comes first.
///
/// Mirrors the OCaml `get_anadromes` function using iterator combinators.
pub fn get_anadromes(words: &BTreeSet<String>) -> Vec<(String, String)> {
    words
        .iter()
        .filter_map(|s| {
            let r = string_rev(s);
            // Only emit the pair when s < r to avoid duplicates
            if s.as_str() < r.as_str() && words.contains(&r) {
                Some((s.clone(), r))
            } else {
                None
            }
        })
        .collect()
}

/// Build a word set from an iterator of strings, filtering by minimum length
/// and normalising to lowercase — mirrors the OCaml pipeline in `main`.
pub fn build_word_set<I>(words: I, min_len: usize) -> BTreeSet<String>
where
    I: IntoIterator,
    I::Item: AsRef<str>,
{
    words
        .into_iter()
        .map(|w| w.as_ref().to_lowercase())
        .filter(|w| w.len() > min_len)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── string_rev ────────────────────────────────────────────────────────────

    #[test]
    fn test_rev_empty() {
        assert_eq!(string_rev(""), "");
    }

    #[test]
    fn test_rev_single_char() {
        assert_eq!(string_rev("a"), "a");
    }

    #[test]
    fn test_rev_palindrome() {
        assert_eq!(string_rev("racecar"), "racecar");
    }

    #[test]
    fn test_rev_typical_word() {
        assert_eq!(string_rev("stressed"), "desserts");
    }

    #[test]
    fn test_rev_unicode() {
        // Reversing multi-byte chars by codepoint (not byte)
        assert_eq!(string_rev("café"), "éfac");
    }

    // ── get_anadromes ─────────────────────────────────────────────────────────

    #[test]
    fn test_no_anadromes() {
        let set: BTreeSet<String> = ["hello", "world"].iter().map(|s| s.to_string()).collect();
        assert!(get_anadromes(&set).is_empty());
    }

    #[test]
    fn test_single_anadrome_pair() {
        let set: BTreeSet<String> = ["stressed", "desserts", "unrelated"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let pairs = get_anadromes(&set);
        assert_eq!(pairs.len(), 1);
        assert_eq!(pairs[0], ("desserts".to_string(), "stressed".to_string()));
    }

    #[test]
    fn test_multiple_anadrome_pairs() {
        let set: BTreeSet<String> = ["stressed", "desserts", "repaid", "diaper", "unrelated"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let pairs = get_anadromes(&set);
        // Should find both pairs, each only once
        assert_eq!(pairs.len(), 2);
        let pair_strs: Vec<_> = pairs
            .iter()
            .map(|(a, b)| (a.as_str(), b.as_str()))
            .collect();
        assert!(pair_strs.contains(&("desserts", "stressed")));
        assert!(pair_strs.contains(&("diaper", "repaid")));
    }

    #[test]
    fn test_palindromes_not_included() {
        // A palindrome reversed equals itself — s < r is false, so excluded
        let set: BTreeSet<String> = ["racecar"].iter().map(|s| s.to_string()).collect();
        assert!(get_anadromes(&set).is_empty());
    }

    // ── build_word_set ────────────────────────────────────────────────────────

    #[test]
    fn test_build_word_set_filters_short_words() {
        // min_len = 6 means words with length <= 6 are dropped
        let words = ["stressed", "desserts", "hi", "ok", "longer"];
        let set = build_word_set(words.iter(), 6);
        assert!(set.contains("stressed"));
        assert!(set.contains("desserts"));
        assert!(!set.contains("hi"));
        assert!(!set.contains("ok"));
        assert!(!set.contains("longer")); // length == 6, not > 6
    }

    #[test]
    fn test_build_word_set_lowercases() {
        let words = ["STRESSED", "Desserts"];
        let set = build_word_set(words.iter(), 6);
        assert!(set.contains("stressed"));
        assert!(set.contains("desserts"));
    }

    #[test]
    fn test_build_word_set_deduplicates() {
        let words = ["stressed", "Stressed", "STRESSED"];
        let set = build_word_set(words.iter(), 6);
        assert_eq!(set.len(), 1);
    }

    // ── end-to-end ────────────────────────────────────────────────────────────

    #[test]
    fn test_end_to_end_pipeline() {
        let raw = vec![
            "Stressed",  // length 8, anadrome of "desserts"
            "Desserts",  // length 8, anadrome of "stressed"
            "Repaid",    // length 6, filtered OUT (not > 6)
            "Diaper",    // length 6, filtered OUT
            "short",     // length 5, filtered
            "unrelated", // length 9, no anadrome
        ];
        let set = build_word_set(raw.iter(), 6);
        let pairs = get_anadromes(&set);
        // Only the stressed/desserts pair survives the length filter
        assert_eq!(pairs.len(), 1);
        assert_eq!(pairs[0], ("desserts".to_string(), "stressed".to_string()));
    }
}
