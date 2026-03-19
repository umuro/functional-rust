// 1054: 0/1 Knapsack — 2D DP Table

use std::collections::HashMap;

// Approach 1: 2D Vec DP
fn knapsack_2d(weights: &[usize], values: &[i64], capacity: usize) -> i64 {
    let n = weights.len();
    let mut dp = vec![vec![0i64; capacity + 1]; n + 1];
    for i in 1..=n {
        for w in 0..=capacity {
            dp[i][w] = dp[i - 1][w];
            if weights[i - 1] <= w {
                dp[i][w] = dp[i][w].max(dp[i - 1][w - weights[i - 1]] + values[i - 1]);
            }
        }
    }
    dp[n][capacity]
}

// Approach 2: 1D rolling array
fn knapsack_1d(weights: &[usize], values: &[i64], capacity: usize) -> i64 {
    let mut dp = vec![0i64; capacity + 1];
    for i in 0..weights.len() {
        for w in (weights[i]..=capacity).rev() {
            dp[w] = dp[w].max(dp[w - weights[i]] + values[i]);
        }
    }
    dp[capacity]
}

// Approach 3: Recursive with HashMap memoization
fn knapsack_memo(weights: &[usize], values: &[i64], capacity: usize) -> i64 {
    fn solve(
        i: usize,
        w: usize,
        weights: &[usize],
        values: &[i64],
        cache: &mut HashMap<(usize, usize), i64>,
    ) -> i64 {
        if i == 0 || w == 0 {
            return 0;
        }
        if let Some(&v) = cache.get(&(i, w)) {
            return v;
        }
        let skip = solve(i - 1, w, weights, values, cache);
        let take = if weights[i - 1] <= w {
            solve(i - 1, w - weights[i - 1], weights, values, cache) + values[i - 1]
        } else {
            0
        };
        let result = skip.max(take);
        cache.insert((i, w), result);
        result
    }
    let mut cache = HashMap::new();
    solve(weights.len(), capacity, weights, values, &mut cache)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_knapsack_2d() {
        assert_eq!(knapsack_2d(&[2, 3, 4, 5], &[3, 4, 5, 6], 5), 7);
        assert_eq!(knapsack_2d(&[1, 2, 3], &[6, 10, 12], 5), 22);
    }

    #[test]
    fn test_knapsack_1d() {
        assert_eq!(knapsack_1d(&[2, 3, 4, 5], &[3, 4, 5, 6], 5), 7);
        assert_eq!(knapsack_1d(&[1, 2, 3], &[6, 10, 12], 5), 22);
    }

    #[test]
    fn test_knapsack_memo() {
        assert_eq!(knapsack_memo(&[2, 3, 4, 5], &[3, 4, 5, 6], 5), 7);
        assert_eq!(knapsack_memo(&[1, 2, 3], &[6, 10, 12], 5), 22);
    }
}
