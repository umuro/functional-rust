#![allow(clippy::all)]
//! # Rod Cutting Problem

pub fn rod_cutting(prices: &[usize], n: usize) -> usize {
    let mut dp = vec![0; n + 1];
    for i in 1..=n {
        for j in 1..=i.min(prices.len()) {
            dp[i] = dp[i].max(prices[j - 1] + dp[i - j]);
        }
    }
    dp[n]
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_rod() {
        assert_eq!(rod_cutting(&[1, 5, 8, 9, 10, 17, 17, 20], 8), 22);
    }
    #[test]
    fn test_small() {
        assert_eq!(rod_cutting(&[1, 5, 8, 9], 4), 10);
    }
}
