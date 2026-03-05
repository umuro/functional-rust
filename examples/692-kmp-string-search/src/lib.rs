//! # KMP String Search
//! Knuth-Morris-Pratt O(n+m) algorithm

pub fn kmp_search(text: &str, pattern: &str) -> Vec<usize> {
    let (t, p) = (text.as_bytes(), pattern.as_bytes());
    if p.is_empty() { return vec![]; }
    
    let lps = compute_lps(p);
    let mut result = vec![];
    let (mut i, mut j) = (0, 0);
    
    while i < t.len() {
        if t[i] == p[j] { i += 1; j += 1; }
        if j == p.len() { result.push(i - j); j = lps[j - 1]; }
        else if i < t.len() && t[i] != p[j] {
            if j != 0 { j = lps[j - 1]; } else { i += 1; }
        }
    }
    result
}

fn compute_lps(pattern: &[u8]) -> Vec<usize> {
    let mut lps = vec![0; pattern.len()];
    let mut len = 0;
    let mut i = 1;
    while i < pattern.len() {
        if pattern[i] == pattern[len] { len += 1; lps[i] = len; i += 1; }
        else if len != 0 { len = lps[len - 1]; }
        else { lps[i] = 0; i += 1; }
    }
    lps
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_kmp() { assert_eq!(kmp_search("abcabcabc", "abc"), vec![0, 3, 6]); }
}
