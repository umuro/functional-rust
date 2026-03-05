//! # String Algorithms Overview

/// Naive string search O(nm)
pub fn naive_search(text: &str, pattern: &str) -> Vec<usize> {
    let (t, p) = (text.as_bytes(), pattern.as_bytes());
    if p.is_empty() || p.len() > t.len() { return vec![]; }
    (0..=t.len() - p.len()).filter(|&i| t[i..i + p.len()] == *p).collect()
}

/// Check if string is palindrome
pub fn is_palindrome(s: &str) -> bool {
    let chars: Vec<_> = s.chars().collect();
    chars.iter().eq(chars.iter().rev())
}

/// Longest common prefix
pub fn lcp(a: &str, b: &str) -> usize {
    a.chars().zip(b.chars()).take_while(|(x, y)| x == y).count()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_naive() { assert_eq!(naive_search("abcabc", "abc"), vec![0, 3]); }
    #[test]
    fn test_palindrome() { assert!(is_palindrome("racecar")); }
    #[test]
    fn test_lcp() { assert_eq!(lcp("prefix", "preach"), 3); }
}
