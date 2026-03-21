#![allow(dead_code)]
#![allow(clippy::all)]
// 1056: Edit Distance (Levenshtein) — 2D DP Table

use std::collections::HashMap;

// Approach 1: 2D DP table
fn edit_distance(s1: &str, s2: &str) -> usize {
    let a: Vec<char> = s1.chars().collect();
    let b: Vec<char> = s2.chars().collect();
    let (m, n) = (a.len(), b.len());
    let mut dp = vec![vec![0; n + 1]; m + 1];
    for i in 0..=m {
        dp[i][0] = i;
    }
    for j in 0..=n {
        dp[0][j] = j;
    }
    for i in 1..=m {
        for j in 1..=n {
            dp[i][j] = if a[i - 1] == b[j - 1] {
                dp[i - 1][j - 1]
            } else {
                1 + dp[i - 1][j].min(dp[i][j - 1]).min(dp[i - 1][j - 1])
            };
        }
    }
    dp[m][n]
}

// Approach 2: Space-optimized with two rows
fn edit_distance_opt(s1: &str, s2: &str) -> usize {
    let a: Vec<char> = s1.chars().collect();
    let b: Vec<char> = s2.chars().collect();
    let (m, n) = (a.len(), b.len());
    let mut prev: Vec<usize> = (0..=n).collect();
    let mut curr = vec![0; n + 1];
    for i in 1..=m {
        curr[0] = i;
        for j in 1..=n {
            curr[j] = if a[i - 1] == b[j - 1] {
                prev[j - 1]
            } else {
                1 + prev[j].min(curr[j - 1]).min(prev[j - 1])
            };
        }
        std::mem::swap(&mut prev, &mut curr);
    }
    prev[n]
}

// Approach 3: Recursive with memoization
fn edit_distance_memo(s1: &str, s2: &str) -> usize {
    let a: Vec<char> = s1.chars().collect();
    let b: Vec<char> = s2.chars().collect();
    fn solve(
        i: usize,
        j: usize,
        a: &[char],
        b: &[char],
        cache: &mut HashMap<(usize, usize), usize>,
    ) -> usize {
        if i == 0 {
            return j;
        }
        if j == 0 {
            return i;
        }
        if let Some(&v) = cache.get(&(i, j)) {
            return v;
        }
        let v = if a[i - 1] == b[j - 1] {
            solve(i - 1, j - 1, a, b, cache)
        } else {
            1 + solve(i - 1, j, a, b, cache)
                .min(solve(i, j - 1, a, b, cache))
                .min(solve(i - 1, j - 1, a, b, cache))
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
    fn test_edit_distance() {
        assert_eq!(edit_distance("kitten", "sitting"), 3);
        assert_eq!(edit_distance("saturday", "sunday"), 3);
        assert_eq!(edit_distance("", "abc"), 3);
        assert_eq!(edit_distance("abc", "abc"), 0);
    }

    #[test]
    fn test_edit_distance_opt() {
        assert_eq!(edit_distance_opt("kitten", "sitting"), 3);
        assert_eq!(edit_distance_opt("saturday", "sunday"), 3);
    }

    #[test]
    fn test_edit_distance_memo() {
        assert_eq!(edit_distance_memo("kitten", "sitting"), 3);
        assert_eq!(edit_distance_memo("saturday", "sunday"), 3);
    }
}
