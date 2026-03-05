//! # Bellman-Ford Algorithm
//!
//! Shortest path with negative weights. Detects negative cycles.
//! Time: O(V*E), Space: O(V)

pub fn bellman_ford(edges: &[(usize, usize, i32)], n: usize, start: usize) -> Option<Vec<i32>> {
    let mut dist = vec![i32::MAX; n];
    dist[start] = 0;
    
    // Relax V-1 times
    for _ in 0..n - 1 {
        for &(u, v, w) in edges {
            if dist[u] != i32::MAX && dist[u] + w < dist[v] {
                dist[v] = dist[u] + w;
            }
        }
    }
    
    // Check for negative cycle
    for &(u, v, w) in edges {
        if dist[u] != i32::MAX && dist[u] + w < dist[v] {
            return None; // Negative cycle detected
        }
    }
    
    Some(dist)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bellman_ford() {
        let edges = vec![(0, 1, 4), (0, 2, 5), (1, 2, -3), (2, 3, 4)];
        let dist = bellman_ford(&edges, 4, 0).unwrap();
        assert_eq!(dist[2], 1); // 0->1->2 = 4-3 = 1
        assert_eq!(dist[3], 5);
    }

    #[test]
    fn test_negative_cycle() {
        let edges = vec![(0, 1, 1), (1, 2, -1), (2, 0, -1)];
        assert!(bellman_ford(&edges, 3, 0).is_none());
    }
}
