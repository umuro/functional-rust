//! # Floyd-Warshall Algorithm
//!
//! All-pairs shortest path. Time: O(V³), Space: O(V²)

/// Floyd-Warshall all-pairs shortest path
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

/// Detect negative cycle
pub fn has_negative_cycle(dist: &[Vec<i32>]) -> bool {
    dist.iter().enumerate().any(|(i, row)| row[i] < 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_floyd_warshall() {
        let edges = vec![(0, 1, 3), (1, 2, 1), (0, 2, 6), (2, 0, 2)];
        let dist = floyd_warshall(3, &edges);
        assert_eq!(dist[0][2], 4); // 0->1->2
        assert_eq!(dist[2][1], 5); // 2->0->1
    }
}
