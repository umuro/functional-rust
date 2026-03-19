#![allow(clippy::all)]
// 1053: Coin Change — Minimum Coins for Amount

use std::collections::HashMap;

// Approach 1: Bottom-up DP
fn coin_change_dp(coins: &[i32], amount: i32) -> i32 {
    let amount = amount as usize;
    let max_val = amount + 1;
    let mut dp = vec![max_val; amount + 1];
    dp[0] = 0;
    for i in 1..=amount {
        for &coin in coins {
            let c = coin as usize;
            if c <= i && dp[i - c] + 1 < dp[i] {
                dp[i] = dp[i - c] + 1;
            }
        }
    }
    if dp[amount] > amount {
        -1
    } else {
        dp[amount] as i32
    }
}

// Approach 2: Recursive with HashMap memoization
fn coin_change_memo(coins: &[i32], amount: i32) -> i32 {
    fn solve(coins: &[i32], amt: i32, cache: &mut HashMap<i32, i32>) -> i32 {
        if amt == 0 {
            return 0;
        }
        if amt < 0 {
            return i32::MAX;
        }
        if let Some(&v) = cache.get(&amt) {
            return v;
        }
        let result = coins.iter().fold(i32::MAX, |best, &coin| {
            let sub = solve(coins, amt - coin, cache);
            if sub < i32::MAX {
                best.min(sub + 1)
            } else {
                best
            }
        });
        cache.insert(amt, result);
        result
    }
    let mut cache = HashMap::new();
    let r = solve(coins, amount, &mut cache);
    if r == i32::MAX {
        -1
    } else {
        r
    }
}

// Approach 3: BFS (shortest path interpretation)
fn coin_change_bfs(coins: &[i32], amount: i32) -> i32 {
    if amount == 0 {
        return 0;
    }
    let mut visited = vec![false; amount as usize + 1];
    let mut queue = std::collections::VecDeque::new();
    queue.push_back((0i32, 0i32));
    visited[0] = true;
    while let Some((current, steps)) = queue.pop_front() {
        for &coin in coins {
            let next = current + coin;
            if next == amount {
                return steps + 1;
            }
            if next < amount && !visited[next as usize] {
                visited[next as usize] = true;
                queue.push_back((next, steps + 1));
            }
        }
    }
    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coin_change_dp() {
        assert_eq!(coin_change_dp(&[1, 5, 10, 25], 30), 2);
        assert_eq!(coin_change_dp(&[1, 5, 10, 25], 11), 2);
        assert_eq!(coin_change_dp(&[2], 3), -1);
        assert_eq!(coin_change_dp(&[1], 0), 0);
        assert_eq!(coin_change_dp(&[1, 2, 5], 11), 3);
    }

    #[test]
    fn test_coin_change_memo() {
        assert_eq!(coin_change_memo(&[1, 5, 10, 25], 30), 2);
        assert_eq!(coin_change_memo(&[1, 5, 10, 25], 11), 2);
        assert_eq!(coin_change_memo(&[2], 3), -1);
        assert_eq!(coin_change_memo(&[1], 0), 0);
        assert_eq!(coin_change_memo(&[1, 2, 5], 11), 3);
    }

    #[test]
    fn test_coin_change_bfs() {
        assert_eq!(coin_change_bfs(&[1, 5, 10, 25], 30), 2);
        assert_eq!(coin_change_bfs(&[1, 5, 10, 25], 11), 2);
        assert_eq!(coin_change_bfs(&[2], 3), -1);
        assert_eq!(coin_change_bfs(&[1], 0), 0);
        assert_eq!(coin_change_bfs(&[1, 2, 5], 11), 3);
    }
}
