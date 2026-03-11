// 1110: Dijkstra's Shortest Path — Priority Queue
//
// OCaml uses `Set.Make` as a sorted priority queue (ordered BST of (dist, node) tuples)
// and `Map.Make(String)` as the distance map.
//
// Rust translates this to two variants:
//   1. Idiomatic: `BinaryHeap<Reverse<(i32, String)>>` + `HashMap` — fast, imperative
//   2. Functional: `BTreeSet<(i32, String)>` + `BTreeMap` — mirrors OCaml's ordered-set PQ
//
// Key OCaml→Rust translation:
//   - `PQ.min_elt` + `PQ.remove` → `BTreeSet::pop_first()` (stable since Rust 1.66)
//   - `try SMap.find v dist with Not_found -> max_int` → `.unwrap_or(i32::MAX)`
//   - `List.fold_left` over neighbors → `.iter().fold(...)`
//   - `let rec go pq dist = ...` → inner `fn go(...)` (no TCO guarantee in Rust)

use std::cmp::Reverse;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap};

/// Idiomatic Rust — BinaryHeap (min-heap via Reverse) + HashMap.
///
/// OCaml's `Set.Make` priority queue is a balanced BST; it deduplicates entries
/// automatically. Rust's `BinaryHeap` is a binary heap that allows duplicates.
/// We compensate with a stale-entry check: if the popped distance `d` is greater
/// than the best known `dist[u]`, the entry is outdated and we skip it.
///
/// Complexity: O((V + E) log V) — same asymptotic as the OCaml version.
pub fn dijkstra(graph: &HashMap<String, Vec<(String, i32)>>, start: &str) -> HashMap<String, i32> {
    let mut dist: HashMap<String, i32> = HashMap::from([(start.to_string(), 0)]);
    // Reverse turns the max-heap into a min-heap: smallest distance is popped first.
    let mut heap: BinaryHeap<Reverse<(i32, String)>> =
        BinaryHeap::from([Reverse((0, start.to_string()))]);

    while let Some(Reverse((d, u))) = heap.pop() {
        // OCaml's Set prevents stale entries; we skip them here instead.
        if dist.get(&u).is_some_and(|&best| d > best) {
            continue;
        }
        // OCaml: `try SMap.find u graph with Not_found -> []`
        let neighbors = graph.get(&u).map(Vec::as_slice).unwrap_or(&[]);
        for (v, w) in neighbors {
            let alt = d + w;
            // OCaml: `try SMap.find v dist with Not_found -> max_int`
            if alt < dist.get(v).copied().unwrap_or(i32::MAX) {
                dist.insert(v.clone(), alt); // clone: need owned String for the map key
                heap.push(Reverse((alt, v.clone()))); // clone: heap takes ownership
            }
        }
    }
    dist
}

/// Functional / set-based — mirrors OCaml's `module PQ = Set.Make` approach.
///
/// Uses `BTreeSet<(i32, String)>` as an ordered priority queue (like OCaml's `Set.Make`)
/// and `BTreeMap<String, i32>` as the distance map (like OCaml's `Map.Make(String)`).
///
/// `pop_first()` atomically removes and returns the minimum element, directly
/// translating OCaml's `PQ.min_elt pq` + `PQ.remove (d, u) pq`.
///
/// The inner `go` mirrors OCaml's `let rec go pq dist = if PQ.is_empty pq then dist`.
///
/// Note: Rust lacks guaranteed tail-call optimisation. For large graphs, prefer
/// the iterative `dijkstra` above to avoid stack overflow.
pub fn dijkstra_functional(
    graph: &BTreeMap<String, Vec<(String, i32)>>,
    start: &str,
) -> BTreeMap<String, i32> {
    // Inner helper — mirrors `let rec go pq dist = ...` in OCaml.
    fn go(
        graph: &BTreeMap<String, Vec<(String, i32)>>,
        mut pq: BTreeSet<(i32, String)>,
        dist: BTreeMap<String, i32>,
    ) -> BTreeMap<String, i32> {
        // OCaml: `if PQ.is_empty pq then dist`
        let Some((d, u)) = pq.pop_first() else {
            return dist;
        };

        // OCaml: `let dist, pq = List.fold_left (...) (dist, pq) neighbors`
        let (dist, pq) = graph.get(&u).map(Vec::as_slice).unwrap_or(&[]).iter().fold(
            (dist, pq),
            |(mut dist, mut pq), (v, w)| {
                let alt = d + w;
                let current = dist.get(v).copied().unwrap_or(i32::MAX);
                if alt < current {
                    dist.insert(v.clone(), alt); // clone: BTreeMap needs owned key
                    pq.insert((alt, v.clone())); // clone: BTreeSet needs owned value
                }
                (dist, pq)
            },
        );

        go(graph, pq, dist)
    }

    let dist = BTreeMap::from([(start.to_string(), 0)]);
    let pq = BTreeSet::from([(0i32, start.to_string())]);
    go(graph, pq, dist)
}

#[cfg(test)]
mod tests {
    use super::*;

    // Graph from the OCaml source:
    // a→b(1), a→c(4), b→c(2), b→d(6), c→d(3)
    // Shortest paths from a: a=0, b=1, c=3 (a→b→c), d=6 (a→b→c→d)
    fn make_graph() -> HashMap<String, Vec<(String, i32)>> {
        [
            ("a", vec![("b", 1), ("c", 4)]),
            ("b", vec![("c", 2), ("d", 6)]),
            ("c", vec![("d", 3)]),
            ("d", vec![]),
        ]
        .into_iter()
        .map(|(k, vs)| {
            (
                k.to_string(),
                vs.into_iter().map(|(v, w)| (v.to_string(), w)).collect(),
            )
        })
        .collect()
    }

    fn make_btree_graph() -> BTreeMap<String, Vec<(String, i32)>> {
        [
            ("a", vec![("b", 1), ("c", 4)]),
            ("b", vec![("c", 2), ("d", 6)]),
            ("c", vec![("d", 3)]),
            ("d", vec![]),
        ]
        .into_iter()
        .map(|(k, vs)| {
            (
                k.to_string(),
                vs.into_iter().map(|(v, w)| (v.to_string(), w)).collect(),
            )
        })
        .collect()
    }

    #[test]
    fn test_shortest_paths_from_a() {
        let dist = dijkstra(&make_graph(), "a");
        assert_eq!(dist["a"], 0);
        assert_eq!(dist["b"], 1); // a→b direct
        assert_eq!(dist["c"], 3); // a→b→c = 1+2, not a→c direct = 4
        assert_eq!(dist["d"], 6); // a→b→c→d = 1+2+3
    }

    #[test]
    fn test_single_node_graph() {
        let graph: HashMap<String, Vec<(String, i32)>> = HashMap::from([("x".to_string(), vec![])]);
        let dist = dijkstra(&graph, "x");
        assert_eq!(dist["x"], 0);
        assert_eq!(dist.len(), 1);
    }

    #[test]
    fn test_unreachable_node_absent_from_dist() {
        // "z" exists as a node but has no incoming edges from "a"
        let mut graph = make_graph();
        graph.insert("z".to_string(), vec![]);
        let dist = dijkstra(&graph, "a");
        assert!(!dist.contains_key("z"));
    }

    #[test]
    fn test_functional_matches_idiomatic() {
        let g_hash = make_graph();
        let g_btree = make_btree_graph();
        let dist_hash = dijkstra(&g_hash, "a");
        let dist_btree = dijkstra_functional(&g_btree, "a");
        for key in ["a", "b", "c", "d"] {
            assert_eq!(dist_hash[key], dist_btree[key], "mismatch for node {key}");
        }
    }

    #[test]
    fn test_direct_edge_not_used_when_longer() {
        // a→c direct = 4, but a→b→c = 3; algorithm must choose the shorter path
        let dist = dijkstra(&make_graph(), "a");
        assert_eq!(dist["c"], 3);
    }

    #[test]
    fn test_start_from_intermediate_node() {
        // From b: b→c=2, b→d=6; then c→d gives 2+3=5 < 6
        let dist = dijkstra(&make_graph(), "b");
        assert_eq!(dist["b"], 0);
        assert_eq!(dist["c"], 2);
        assert_eq!(dist["d"], 5); // via b→c→d, not b→d directly
        assert!(!dist.contains_key("a")); // a is not reachable from b
    }

    #[test]
    fn test_linear_chain() {
        // a→b(3)→c(4)→d(5): distances should be cumulative
        let graph: HashMap<String, Vec<(String, i32)>> = [
            ("a", vec![("b", 3)]),
            ("b", vec![("c", 4)]),
            ("c", vec![("d", 5)]),
            ("d", vec![]),
        ]
        .into_iter()
        .map(|(k, vs)| {
            (
                k.to_string(),
                vs.into_iter().map(|(v, w)| (v.to_string(), w)).collect(),
            )
        })
        .collect();
        let dist = dijkstra(&graph, "a");
        assert_eq!(dist["a"], 0);
        assert_eq!(dist["b"], 3);
        assert_eq!(dist["c"], 7);
        assert_eq!(dist["d"], 12);
    }
}
