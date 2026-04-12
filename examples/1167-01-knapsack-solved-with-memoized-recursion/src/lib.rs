#![allow(dead_code)]
//! 0/1 Knapsack Solved with Memoized Recursion
//! See example.ml for OCaml reference
//!
//! Top-down dynamic programming: the exponential recursive solution is made polynomial
//! by caching results in a HashMap keyed on (item_index, remaining_capacity).

use std::collections::HashMap;

/// An item with a weight and value.
#[derive(Debug, Clone, Copy)]
pub struct Item {
    pub weight: usize,
    pub value: u64,
}

/// Idiomatic Rust: memoized recursive knapsack.
/// `items` is the item list, `capacity` is the knapsack limit.
/// Returns the maximum value achievable without exceeding `capacity`.
pub fn knapsack(items: &[Item], capacity: usize) -> u64 {
    let mut memo: HashMap<(usize, usize), u64> = HashMap::new();
    solve(items, 0, capacity, &mut memo)
}

/// Internal recursive helper.
/// `i` = current item index, `cap` = remaining capacity.
fn solve(items: &[Item], i: usize, cap: usize, memo: &mut HashMap<(usize, usize), u64>) -> u64 {
    // Base case: no items left or no capacity remaining.
    if i >= items.len() || cap == 0 {
        return 0;
    }
    // Check memo table before recursing.
    if let Some(&cached) = memo.get(&(i, cap)) {
        return cached;
    }
    let item = items[i];
    // Option 1: skip this item.
    let without = solve(items, i + 1, cap, memo);
    // Option 2: take this item (only if it fits).
    let with_item = if item.weight <= cap {
        item.value + solve(items, i + 1, cap - item.weight, memo)
    } else {
        0
    };
    let best = without.max(with_item);
    memo.insert((i, cap), best);
    best
}

/// Functional/recursive: mirrors the OCaml solution exactly.
/// Uses a closure-captured `Hashtbl` analogue — here `&mut HashMap` threaded explicitly.
pub fn knapsack_from_pairs(items: &[(usize, u64)], capacity: usize) -> u64 {
    let items: Vec<Item> = items
        .iter()
        .map(|&(w, v)| Item {
            weight: w,
            value: v,
        })
        .collect();
    knapsack(&items, capacity)
}

/// Bottom-up tabulation: builds the DP table iteratively.
/// Same asymptotic complexity as memoized recursion but avoids recursion overhead.
pub fn knapsack_tabulation(items: &[Item], capacity: usize) -> u64 {
    let n = items.len();
    // dp[i][w] = best value using items[0..i] with capacity w.
    let mut dp = vec![vec![0u64; capacity + 1]; n + 1];
    for i in 1..=n {
        let item = items[i - 1];
        for w in 0..=capacity {
            // Skip this item.
            dp[i][w] = dp[i - 1][w];
            // Take this item if it fits.
            if item.weight <= w {
                let take = item.value + dp[i - 1][w - item.weight];
                if take > dp[i][w] {
                    dp[i][w] = take;
                }
            }
        }
    }
    dp[n][capacity]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_items() -> Vec<Item> {
        vec![
            Item {
                weight: 2,
                value: 3,
            },
            Item {
                weight: 3,
                value: 4,
            },
            Item {
                weight: 4,
                value: 5,
            },
            Item {
                weight: 5,
                value: 6,
            },
        ]
    }

    #[test]
    fn test_knapsack_empty_items() {
        assert_eq!(knapsack(&[], 10), 0);
    }

    #[test]
    fn test_knapsack_zero_capacity() {
        let items = sample_items();
        assert_eq!(knapsack(&items, 0), 0);
    }

    #[test]
    fn test_knapsack_single_item_fits() {
        let items = vec![Item {
            weight: 3,
            value: 10,
        }];
        assert_eq!(knapsack(&items, 5), 10);
    }

    #[test]
    fn test_knapsack_single_item_too_heavy() {
        let items = vec![Item {
            weight: 10,
            value: 100,
        }];
        assert_eq!(knapsack(&items, 5), 0);
    }

    #[test]
    fn test_knapsack_multiple_items() {
        // Items: (2,3), (3,4), (4,5), (5,6); capacity 8
        // Optimal: take (2,3) + (3,4) + ... let's check: (2+3+4=9>8), (2+3=5, val=7), (3+4=7, val=9), (2+4=6, val=8), (3+5=8, val=10)
        assert_eq!(knapsack(&sample_items(), 8), 10);
    }

    #[test]
    fn test_knapsack_memoized_matches_tabulation() {
        let items = sample_items();
        for cap in 0..=15 {
            assert_eq!(
                knapsack(&items, cap),
                knapsack_tabulation(&items, cap),
                "mismatch at capacity {}",
                cap
            );
        }
    }

    #[test]
    fn test_knapsack_from_pairs() {
        assert_eq!(
            knapsack_from_pairs(&[(2, 3), (3, 4), (4, 5), (5, 6)], 8),
            10
        );
    }

    #[test]
    fn test_knapsack_all_items_fit() {
        let items = vec![
            Item {
                weight: 1,
                value: 1,
            },
            Item {
                weight: 1,
                value: 2,
            },
            Item {
                weight: 1,
                value: 3,
            },
        ];
        // All fit; take all for value 6.
        assert_eq!(knapsack(&items, 10), 6);
    }
}
