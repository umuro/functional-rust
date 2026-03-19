//! # Coin Change

pub fn coin_change(coins: &[usize], amount: usize) -> Option<usize> {
    let mut dp = vec![usize::MAX; amount + 1];
    dp[0] = 0;
    for i in 1..=amount {
        for &coin in coins {
            if coin <= i && dp[i - coin] != usize::MAX {
                dp[i] = dp[i].min(dp[i - coin] + 1);
            }
        }
    }
    if dp[amount] == usize::MAX {
        None
    } else {
        Some(dp[amount])
    }
}

pub fn coin_change_ways(coins: &[usize], amount: usize) -> usize {
    let mut dp = vec![0usize; amount + 1];
    dp[0] = 1;
    for &coin in coins {
        for i in coin..=amount {
            dp[i] += dp[i - coin];
        }
    }
    dp[amount]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_change() {
        assert_eq!(coin_change(&[1, 2, 5], 11), Some(3));
    }
    #[test]
    fn test_ways() {
        assert_eq!(coin_change_ways(&[1, 2, 5], 5), 4);
    }
    #[test]
    fn test_impossible() {
        assert_eq!(coin_change(&[2], 3), None);
    }
}
