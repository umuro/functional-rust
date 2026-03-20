#![allow(clippy::all)]
//! # Edit Distance (Levenshtein)

pub fn edit_distance(a: &str, b: &str) -> usize {
    let (m, n) = (a.len(), b.len());
    let a: Vec<char> = a.chars().collect();
    let b: Vec<char> = b.chars().collect();
    let mut dp = vec![vec![0; n + 1]; m + 1];

    for i in 0..=m {
        dp[i][0] = i;
    }
    for j in 0..=n {
        dp[0][j] = j;
    }

    for i in 1..=m {
        for j in 1..=n {
            let cost = if a[i - 1] == b[j - 1] { 0 } else { 1 };
            dp[i][j] = (dp[i - 1][j] + 1)
                .min(dp[i][j - 1] + 1)
                .min(dp[i - 1][j - 1] + cost);
        }
    }
    dp[m][n]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_edit() {
        assert_eq!(edit_distance("kitten", "sitting"), 3);
    }
    #[test]
    fn test_empty() {
        assert_eq!(edit_distance("", "abc"), 3);
    }
    #[test]
    fn test_identical() {
        assert_eq!(edit_distance("abc", "abc"), 0);
    }
    #[test]
    fn test_delete() {
        assert_eq!(edit_distance("abc", "ab"), 1);
    }
}
