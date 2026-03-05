// 788. Coin Change: Minimum Coins DP
// Top-down memo + bottom-up tabulation + counting ways

use std::collections::HashMap;

// ── 1. Top-down memoisation ────────────────────────────────────────────────────

fn coin_change_memo_inner(
    coins: &[u64],
    amount: u64,
    memo: &mut HashMap<u64, Option<u64>>,
) -> Option<u64> {
    if amount == 0 { return Some(0); }
    if let Some(&cached) = memo.get(&amount) { return cached; }
    let best = coins.iter()
        .filter(|&&c| c <= amount)
        .filter_map(|&c| coin_change_memo_inner(coins, amount - c, memo).map(|sub| sub + 1))
        .min();
    memo.insert(amount, best);
    best
}

pub fn coin_change_memo(coins: &[u64], amount: u64) -> Option<u64> {
    coin_change_memo_inner(coins, amount, &mut HashMap::new())
}

// ── 2. Bottom-up tabulation ───────────────────────────────────────────────────

pub fn coin_change_tab(coins: &[u64], amount: usize) -> Option<u64> {
    let mut dp = vec![u64::MAX; amount + 1];
    dp[0] = 0;

    for i in 1..=amount {
        for &coin in coins {
            let coin = coin as usize;
            if coin <= i && dp[i - coin] != u64::MAX {
                dp[i] = dp[i].min(dp[i - coin] + 1);
            }
        }
    }

    if dp[amount] == u64::MAX { None } else { Some(dp[amount]) }
}

// ── 3. Count distinct ways ────────────────────────────────────────────────────

pub fn count_ways(coins: &[u64], amount: usize) -> u64 {
    let mut dp = vec![0u64; amount + 1];
    dp[0] = 1;
    for &coin in coins {
        for i in coin as usize..=amount {
            dp[i] = dp[i].saturating_add(dp[i - coin as usize]);
        }
    }
    dp[amount]
}

// ── 4. Reconstruct coin selection ─────────────────────────────────────────────

pub fn coin_change_with_coins(coins: &[u64], amount: usize) -> Option<Vec<u64>> {
    let mut dp = vec![u64::MAX; amount + 1];
    let mut last_coin = vec![0u64; amount + 1];
    dp[0] = 0;

    for i in 1..=amount {
        for &coin in coins {
            let coin_usize = coin as usize;
            if coin_usize <= i && dp[i - coin_usize] != u64::MAX {
                let candidate = dp[i - coin_usize] + 1;
                if candidate < dp[i] {
                    dp[i] = candidate;
                    last_coin[i] = coin;
                }
            }
        }
    }

    if dp[amount] == u64::MAX { return None; }

    // Reconstruct
    let mut result = Vec::new();
    let mut remaining = amount;
    while remaining > 0 {
        let coin = last_coin[remaining];
        result.push(coin);
        remaining -= coin as usize;
    }
    Some(result)
}

fn main() {
    let coins: &[u64] = &[1, 5, 10, 25];
    let amounts = [0, 11, 30, 41, 100, 3];

    println!("{:>10} {:>12} {:>12} {:>12}", "Amount", "MinCoins", "Ways", "Coins");
    println!("{}", "-".repeat(55));
    for &amt in &amounts {
        let min_coins = coin_change_tab(coins, amt);
        let ways = count_ways(coins, amt);
        let selection = coin_change_with_coins(coins, amt);
        print!("{amt:>10} ");
        match min_coins {
            Some(n) => print!("{n:>12} "),
            None    => print!("{:>12} ", "impossible"),
        }
        print!("{ways:>12} ");
        match selection {
            Some(s) => println!("{s:?}"),
            None    => println!("-"),
        }
    }

    // Verify memo == tab
    println!("\nVerifying memo == tab:");
    for &amt in &amounts {
        let m = coin_change_memo(coins, amt as u64);
        let t = coin_change_tab(coins, amt);
        assert_eq!(m, t, "mismatch at amount={amt}");
        println!("  amount={amt}: {m:?} ✓");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const COINS: &[u64] = &[1, 5, 10, 25];

    #[test]
    fn zero_amount() {
        assert_eq!(coin_change_tab(COINS, 0), Some(0));
        assert_eq!(count_ways(COINS, 0), 1);
    }

    #[test]
    fn known_values() {
        assert_eq!(coin_change_tab(COINS, 11), Some(2));   // 10+1
        assert_eq!(coin_change_tab(COINS, 30), Some(2));   // 25+5
        assert_eq!(coin_change_tab(COINS, 41), Some(4));   // 25+10+5+1
    }

    #[test]
    fn impossible_case() {
        let coins: &[u64] = &[2];
        assert_eq!(coin_change_tab(coins, 3), None); // 3 cannot be made with only 2s
    }

    #[test]
    fn memo_matches_tab() {
        for amt in 0..=50usize {
            let m = coin_change_memo(COINS, amt as u64);
            let t = coin_change_tab(COINS, amt);
            assert_eq!(m, t, "amount={amt}");
        }
    }

    #[test]
    fn reconstruction_correct() {
        let coins = coin_change_with_coins(COINS, 41).unwrap();
        let total: u64 = coins.iter().sum();
        assert_eq!(total, 41);
        assert_eq!(coins.len(), 4); // 25+10+5+1
    }

    #[test]
    fn count_ways_11() {
        // 11¢: 10+1, 5+5+1, 5+1+1+1+1+1, 1×11
        assert_eq!(count_ways(COINS, 11), 4);
    }
}
