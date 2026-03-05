//! # Floyd-Warshall Algorithm

pub fn floyd_warshall(n: usize, edges: &[(usize, usize, i32)]) -> Vec<Vec<i32>> {
    let mut dist = vec![vec![i32::MAX / 2; n]; n];
    for i in 0..n { dist[i][i] = 0; }
    for &(u, v, w) in edges { dist[u][v] = w; }
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                if dist[i][k] + dist[k][j] < dist[i][j] {
                    dist[i][j] = dist[i][k] + dist[k][j];
                }
            }
        }
    }
    dist
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn test_floyd() {
        let edges = [(0, 1, 3), (0, 2, 8), (1, 2, 2), (2, 0, 5)];
        let dist = floyd_warshall(3, &edges);
        assert_eq!(dist[0][2], 5);
    }
}
