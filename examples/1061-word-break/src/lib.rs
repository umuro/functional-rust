// 1061: Word Break — DP + HashSet

use std::collections::{HashMap, HashSet, VecDeque};

// Approach 1: Bottom-up DP
fn word_break(s: &str, words: &[&str]) -> bool {
    let dict: HashSet<&str> = words.iter().copied().collect();
    let n = s.len();
    let mut dp = vec![false; n + 1];
    dp[0] = true;
    for i in 1..=n {
        for j in 0..i {
            if dp[j] && dict.contains(&s[j..i]) {
                dp[i] = true;
                break;
            }
        }
    }
    dp[n]
}

// Approach 2: Recursive with memoization
fn word_break_memo(s: &str, words: &[&str]) -> bool {
    let dict: HashSet<&str> = words.iter().copied().collect();
    fn solve<'a>(
        s: &'a str,
        start: usize,
        dict: &HashSet<&str>,
        cache: &mut HashMap<usize, bool>,
    ) -> bool {
        if start == s.len() {
            return true;
        }
        if let Some(&v) = cache.get(&start) {
            return v;
        }
        let mut result = false;
        for end_ in (start + 1)..=s.len() {
            if dict.contains(&s[start..end_]) && solve(s, end_, dict, cache) {
                result = true;
                break;
            }
        }
        cache.insert(start, result);
        result
    }
    let mut cache = HashMap::new();
    solve(s, 0, &dict, &mut cache)
}

// Approach 3: BFS approach
fn word_break_bfs(s: &str, words: &[&str]) -> bool {
    let dict: HashSet<&str> = words.iter().copied().collect();
    let n = s.len();
    let mut visited = vec![false; n + 1];
    let mut queue = VecDeque::new();
    queue.push_back(0);
    visited[0] = true;
    while let Some(start) = queue.pop_front() {
        for end_ in (start + 1)..=n {
            if !visited[end_] && dict.contains(&s[start..end_]) {
                if end_ == n {
                    return true;
                }
                visited[end_] = true;
                queue.push_back(end_);
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_word_break() {
        assert!(word_break("leetcode", &["leet", "code"]));
        assert!(word_break("applepenapple", &["apple", "pen"]));
        assert!(!word_break(
            "catsandog",
            &["cats", "dog", "sand", "and", "cat"]
        ));
    }

    #[test]
    fn test_word_break_memo() {
        assert!(word_break_memo("leetcode", &["leet", "code"]));
        assert!(!word_break_memo(
            "catsandog",
            &["cats", "dog", "sand", "and", "cat"]
        ));
    }

    #[test]
    fn test_word_break_bfs() {
        assert!(word_break_bfs("leetcode", &["leet", "code"]));
        assert!(!word_break_bfs(
            "catsandog",
            &["cats", "dog", "sand", "and", "cat"]
        ));
    }
}
