/// Hamming Distance
///
/// Count positional differences between two equal-length strings.
/// A clean example of zip + filter + count pattern in both languages.

/// Using zip + filter + count — idiomatic Rust.
pub fn hamming_distance(s1: &str, s2: &str) -> Result<usize, String> {
    if s1.len() != s2.len() {
        return Err("strands must be of equal length".to_string());
    }
    Ok(s1.chars().zip(s2.chars()).filter(|(a, b)| a != b).count())
}

/// Using fold — mirrors OCaml's fold_left2 approach.
pub fn hamming_fold(s1: &str, s2: &str) -> Result<usize, String> {
    if s1.len() != s2.len() {
        return Err("strands must be of equal length".to_string());
    }
    Ok(s1
        .chars()
        .zip(s2.chars())
        .fold(0, |acc, (a, b)| if a != b { acc + 1 } else { acc }))
}

/// Byte-level comparison — faster for ASCII strings.
pub fn hamming_bytes(s1: &[u8], s2: &[u8]) -> Result<usize, String> {
    if s1.len() != s2.len() {
        return Err("strands must be of equal length".to_string());
    }
    Ok(s1.iter().zip(s2.iter()).filter(|(a, b)| a != b).count())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hamming_basic() {
        assert_eq!(
            hamming_distance("GAGCCTACTAACGGGAT", "CATCGTAATGACGGCCT"),
            Ok(7)
        );
    }

    #[test]
    fn test_identical() {
        assert_eq!(hamming_distance("AAAA", "AAAA"), Ok(0));
    }

    #[test]
    fn test_completely_different() {
        assert_eq!(hamming_distance("AAAA", "TTTT"), Ok(4));
    }

    #[test]
    fn test_unequal_length() {
        assert!(hamming_distance("AA", "AAA").is_err());
    }

    #[test]
    fn test_empty() {
        assert_eq!(hamming_distance("", ""), Ok(0));
    }

    #[test]
    fn test_fold_variant() {
        assert_eq!(hamming_fold("GAGCCTACTAACGGGAT", "CATCGTAATGACGGCCT"), Ok(7));
    }

    #[test]
    fn test_single_char() {
        assert_eq!(hamming_distance("A", "G"), Ok(1));
        assert_eq!(hamming_distance("A", "A"), Ok(0));
    }
}
