/// # Pangram Check
///
/// A pangram is a sentence using every letter of the alphabet at least once.
/// Demonstrates set-based string analysis.

use std::collections::HashSet;

/// Idiomatic Rust using HashSet and iterator chains.
/// The `collect()` into HashSet automatically deduplicates.
pub fn is_pangram(sentence: &str) -> bool {
    // Filter to only alphabetic chars, lowercase them, collect unique into set
    let unique_letters: HashSet<char> = sentence
        .chars()
        .filter(|c| c.is_ascii_alphabetic())
        .map(|c| c.to_ascii_lowercase())
        .collect();
    unique_letters.len() == 26
}

/// Bitflag approach — uses a u32 as a compact set of 26 bits.
/// Each bit represents a letter: bit 0 = 'a', bit 1 = 'b', etc.
/// No heap allocation needed!
pub fn is_pangram_bitflag(sentence: &str) -> bool {
    let mut seen: u32 = 0;
    for c in sentence.chars() {
        if c.is_ascii_alphabetic() {
            let idx = c.to_ascii_lowercase() as u32 - 'a' as u32;
            seen |= 1 << idx;
        }
    }
    seen == (1 << 26) - 1
}

/// Recursive approach — checks if each letter 'a'..'z' exists in the string.
pub fn is_pangram_recursive(sentence: &str) -> bool {
    fn has_all(sentence: &str, letter: u8) -> bool {
        if letter > b'z' {
            return true;
        }
        let lower = sentence.to_ascii_lowercase();
        lower.contains(letter as char) && has_all(sentence, letter + 1)
    }
    has_all(sentence, b'a')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_classic_pangram() {
        assert!(is_pangram("The quick brown fox jumps over the lazy dog"));
    }

    #[test]
    fn test_not_pangram() {
        assert!(!is_pangram("Hello world"));
    }

    #[test]
    fn test_empty_string() {
        assert!(!is_pangram(""));
    }

    #[test]
    fn test_missing_x() {
        assert!(!is_pangram("The quick brown fo jumps over the lazy dog"));
    }

    #[test]
    fn test_mixed_case() {
        assert!(is_pangram("THE QUICK BROWN FOX JUMPS OVER THE LAZY DOG"));
    }

    #[test]
    fn test_with_numbers_and_punctuation() {
        assert!(is_pangram("The 1 quick brown fox jumps! over the 2 lazy dogs."));
    }

    #[test]
    fn test_bitflag_version() {
        assert!(is_pangram_bitflag("The quick brown fox jumps over the lazy dog"));
        assert!(!is_pangram_bitflag("Hello world"));
    }

    #[test]
    fn test_recursive_version() {
        assert!(is_pangram_recursive("The quick brown fox jumps over the lazy dog"));
        assert!(!is_pangram_recursive("abc"));
    }
}
