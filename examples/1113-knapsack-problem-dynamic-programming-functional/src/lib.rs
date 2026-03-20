#![allow(clippy::all)]
//! 0/1 Knapsack problem solved with dynamic programming (functional style).

/// Solves the 0/1 Knapsack problem using dynamic programming.
/// Returns the maximum value that can be put in a knapsack of capacity `capacity`.
///
/// # Arguments
/// * `weights` - A slice of item weights.
/// * `values` - A slice of item values (should have same length as `weights`).
/// * `capacity` - The maximum capacity of the knapsack.
///
/// # Example
/// ```
/// use example_1113_knapsack_problem_dynamic_programming_functional::knapsack;
/// let weights = vec![1, 2, 3];
/// let values = vec![10, 15, 40];
/// let capacity = 6;
/// assert_eq!(knapsack(&weights, &values, capacity), 65);
/// ```
pub fn knapsack(weights: &[usize], values: &[usize], capacity: usize) -> usize {
    let n = weights.len();

    // dp table: dp[i][w] is max value using first i items and capacity w
    // Using a single row to optimize space, as dp[i] only depends on dp[i-1]
    let mut dp = vec![0; capacity + 1];

    (0..n).for_each(|i| {
        (0..=capacity).rev().for_each(|w| {
            if weights[i] <= w {
                dp[w] = dp[w].max(values[i] + dp[w - weights[i]]);
            }
        });
    });

    dp[capacity]
}

#[cfg(test)]
mod tests {
    use super::knapsack;

    #[test]
    fn test_knapsack_basic() {
        let weights = vec![1, 2, 3];
        let values = vec![10, 15, 40];
        let capacity = 6;
        assert_eq!(knapsack(&weights, &values, capacity), 65); // Take items 2 (15) and 3 (40) whose weights are 2 and 3 and sum 5, or take items 1 (10) and 3 (40) whose weights are 1 and 3 and sum 4
    }

    #[test]
    fn test_knapsack_no_items() {
        let weights = vec![];
        let values = vec![];
        let capacity = 10;
        assert_eq!(knapsack(&weights, &values, capacity), 0);
    }

    #[test]
    fn test_knapsack_zero_capacity() {
        let weights = vec![1, 2, 3];
        let values = vec![10, 15, 40];
        let capacity = 0;
        assert_eq!(knapsack(&weights, &values, capacity), 0);
    }

    #[test]
    fn test_knapsack_full_capacity_multiple_items() {
        let weights = vec![2, 3, 4, 5];
        let values = vec![3, 4, 5, 6];
        let capacity = 5;
        assert_eq!(knapsack(&weights, &values, capacity), 7); // Take 2 (3) and 3 (4)
    }

    #[test]
    fn test_knapsack_item_too_heavy() {
        let weights = vec![10];
        let values = vec![100];
        let capacity = 5;
        assert_eq!(knapsack(&weights, &values, capacity), 0);
    }

    #[test]
    fn test_knapsack_complex() {
        let weights = vec![3, 4, 6, 5];
        let values = vec![2, 3, 1, 4];
        let capacity = 8;
        assert_eq!(knapsack(&weights, &values, capacity), 6); // Items 3 (weight 6, value 1) + 4 (weight 5, value 4) = cap 11 (too much!) ; it must be items with weight 3 (value 2) and item with weight 5 (value 4)
    }

    #[test]
    fn test_knapsack_duplicate_weights_values() {
        let weights = vec![1, 1, 2];
        let values = vec![10, 15, 20];
        let capacity = 2;
        assert_eq!(knapsack(&weights, &values, capacity), 25); // Take both items of weight 1
    }
}
