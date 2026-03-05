//! Weighted Graph and Dijkstra's Algorithm
//!
//! Shortest paths with non-negative edge weights.

use std::cmp::Reverse;
use std::collections::BinaryHeap;

/// Dijkstra's shortest path algorithm
/// Returns distances from src to all vertices
pub fn dijkstra(adj: &[Vec<(usize, u64)>], n: usize, src: usize) -> Vec<u64> {
    let mut dist = vec![u64::MAX; n];
    dist[src] = 0;
    let mut heap = BinaryHeap::new();
    heap.push(Reverse((0u64, src)));

    while let Some(Reverse((d, u))) = heap.pop() {
        if d > dist[u] {
            continue;
        }
        for &(v, w) in &adj[u] {
            let nd = dist[u].saturating_add(w);
            if nd < dist[v] {
                dist[v] = nd;
                heap.push(Reverse((nd, v)));
            }
        }
    }
    dist
}

/// Dijkstra with path reconstruction
pub fn dijkstra_with_path(
    adj: &[Vec<(usize, u64)>],
    n: usize,
    src: usize,
    dst: usize,
) -> (u64, Vec<usize>) {
    let mut dist = vec![u64::MAX; n];
    let mut prev = vec![usize::MAX; n];
    dist[src] = 0;
    let mut heap = BinaryHeap::new();
    heap.push(Reverse((0u64, src)));

    while let Some(Reverse((d, u))) = heap.pop() {
        if d > dist[u] {
            continue;
        }
        if u == dst {
            break;
        }
        for &(v, w) in &adj[u] {
            let nd = dist[u].saturating_add(w);
            if nd < dist[v] {
                dist[v] = nd;
                prev[v] = u;
                heap.push(Reverse((nd, v)));
            }
        }
    }

    // Reconstruct path
    let mut path = Vec::new();
    if dist[dst] != u64::MAX {
        let mut cur = dst;
        while cur != usize::MAX {
            path.push(cur);
            cur = prev[cur];
        }
        path.reverse();
    }
    (dist[dst], path)
}

/// Bellman-Ford for graphs with negative weights
pub fn bellman_ford(edges: &[(usize, usize, i64)], n: usize, src: usize) -> Option<Vec<i64>> {
    let mut dist = vec![i64::MAX; n];
    dist[src] = 0;

    for _ in 0..n - 1 {
        for &(u, v, w) in edges {
            if dist[u] != i64::MAX && dist[u] + w < dist[v] {
                dist[v] = dist[u] + w;
            }
        }
    }

    // Check for negative cycles
    for &(u, v, w) in edges {
        if dist[u] != i64::MAX && dist[u] + w < dist[v] {
            return None; // negative cycle
        }
    }
    Some(dist)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dijkstra() {
        let mut adj = vec![vec![]; 4];
        adj[0].push((1, 1u64));
        adj[0].push((2, 4));
        adj[1].push((2, 2));
        adj[1].push((3, 5));
        adj[2].push((3, 1));
        let dist = dijkstra(&adj, 4, 0);
        assert_eq!(dist[0], 0);
        assert_eq!(dist[1], 1);
        assert_eq!(dist[2], 3);
        assert_eq!(dist[3], 4);
    }

    #[test]
    fn test_unreachable() {
        let adj = vec![vec![(1, 1u64)], vec![], vec![]];
        let dist = dijkstra(&adj, 3, 0);
        assert_eq!(dist[2], u64::MAX);
    }

    #[test]
    fn test_path_reconstruction() {
        let mut adj = vec![vec![]; 4];
        adj[0].push((1, 1u64));
        adj[1].push((2, 1));
        adj[2].push((3, 1));
        let (dist, path) = dijkstra_with_path(&adj, 4, 0, 3);
        assert_eq!(dist, 3);
        assert_eq!(path, vec![0, 1, 2, 3]);
    }

    #[test]
    fn test_bellman_ford() {
        let edges = vec![(0, 1, 4i64), (0, 2, 5), (1, 2, -3), (2, 3, 4)];
        let dist = bellman_ford(&edges, 4, 0).unwrap();
        assert_eq!(dist[0], 0);
        assert_eq!(dist[2], 1); // 0->1->2: 4-3=1
    }

    #[test]
    fn test_negative_cycle() {
        let edges = vec![(0, 1, 1i64), (1, 2, -1), (2, 0, -1)];
        assert!(bellman_ford(&edges, 3, 0).is_none());
    }
}
