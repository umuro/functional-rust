use std::collections::HashMap;

/// Solution 1: Memoized top-down DP (mirrors OCaml hashtable approach)
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

/// Solution 2: Bottom-up DP table
pub fn knapsack_bottom_up(items: &[(usize, usize)], capacity: usize) -> usize {
    let n = items.len();
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

/// Solution 3: Space-optimised rolling array
pub fn knapsack_rolling(items: &[(usize, usize)], capacity: usize) -> usize {
    let mut dp = vec![0usize; capacity + 1];
    for &(w, v) in items {
        for c in (w..=capacity).rev() {
            dp[c] = dp[c].max(v + dp[c - w]);
        }
    }
    dp[capacity]
}

fn main() {
    let items: &[(usize, usize)] = &[(2, 3), (3, 4), (4, 5), (5, 6)];
    let capacity = 8;

    println!(
        "Top-down memoised  → {}",
        knapsack(items, capacity)
    );
    println!(
        "Bottom-up table    → {}",
        knapsack_bottom_up(items, capacity)
    );
    println!(
        "Rolling array      → {}",
        knapsack_rolling(items, capacity)
    );
}

/* Output:
   Top-down memoised  → 10
   Bottom-up table    → 10
   Rolling array      → 10
*/
