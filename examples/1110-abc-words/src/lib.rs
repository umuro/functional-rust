//! ABC Words — find words containing 'a', 'b', 'c' in order as a subsequence.
//!
//! A word is an "ABC word" if it contains the letters a, b, c as an ordered
//! subsequence (not necessarily consecutive).

/// Idiomatic Rust: stateful iterator — the iterator cursor advances past each
/// matched letter, so each subsequent `.any()` searches only the remaining suffix.
///
/// Short-circuit `&&` means we skip later searches if an earlier one fails.
pub fn is_abc_word(word: &str) -> bool {
    let mut chars = word.chars();
    chars.any(|c| c == 'a') && chars.any(|c| c == 'b') && chars.any(|c| c == 'c')
}

/// Fold-based: accumulate progress through the target sequence `['a','b','c']`
/// without mutation.  Returns true when all three letters are found in order.
pub fn is_abc_word_fold(word: &str) -> bool {
    const TARGET: [char; 3] = ['a', 'b', 'c'];
    word.chars().fold(0usize, |idx, ch| {
        if idx < TARGET.len() && ch == TARGET[idx] {
            idx + 1
        } else {
            idx
        }
    }) == TARGET.len()
}

/// Recursive: mirrors the OCaml pattern-match style explicitly.
pub fn is_abc_word_recursive(word: &str) -> bool {
    fn find_seq(chars: &[char], seq: &[char]) -> bool {
        match (chars, seq) {
            (_, []) => true,
            ([], _) => false,
            ([ch, rest @ ..], [target, remaining @ ..]) if ch == target => {
                find_seq(rest, remaining)
            }
            ([_, rest @ ..], _) => find_seq(rest, seq),
        }
    }

    let chars: Vec<char> = word.chars().collect();
    find_seq(&chars, &['a', 'b', 'c'])
}

/// Filter a slice of words, returning those that are ABC words.
pub fn filter_abc_words<'a>(words: &[&'a str]) -> Vec<&'a str> {
    words.iter().copied().filter(|w| is_abc_word(w)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── is_abc_word ──────────────────────────────────────────────────────────

    #[test]
    fn test_empty_word() {
        assert!(!is_abc_word(""));
    }

    #[test]
    fn test_exact_abc() {
        assert!(is_abc_word("abc"));
    }

    #[test]
    fn test_abc_subsequence_interleaved() {
        // 'a' at 0, 'b' at 2, 'c' at 5
        assert!(is_abc_word("abacus"));
    }

    #[test]
    fn test_abc_not_in_order_cab() {
        assert!(!is_abc_word("cab"));
    }

    #[test]
    fn test_missing_c() {
        assert!(!is_abc_word("abba"));
    }

    #[test]
    fn test_missing_a() {
        assert!(!is_abc_word("bc"));
    }

    #[test]
    fn test_multiple_abc_patterns() {
        // contains "abc" twice — still true
        assert!(is_abc_word("abcabc"));
    }

    #[test]
    fn test_abc_at_end() {
        assert!(is_abc_word("xyzabc"));
    }

    #[test]
    fn test_long_word_with_subsequence() {
        // "aberrance" — a(0) b(1) e r r a n c(7) e — 'c' is present after 'b'. True.
        assert!(is_abc_word("aberrance"));
        // "abstracted" — a(0) b(1) s t r a c(6) t e d — 'c' at 6. True.
        assert!(is_abc_word("abstracted"));
        // "abc" backwards: only 'c','b','a' — no forward subsequence
        assert!(!is_abc_word("zbc"));
    }

    // ── is_abc_word_fold ─────────────────────────────────────────────────────

    #[test]
    fn test_fold_empty() {
        assert!(!is_abc_word_fold(""));
    }

    #[test]
    fn test_fold_exact() {
        assert!(is_abc_word_fold("abc"));
    }

    #[test]
    fn test_fold_interleaved() {
        assert!(is_abc_word_fold("abacus"));
    }

    #[test]
    fn test_fold_no_match() {
        assert!(!is_abc_word_fold("cab"));
    }

    // ── is_abc_word_recursive ────────────────────────────────────────────────

    #[test]
    fn test_recursive_empty() {
        assert!(!is_abc_word_recursive(""));
    }

    #[test]
    fn test_recursive_exact() {
        assert!(is_abc_word_recursive("abc"));
    }

    #[test]
    fn test_recursive_interleaved() {
        assert!(is_abc_word_recursive("abacus"));
    }

    #[test]
    fn test_recursive_no_match() {
        assert!(!is_abc_word_recursive("cab"));
    }

    // ── filter_abc_words ─────────────────────────────────────────────────────

    #[test]
    fn test_filter_empty_list() {
        assert_eq!(filter_abc_words(&[]), Vec::<&str>::new());
    }

    #[test]
    fn test_filter_no_matches() {
        assert_eq!(filter_abc_words(&["cab", "bca", "xyz"]), Vec::<&str>::new());
    }

    #[test]
    fn test_filter_some_matches() {
        let words = ["abc", "cab", "abacus", "dog", "abstracted"];
        let result = filter_abc_words(&words);
        assert_eq!(result, vec!["abc", "abacus", "abstracted"]);
    }

    #[test]
    fn test_filter_all_match() {
        let words = ["abc", "xyzabc", "abcabc"];
        let result = filter_abc_words(&words);
        assert_eq!(result, vec!["abc", "xyzabc", "abcabc"]);
    }

    // ── all three implementations agree ──────────────────────────────────────

    #[test]
    fn test_all_implementations_agree() {
        let cases = [
            ("abc", true),
            ("", false),
            ("cab", false),
            ("abacus", true),
            ("abstracted", true),
            ("abba", false),
            ("xyzabc", true),
            ("aababc", true),
        ];
        for (word, expected) in cases {
            assert_eq!(
                is_abc_word(word),
                expected,
                "is_abc_word({word:?}) should be {expected}"
            );
            assert_eq!(
                is_abc_word_fold(word),
                expected,
                "is_abc_word_fold({word:?}) should be {expected}"
            );
            assert_eq!(
                is_abc_word_recursive(word),
                expected,
                "is_abc_word_recursive({word:?}) should be {expected}"
            );
        }
    }
}
