//! # Bellman-Ford Algorithm

pub fn bellman_ford(n: usize, edges: &[(usize, usize, i32)], src: usize) -> Option<Vec<i32>> {
    let mut dist = vec![i32::MAX; n];
    dist[src] = 0;
    for _ in 0..n - 1 {
        for &(u, v, w) in edges {
            if dist[u] != i32::MAX && dist[u] + w < dist[v] {
                dist[v] = dist[u] + w;
            }
        }
    }
    for &(u, v, w) in edges {
        if dist[u] != i32::MAX && dist[u] + w < dist[v] {
            return None; // Negative cycle
        }
    }
    Some(dist)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_bellman() {
        let edges = [
            (0, 1, -1),
            (0, 2, 4),
            (1, 2, 3),
            (1, 3, 2),
            (1, 4, 2),
            (3, 2, 5),
            (3, 1, 1),
            (4, 3, -3),
        ];
        let dist = bellman_ford(5, &edges, 0).unwrap();
        // Shortest paths from 0: 0→1=-1, 0→1→4=1, 0→1→4→3=-2
        assert_eq!(dist[0], 0);
        assert_eq!(dist[1], -1);
        assert_eq!(dist[3], -2);
        assert_eq!(dist[4], 1);
    }
}
