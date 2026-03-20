#![allow(clippy::all)]
use std::cmp::Reverse;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap};

/// Graph as adjacency list: node name → list of (neighbor, weight)
pub type Graph = HashMap<String, Vec<(String, u32)>>;

/// Dijkstra's shortest-path — idiomatic Rust
///
/// Uses `BinaryHeap<Reverse<...>>` as a standard min-heap.
/// Stale entries are discarded lazily when popped (lazy deletion).
pub fn dijkstra(graph: &Graph, start: &str) -> HashMap<String, u32> {
    let mut dist: HashMap<String, u32> = HashMap::new();
    dist.insert(start.to_string(), 0);

    // Reverse turns the max-heap into a min-heap: smallest distance pops first
    let mut heap: BinaryHeap<Reverse<(u32, String)>> = BinaryHeap::new();
    heap.push(Reverse((0, start.to_string())));

    while let Some(Reverse((d, u))) = heap.pop() {
        // Lazy deletion: skip stale entries where a shorter path was already recorded
        if d > dist.get(&u).copied().unwrap_or(u32::MAX) {
            continue;
        }

        if let Some(neighbors) = graph.get(&u) {
            for (v, w) in neighbors {
                let alt = d + w;
                if alt < dist.get(v).copied().unwrap_or(u32::MAX) {
                    dist.insert(v.clone(), alt);
                    heap.push(Reverse((alt, v.clone())));
                }
            }
        }
    }

    dist
}

/// Graph for the functional implementation (BTreeMap for sorted, deterministic output)
pub type FunctionalGraph = BTreeMap<String, Vec<(String, u32)>>;

/// Dijkstra's shortest-path — functional style mirroring OCaml's Set.Make PQ
///
/// Uses `BTreeSet<(u32, String)>` as an ordered set, exactly as OCaml uses
/// `Set.Make` for the priority queue. The smallest element (min_elt) is always
/// the first element in BTreeSet's iteration order.
pub fn dijkstra_functional(graph: &FunctionalGraph, start: &str) -> BTreeMap<String, u32> {
    let mut dist: BTreeMap<String, u32> = BTreeMap::new();
    dist.insert(start.to_string(), 0);

    // BTreeSet mimics OCaml's Set.Make — ordered set, min element is first
    let mut pq: BTreeSet<(u32, String)> = BTreeSet::new();
    pq.insert((0, start.to_string()));

    // OCaml: if PQ.is_empty pq then dist else ...
    // .cloned() releases the immutable borrow on pq so we can mutate it
    while let Some(entry) = pq.iter().next().cloned() {
        // OCaml: PQ.remove (d, u) pq
        pq.remove(&entry);
        let (d, u) = entry;

        // OCaml: if d > dist_u then go pq' dist (skip stale entries)
        if d > dist.get(&u).copied().unwrap_or(u32::MAX) {
            continue;
        }

        // OCaml: List.fold_left over neighbors updating dist and pq
        if let Some(neighbors) = graph.get(&u) {
            for (v, w) in neighbors {
                let alt = d + w;
                if alt < dist.get(v).copied().unwrap_or(u32::MAX) {
                    dist.insert(v.clone(), alt);
                    pq.insert((alt, v.clone()));
                }
            }
        }
    }

    dist
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_graph() -> Graph {
        let mut g: Graph = HashMap::new();
        g.insert("a".into(), vec![("b".into(), 1), ("c".into(), 4)]);
        g.insert("b".into(), vec![("c".into(), 2), ("d".into(), 6)]);
        g.insert("c".into(), vec![("d".into(), 3)]);
        g.insert("d".into(), vec![]);
        g
    }

    fn sample_functional_graph() -> FunctionalGraph {
        let mut g: FunctionalGraph = BTreeMap::new();
        g.insert("a".into(), vec![("b".into(), 1), ("c".into(), 4)]);
        g.insert("b".into(), vec![("c".into(), 2), ("d".into(), 6)]);
        g.insert("c".into(), vec![("d".into(), 3)]);
        g.insert("d".into(), vec![]);
        g
    }

    #[test]
    fn test_start_node_has_zero_distance() {
        let dist = dijkstra(&sample_graph(), "a");
        assert_eq!(dist.get("a").copied(), Some(0));
    }

    #[test]
    fn test_shortest_paths_indirect_beats_direct() {
        // a->c direct = 4, but a->b->c = 1+2 = 3 wins
        let dist = dijkstra(&sample_graph(), "a");
        assert_eq!(dist.get("a").copied(), Some(0));
        assert_eq!(dist.get("b").copied(), Some(1)); // a->b
        assert_eq!(dist.get("c").copied(), Some(3)); // a->b->c, not a->c=4
        assert_eq!(dist.get("d").copied(), Some(6)); // a->b->c->d
    }

    #[test]
    fn test_unreachable_node_absent_from_result() {
        let mut g = sample_graph();
        g.insert("z".into(), vec![]); // z is disconnected
        let dist = dijkstra(&g, "a");
        assert_eq!(dist.get("z"), None);
    }

    #[test]
    fn test_single_node_graph() {
        let mut g: Graph = HashMap::new();
        g.insert("x".into(), vec![]);
        let dist = dijkstra(&g, "x");
        assert_eq!(dist.get("x").copied(), Some(0));
        assert_eq!(dist.len(), 1);
    }

    #[test]
    fn test_functional_matches_idiomatic() {
        let dist_std = dijkstra(&sample_graph(), "a");
        let dist_btree = dijkstra_functional(&sample_functional_graph(), "a");
        for key in ["a", "b", "c", "d"] {
            assert_eq!(
                dist_std.get(key),
                dist_btree.get(key),
                "mismatch for node {key}"
            );
        }
    }

    #[test]
    fn test_functional_start_zero_unreachable_absent() {
        let mut g: FunctionalGraph = BTreeMap::new();
        g.insert("a".into(), vec![("b".into(), 5)]);
        g.insert("b".into(), vec![]);
        g.insert("c".into(), vec![]); // disconnected
        let dist = dijkstra_functional(&g, "a");
        assert_eq!(dist.get("a").copied(), Some(0));
        assert_eq!(dist.get("b").copied(), Some(5));
        assert_eq!(dist.get("c"), None);
    }
}
