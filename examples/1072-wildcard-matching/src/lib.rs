// 1072: Wildcard Matching — '?' and '*' — DP

use std::collections::HashMap;

// Approach 1: Bottom-up DP
fn is_match_dp(s: &str, p: &str) -> bool {
    let s: Vec<char> = s.chars().collect();
    let p: Vec<char> = p.chars().collect();
    let (m, n) = (s.len(), p.len());
    let mut dp = vec![vec![false; n + 1]; m + 1];
    dp[0][0] = true;
    for j in 1..=n {
        if p[j - 1] == '*' {
            dp[0][j] = dp[0][j - 1];
        }
    }
    for i in 1..=m {
        for j in 1..=n {
            if p[j - 1] == '*' {
                dp[i][j] = dp[i - 1][j] || dp[i][j - 1];
            } else if p[j - 1] == '?' || p[j - 1] == s[i - 1] {
                dp[i][j] = dp[i - 1][j - 1];
            }
        }
    }
    dp[m][n]
}

// Approach 2: Recursive with memoization
fn is_match_memo(s: &str, p: &str) -> bool {
    let s: Vec<char> = s.chars().collect();
    let p: Vec<char> = p.chars().collect();

    fn solve(
        i: usize,
        j: usize,
        s: &[char],
        p: &[char],
        cache: &mut HashMap<(usize, usize), bool>,
    ) -> bool {
        if j == p.len() {
            return i == s.len();
        }
        if i == s.len() {
            return p[j..].iter().all(|&c| c == '*');
        }
        if let Some(&v) = cache.get(&(i, j)) {
            return v;
        }
        let v = if p[j] == '*' {
            solve(i, j + 1, s, p, cache) || solve(i + 1, j, s, p, cache)
        } else if p[j] == '?' || p[j] == s[i] {
            solve(i + 1, j + 1, s, p, cache)
        } else {
            false
        };
        cache.insert((i, j), v);
        v
    }

    let mut cache = HashMap::new();
    solve(0, 0, &s, &p, &mut cache)
}

// Approach 3: Two-pointer greedy (O(m*n) worst case, O(m+n) average)
fn is_match_greedy(s: &str, p: &str) -> bool {
    let s: Vec<char> = s.chars().collect();
    let p: Vec<char> = p.chars().collect();
    let (m, n) = (s.len(), p.len());
    let (mut si, mut pi) = (0usize, 0usize);
    let mut star_idx: Option<usize> = None;
    let mut match_idx = 0usize;

    while si < m {
        if pi < n && (p[pi] == '?' || p[pi] == s[si]) {
            si += 1;
            pi += 1;
        } else if pi < n && p[pi] == '*' {
            star_idx = Some(pi);
            match_idx = si;
            pi += 1;
        } else if let Some(star) = star_idx {
            pi = star + 1;
            match_idx += 1;
            si = match_idx;
        } else {
            return false;
        }
    }

    while pi < n && p[pi] == '*' {
        pi += 1;
    }
    pi == n
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dp() {
        assert!(is_match_dp("adceb", "*a*b"));
        assert!(!is_match_dp("acdcb", "a*c?b"));
        assert!(is_match_dp("", "*"));
        assert!(!is_match_dp("cb", "?a"));
        assert!(is_match_dp("aa", "*"));
    }

    #[test]
    fn test_memo() {
        assert!(is_match_memo("adceb", "*a*b"));
        assert!(!is_match_memo("acdcb", "a*c?b"));
    }

    #[test]
    fn test_greedy() {
        assert!(is_match_greedy("adceb", "*a*b"));
        assert!(!is_match_greedy("acdcb", "a*c?b"));
        assert!(is_match_greedy("", "*"));
    }
}
