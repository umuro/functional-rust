//! # String Operations — Common String Methods

pub fn concatenate(a: &str, b: &str) -> String {
    format!("{}{}", a, b)
}

pub fn repeat_string(s: &str, n: usize) -> String {
    s.repeat(n)
}

pub fn reverse_string(s: &str) -> String {
    s.chars().rev().collect()
}

pub fn is_empty_or_whitespace(s: &str) -> bool {
    s.trim().is_empty()
}

pub fn char_count(s: &str) -> usize {
    s.chars().count()
}

pub fn byte_count(s: &str) -> usize {
    s.len()
}

pub fn starts_ends_with(s: &str, prefix: &str, suffix: &str) -> (bool, bool) {
    (s.starts_with(prefix), s.ends_with(suffix))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_concat() {
        assert_eq!(concatenate("hello", " world"), "hello world");
    }

    #[test]
    fn test_repeat() {
        assert_eq!(repeat_string("ab", 3), "ababab");
    }

    #[test]
    fn test_reverse() {
        assert_eq!(reverse_string("hello"), "olleh");
    }

    #[test]
    fn test_empty_whitespace() {
        assert!(is_empty_or_whitespace("   "));
        assert!(!is_empty_or_whitespace(" a "));
    }

    #[test]
    fn test_counts() {
        let s = "héllo";
        assert_eq!(char_count(s), 5);
        assert_eq!(byte_count(s), 6); // é is 2 bytes
    }
}
