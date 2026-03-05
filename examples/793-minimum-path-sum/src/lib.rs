//! # Minimum Path Sum

pub fn min_path_sum(grid: &[Vec<i32>]) -> i32 {
    if grid.is_empty() || grid[0].is_empty() { return 0; }
    let (m, n) = (grid.len(), grid[0].len());
    let mut dp = vec![vec![0; n]; m];
    dp[0][0] = grid[0][0];
    for j in 1..n { dp[0][j] = dp[0][j-1] + grid[0][j]; }
    for i in 1..m { dp[i][0] = dp[i-1][0] + grid[i][0]; }
    for i in 1..m {
        for j in 1..n {
            dp[i][j] = grid[i][j] + dp[i-1][j].min(dp[i][j-1]);
        }
    }
    dp[m-1][n-1]
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn test_path() {
        let grid = vec![vec![1,3,1], vec![1,5,1], vec![4,2,1]];
        assert_eq!(min_path_sum(&grid), 7);
    }
}
