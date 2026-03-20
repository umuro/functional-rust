//! Example 1170: Dijkstra's Shortest Path — Priority Queue
//!
//! Demonstrates Dijkstra's algorithm using a binary heap (min-heap via `Reverse`)
//! and BTreeMap, mirroring the OCaml functional priority queue approach.

use std::cmp::Reverse;
use std::collections::{BTreeMap, BinaryHeap};

/// Type alias: adjacency list graph with string node names and integer weights.
pub type Graph = BTreeMap<String, Vec<(String, u32)>>;

/// Solution 1: Idiomatic Rust — BinaryHeap with Reverse for min-heap behaviour.
///
/// Returns a map from node name to shortest distance from `start`.
/// Nodes unreachable from `start` are absent from the result.
pub fn dijkstra(graph: &Graph, start: &str) -> BTreeMap<String, u32> {
    // dist maps node -> best known distance
    let mut dist: BTreeMap<String, u32> = BTreeMap::new();
    dist.insert(start.to_owned(), 0);

    // BinaryHeap is a max-heap; wrap in Reverse to get min-heap on distance.
    // Tuple ordering: (Reverse(dist), node)
    let mut heap: BinaryHeap<(Reverse<u32>, String)> = BinaryHeap::new();
    heap.push((Reverse(0), start.to_owned()));

    while let Some((Reverse(d), u)) = heap.pop() {
        // If we've already found a better path, skip this stale entry.
        if dist.get(&u).is_some_and(|&best| d > best) {
            continue;
        }

        let neighbors = graph.get(&u).map(Vec::as_slice).unwrap_or(&[]);
        for (v, w) in neighbors {
            let alt = d + w;
            let current = dist.get(v).copied().unwrap_or(u32::MAX);
            if alt < current {
                dist.insert(v.clone(), alt);
                heap.push((Reverse(alt), v.clone()));
            }
        }
    }

    dist
}

/// Solution 2: Functional/recursive — mirrors the OCaml `go pq dist` tail recursion.
///
/// Uses a sorted `BTreeMap` as a functional priority queue (keyed by `(dist, node)`),
/// matching OCaml's `Set.Make` approach where `min_elt` gives the cheapest entry.
pub fn dijkstra_recursive(graph: &Graph, start: &str) -> BTreeMap<String, u32> {
    let mut dist = BTreeMap::new();
    dist.insert(start.to_owned(), 0u32);

    // pq: BTreeMap<(dist, node), ()> — ordered so first key is cheapest.
    let mut pq: BTreeMap<(u32, String), ()> = BTreeMap::new();
    pq.insert((0, start.to_owned()), ());

    go(graph, pq, dist)
}

/// Recursive driver that processes the priority queue functionally.
fn go(
    graph: &Graph,
    mut pq: BTreeMap<(u32, String), ()>,
    dist: BTreeMap<String, u32>,
) -> BTreeMap<String, u32> {
    // Base case: empty queue → done
    let Some(((d, u), _)) = pq.pop_first() else {
        return dist;
    };

    // Stale entry guard
    if dist.get(&u).is_some_and(|&best| d > best) {
        return go(graph, pq, dist);
    }

    let neighbors = graph.get(&u).map(Vec::as_slice).unwrap_or(&[]);

    // Fold over neighbours — mirrors OCaml's List.fold_left
    let (dist, pq) = neighbors
        .iter()
        .fold((dist, pq), |(mut d_map, mut q), (v, w)| {
            let alt = d + w;
            let current = d_map.get(v).copied().unwrap_or(u32::MAX);
            if alt < current {
                d_map.insert(v.clone(), alt);
                q.insert((alt, v.clone()), ());
            }
            (d_map, q)
        });

    go(graph, pq, dist)
}

// ---------------------------------------------------------------------------
// Helpers for building graphs in tests
// ---------------------------------------------------------------------------

/// Convenience builder: constructs a `Graph` from a slice of `(node, neighbours)`.
pub fn build_graph(edges: &[(&str, &[(&str, u32)])]) -> Graph {
    edges
        .iter()
        .map(|(node, neighbours)| {
            let ns = neighbours
                .iter()
                .map(|(n, w)| ((*n).to_owned(), *w))
                .collect();
            ((*node).to_owned(), ns)
        })
        .collect()
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_graph() -> Graph {
        build_graph(&[
            ("a", &[("b", 1), ("c", 4)]),
            ("b", &[("c", 2), ("d", 6)]),
            ("c", &[("d", 3)]),
            ("d", &[]),
        ])
    }

    #[test]
    fn test_dijkstra_distances_from_a() {
        let g = sample_graph();
        let dist = dijkstra(&g, "a");
        assert_eq!(dist["a"], 0);
        assert_eq!(dist["b"], 1);
        assert_eq!(dist["c"], 3); // a→b→c = 1+2
        assert_eq!(dist["d"], 6); // a→b→c→d = 1+2+3
    }

    #[test]
    fn test_dijkstra_recursive_matches_idiomatic() {
        let g = sample_graph();
        let d1 = dijkstra(&g, "a");
        let d2 = dijkstra_recursive(&g, "a");
        assert_eq!(d1, d2);
    }

    #[test]
    fn test_start_node_distance_is_zero() {
        let g = sample_graph();
        for start in ["a", "b", "c", "d"] {
            let dist = dijkstra(&g, start);
            assert_eq!(dist[start], 0, "start node {start} must have distance 0");
        }
    }

    #[test]
    fn test_unreachable_node_absent() {
        // d has no outgoing edges, so starting from d only reaches d itself.
        let g = sample_graph();
        let dist = dijkstra(&g, "d");
        assert_eq!(dist.len(), 1);
        assert_eq!(dist["d"], 0);
    }

    #[test]
    fn test_single_node_graph() {
        let g = build_graph(&[("x", &[])]);
        let dist = dijkstra(&g, "x");
        assert_eq!(dist["x"], 0);
        assert_eq!(dist.len(), 1);
    }

    #[test]
    fn test_direct_vs_indirect_path() {
        // a→c direct costs 10; a→b→c costs 1+2=3 — algorithm must pick shorter.
        let g = build_graph(&[
            ("a", &[("b", 1), ("c", 10)]),
            ("b", &[("c", 2)]),
            ("c", &[]),
        ]);
        let dist = dijkstra(&g, "a");
        assert_eq!(dist["c"], 3);
    }
}
