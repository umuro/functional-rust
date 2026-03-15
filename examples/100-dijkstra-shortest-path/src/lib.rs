//! Dijkstra's Shortest Path — Functional Rust with BinaryHeap and immutable-style updates.
//!
//! Demonstrates how Rust's ownership model naturally prevents the aliasing bugs
//! that plague mutable graph algorithms in imperative languages.

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

/// A weighted edge in the graph.
#[derive(Clone, Debug)]
pub struct Edge {
    pub to: usize,
    pub weight: u64,
}

/// Wrapper for BinaryHeap to get min-heap behavior (Rust's BinaryHeap is max-heap).
#[derive(Clone, Eq, PartialEq)]
struct State {
    cost: u64,
    node: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse ordering for min-heap
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.node.cmp(&other.node))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Adjacency list representation of a weighted directed graph.
pub type Graph = HashMap<usize, Vec<Edge>>;

/// Build a graph from a list of (from, to, weight) tuples.
/// Functional style: fold over edges to construct the adjacency map.
pub fn build_graph(edges: &[(usize, usize, u64)]) -> Graph {
    edges
        .iter()
        .fold(HashMap::new(), |mut graph, &(from, to, weight)| {
            graph.entry(from).or_default().push(Edge { to, weight });
            graph
        })
}

/// Compute shortest distances from `source` to all reachable nodes.
///
/// Returns a HashMap<usize, u64> mapping each node to its shortest distance.
/// Unreachable nodes are absent from the result.
///
/// # Functional patterns used:
/// - **Fold-like iteration** over neighbors (functional accumulation)
/// - **Immutable result map** built incrementally
/// - **Pattern matching** on heap state
pub fn dijkstra(graph: &Graph, source: usize) -> HashMap<usize, u64> {
    let mut dist: HashMap<usize, u64> = HashMap::new();
    let mut heap = BinaryHeap::new();

    dist.insert(source, 0);
    heap.push(State {
        cost: 0,
        node: source,
    });

    while let Some(State { cost, node }) = heap.pop() {
        // Skip stale entries — functional equivalent of "already visited"
        if cost > *dist.get(&node).unwrap_or(&u64::MAX) {
            continue;
        }

        // Fold over neighbors: accumulate new distances
        if let Some(edges) = graph.get(&node) {
            for edge in edges {
                let new_dist = cost + edge.weight;
                let current = *dist.get(&edge.to).unwrap_or(&u64::MAX);

                if new_dist < current {
                    dist.insert(edge.to, new_dist);
                    heap.push(State {
                        cost: new_dist,
                        node: edge.to,
                    });
                }
            }
        }
    }

    dist
}

/// Reconstruct the shortest path from source to target.
/// Uses functional backtracking through the distance map.
pub fn shortest_path(graph: &Graph, source: usize, target: usize) -> Option<(u64, Vec<usize>)> {
    let dist = dijkstra(graph, source);
    let total_dist = *dist.get(&target)?;

    // Backtrack from target to source
    let mut path = vec![target];
    let mut current = target;

    while current != source {
        let prev = graph
            .iter()
            .flat_map(|(&from, edges)| {
                edges
                    .iter()
                    .filter(move |e| e.to == current)
                    .map(move |e| (from, e.weight))
            })
            .filter(|&(from, weight)| {
                dist.get(&from)
                    .is_some_and(|&d| d + weight == *dist.get(&current).unwrap())
            })
            .map(|(from, _)| from)
            .next()?;

        path.push(prev);
        current = prev;
    }

    path.reverse();
    Some((total_dist, path))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_graph() -> Graph {
        //  0 --1-- 1 --2-- 2
        //  |              |
        //  4              1
        //  |              |
        //  3 ------3----- 4
        build_graph(&[(0, 1, 1), (1, 2, 2), (0, 3, 4), (2, 4, 1), (3, 4, 3)])
    }

    #[test]
    fn test_source_distance_is_zero() {
        let g = sample_graph();
        let dist = dijkstra(&g, 0);
        assert_eq!(dist[&0], 0);
    }

    #[test]
    fn test_direct_neighbor() {
        let g = sample_graph();
        let dist = dijkstra(&g, 0);
        assert_eq!(dist[&1], 1);
    }

    #[test]
    fn test_two_hops() {
        let g = sample_graph();
        let dist = dijkstra(&g, 0);
        assert_eq!(dist[&2], 3); // 0->1 (1) + 1->2 (2) = 3
    }

    #[test]
    fn test_shortest_via_longer_path() {
        let g = sample_graph();
        let dist = dijkstra(&g, 0);
        assert_eq!(dist[&3], 4); // direct 0->3 = 4
    }

    #[test]
    fn test_shortest_to_distant_node() {
        let g = sample_graph();
        let dist = dijkstra(&g, 0);
        assert_eq!(dist[&4], 4); // 0->1->2->4 = 1+2+1 = 4 (shorter than 0->3->4 = 4+3 = 7)
    }

    #[test]
    fn test_unreachable_node() {
        let g = build_graph(&[(0, 1, 1)]);
        let dist = dijkstra(&g, 0);
        assert!(!dist.contains_key(&99));
    }

    #[test]
    fn test_single_node_graph() {
        let g = build_graph(&[]);
        let dist = dijkstra(&g, 0);
        assert_eq!(dist[&0], 0);
        assert_eq!(dist.len(), 1);
    }

    #[test]
    fn test_shortest_path_reconstruction() {
        let g = sample_graph();
        let (cost, path) = shortest_path(&g, 0, 4).unwrap();
        assert_eq!(cost, 4);
        assert_eq!(path, vec![0, 1, 2, 4]);
    }

    #[test]
    fn test_path_to_self() {
        let g = sample_graph();
        let (cost, path) = shortest_path(&g, 0, 0).unwrap();
        assert_eq!(cost, 0);
        assert_eq!(path, vec![0]);
    }

    #[test]
    fn test_empty_graph_isolation() {
        let g = build_graph(&[]);
        let result = shortest_path(&g, 0, 5);
        assert!(result.is_none());
    }
}
