#![allow(clippy::all)]
// 1057: Matrix Chain Multiplication — Optimal Parenthesization

use std::collections::HashMap;

// Approach 1: Bottom-up DP
fn matrix_chain_dp(dims: &[usize]) -> usize {
    let n = dims.len() - 1;
    let mut dp = vec![vec![0usize; n]; n];
    for l in 2..=n {
        for i in 0..=(n - l) {
            let j = i + l - 1;
            dp[i][j] = usize::MAX;
            for k in i..j {
                let cost = dp[i][k] + dp[k + 1][j] + dims[i] * dims[k + 1] * dims[j + 1];
                dp[i][j] = dp[i][j].min(cost);
            }
        }
    }
    dp[0][n - 1]
}

// Approach 2: With parenthesization tracking
fn matrix_chain_parens(dims: &[usize]) -> (usize, String) {
    let n = dims.len() - 1;
    let mut dp = vec![vec![0usize; n]; n];
    let mut split = vec![vec![0usize; n]; n];
    for l in 2..=n {
        for i in 0..=(n - l) {
            let j = i + l - 1;
            dp[i][j] = usize::MAX;
            for k in i..j {
                let cost = dp[i][k] + dp[k + 1][j] + dims[i] * dims[k + 1] * dims[j + 1];
                if cost < dp[i][j] {
                    dp[i][j] = cost;
                    split[i][j] = k;
                }
            }
        }
    }
    fn build(i: usize, j: usize, split: &[Vec<usize>]) -> String {
        if i == j {
            format!("A{}", i + 1)
        } else {
            format!(
                "({}*{})",
                build(i, split[i][j], split),
                build(split[i][j] + 1, j, split)
            )
        }
    }
    (dp[0][n - 1], build(0, n - 1, &split))
}

// Approach 3: Recursive with memoization
fn matrix_chain_memo(dims: &[usize]) -> usize {
    fn solve(
        i: usize,
        j: usize,
        dims: &[usize],
        cache: &mut HashMap<(usize, usize), usize>,
    ) -> usize {
        if i == j {
            return 0;
        }
        if let Some(&v) = cache.get(&(i, j)) {
            return v;
        }
        let mut best = usize::MAX;
        for k in i..j {
            let cost = solve(i, k, dims, cache)
                + solve(k + 1, j, dims, cache)
                + dims[i] * dims[k + 1] * dims[j + 1];
            best = best.min(cost);
        }
        cache.insert((i, j), best);
        best
    }
    let mut cache = HashMap::new();
    solve(0, dims.len() - 2, dims, &mut cache)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix_chain_dp() {
        assert_eq!(matrix_chain_dp(&[30, 35, 15, 5, 10, 20, 25]), 15125);
        assert_eq!(matrix_chain_dp(&[10, 20, 30, 40]), 18000);
    }

    #[test]
    fn test_matrix_chain_parens() {
        let (cost, parens) = matrix_chain_parens(&[30, 35, 15, 5, 10, 20, 25]);
        assert_eq!(cost, 15125);
        assert!(!parens.is_empty());
    }

    #[test]
    fn test_matrix_chain_memo() {
        assert_eq!(matrix_chain_memo(&[30, 35, 15, 5, 10, 20, 25]), 15125);
        assert_eq!(matrix_chain_memo(&[10, 20, 30, 40]), 18000);
    }
}
