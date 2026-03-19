/// # Isogram Check
///
/// An isogram is a word with no repeating letters (ignoring case, hyphens, spaces).
/// Demonstrates set-based duplicate detection.
use std::collections::HashSet;

/// Idiomatic Rust: filter to alphabetic, lowercase, check uniqueness via set size.
pub fn is_isogram(word: &str) -> bool {
    let letters: Vec<char> = word
        .chars()
        .filter(|c| c.is_ascii_alphabetic())
        .map(|c| c.to_ascii_lowercase())
        .collect();
    let unique: HashSet<char> = letters.iter().copied().collect();
    letters.len() == unique.len()
}

/// Early-exit approach: insert into set, return false on first duplicate.
/// More efficient for long strings with early duplicates.
pub fn is_isogram_early_exit(word: &str) -> bool {
    let mut seen = HashSet::new();
    for c in word.chars() {
        if c.is_ascii_alphabetic() {
            if !seen.insert(c.to_ascii_lowercase()) {
                return false; // insert returns false if already present
            }
        }
    }
    true
}

/// Bitflag approach — same idea as pangram but checking for collisions.
pub fn is_isogram_bitflag(word: &str) -> bool {
    let mut seen: u32 = 0;
    for c in word.chars() {
        if c.is_ascii_alphabetic() {
            let bit = 1 << (c.to_ascii_lowercase() as u32 - 'a' as u32);
            if seen & bit != 0 {
                return false;
            }
            seen |= bit;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_isogram() {
        assert!(is_isogram("lumberjacks"));
        assert!(is_isogram("subdermatoglyphic"));
    }

    #[test]
    fn test_not_isogram() {
        assert!(!is_isogram("eleven"));
        assert!(!is_isogram("balloon")); // 'l' repeats
    }

    #[test]
    fn test_empty() {
        assert!(is_isogram(""));
    }

    #[test]
    fn test_hyphenated() {
        assert!(is_isogram("thumbscrew-japing"));
    }

    #[test]
    fn test_case_insensitive() {
        assert!(!is_isogram("Alphabet")); // 'a' appears twice
    }

    #[test]
    fn test_early_exit() {
        assert!(is_isogram_early_exit("lumberjacks"));
        assert!(!is_isogram_early_exit("eleven"));
    }

    #[test]
    fn test_bitflag() {
        assert!(is_isogram_bitflag("lumberjacks"));
        assert!(!is_isogram_bitflag("eleven"));
    }
}
