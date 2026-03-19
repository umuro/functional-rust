//! # 0/1 Knapsack Problem
//!
//! Classic dynamic programming problem.

/// Solve 0/1 knapsack problem
pub fn knapsack(weights: &[usize], values: &[usize], capacity: usize) -> usize {
    let n = weights.len();
    let mut dp = vec![vec![0; capacity + 1]; n + 1];

    for i in 1..=n {
        for w in 0..=capacity {
            if weights[i - 1] <= w {
                dp[i][w] = dp[i - 1][w].max(dp[i - 1][w - weights[i - 1]] + values[i - 1]);
            } else {
                dp[i][w] = dp[i - 1][w];
            }
        }
    }
    dp[n][capacity]
}

/// Space-optimized version
pub fn knapsack_optimized(weights: &[usize], values: &[usize], capacity: usize) -> usize {
    let mut dp = vec![0; capacity + 1];
    for i in 0..weights.len() {
        for w in (weights[i]..=capacity).rev() {
            dp[w] = dp[w].max(dp[w - weights[i]] + values[i]);
        }
    }
    dp[capacity]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_knapsack() {
        let weights = [1, 2, 3];
        let values = [6, 10, 12];
        assert_eq!(knapsack(&weights, &values, 5), 22);
    }

    #[test]
    fn test_knapsack_optimized() {
        let weights = [1, 2, 3];
        let values = [6, 10, 12];
        assert_eq!(knapsack_optimized(&weights, &values, 5), 22);
    }

    #[test]
    fn test_empty() {
        assert_eq!(knapsack(&[], &[], 10), 0);
    }

    #[test]
    fn test_no_capacity() {
        let weights = [1, 2];
        let values = [10, 20];
        assert_eq!(knapsack(&weights, &values, 0), 0);
    }
}
