//! # Pangram Check
//!
//! Determine if a string contains every letter of the alphabet.
//! Compares OCaml's `Set.Make(Char)` with Rust's `HashSet` and bitset approaches.

use std::collections::HashSet;

// ---------------------------------------------------------------------------
// Approach A: HashSet — mirrors OCaml's Set approach
// ---------------------------------------------------------------------------

pub fn is_pangram_hashset(s: &str) -> bool {
    let chars: HashSet<char> = s
        .chars()
        .filter_map(|c| {
            let lc = c.to_ascii_lowercase();
            if lc.is_ascii_lowercase() { Some(lc) } else { None }
        })
        .collect();
    chars.len() == 26
}

// ---------------------------------------------------------------------------
// Approach B: Bitset — 26 bits for 26 letters
// ---------------------------------------------------------------------------

pub fn is_pangram_bitset(s: &str) -> bool {
    let mut bits: u32 = 0;
    for c in s.chars() {
        let lc = c.to_ascii_lowercase();
        if lc.is_ascii_lowercase() {
            bits |= 1 << (lc as u32 - 'a' as u32);
        }
    }
    bits == (1 << 26) - 1
}

// ---------------------------------------------------------------------------
// Approach C: Functional — all() check
// ---------------------------------------------------------------------------

pub fn is_pangram_all(s: &str) -> bool {
    let lower = s.to_ascii_lowercase();
    ('a'..='z').all(|c| lower.contains(c))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pangram() {
        let s = "The quick brown fox jumps over the lazy dog";
        assert!(is_pangram_hashset(s));
        assert!(is_pangram_bitset(s));
        assert!(is_pangram_all(s));
    }

    #[test]
    fn test_not_pangram() {
        assert!(!is_pangram_hashset("Hello world"));
        assert!(!is_pangram_bitset("Hello world"));
        assert!(!is_pangram_all("Hello world"));
    }

    #[test]
    fn test_empty() {
        assert!(!is_pangram_hashset(""));
        assert!(!is_pangram_bitset(""));
    }

    #[test]
    fn test_with_numbers() {
        let s = "The 1 quick brown fox jumps over the 2 lazy dogs";
        assert!(is_pangram_hashset(s));
        assert!(is_pangram_bitset(s));
    }

    #[test]
    fn test_missing_one() {
        assert!(!is_pangram_bitset("The quick brown fox jumps over the lazy do"));
    }
}

fn main() {
    println!("{:?}", is_pangram_hashset(s));
    println!("{:?}", is_pangram_bitset(s));
    println!("{:?}", is_pangram_all(s));
}
