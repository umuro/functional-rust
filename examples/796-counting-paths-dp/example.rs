// Counting Paths in Grid with Obstacles — DP O(m×n)
// grid[i][j] = 0 open, 1 blocked

fn count_paths(grid: &[Vec<u8>]) -> u64 {
    let m = grid.len();
    if m == 0 { return 0; }
    let n = grid[0].len();
    if grid[0][0] == 1 || grid[m - 1][n - 1] == 1 { return 0; }

    let mut dp = vec![vec![0u64; n]; m];
    dp[0][0] = 1;

    for j in 1..n { dp[0][j] = if grid[0][j] == 0 { dp[0][j - 1] } else { 0 }; }
    for i in 1..m { dp[i][0] = if grid[i][0] == 0 { dp[i - 1][0] } else { 0 }; }
    for i in 1..m {
        for j in 1..n {
            dp[i][j] = if grid[i][j] == 1 { 0 } else { dp[i - 1][j] + dp[i][j - 1] };
        }
    }
    dp[m - 1][n - 1]
}

fn main() {
    let grid1 = vec![vec![0u8,0,0], vec![0,1,0], vec![0,0,0]];
    println!("3×3 center obstacle:    {}", count_paths(&grid1));

    let grid2 = vec![vec![0u8,0,0], vec![0,0,0], vec![0,0,0]];
    println!("3×3 open:               {}", count_paths(&grid2));

    let grid3 = vec![vec![0u8,1], vec![0,0]];
    println!("2×2 [[0,1],[0,0]]:      {}", count_paths(&grid3));
}
