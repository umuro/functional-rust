// 1073: Burst Balloons — Interval DP

use std::collections::HashMap;

// Approach 1: Bottom-up interval DP
fn max_coins_dp(nums: &[i32]) -> i32 {
    let n = nums.len();
    let mut balloons = vec![1i32; n + 2];
    for i in 0..n {
        balloons[i + 1] = nums[i];
    }
    let len = n + 2;
    let mut dp = vec![vec![0i32; len]; len];

    for gap in 2..len {
        for i in 0..len - gap {
            let j = i + gap;
            for k in (i + 1)..j {
                let coins = dp[i][k] + dp[k][j] + balloons[i] * balloons[k] * balloons[j];
                dp[i][j] = dp[i][j].max(coins);
            }
        }
    }
    dp[0][len - 1]
}

// Approach 2: Recursive with memoization
fn max_coins_memo(nums: &[i32]) -> i32 {
    let n = nums.len();
    let mut balloons = vec![1i32; n + 2];
    for i in 0..n {
        balloons[i + 1] = nums[i];
    }
    let len = n + 2;

    fn solve(
        left: usize,
        right: usize,
        balloons: &[i32],
        cache: &mut HashMap<(usize, usize), i32>,
    ) -> i32 {
        if right.saturating_sub(left) < 2 {
            return 0;
        }
        if let Some(&v) = cache.get(&(left, right)) {
            return v;
        }
        let mut best = 0;
        for k in (left + 1)..right {
            let coins = solve(left, k, balloons, cache)
                + solve(k, right, balloons, cache)
                + balloons[left] * balloons[k] * balloons[right];
            best = best.max(coins);
        }
        cache.insert((left, right), best);
        best
    }

    let mut cache = HashMap::new();
    solve(0, len - 1, &balloons, &mut cache)
}

// Approach 3: Divide and conquer (same logic, different framing)
fn max_coins_dc(nums: &[i32]) -> i32 {
    let n = nums.len();
    let mut balloons = vec![1i32; n + 2];
    for i in 0..n {
        balloons[i + 1] = nums[i];
    }
    let len = n + 2;
    let mut memo = vec![vec![-1i32; len]; len];

    fn solve(l: usize, r: usize, balloons: &[i32], memo: &mut Vec<Vec<i32>>) -> i32 {
        if r.saturating_sub(l) < 2 {
            return 0;
        }
        if memo[l][r] >= 0 {
            return memo[l][r];
        }
        let mut best = 0;
        for k in (l + 1)..r {
            let coins = solve(l, k, balloons, memo)
                + solve(k, r, balloons, memo)
                + balloons[l] * balloons[k] * balloons[r];
            best = best.max(coins);
        }
        memo[l][r] = best;
        best
    }

    solve(0, len - 1, &balloons, &mut memo)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dp() {
        assert_eq!(max_coins_dp(&[3, 1, 5, 8]), 167);
        assert_eq!(max_coins_dp(&[1, 5]), 10);
        assert_eq!(max_coins_dp(&[1]), 1);
    }

    #[test]
    fn test_memo() {
        assert_eq!(max_coins_memo(&[3, 1, 5, 8]), 167);
        assert_eq!(max_coins_memo(&[1, 5]), 10);
    }

    #[test]
    fn test_dc() {
        assert_eq!(max_coins_dc(&[3, 1, 5, 8]), 167);
        assert_eq!(max_coins_dc(&[1, 5]), 10);
    }
}
