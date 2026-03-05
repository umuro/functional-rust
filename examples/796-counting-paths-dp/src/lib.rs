//! # Counting Paths

pub fn unique_paths(m: usize, n: usize) -> usize {
    let mut dp = vec![vec![1; n]; m];
    for i in 1..m {
        for j in 1..n {
            dp[i][j] = dp[i-1][j] + dp[i][j-1];
        }
    }
    dp[m-1][n-1]
}

pub fn unique_paths_obstacles(grid: &[Vec<i32>]) -> usize {
    if grid.is_empty() || grid[0][0] == 1 { return 0; }
    let (m, n) = (grid.len(), grid[0].len());
    let mut dp = vec![vec![0usize; n]; m];
    dp[0][0] = 1;
    for j in 1..n { dp[0][j] = if grid[0][j] == 1 { 0 } else { dp[0][j-1] }; }
    for i in 1..m { dp[i][0] = if grid[i][0] == 1 { 0 } else { dp[i-1][0] }; }
    for i in 1..m {
        for j in 1..n {
            dp[i][j] = if grid[i][j] == 1 { 0 } else { dp[i-1][j] + dp[i][j-1] };
        }
    }
    dp[m-1][n-1]
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn test_paths() { assert_eq!(unique_paths(3, 7), 28); }
}
