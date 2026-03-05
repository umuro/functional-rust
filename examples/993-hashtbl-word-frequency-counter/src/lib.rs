use std::collections::HashMap;

/// Idiomatic Rust: count word frequencies using HashMap and iterator combinators.
/// Words are lowercased and split on whitespace.
pub fn count_words(text: &str) -> HashMap<String, usize> {
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let w = word.to_lowercase();
        *map.entry(w).or_insert(0) += 1;
    }
    map
}

/// Functional style: fold over words to build the frequency map.
pub fn count_words_fold(text: &str) -> HashMap<String, usize> {
    text.split_whitespace()
        .map(|w| w.to_lowercase())
        .fold(HashMap::new(), |mut acc, w| {
            *acc.entry(w).or_insert(0) += 1;
            acc
        })
}

/// Return words sorted by frequency (descending), then alphabetically for ties.
pub fn top_words(freq: &HashMap<String, usize>) -> Vec<(&str, usize)> {
    let mut pairs: Vec<(&str, usize)> = freq.iter().map(|(k, &v)| (k.as_str(), v)).collect();
    pairs.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(b.0)));
    pairs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_string() {
        let freq = count_words("");
        assert!(freq.is_empty());
    }

    #[test]
    fn test_single_word() {
        let freq = count_words("hello");
        assert_eq!(freq["hello"], 1);
        assert_eq!(freq.len(), 1);
    }

    #[test]
    fn test_repeated_words() {
        let freq = count_words("the cat sat on the mat the cat");
        assert_eq!(freq["the"], 3);
        assert_eq!(freq["cat"], 2);
        assert_eq!(freq["sat"], 1);
        assert_eq!(freq["on"], 1);
        assert_eq!(freq["mat"], 1);
    }

    #[test]
    fn test_case_insensitive() {
        let freq = count_words("Rust rust RUST");
        assert_eq!(freq["rust"], 3);
        assert_eq!(freq.len(), 1);
    }

    #[test]
    fn test_fold_matches_idiomatic() {
        let text = "the cat sat on the mat the cat";
        let a = count_words(text);
        let b = count_words_fold(text);
        assert_eq!(a, b);
    }

    #[test]
    fn test_top_words_ordering() {
        let freq = count_words("the cat sat on the mat the cat");
        let top = top_words(&freq);
        // "the" appears 3 times — must be first
        assert_eq!(top[0], ("the", 3));
        // "cat" appears 2 times — must be second
        assert_eq!(top[1], ("cat", 2));
    }

    #[test]
    fn test_multiple_whitespace() {
        // split_whitespace handles tabs, multiple spaces, newlines
        let freq = count_words("hello   world\nhello\there");
        assert_eq!(freq["hello"], 2);
        assert_eq!(freq["world"], 1);
        assert_eq!(freq["here"], 1);
    }
}
