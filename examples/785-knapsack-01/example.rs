// 785. 0/1 Knapsack: DP Table Approach
// Recursive memoised + iterative tabulation + traceback

use std::collections::HashMap;

// ── Item type ─────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct Item {
    pub name: &'static str,
    pub weight: usize,
    pub value: u64,
}

// ── 1. Recursive with memoisation ─────────────────────────────────────────────

fn knapsack_memo_inner(
    items: &[Item],
    i: usize,
    w: usize,
    memo: &mut HashMap<(usize, usize), u64>,
) -> u64 {
    if i == 0 || w == 0 { return 0; }
    if let Some(&v) = memo.get(&(i, w)) { return v; }
    let item = &items[i - 1];
    let best = if item.weight > w {
        knapsack_memo_inner(items, i - 1, w, memo)
    } else {
        let take   = item.value + knapsack_memo_inner(items, i - 1, w - item.weight, memo);
        let skip   = knapsack_memo_inner(items, i - 1, w, memo);
        take.max(skip)
    };
    memo.insert((i, w), best);
    best
}

pub fn knapsack_memo(items: &[Item], capacity: usize) -> u64 {
    knapsack_memo_inner(items, items.len(), capacity, &mut HashMap::new())
}

// ── 2. Bottom-up tabulation with traceback ────────────────────────────────────

pub struct KnapsackResult {
    pub max_value: u64,
    pub selected: Vec<usize>,  // indices of selected items
}

pub fn knapsack_tab(items: &[Item], capacity: usize) -> KnapsackResult {
    let n = items.len();
    // dp[i][w] = max value using first i items with capacity w
    let mut dp = vec![vec![0u64; capacity + 1]; n + 1];

    for i in 1..=n {
        let item = &items[i - 1];
        for w in 0..=capacity {
            dp[i][w] = if item.weight > w {
                dp[i-1][w]
            } else {
                dp[i-1][w].max(item.value + dp[i-1][w - item.weight])
            };
        }
    }

    // Traceback: reconstruct which items were selected
    let mut selected = Vec::new();
    let mut w = capacity;
    for i in (1..=n).rev() {
        if dp[i][w] != dp[i-1][w] {
            selected.push(i - 1); // 0-indexed
            w = w.saturating_sub(items[i-1].weight);
        }
    }
    selected.reverse();

    KnapsackResult {
        max_value: dp[n][capacity],
        selected,
    }
}

// ── 3. Space-optimised (1D rolling array) ─────────────────────────────────────

pub fn knapsack_1d(items: &[Item], capacity: usize) -> u64 {
    let mut dp = vec![0u64; capacity + 1];
    for item in items {
        // Iterate right-to-left to avoid using an item twice
        for w in (item.weight..=capacity).rev() {
            dp[w] = dp[w].max(item.value + dp[w - item.weight]);
        }
    }
    dp[capacity]
}

fn main() {
    let items = vec![
        Item { name: "camera", weight: 2, value: 6  },
        Item { name: "laptop", weight: 2, value: 10 },
        Item { name: "guitar", weight: 3, value: 12 },
        Item { name: "book",   weight: 1, value: 4  },
        Item { name: "drone",  weight: 4, value: 15 },
    ];
    let capacity = 7;

    println!("Items: {} items, capacity={capacity}", items.len());
    for item in &items { println!("  {:8}: w={}, v={}", item.name, item.weight, item.value); }

    let memo_val = knapsack_memo(&items, capacity);
    println!("\nMemo  best value: {memo_val}");

    let result = knapsack_tab(&items, capacity);
    println!("Table best value: {}", result.max_value);
    println!("Selected items:");
    let mut total_w = 0;
    let mut total_v = 0u64;
    for &i in &result.selected {
        println!("  {:8}: w={}, v={}", items[i].name, items[i].weight, items[i].value);
        total_w += items[i].weight;
        total_v += items[i].value;
    }
    println!("Total weight: {total_w}/{capacity}, total value: {total_v}");

    let opt_val = knapsack_1d(&items, capacity);
    println!("\n1D (space-optimised) best value: {opt_val}");

    // All three methods agree
    assert_eq!(memo_val, result.max_value);
    assert_eq!(memo_val, opt_val);
    println!("\nAll three methods agree ✓");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_items() -> Vec<Item> {
        vec![
            Item { name: "a", weight: 2, value: 6  },
            Item { name: "b", weight: 2, value: 10 },
            Item { name: "c", weight: 3, value: 12 },
        ]
    }

    #[test]
    fn all_methods_agree() {
        let items = test_items();
        let cap = 5;
        let m = knapsack_memo(&items, cap);
        let t = knapsack_tab(&items, cap).max_value;
        let o = knapsack_1d(&items, cap);
        assert_eq!(m, t);
        assert_eq!(t, o);
    }

    #[test]
    fn empty_knapsack() {
        let items: Vec<Item> = vec![];
        assert_eq!(knapsack_1d(&items, 10), 0);
    }

    #[test]
    fn zero_capacity() {
        let items = test_items();
        assert_eq!(knapsack_1d(&items, 0), 0);
    }

    #[test]
    fn known_value() {
        let items = vec![
            Item { name: "x", weight: 1, value: 1 },
            Item { name: "y", weight: 1, value: 1 },
        ];
        // capacity=1: take one item, value=1
        assert_eq!(knapsack_1d(&items, 1), 1);
        // capacity=2: take both, value=2
        assert_eq!(knapsack_1d(&items, 2), 2);
    }
}
