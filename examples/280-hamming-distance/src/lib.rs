#![allow(clippy::all)]
/// Compute the Hamming distance between two strings of equal length.
///
/// The Hamming distance is the number of positions where the corresponding
/// characters differ. Returns an error if the strings have different lengths.
///
/// # Idiomatic Rust — zip + filter + count
pub fn hamming(s1: &str, s2: &str) -> Result<usize, &'static str> {
    if s1.len() != s2.len() {
        return Err("strands must be of equal length");
    }
    Ok(s1.chars().zip(s2.chars()).filter(|(a, b)| a != b).count())
}

/// Imperative version — closer to OCaml's ref-based approach.
pub fn hamming_imperative(s1: &str, s2: &str) -> Result<usize, &'static str> {
    if s1.len() != s2.len() {
        return Err("strands must be of equal length");
    }
    let mut dist = 0;
    for (a, b) in s1.chars().zip(s2.chars()) {
        if a != b {
            dist += 1;
        }
    }
    Ok(dist)
}

/// Fold-based version — functional accumulator pattern.
pub fn hamming_fold(s1: &str, s2: &str) -> Result<usize, &'static str> {
    if s1.len() != s2.len() {
        return Err("strands must be of equal length");
    }
    Ok(s1
        .chars()
        .zip(s2.chars())
        .fold(0, |acc, (a, b)| if a != b { acc + 1 } else { acc }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_strands() {
        assert_eq!(hamming("", ""), Ok(0));
    }

    #[test]
    fn test_identical_strands() {
        assert_eq!(hamming("GGACTGA", "GGACTGA"), Ok(0));
    }

    #[test]
    fn test_single_difference() {
        assert_eq!(hamming("GGACTGA", "GGACTGT"), Ok(1));
    }

    #[test]
    fn test_long_strands() {
        assert_eq!(hamming("GAGCCTACTAACGGGAT", "CATCGTAATGACGGCCT"), Ok(7));
    }

    #[test]
    fn test_unequal_length() {
        assert_eq!(hamming("AB", "ABC"), Err("strands must be of equal length"));
    }

    #[test]
    fn test_all_different() {
        assert_eq!(hamming("ABCD", "EFGH"), Ok(4));
    }

    #[test]
    fn test_imperative_matches() {
        assert_eq!(
            hamming("GAGCCTACTAACGGGAT", "CATCGTAATGACGGCCT"),
            hamming_imperative("GAGCCTACTAACGGGAT", "CATCGTAATGACGGCCT")
        );
    }

    #[test]
    fn test_fold_matches() {
        assert_eq!(
            hamming("GAGCCTACTAACGGGAT", "CATCGTAATGACGGCCT"),
            hamming_fold("GAGCCTACTAACGGGAT", "CATCGTAATGACGGCCT")
        );
    }
}
