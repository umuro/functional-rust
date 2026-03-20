use std::collections::HashMap;

/// Solution 1: Idiomatic Rust — memoized recursion with HashMap cache
/// Mirrors the OCaml top-down DP approach (hashtable memoization).
pub fn knapsack(items: &[(usize, usize)], capacity: usize) -> usize {
    let mut cache = HashMap::new();
    solve(items, 0, capacity, &mut cache)
}

fn solve(
    items: &[(usize, usize)],
    i: usize,
    cap: usize,
    cache: &mut HashMap<(usize, usize), usize>,
) -> usize {
    if i >= items.len() || cap == 0 {
        return 0;
    }
    if let Some(&v) = cache.get(&(i, cap)) {
        return v;
    }
    let (w, v) = items[i];
    let without = solve(items, i + 1, cap, cache);
    let with_item = if w <= cap {
        v + solve(items, i + 1, cap - w, cache)
    } else {
        0
    };
    let best = without.max(with_item);
    cache.insert((i, cap), best);
    best
}

/// Solution 2: Bottom-up DP table — classic iterative Rust style
/// Builds a 2D table: dp[i][c] = best value using items[0..i] with capacity c.
pub fn knapsack_bottom_up(items: &[(usize, usize)], capacity: usize) -> usize {
    let n = items.len();
    // dp[i][c]: max value using first i items with capacity c
    let mut dp = vec![vec![0usize; capacity + 1]; n + 1];
    for i in 1..=n {
        let (w, v) = items[i - 1];
        for c in 0..=capacity {
            dp[i][c] = if w <= c {
                dp[i - 1][c].max(v + dp[i - 1][c - w])
            } else {
                dp[i - 1][c]
            };
        }
    }
    dp[n][capacity]
}

/// Solution 3: Bottom-up DP with a single rolling row (space-optimised)
/// Iterates capacity in reverse to avoid overwriting values we still need.
pub fn knapsack_rolling(items: &[(usize, usize)], capacity: usize) -> usize {
    let mut dp = vec![0usize; capacity + 1];
    for &(w, v) in items {
        // Traverse high → low so each item is counted at most once
        for c in (w..=capacity).rev() {
            dp[c] = dp[c].max(v + dp[c - w]);
        }
    }
    dp[capacity]
}

#[cfg(test)]
mod tests {
    use super::*;

    const ITEMS: &[(usize, usize)] = &[(2, 3), (3, 4), (4, 5), (5, 6)];

    #[test]
    fn test_empty_items() {
        assert_eq!(knapsack(&[], 10), 0);
        assert_eq!(knapsack_bottom_up(&[], 10), 0);
        assert_eq!(knapsack_rolling(&[], 10), 0);
    }

    #[test]
    fn test_zero_capacity() {
        assert_eq!(knapsack(ITEMS, 0), 0);
        assert_eq!(knapsack_bottom_up(ITEMS, 0), 0);
        assert_eq!(knapsack_rolling(ITEMS, 0), 0);
    }

    #[test]
    fn test_ocaml_example() {
        // OCaml example: items = [(2,3);(3,4);(4,5);(5,6)], capacity = 8
        // Optimal: take (3,4) + (5,6) → weight=8, value=10
        assert_eq!(knapsack(ITEMS, 8), 10);
        assert_eq!(knapsack_bottom_up(ITEMS, 8), 10);
        assert_eq!(knapsack_rolling(ITEMS, 8), 10);
    }

    #[test]
    fn test_single_item_fits() {
        // weight=3 does NOT fit in capacity=2
        assert_eq!(knapsack(&[(3, 7)], 2), 0);
        assert_eq!(knapsack_bottom_up(&[(3, 7)], 2), 0);
        assert_eq!(knapsack_rolling(&[(3, 7)], 2), 0);
        // weight=3 fits exactly in capacity=3
        assert_eq!(knapsack(&[(3, 7)], 3), 7);
        assert_eq!(knapsack_bottom_up(&[(3, 7)], 3), 7);
        assert_eq!(knapsack_rolling(&[(3, 7)], 3), 7);
    }

    #[test]
    fn test_all_items_fit() {
        // Total weight = 2+3+4+5 = 14, capacity 20 — take everything
        let total_value: usize = ITEMS.iter().map(|&(_, v)| v).sum();
        assert_eq!(knapsack(ITEMS, 20), total_value);
        assert_eq!(knapsack_bottom_up(ITEMS, 20), total_value);
        assert_eq!(knapsack_rolling(ITEMS, 20), total_value);
    }

    #[test]
    fn test_small_capacity() {
        // Only item (2,3) fits within capacity 2
        assert_eq!(knapsack(ITEMS, 2), 3);
        assert_eq!(knapsack_bottom_up(ITEMS, 2), 3);
        assert_eq!(knapsack_rolling(ITEMS, 2), 3);
    }

    #[test]
    fn test_all_variants_agree() {
        for cap in 0..=15 {
            let a = knapsack(ITEMS, cap);
            let b = knapsack_bottom_up(ITEMS, cap);
            let c = knapsack_rolling(ITEMS, cap);
            assert_eq!(a, b, "mismatch at cap={cap}: top-down={a} bottom-up={b}");
            assert_eq!(b, c, "mismatch at cap={cap}: bottom-up={b} rolling={c}");
        }
    }
}
