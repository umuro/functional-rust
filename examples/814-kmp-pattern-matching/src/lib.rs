//! # KMP Pattern Matching
pub fn compute_lps(pattern: &str) -> Vec<usize> {
    let p: Vec<char> = pattern.chars().collect();
    let m = p.len(); let mut lps = vec![0; m]; let mut len = 0; let mut i = 1;
    while i < m {
        if p[i] == p[len] { len += 1; lps[i] = len; i += 1; }
        else if len != 0 { len = lps[len - 1]; }
        else { lps[i] = 0; i += 1; }
    }
    lps
}
pub fn kmp_search(text: &str, pattern: &str) -> Vec<usize> {
    let t: Vec<char> = text.chars().collect();
    let p: Vec<char> = pattern.chars().collect();
    let (n, m) = (t.len(), p.len()); if m == 0 { return vec![]; }
    let lps = compute_lps(pattern); let mut results = vec![];
    let (mut i, mut j) = (0, 0);
    while i < n {
        if t[i] == p[j] { i += 1; j += 1; }
        if j == m { results.push(i - j); j = lps[j - 1]; }
        else if i < n && t[i] != p[j] { if j != 0 { j = lps[j - 1]; } else { i += 1; } }
    }
    results
}
#[cfg(test)] mod tests { use super::*;
    #[test] fn test_kmp() { assert_eq!(kmp_search("ABABDABACDABABCABAB", "ABABCABAB"), vec![10]); }
}
