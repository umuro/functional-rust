//! # Longest Increasing Subsequence

pub fn lis(arr: &[i32]) -> usize {
    if arr.is_empty() { return 0; }
    let mut dp = vec![1; arr.len()];
    for i in 1..arr.len() {
        for j in 0..i {
            if arr[j] < arr[i] { dp[i] = dp[i].max(dp[j] + 1); }
        }
    }
    *dp.iter().max().unwrap()
}

pub fn lis_binary_search(arr: &[i32]) -> usize {
    let mut tails = Vec::new();
    for &x in arr {
        match tails.binary_search(&x) {
            Ok(_) => {}
            Err(pos) => {
                if pos == tails.len() { tails.push(x); }
                else { tails[pos] = x; }
            }
        }
    }
    tails.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test] fn test_lis() { assert_eq!(lis(&[10, 9, 2, 5, 3, 7, 101, 18]), 4); }
    #[test] fn test_binary() { assert_eq!(lis_binary_search(&[10, 9, 2, 5, 3, 7, 101, 18]), 4); }
    #[test] fn test_empty() { assert_eq!(lis(&[]), 0); }
}
