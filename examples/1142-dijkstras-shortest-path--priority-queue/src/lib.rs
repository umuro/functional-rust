#![allow(clippy::all)]
//! Dijkstra's shortest path algorithm using a BinaryHeap priority queue.
//!
//! OCaml uses a functional `Set` as a priority queue (ordered by (dist, node)).
//! Rust uses `BinaryHeap` (max-heap) with `Reverse` for min-heap behaviour.
//! Both share the same relaxation logic: if `alt < current`, update and enqueue.

use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

/// Graph: adjacency list mapping node name → list of (neighbour, weight).
pub type Graph<'a> = HashMap<&'a str, Vec<(&'a str, u32)>>;

/// Returns a map of shortest distances from `start` to every reachable node.
///
/// Uses a standard binary-heap based Dijkstra.
/// Nodes never visited are absent from the returned map.
pub fn dijkstra<'a>(graph: &'a Graph, start: &'a str) -> HashMap<&'a str, u32> {
    let mut dist: HashMap<&str, u32> = HashMap::new();
    dist.insert(start, 0);

    // (Reverse(distance), node) — Reverse turns max-heap into min-heap
    let mut heap: BinaryHeap<(Reverse<u32>, &str)> = BinaryHeap::new();
    heap.push((Reverse(0), start));

    while let Some((Reverse(d), u)) = heap.pop() {
        // Skip stale entries (we may have inserted a node multiple times)
        if dist.get(u).is_some_and(|&best| d > best) {
            continue;
        }

        let neighbors = graph.get(u).map(Vec::as_slice).unwrap_or(&[]);
        for &(v, w) in neighbors {
            let alt = d + w;
            let current = dist.get(v).copied().unwrap_or(u32::MAX);
            if alt < current {
                dist.insert(v, alt);
                heap.push((Reverse(alt), v));
            }
        }
    }

    dist
}

/// Functional-style Dijkstra: mirrors the OCaml tail-recursive `go` loop.
///
/// Instead of mutating a HashMap we pass `dist` and `pq` as owned values
/// through a recursive helper — identical structure to the OCaml version.
pub fn dijkstra_functional<'a>(graph: &'a Graph, start: &'a str) -> HashMap<&'a str, u32> {
    let dist: HashMap<&str, u32> = std::iter::once((start, 0)).collect();
    let mut heap: BinaryHeap<(Reverse<u32>, &str)> = BinaryHeap::new();
    heap.push((Reverse(0), start));
    go(graph, heap, dist)
}

fn go<'a>(
    graph: &'a Graph,
    mut heap: BinaryHeap<(Reverse<u32>, &'a str)>,
    dist: HashMap<&'a str, u32>,
) -> HashMap<&'a str, u32> {
    match heap.pop() {
        None => dist,
        Some((Reverse(d), u)) => {
            if dist.get(u).is_some_and(|&best| d > best) {
                // stale entry — skip without branching the recursion visibly
                return go(graph, heap, dist);
            }
            let neighbors = graph.get(u).map(Vec::as_slice).unwrap_or(&[]);
            // fold mirrors OCaml's List.fold_left
            let (dist, heap) =
                neighbors
                    .iter()
                    .fold((dist, heap), |(mut dist, mut heap), &(v, w)| {
                        let alt = d + w;
                        let current = dist.get(v).copied().unwrap_or(u32::MAX);
                        if alt < current {
                            dist.insert(v, alt);
                            heap.push((Reverse(alt), v));
                        }
                        (dist, heap)
                    });
            go(graph, heap, dist)
        }
    }
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

    // ── dijkstra (imperative heap) ────────────────────────────────────────────

    #[test]
    fn test_basic_distances() {
        let g = sample_graph();
        let dist = dijkstra(&g, "a");
        assert_eq!(dist["a"], 0);
        assert_eq!(dist["b"], 1);
        assert_eq!(dist["c"], 3); // a→b→c = 1+2
        assert_eq!(dist["d"], 6); // a→b→c→d = 1+2+3
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
    fn test_unreachable_node_absent() {
        let mut g: Graph = HashMap::new();
        g.insert("a", vec![]);
        g.insert("b", vec![]);
        let dist = dijkstra(&g, "a");
        assert_eq!(dist.get("b"), None);
    }

    #[test]
    fn test_direct_shorter_than_indirect() {
        // a→c direct = 4, a→b→c = 3 — algorithm should pick 3
        let g = sample_graph();
        let dist = dijkstra(&g, "a");
        assert_eq!(dist["c"], 3);
    }

    #[test]
    fn test_start_from_different_node() {
        let g = sample_graph();
        let dist = dijkstra(&g, "b");
        assert_eq!(dist["b"], 0);
        assert_eq!(dist["c"], 2);
        assert_eq!(dist["d"], 5);
        assert_eq!(dist.get("a"), None); // "a" not reachable from "b"
    }

    // ── dijkstra_functional (recursive / OCaml-style) ────────────────────────

    #[test]
    fn test_functional_matches_imperative() {
        let g = sample_graph();
        let imp = dijkstra(&g, "a");
        let fun = dijkstra_functional(&g, "a");
        assert_eq!(imp, fun);
    }

    #[test]
    fn test_functional_single_node() {
        let mut g: Graph = HashMap::new();
        g.insert("x", vec![]);
        let dist = dijkstra_functional(&g, "x");
        assert_eq!(dist["x"], 0);
    }

    #[test]
    fn test_functional_from_mid_graph() {
        let g = sample_graph();
        let dist = dijkstra_functional(&g, "c");
        assert_eq!(dist["c"], 0);
        assert_eq!(dist["d"], 3);
        assert_eq!(dist.get("a"), None);
    }
}
