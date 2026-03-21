#![allow(dead_code)]
#![allow(clippy::all)]
// 1059: Rod Cutting — Maximize Revenue

use std::collections::HashMap;

// Approach 1: Bottom-up DP
fn rod_cut_dp(prices: &[i64], n: usize) -> i64 {
    let mut dp = vec![0i64; n + 1];
    for i in 1..=n {
        for j in 1..=i.min(prices.len()) {
            dp[i] = dp[i].max(prices[j - 1] + dp[i - j]);
        }
    }
    dp[n]
}

// Approach 2: Top-down with memoization
fn rod_cut_memo(prices: &[i64], n: usize) -> i64 {
    fn solve(len: usize, prices: &[i64], cache: &mut HashMap<usize, i64>) -> i64 {
        if len == 0 {
            return 0;
        }
        if let Some(&v) = cache.get(&len) {
            return v;
        }
        let mut best = 0;
        for j in 1..=len.min(prices.len()) {
            best = best.max(prices[j - 1] + solve(len - j, prices, cache));
        }
        cache.insert(len, best);
        best
    }
    let mut cache = HashMap::new();
    solve(n, prices, &mut cache)
}

// Approach 3: With cut reconstruction
fn rod_cut_with_cuts(prices: &[i64], n: usize) -> (i64, Vec<usize>) {
    let mut dp = vec![0i64; n + 1];
    let mut cuts = vec![0usize; n + 1];
    for i in 1..=n {
        for j in 1..=i.min(prices.len()) {
            let val = prices[j - 1] + dp[i - j];
            if val > dp[i] {
                dp[i] = val;
                cuts[i] = j;
            }
        }
    }
    let mut result = Vec::new();
    let mut remaining = n;
    while remaining > 0 {
        result.push(cuts[remaining]);
        remaining -= cuts[remaining];
    }
    (dp[n], result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rod_cut_dp() {
        assert_eq!(rod_cut_dp(&[1, 5, 8, 9, 10, 17, 17, 20], 8), 22);
        assert_eq!(rod_cut_dp(&[1, 5, 8, 9, 10, 17, 17, 20], 4), 10);
        assert_eq!(rod_cut_dp(&[3, 5, 8, 9, 10, 17, 17, 20], 4), 12);
    }

    #[test]
    fn test_rod_cut_memo() {
        assert_eq!(rod_cut_memo(&[1, 5, 8, 9, 10, 17, 17, 20], 8), 22);
        assert_eq!(rod_cut_memo(&[1, 5, 8, 9, 10, 17, 17, 20], 4), 10);
    }

    #[test]
    fn test_rod_cut_with_cuts() {
        let (revenue, cuts) = rod_cut_with_cuts(&[1, 5, 8, 9, 10, 17, 17, 20], 8);
        assert_eq!(revenue, 22);
        assert_eq!(cuts.iter().sum::<usize>(), 8);
    }
}
