// Minimum Path Sum — in-place DP O(m×n), O(1) extra space

fn min_path_sum(grid: &[Vec<u64>]) -> u64 {
    let m = grid.len();
    if m == 0 { return 0; }
    let n = grid[0].len();
    let mut g: Vec<Vec<u64>> = grid.to_vec();

    for j in 1..n { g[0][j] += g[0][j - 1]; }
    for i in 1..m { g[i][0] += g[i - 1][0]; }
    for i in 1..m {
        for j in 1..n {
            g[i][j] += g[i - 1][j].min(g[i][j - 1]);
        }
    }
    g[m - 1][n - 1]
}

fn min_path_reconstruct(grid: &[Vec<u64>]) -> (Vec<(usize, usize)>, u64) {
    let m = grid.len();
    let n = grid[0].len();
    let mut g: Vec<Vec<u64>> = grid.to_vec();
    for j in 1..n { g[0][j] += g[0][j - 1]; }
    for i in 1..m { g[i][0] += g[i - 1][0]; }
    for i in 1..m {
        for j in 1..n {
            g[i][j] += g[i - 1][j].min(g[i][j - 1]);
        }
    }

    let mut path = Vec::new();
    let (mut i, mut j) = (m - 1, n - 1);
    loop {
        path.push((i, j));
        if i == 0 && j == 0 { break; }
        if i == 0 { j -= 1; }
        else if j == 0 { i -= 1; }
        else if g[i - 1][j] < g[i][j - 1] { i -= 1; }
        else { j -= 1; }
    }
    path.reverse();
    (path, g[m - 1][n - 1])
}

fn main() {
    let grid = vec![
        vec![1u64, 3, 1],
        vec![1, 5, 1],
        vec![4, 2, 1],
    ];
    println!("Min path sum: {}", min_path_sum(&grid));
    let (path, cost) = min_path_reconstruct(&grid);
    println!("Path (cost={cost}): {:?}", path);
}
