#![allow(dead_code)]

use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
pub struct Item {
    pub weight: usize,
    pub value: u64,
}

pub fn knapsack(items: &[Item], capacity: usize) -> u64 {
    let mut memo: HashMap<(usize, usize), u64> = HashMap::new();
    solve(items, 0, capacity, &mut memo)
}

fn solve(items: &[Item], i: usize, cap: usize,
         memo: &mut HashMap<(usize, usize), u64>) -> u64 {
    if i >= items.len() || cap == 0 { return 0; }
    if let Some(&cached) = memo.get(&(i, cap)) { return cached; }
    let item = items[i];
    let without = solve(items, i + 1, cap, memo);
    let with_item = if item.weight <= cap {
        item.value + solve(items, i + 1, cap - item.weight, memo)
    } else { 0 };
    let best = without.max(with_item);
    memo.insert((i, cap), best);
    best
}

fn main() {
    let items = vec![
        Item { weight: 2, value: 3 },
        Item { weight: 3, value: 4 },
        Item { weight: 4, value: 5 },
        Item { weight: 5, value: 6 },
    ];
    println!("Items: {:?}", items);
    for cap in [5, 8, 10, 14] {
        println!("  capacity={}: max_value={}", cap, knapsack(&items, cap));
    }
}

/* Output:
   Items: [Item { weight: 2, value: 3 }, Item { weight: 3, value: 4 }, ...]
     capacity=5: max_value=7
     capacity=8: max_value=10
     capacity=10: max_value=13
     capacity=14: max_value=18
*/
