#![allow(dead_code)]
#![allow(clippy::all)]
// 1071: Regex Matching — '.' and '*' — DP

use std::collections::HashMap;

// Approach 1: Bottom-up DP
fn is_match_dp(s: &str, p: &str) -> bool {
    let s: Vec<char> = s.chars().collect();
    let p: Vec<char> = p.chars().collect();
    let (m, n) = (s.len(), p.len());
    let mut dp = vec![vec![false; n + 1]; m + 1];
    dp[0][0] = true;

    // Pattern like a*, a*b* can match empty string
    for j in 2..=n {
        if p[j - 1] == '*' {
            dp[0][j] = dp[0][j - 2];
        }
    }

    for i in 1..=m {
        for j in 1..=n {
            if p[j - 1] == '*' {
                dp[i][j] = dp[i][j - 2]; // zero occurrences
                if p[j - 2] == '.' || p[j - 2] == s[i - 1] {
                    dp[i][j] = dp[i][j] || dp[i - 1][j]; // one+ occurrences
                }
            } else if p[j - 1] == '.' || p[j - 1] == s[i - 1] {
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
        if let Some(&v) = cache.get(&(i, j)) {
            return v;
        }
        let first_match = i < s.len() && (p[j] == '.' || p[j] == s[i]);
        let v = if j + 1 < p.len() && p[j + 1] == '*' {
            solve(i, j + 2, s, p, cache) || (first_match && solve(i + 1, j, s, p, cache))
        } else {
            first_match && solve(i + 1, j + 1, s, p, cache)
        };
        cache.insert((i, j), v);
        v
    }

    let mut cache = HashMap::new();
    solve(0, 0, &s, &p, &mut cache)
}

// Approach 3: NFA simulation
fn is_match_nfa(s: &str, p: &str) -> bool {
    let p: Vec<char> = p.chars().collect();
    let s: Vec<char> = s.chars().collect();
    let n = p.len();

    // States are pattern positions; epsilon transitions on '*'
    let mut states = std::collections::HashSet::new();
    states.insert(0);

    // Add epsilon transitions (skip x* pairs)
    fn add_epsilon(states: &mut std::collections::HashSet<usize>, p: &[char]) {
        let mut changed = true;
        while changed {
            changed = false;
            let current: Vec<usize> = states.iter().copied().collect();
            for &st in &current {
                if st + 1 < p.len() && p[st + 1] == '*' && !states.contains(&(st + 2)) {
                    states.insert(st + 2);
                    changed = true;
                }
            }
        }
    }

    add_epsilon(&mut states, &p);

    for &ch in &s {
        let mut next = std::collections::HashSet::new();
        for &st in &states {
            if st < n {
                if p[st] == '.' || p[st] == ch {
                    if st + 1 < n && p[st + 1] == '*' {
                        next.insert(st); // stay (one more match)
                    } else {
                        next.insert(st + 1);
                    }
                }
                // Also handle: if current is 'x' and next is '*', and we're matching via '*'
                if st + 1 < n && p[st + 1] == '*' && (p[st] == '.' || p[st] == ch) {
                    next.insert(st + 2); // consume and move past *
                    next.insert(st); // consume and stay
                }
            }
        }
        add_epsilon(&mut next, &p);
        states = next;
    }

    states.contains(&n)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dp() {
        assert!(!is_match_dp("aa", "a"));
        assert!(is_match_dp("aa", "a*"));
        assert!(is_match_dp("ab", ".*"));
        assert!(is_match_dp("aab", "c*a*b"));
        assert!(!is_match_dp("mississippi", "mis*is*p*."));
        assert!(is_match_dp("", "a*b*"));
    }

    #[test]
    fn test_memo() {
        assert!(!is_match_memo("aa", "a"));
        assert!(is_match_memo("aa", "a*"));
        assert!(is_match_memo("ab", ".*"));
        assert!(is_match_memo("aab", "c*a*b"));
    }

    #[test]
    fn test_nfa() {
        assert!(!is_match_nfa("aa", "a"));
        assert!(is_match_nfa("aa", "a*"));
        assert!(is_match_nfa("ab", ".*"));
        assert!(is_match_nfa("aab", "c*a*b"));
    }
}
