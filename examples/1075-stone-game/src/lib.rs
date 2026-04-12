#![allow(dead_code)]
#![allow(clippy::all)]
// 1075: Stone Game — Minimax DP

use std::collections::HashMap;

// Approach 1: Bottom-up interval DP
fn stone_game_dp(piles: &[i32]) -> bool {
    let n = piles.len();
    // dp[i][j] = max score difference (current player - opponent) for piles[i..=j]
    let mut dp = vec![vec![0i32; n]; n];
    for i in 0..n {
        dp[i][i] = piles[i];
    }
    for len in 2..=n {
        for i in 0..=n - len {
            let j = i + len - 1;
            dp[i][j] = (piles[i] - dp[i + 1][j]).max(piles[j] - dp[i][j - 1]);
        }
    }
    dp[0][n - 1] > 0
}

// Approach 2: Recursive with memoization
fn stone_game_memo(piles: &[i32]) -> bool {
    fn solve(i: usize, j: usize, piles: &[i32], cache: &mut HashMap<(usize, usize), i32>) -> i32 {
        if i > j {
            return 0;
        }
        if i == j {
            return piles[i];
        }
        if let Some(&v) = cache.get(&(i, j)) {
            return v;
        }
        let v = (piles[i] - solve(i + 1, j, piles, cache))
            .max(piles[j] - solve(i, j - 1, piles, cache));
        cache.insert((i, j), v);
        v
    }
    let mut cache = HashMap::new();
    solve(0, piles.len() - 1, piles, &mut cache) > 0
}

// Approach 3: Mathematical — first player always wins with even piles
fn stone_game_math(_piles: &[i32]) -> bool {
    // With even number of piles, first player can always choose
    // all odd-indexed or all even-indexed piles (by choosing first/last).
    // One of those sums is strictly greater, so first player always wins.
    true
}

// Bonus: compute actual scores
fn stone_game_scores(piles: &[i32]) -> (i32, i32) {
    let n = piles.len();
    let total: i32 = piles.iter().sum();
    let mut dp = vec![vec![0i32; n]; n];
    for i in 0..n {
        dp[i][i] = piles[i];
    }
    for len in 2..=n {
        for i in 0..=n - len {
            let j = i + len - 1;
            dp[i][j] = (piles[i] - dp[i + 1][j]).max(piles[j] - dp[i][j - 1]);
        }
    }
    let diff = dp[0][n - 1];
    let p1 = (total + diff) / 2;
    let p2 = total - p1;
    (p1, p2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dp() {
        assert!(stone_game_dp(&[5, 3, 4, 5]));
        assert!(stone_game_dp(&[3, 7, 2, 3]));
    }

    #[test]
    fn test_memo() {
        assert!(stone_game_memo(&[5, 3, 4, 5]));
        assert!(stone_game_memo(&[3, 7, 2, 3]));
    }

    #[test]
    fn test_math() {
        assert!(stone_game_math(&[5, 3, 4, 5]));
    }

    #[test]
    fn test_scores() {
        let (p1, p2) = stone_game_scores(&[5, 3, 4, 5]);
        assert!(p1 > p2);
        assert_eq!(p1 + p2, 17);
    }
}
