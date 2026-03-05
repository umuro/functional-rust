// 1055: Longest Common Subsequence — 2D DP + Backtrack

use std::collections::HashMap;

// Approach 1: 2D DP table for length
fn lcs_length(s1: &str, s2: &str) -> usize {
    let (a, b): (Vec<char>, Vec<char>) = (s1.chars().collect(), s2.chars().collect());
    let (m, n) = (a.len(), b.len());
    let mut dp = vec![vec![0usize; n + 1]; m + 1];
    for i in 1..=m {
        for j in 1..=n {
            dp[i][j] = if a[i - 1] == b[j - 1] {
                dp[i - 1][j - 1] + 1
            } else {
                dp[i - 1][j].max(dp[i][j - 1])
            };
        }
    }
    dp[m][n]
}

// Approach 2: DP + backtrack to reconstruct
fn lcs_string(s1: &str, s2: &str) -> String {
    let (a, b): (Vec<char>, Vec<char>) = (s1.chars().collect(), s2.chars().collect());
    let (m, n) = (a.len(), b.len());
    let mut dp = vec![vec![0usize; n + 1]; m + 1];
    for i in 1..=m {
        for j in 1..=n {
            dp[i][j] = if a[i - 1] == b[j - 1] {
                dp[i - 1][j - 1] + 1
            } else {
                dp[i - 1][j].max(dp[i][j - 1])
            };
        }
    }
    // Backtrack
    let mut result = Vec::new();
    let (mut i, mut j) = (m, n);
    while i > 0 && j > 0 {
        if a[i - 1] == b[j - 1] {
            result.push(a[i - 1]);
            i -= 1;
            j -= 1;
        } else if dp[i - 1][j] > dp[i][j - 1] {
            i -= 1;
        } else {
            j -= 1;
        }
    }
    result.reverse();
    result.into_iter().collect()
}

// Approach 3: Recursive with memoization
fn lcs_memo(s1: &str, s2: &str) -> usize {
    let (a, b): (Vec<char>, Vec<char>) = (s1.chars().collect(), s2.chars().collect());
    fn solve(i: usize, j: usize, a: &[char], b: &[char], cache: &mut HashMap<(usize, usize), usize>) -> usize {
        if i == 0 || j == 0 { return 0; }
        if let Some(&v) = cache.get(&(i, j)) { return v; }
        let v = if a[i - 1] == b[j - 1] {
            solve(i - 1, j - 1, a, b, cache) + 1
        } else {
            solve(i - 1, j, a, b, cache).max(solve(i, j - 1, a, b, cache))
        };
        cache.insert((i, j), v);
        v
    }
    let mut cache = HashMap::new();
    solve(a.len(), b.len(), &a, &b, &mut cache)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lcs_length() {
        assert_eq!(lcs_length("abcde", "ace"), 3);
        assert_eq!(lcs_length("abc", "abc"), 3);
        assert_eq!(lcs_length("abc", "def"), 0);
    }

    #[test]
    fn test_lcs_string() {
        assert_eq!(lcs_string("abcde", "ace"), "ace");
        assert_eq!(lcs_string("AGGTAB", "GXTXAYB"), "GTAB");
    }

    #[test]
    fn test_lcs_memo() {
        assert_eq!(lcs_memo("abcde", "ace"), 3);
        assert_eq!(lcs_memo("AGGTAB", "GXTXAYB"), 4);
    }
}
