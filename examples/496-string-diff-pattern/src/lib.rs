#![allow(clippy::all)]
// 496. String diff / edit distance
fn levenshtein(s: &str, t: &str) -> usize {
    let sv: Vec<char> = s.chars().collect();
    let tv: Vec<char> = t.chars().collect();
    let (m, n) = (sv.len(), tv.len());
    let mut dp = vec![vec![0usize; n + 1]; m + 1];
    for i in 0..=m {
        dp[i][0] = i;
    }
    for j in 0..=n {
        dp[0][j] = j;
    }
    for i in 1..=m {
        for j in 1..=n {
            dp[i][j] = if sv[i - 1] == tv[j - 1] {
                dp[i - 1][j - 1]
            } else {
                1 + dp[i - 1][j].min(dp[i][j - 1]).min(dp[i - 1][j - 1])
            };
        }
    }
    dp[m][n]
}

fn closest<'a>(query: &str, candidates: &[&'a str]) -> Option<&'a str> {
    candidates
        .iter()
        .min_by_key(|&&c| levenshtein(query, c))
        .copied()
}

fn starts_with_ignore_case(s: &str, prefix: &str) -> bool {
    s.len() >= prefix.len() && s[..prefix.len()].eq_ignore_ascii_case(prefix)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_kitten() {
        assert_eq!(levenshtein("kitten", "sitting"), 3);
    }
    #[test]
    fn test_same() {
        assert_eq!(levenshtein("abc", "abc"), 0);
    }
    #[test]
    fn test_empty() {
        assert_eq!(levenshtein("", "abc"), 3);
        assert_eq!(levenshtein("abc", ""), 3);
    }
    #[test]
    fn test_closest() {
        assert_eq!(closest("rast", &["rust", "bust", "just"]), Some("rust"));
    }
}
