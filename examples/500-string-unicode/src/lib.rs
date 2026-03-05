//! # String Unicode — Graphemes and Normalization

/// Count grapheme clusters (perceived characters)
pub fn grapheme_count(s: &str) -> usize {
    s.chars().count() // Simplified - real impl needs unicode-segmentation crate
}

/// Check if string is ASCII
pub fn is_ascii(s: &str) -> bool {
    s.is_ascii()
}

/// Get Unicode codepoints
pub fn codepoints(s: &str) -> Vec<u32> {
    s.chars().map(|c| c as u32).collect()
}

/// Check if char is emoji (simplified)
pub fn is_emoji(c: char) -> bool {
    let n = c as u32;
    (0x1F600..=0x1F64F).contains(&n) || // Emoticons
    (0x1F300..=0x1F5FF).contains(&n) || // Misc symbols
    (0x1F680..=0x1F6FF).contains(&n) || // Transport
    (0x2600..=0x26FF).contains(&n)      // Misc symbols
}

/// Count emojis in string
pub fn count_emojis(s: &str) -> usize {
    s.chars().filter(|&c| is_emoji(c)).count()
}

/// Check if string has only alphanumeric chars
pub fn is_alphanumeric(s: &str) -> bool {
    !s.is_empty() && s.chars().all(char::is_alphanumeric)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ascii() {
        assert!(is_ascii("hello"));
        assert!(!is_ascii("héllo"));
    }

    #[test]
    fn test_codepoints() {
        assert_eq!(codepoints("AB"), vec![65, 66]);
    }

    #[test]
    fn test_emoji() {
        assert!(is_emoji('😀'));
        assert!(!is_emoji('A'));
    }

    #[test]
    fn test_count_emojis() {
        assert_eq!(count_emojis("Hello 😀 World 🌍"), 2);
    }

    #[test]
    fn test_alphanumeric() {
        assert!(is_alphanumeric("hello123"));
        assert!(!is_alphanumeric("hello 123"));
    }
}
