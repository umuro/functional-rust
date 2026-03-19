//! # Word Break

use std::collections::HashSet;

pub fn word_break(s: &str, dict: &[&str]) -> bool {
    let words: HashSet<&str> = dict.iter().copied().collect();
    let n = s.len();
    let mut dp = vec![false; n + 1];
    dp[0] = true;
    for i in 1..=n {
        for j in 0..i {
            if dp[j] && words.contains(&s[j..i]) {
                dp[i] = true;
                break;
            }
        }
    }
    dp[n]
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_word_break() {
        assert!(word_break("leetcode", &["leet", "code"]));
    }
    #[test]
    fn test_no_break() {
        assert!(!word_break(
            "catsandog",
            &["cats", "dog", "sand", "and", "cat"]
        ));
    }
}
