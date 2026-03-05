//! # String Search — Find and Match

pub fn find_first(haystack: &str, needle: &str) -> Option<usize> {
    haystack.find(needle)
}

pub fn find_last(haystack: &str, needle: &str) -> Option<usize> {
    haystack.rfind(needle)
}

pub fn find_all(haystack: &str, needle: &str) -> Vec<usize> {
    haystack.match_indices(needle).map(|(i, _)| i).collect()
}

pub fn contains_any(s: &str, chars: &[char]) -> bool {
    s.chars().any(|c| chars.contains(&c))
}

pub fn count_occurrences(haystack: &str, needle: &str) -> usize {
    haystack.matches(needle).count()
}

pub fn find_char(s: &str, c: char) -> Option<usize> {
    s.find(c)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find() {
        assert_eq!(find_first("hello world", "o"), Some(4));
        assert_eq!(find_last("hello world", "o"), Some(7));
    }

    #[test]
    fn test_find_all() {
        assert_eq!(find_all("ababa", "a"), vec![0, 2, 4]);
    }

    #[test]
    fn test_contains_any() {
        assert!(contains_any("hello", &['a', 'e', 'i']));
        assert!(!contains_any("xyz", &['a', 'e', 'i']));
    }

    #[test]
    fn test_count() {
        assert_eq!(count_occurrences("banana", "an"), 2);
    }
}
