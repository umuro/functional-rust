//! # Palindrome Partitioning

pub fn min_cuts(s: &str) -> usize {
    let chars: Vec<char> = s.chars().collect();
    let n = chars.len();
    if n <= 1 { return 0; }
    
    let mut is_pal = vec![vec![false; n]; n];
    for i in 0..n { is_pal[i][i] = true; }
    for i in 0..n-1 { is_pal[i][i+1] = chars[i] == chars[i+1]; }
    for len in 3..=n {
        for i in 0..=n-len {
            let j = i + len - 1;
            is_pal[i][j] = chars[i] == chars[j] && is_pal[i+1][j-1];
        }
    }
    
    let mut dp = vec![0; n];
    for i in 0..n {
        if is_pal[0][i] { dp[i] = 0; }
        else {
            dp[i] = i;
            for j in 1..=i {
                if is_pal[j][i] { dp[i] = dp[i].min(dp[j-1] + 1); }
            }
        }
    }
    dp[n-1]
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn test_cuts() { assert_eq!(min_cuts("aab"), 1); }
    #[test] fn test_palindrome() { assert_eq!(min_cuts("aa"), 0); }
}
