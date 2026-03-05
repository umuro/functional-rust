/// Split a word into two alternade sub-words by interleaving even/odd indices.
///
/// For a word of length n:
/// - even alternade: characters at positions 0, 2, 4, ... (length = ceil(n/2))
/// - odd alternade:  characters at positions 1, 3, 5, ... (length = floor(n/2))
pub fn split_alternade(word: &str) -> (String, String) {
    let even: String = word.chars().step_by(2).collect();
    let odd: String = word.chars().skip(1).step_by(2).collect();
    (even, odd)
}

/// Find all alternade words in a word list.
///
/// A word qualifies when:
/// 1. Its length is >= 6 (so both alternades are >= 3 chars)
/// 2. Both alternades produced by `split_alternade` are present in the word set
///
/// The word set is built from words of length >= 3.
/// Returns formatted strings: `"word | even_part odd_part"`.
pub fn find_alternades(words: &[&str]) -> Vec<String> {
    use std::collections::HashSet;

    let word_set: HashSet<&str> = words.iter().copied().filter(|w| w.len() >= 3).collect();

    let mut results: Vec<String> = words
        .iter()
        .copied()
        .filter(|w| w.len() >= 6)
        .filter_map(|word| {
            let (even, odd) = split_alternade(word);
            if word_set.contains(even.as_str()) && word_set.contains(odd.as_str()) {
                Some(format!("{word} | {even} {odd}"))
            } else {
                None
            }
        })
        .collect();

    results.sort();
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_alternade_even_length() {
        let (even, odd) = split_alternade("abcdef");
        assert_eq!(even, "ace");
        assert_eq!(odd, "bdf");
    }

    #[test]
    fn test_split_alternade_odd_length() {
        // "abcde": a,b,c,d,e -> even = "ace" (len 3), odd = "bd" (len 2)
        let (even, odd) = split_alternade("abcde");
        assert_eq!(even, "ace");
        assert_eq!(odd, "bd");
    }

    #[test]
    fn test_split_alternade_minimal() {
        let (even, odd) = split_alternade("ab");
        assert_eq!(even, "a");
        assert_eq!(odd, "b");
    }

    #[test]
    fn test_split_alternade_single() {
        let (even, odd) = split_alternade("a");
        assert_eq!(even, "a");
        assert_eq!(odd, "");
    }

    #[test]
    fn test_find_alternades_basic_no_match() {
        // "aaabbb": a,a,a,b,b,b -> even="aab", odd="abb" — neither in set
        let words = vec!["aaabbb", "aaa", "bbb"];
        let results = find_alternades(&words);
        assert!(results.is_empty());
    }

    #[test]
    fn test_find_alternades_single_match() {
        // "abcdef": even="ace", odd="bdf"
        let words = vec!["abcdef", "ace", "bdf", "unrelated"];
        let results = find_alternades(&words);
        assert_eq!(results, vec!["abcdef | ace bdf"]);
    }

    #[test]
    fn test_find_alternades_multiple() {
        // "abcdef": even="ace", odd="bdf"
        // "123456": even="135", odd="246"
        let words = vec!["abcdef", "ace", "bdf", "123456", "135", "246", "other"];
        let results = find_alternades(&words);
        assert_eq!(results.len(), 2);
        assert!(results.contains(&"abcdef | ace bdf".to_string()));
        assert!(results.contains(&"123456 | 135 246".to_string()));
    }

    #[test]
    fn test_find_alternades_length_filter() {
        // Words shorter than 6 are not candidates.
        // "abcde" (len 5): even="ace", odd="bd" (len 2, won't be in word_set)
        let words = vec!["abcde", "ace", "bd"];
        let results = find_alternades(&words);
        assert!(results.is_empty());
    }

    #[test]
    fn test_find_alternades_empty_input() {
        let results = find_alternades(&[]);
        assert!(results.is_empty());
    }

    #[test]
    fn test_find_alternades_partial_match() {
        // Only one of the two alternades is in the word list — should not match.
        let words = vec!["abcdef", "ace"]; // "bdf" is missing
        let results = find_alternades(&words);
        assert!(results.is_empty());
    }
}
