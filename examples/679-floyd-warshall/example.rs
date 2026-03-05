// Floyd-Warshall
fn floyd_warshall(n: usize, edges: &[(usize, usize, i32)]) -> Vec<Vec<i32>> {
    let mut dist = vec![vec![i32::MAX / 2; n]; n];
    for i in 0..n { dist[i][i] = 0; }
    for &(u, v, w) in edges { dist[u][v] = w; }
    for k in 0..n {
        for i in 0..n {
            for j in 0..n { dist[i][j] = dist[i][j].min(dist[i][k] + dist[k][j]); }
        }
    }
    dist
}

fn main() {
    let edges = vec![(0, 1, 3), (1, 2, 1), (0, 2, 6)];
    let dist = floyd_warshall(3, &edges);
    println!("0 to 2: {}", dist[0][2]); // 4
}
