//! Dijkstra's shortest path algorithm using a BinaryHeap priority queue.
//!
//! OCaml uses a functional Set (ordered tree) as a priority queue.
//! Rust uses BinaryHeap<Reverse<...>> for the same min-heap semantics.

use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

/// Graph: adjacency list mapping node name → list of (neighbor, weight).
pub type Graph<'a> = HashMap<&'a str, Vec<(&'a str, u32)>>;

/// Idiomatic Rust: BinaryHeap with Reverse for min-heap, HashMap for distances.
///
/// Returns a map from node → shortest distance from `start`.
/// Nodes unreachable from `start` are absent from the result.
pub fn dijkstra<'a>(graph: &'a Graph, start: &'a str) -> HashMap<&'a str, u32> {
    let mut dist: HashMap<&str, u32> = HashMap::new();
    dist.insert(start, 0);

    // BinaryHeap is a max-heap; Reverse turns it into a min-heap.
    let mut heap: BinaryHeap<Reverse<(u32, &str)>> = BinaryHeap::new();
    heap.push(Reverse((0, start)));

    while let Some(Reverse((d, u))) = heap.pop() {
        // Skip stale entries (we may push duplicates when we find a shorter path)
        if dist.get(u).copied().unwrap_or(u32::MAX) < d {
            continue;
        }

        let neighbors = graph.get(u).map(Vec::as_slice).unwrap_or(&[]);
        for &(v, w) in neighbors {
            let alt = d + w;
            let current = dist.get(v).copied().unwrap_or(u32::MAX);
            if alt < current {
                dist.insert(v, alt);
                heap.push(Reverse((alt, v)));
            }
        }
    }

    dist
}

/// Functional style: tail-recursive via an explicit stack (Vec as worklist),
/// mirroring OCaml's `let rec go pq dist = ...` pattern.
///
/// Semantically identical to `dijkstra`; written to show the OCaml parallel.
pub fn dijkstra_functional<'a>(graph: &'a Graph, start: &'a str) -> HashMap<&'a str, u32> {
    let mut dist: HashMap<&str, u32> = HashMap::new();
    dist.insert(start, 0);

    let mut pq: BinaryHeap<Reverse<(u32, &str)>> = BinaryHeap::new();
    pq.push(Reverse((0, start)));

    fn go<'a>(
        pq: &mut BinaryHeap<Reverse<(u32, &'a str)>>,
        dist: &mut HashMap<&'a str, u32>,
        graph: &'a Graph,
    ) {
        let Some(Reverse((d, u))) = pq.pop() else {
            return;
        };

        if dist.get(u).copied().unwrap_or(u32::MAX) < d {
            return go(pq, dist, graph);
        }

        let neighbors = graph.get(u).map(Vec::as_slice).unwrap_or(&[]);
        // Collect updates first to satisfy the borrow checker (can't
        // hold a shared borrow of `dist` while mutating it in the same chain).
        let updates: Vec<(&str, u32)> = neighbors
            .iter()
            .filter_map(|&(v, w)| {
                let alt = d + w;
                (alt < dist.get(v).copied().unwrap_or(u32::MAX)).then_some((v, alt))
            })
            .collect();
        for (v, alt) in updates {
            dist.insert(v, alt);
            pq.push(Reverse((alt, v)));
        }

        go(pq, dist, graph)
    }

    go(&mut pq, &mut dist, graph);
    dist
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_graph<'a>() -> Graph<'a> {
        let mut g = HashMap::new();
        g.insert("a", vec![("b", 1), ("c", 4)]);
        g.insert("b", vec![("c", 2), ("d", 6)]);
        g.insert("c", vec![("d", 3)]);
        g.insert("d", vec![]);
        g
    }

    #[test]
    fn test_dijkstra_distances() {
        let g = sample_graph();
        let dist = dijkstra(&g, "a");
        assert_eq!(dist["a"], 0);
        assert_eq!(dist["b"], 1);
        assert_eq!(dist["c"], 3); // a→b→c = 1+2 = 3, shorter than a→c = 4
        assert_eq!(dist["d"], 6); // a→b→c→d = 1+2+3 = 6
    }

    #[test]
    fn test_dijkstra_functional_matches_idiomatic() {
        let g = sample_graph();
        let d1 = dijkstra(&g, "a");
        let d2 = dijkstra_functional(&g, "a");
        assert_eq!(d1, d2);
    }

    #[test]
    fn test_single_node_graph() {
        let mut g: Graph = HashMap::new();
        g.insert("x", vec![]);
        let dist = dijkstra(&g, "x");
        assert_eq!(dist["x"], 0);
        assert_eq!(dist.len(), 1);
    }

    #[test]
    fn test_start_not_in_graph_edges() {
        // Start node has no outgoing edges listed in the adjacency map.
        let g: Graph = HashMap::new();
        let dist = dijkstra(&g, "isolated");
        assert_eq!(dist["isolated"], 0);
        assert_eq!(dist.len(), 1);
    }

    #[test]
    fn test_unreachable_node_absent() {
        let mut g: Graph = HashMap::new();
        g.insert("a", vec![("b", 5)]);
        g.insert("b", vec![]);
        g.insert("c", vec![("a", 1)]); // c → a, but not reachable from "a"
        let dist = dijkstra(&g, "a");
        assert!(!dist.contains_key("c"), "c is unreachable from a");
        assert_eq!(dist["b"], 5);
    }

    #[test]
    fn test_multiple_paths_picks_shortest() {
        // a→d direct cost 10, a→b→c→d cost 1+1+1 = 3
        let mut g: Graph = HashMap::new();
        g.insert("a", vec![("b", 1), ("d", 10)]);
        g.insert("b", vec![("c", 1)]);
        g.insert("c", vec![("d", 1)]);
        g.insert("d", vec![]);
        let dist = dijkstra(&g, "a");
        assert_eq!(dist["d"], 3);
    }
}
