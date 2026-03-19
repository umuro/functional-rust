#![allow(clippy::all)]
//! # Isogram Check
//!
//! A word is an isogram if no letter repeats. Compares sort-based,
//! HashSet-based, and bitset approaches.

use std::collections::HashSet;

// ---------------------------------------------------------------------------
// Approach A: Sort + dedup (mirrors OCaml's List.sort_uniq)
// ---------------------------------------------------------------------------

pub fn is_isogram_sort(s: &str) -> bool {
    let mut chars: Vec<char> = s
        .chars()
        .filter_map(|c| {
            let lc = c.to_ascii_lowercase();
            lc.is_ascii_lowercase().then_some(lc)
        })
        .collect();
    let total = chars.len();
    chars.sort_unstable();
    chars.dedup();
    chars.len() == total
}

// ---------------------------------------------------------------------------
// Approach B: HashSet — insert returns false on duplicate
// ---------------------------------------------------------------------------

pub fn is_isogram_hashset(s: &str) -> bool {
    let mut seen = HashSet::new();
    s.chars()
        .filter(|c| c.is_ascii_alphabetic())
        .all(|c| seen.insert(c.to_ascii_lowercase()))
}

// ---------------------------------------------------------------------------
// Approach C: Bitset
// ---------------------------------------------------------------------------

pub fn is_isogram_bitset(s: &str) -> bool {
    let mut bits: u32 = 0;
    for c in s.chars() {
        let lc = c.to_ascii_lowercase();
        if lc.is_ascii_lowercase() {
            let mask = 1 << (lc as u32 - 'a' as u32);
            if bits & mask != 0 {
                return false;
            }
            bits |= mask;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_isogram() {
        assert!(is_isogram_sort("lumberjacks"));
        assert!(is_isogram_hashset("lumberjacks"));
        assert!(is_isogram_bitset("lumberjacks"));
    }

    #[test]
    fn test_not_isogram() {
        assert!(!is_isogram_sort("eleven"));
        assert!(!is_isogram_hashset("eleven"));
        assert!(!is_isogram_bitset("eleven"));
    }

    #[test]
    fn test_long_isogram() {
        assert!(is_isogram_bitset("subdermatoglyphic"));
    }

    #[test]
    fn test_empty() {
        assert!(is_isogram_bitset(""));
    }

    #[test]
    fn test_with_spaces() {
        assert!(is_isogram_hashset("big dwarf"));
    }
}
