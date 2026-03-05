//! # Longest Common Subsequence

pub fn lcs(a: &str, b: &str) -> usize {
    let (m, n) = (a.len(), b.len());
    let a: Vec<char> = a.chars().collect();
    let b: Vec<char> = b.chars().collect();
    let mut dp = vec![vec![0; n + 1]; m + 1];
    
    for i in 1..=m {
        for j in 1..=n {
            dp[i][j] = if a[i-1] == b[j-1] {
                dp[i-1][j-1] + 1
            } else {
                dp[i-1][j].max(dp[i][j-1])
            };
        }
    }
    dp[m][n]
}

pub fn lcs_string(a: &str, b: &str) -> String {
    let (m, n) = (a.len(), b.len());
    let a: Vec<char> = a.chars().collect();
    let b: Vec<char> = b.chars().collect();
    let mut dp = vec![vec![0; n + 1]; m + 1];
    
    for i in 1..=m {
        for j in 1..=n {
            dp[i][j] = if a[i-1] == b[j-1] { dp[i-1][j-1] + 1 } else { dp[i-1][j].max(dp[i][j-1]) };
        }
    }
    
    let mut result = Vec::new();
    let (mut i, mut j) = (m, n);
    while i > 0 && j > 0 {
        if a[i-1] == b[j-1] { result.push(a[i-1]); i -= 1; j -= 1; }
        else if dp[i-1][j] > dp[i][j-1] { i -= 1; }
        else { j -= 1; }
    }
    result.iter().rev().collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test] fn test_lcs() { assert_eq!(lcs("ABCDGH", "AEDFHR"), 3); }
    #[test] fn test_lcs_string() { assert_eq!(lcs_string("ABCDGH", "AEDFHR"), "ADH"); }
    #[test] fn test_empty() { assert_eq!(lcs("", "ABC"), 0); }
    #[test] fn test_identical() { assert_eq!(lcs("ABC", "ABC"), 3); }
}
