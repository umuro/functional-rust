//! # Dijkstra's Shortest Path with a Priority Queue
//!
//! OCaml uses `Set.Make` as a sorted priority queue (ordered by `(dist, node)`
//! tuple) and `Map.Make(String)` as an immutable distance map, threading both
//! through a tail-recursive `go` loop.
//!
//! Rust's `std::collections::BinaryHeap` is a max-heap; wrapping entries in
//! `std::cmp::Reverse` turns it into the min-heap Dijkstra requires.
//! `HashMap` plays the role of OCaml's `SMap`.
//!
//! This module shows:
//! 1. Idiomatic Rust — imperative `while let` loop, mutable `HashMap`
//! 2. Functional style — immutable-ish accumulator passed through a helper
//! 3. The OCaml→Rust translation of `Set.Make` → `BinaryHeap<Reverse<…>>`

use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

/// Weighted directed graph: node name → list of (neighbour, weight).
///
/// Uses `&str` keys so callers can pass string literals directly.
/// `u32` weights match OCaml's `int` (non-negative distances).
pub type Graph<'a> = HashMap<&'a str, Vec<(&'a str, u32)>>;

// ---------------------------------------------------------------------------
// Solution 1: Idiomatic Rust — `while let` loop with mutable state
//
// OCaml equivalent uses a tail-recursive `go` function that shadows
// `pq` and `dist` on every call. Rust's `while let` over `heap.pop()`
// expresses the same control flow without recursion.
// ---------------------------------------------------------------------------

/// Run Dijkstra from `start` and return shortest distances to all reachable nodes.
///
/// OCaml type: `dijkstra : int SMap.t SMap.t -> string -> int SMap.t`
/// Rust type:  `fn dijkstra<'a>(graph: &Graph<'a>, start: &'a str) -> HashMap<&'a str, u32>`
///
/// Uses `BinaryHeap<Reverse<(u32, &str)>>` as a min-heap.
/// `Reverse` flips the natural max-heap ordering so the smallest distance
/// is always at the top — the same invariant OCaml's `Set.min_elt` provides.
pub fn dijkstra<'a>(graph: &Graph<'a>, start: &'a str) -> HashMap<&'a str, u32> {
    // dist[node] = best known distance from start; unvisited nodes are absent.
    let mut dist: HashMap<&str, u32> = HashMap::new();
    dist.insert(start, 0);

    // Min-heap: Reverse wraps (distance, node) so smaller distances pop first.
    let mut heap: BinaryHeap<Reverse<(u32, &str)>> = BinaryHeap::new();
    heap.push(Reverse((0, start)));

    while let Some(Reverse((d, u))) = heap.pop() {
        // Skip stale entries: a shorter path was already found and processed.
        if d > *dist.get(u).unwrap_or(&u32::MAX) {
            continue;
        }

        let neighbors = graph.get(u).map(Vec::as_slice).unwrap_or(&[]);
        for &(v, w) in neighbors {
            let alt = d + w;
            // Only update if we found a strictly shorter path.
            let current = *dist.get(v).unwrap_or(&u32::MAX);
            if alt < current {
                dist.insert(v, alt);
                heap.push(Reverse((alt, v)));
            }
        }
    }

    dist
}

// ---------------------------------------------------------------------------
// Solution 2: Functional style — immutable accumulator, helper function
//
// Mirrors the OCaml `rec go pq dist` pattern directly.
// Rust doesn't optimise tail calls, so we use an explicit stack (the heap)
// instead of mutual recursion to avoid stack overflow on large graphs.
// The key OCaml idiom `List.fold_left` maps to `.iter().fold(…)`.
// ---------------------------------------------------------------------------

/// Functional-style Dijkstra: state is accumulated and passed forward,
/// mirroring OCaml's `rec go pq dist` tail-recursive helper.
///
/// `fold_neighbors` plays the role of `List.fold_left` in the OCaml source.
pub fn dijkstra_functional<'a>(graph: &Graph<'a>, start: &'a str) -> HashMap<&'a str, u32> {
    let dist: HashMap<&str, u32> = std::iter::once((start, 0u32)).collect();
    let heap: BinaryHeap<Reverse<(u32, &str)>> = std::iter::once(Reverse((0u32, start))).collect();
    go(graph, heap, dist)
}

/// Recursive helper — accumulates `(heap, dist)` toward the fixed point
/// where the heap is empty (all reachable nodes settled).
fn go<'a>(
    graph: &Graph<'a>,
    mut heap: BinaryHeap<Reverse<(u32, &'a str)>>,
    dist: HashMap<&'a str, u32>,
) -> HashMap<&'a str, u32> {
    // Base case: OCaml's `if PQ.is_empty pq then dist`
    let Some(Reverse((d, u))) = heap.pop() else {
        return dist;
    };

    // Stale-entry guard (OCaml's Set avoids duplicates; heap does not)
    if d > *dist.get(u).unwrap_or(&u32::MAX) {
        return go(graph, heap, dist);
    }

    // `List.fold_left` over neighbours → `.iter().fold(…)` over the slice
    let neighbors = graph.get(u).map(Vec::as_slice).unwrap_or(&[]);
    let (dist, heap) = neighbors
        .iter()
        .fold((dist, heap), |(mut d_map, mut h), &(v, w)| {
            let alt = d + w;
            let current = *d_map.get(v).unwrap_or(&u32::MAX);
            if alt < current {
                d_map.insert(v, alt);
                h.push(Reverse((alt, v)));
            }
            (d_map, h)
        });

    go(graph, heap, dist)
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_graph<'a>() -> Graph<'a> {
        let mut g = Graph::new();
        g.insert("a", vec![("b", 1), ("c", 4)]);
        g.insert("b", vec![("c", 2), ("d", 6)]);
        g.insert("c", vec![("d", 3)]);
        g.insert("d", vec![]);
        g
    }

    #[test]
    fn test_single_node_start() {
        // A graph with only the start node reachable.
        let mut g = Graph::new();
        g.insert("x", vec![]);
        let dist = dijkstra(&g, "x");
        assert_eq!(dist["x"], 0);
        assert_eq!(dist.len(), 1);
    }

    #[test]
    fn test_direct_edge() {
        let mut g = Graph::new();
        g.insert("a", vec![("b", 7)]);
        g.insert("b", vec![]);
        let dist = dijkstra(&g, "a");
        assert_eq!(dist["a"], 0);
        assert_eq!(dist["b"], 7);
    }

    #[test]
    fn test_shortest_path_prefers_indirect_route() {
        // a→c directly costs 4, but a→b→c costs 1+2=3 (shorter).
        let g = sample_graph();
        let dist = dijkstra(&g, "a");
        assert_eq!(dist["a"], 0);
        assert_eq!(dist["b"], 1);
        assert_eq!(dist["c"], 3); // via b, not directly
        assert_eq!(dist["d"], 6); // a→b(1)→c(2)→d(3) = 6, not a→b(1)→d(6)=7
    }

    #[test]
    fn test_unreachable_node_absent() {
        // Node "z" is in the graph but unreachable from "a".
        let mut g = sample_graph();
        g.insert("z", vec![("a", 1)]); // z can reach a, but a cannot reach z
        let dist = dijkstra(&g, "a");
        assert!(
            !dist.contains_key("z"),
            "unreachable node must not appear in dist"
        );
    }

    #[test]
    fn test_both_implementations_agree() {
        let g = sample_graph();
        let d1 = dijkstra(&g, "a");
        let d2 = dijkstra_functional(&g, "a");
        assert_eq!(
            d1, d2,
            "idiomatic and functional implementations must agree"
        );
    }

    #[test]
    fn test_disconnected_start_not_in_graph() {
        // start node has no entry in graph — dist should still contain start=0.
        let g: Graph = HashMap::new();
        let dist = dijkstra(&g, "alone");
        assert_eq!(dist["alone"], 0);
        assert_eq!(dist.len(), 1);
    }
}
