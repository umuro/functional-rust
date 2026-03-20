#![allow(clippy::all)]
//! # Matrix Chain Multiplication

pub fn matrix_chain(dims: &[usize]) -> usize {
    let n = dims.len() - 1;
    if n <= 1 {
        return 0;
    }
    let mut dp = vec![vec![0; n]; n];
    for len in 2..=n {
        for i in 0..=n - len {
            let j = i + len - 1;
            dp[i][j] = usize::MAX;
            for k in i..j {
                let cost = dp[i][k] + dp[k + 1][j] + dims[i] * dims[k + 1] * dims[j + 1];
                dp[i][j] = dp[i][j].min(cost);
            }
        }
    }
    dp[0][n - 1]
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_matrix_chain() {
        assert_eq!(matrix_chain(&[10, 20, 30, 40, 30]), 30000);
    }
    #[test]
    fn test_two() {
        assert_eq!(matrix_chain(&[10, 20, 30]), 6000);
    }
}
