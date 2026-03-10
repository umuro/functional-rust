use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

/// Adjacency list representation: node -> [(neighbor, weight)]
pub type Graph<'a> = HashMap<&'a str, Vec<(&'a str, u64)>>;

// ---------------------------------------------------------------------------
// Solution 1: Idiomatic Rust — BinaryHeap as a min-priority queue
// Uses Reverse to turn the max-heap into a min-heap, iterators for neighbor
// relaxation, and HashMap for O(1) distance lookups.
// ---------------------------------------------------------------------------
pub fn dijkstra_idiomatic<'a>(graph: &Graph<'a>, start: &'a str) -> HashMap<&'a str, u64> {
    let mut dist: HashMap<&'a str, u64> = HashMap::new();
    let mut heap: BinaryHeap<Reverse<(u64, &'a str)>> = BinaryHeap::new();

    dist.insert(start, 0);
    heap.push(Reverse((0, start)));

    while let Some(Reverse((d, u))) = heap.pop() {
        // Skip stale entries — we already found a shorter path
        if d > *dist.get(u).unwrap_or(&u64::MAX) {
            continue;
        }

        if let Some(neighbors) = graph.get(u) {
            for &(v, w) in neighbors {
                let alt = d + w;
                let current = *dist.get(v).unwrap_or(&u64::MAX);
                if alt < current {
                    dist.insert(v, alt);
                    heap.push(Reverse((alt, v)));
                }
            }
        }
    }

    dist
}

// ---------------------------------------------------------------------------
// Solution 2: Functional/recursive — mirrors the OCaml structure
// Uses a BTreeSet as an ordered set (like OCaml's Set.Make) to always extract
// the minimum element. Accumulates distances in a HashMap, threading state
// through recursive calls just like the OCaml version's `go pq dist`.
// ---------------------------------------------------------------------------
pub fn dijkstra_functional<'a>(graph: &Graph<'a>, start: &'a str) -> HashMap<&'a str, u64> {
    use std::collections::BTreeSet;

    // BTreeSet<(u64, &str)> — ordered by distance then node name, like OCaml's PQ
    type Pq<'a> = BTreeSet<(u64, &'a str)>;

    fn go<'a>(graph: &Graph<'a>, pq: Pq<'a>, dist: HashMap<&'a str, u64>) -> HashMap<&'a str, u64> {
        // Base case: empty priority queue — return accumulated distances
        let &(d, u) = match pq.iter().next() {
            None => return dist,
            Some(min) => min,
        };

        let mut pq = pq;
        pq.remove(&(d, u));

        // Skip stale entries
        if d > *dist.get(u).unwrap_or(&u64::MAX) {
            return go(graph, pq, dist);
        }

        // Fold over neighbors — functional accumulation of (dist, pq)
        let empty: Vec<(&str, u64)> = Vec::new();
        let neighbors = graph.get(u).unwrap_or(&empty);
        let (dist, pq) = neighbors
            .iter()
            .fold((dist, pq), |(mut dist, mut pq), &(v, w)| {
                let alt = d + w;
                let current = *dist.get(v).unwrap_or(&u64::MAX);
                if alt < current {
                    dist.insert(v, alt);
                    pq.insert((alt, v));
                }
                (dist, pq)
            });

        go(graph, pq, dist)
    }

    let mut dist = HashMap::new();
    dist.insert(start, 0);
    let mut pq = BTreeSet::new();
    pq.insert((0u64, start));

    go(graph, pq, dist)
}

// ---------------------------------------------------------------------------
// Solution 3: Iterator-based relaxation with explicit visited set
// Separates the "visited" concept from distance tracking for clarity.
// ---------------------------------------------------------------------------
pub fn dijkstra_visited<'a>(graph: &Graph<'a>, start: &'a str) -> HashMap<&'a str, u64> {
    use std::collections::HashSet;

    let mut dist: HashMap<&'a str, u64> = HashMap::new();
    let mut visited: HashSet<&'a str> = HashSet::new();
    let mut heap: BinaryHeap<Reverse<(u64, &'a str)>> = BinaryHeap::new();

    dist.insert(start, 0);
    heap.push(Reverse((0, start)));

    while let Some(Reverse((d, u))) = heap.pop() {
        if !visited.insert(u) {
            continue; // Already processed this node
        }

        let neighbors = graph.get(u).into_iter().flatten();
        for &(v, w) in neighbors {
            let alt = d + w;
            let current = *dist.get(v).unwrap_or(&u64::MAX);
            if alt < current {
                dist.insert(v, alt);
                heap.push(Reverse((alt, v)));
            }
        }
    }

    dist
}

/// Helper to build a graph from a slice of (node, neighbors) pairs
pub fn build_graph<'a>(edges: &[(&'a str, Vec<(&'a str, u64)>)]) -> Graph<'a> {
    edges.iter().cloned().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_graph() -> Graph<'static> {
        build_graph(&[
            ("a", vec![("b", 1), ("c", 4)]),
            ("b", vec![("c", 2), ("d", 6)]),
            ("c", vec![("d", 3)]),
            ("d", vec![]),
        ])
    }

    // -- Idiomatic --

    #[test]
    fn idiomatic_basic_shortest_paths() {
        let g = sample_graph();
        let dist = dijkstra_idiomatic(&g, "a");
        assert_eq!(dist["a"], 0);
        assert_eq!(dist["b"], 1);
        assert_eq!(dist["c"], 3); // a->b->c = 1+2 = 3, shorter than a->c = 4
        assert_eq!(dist["d"], 6); // a->b->c->d = 1+2+3 = 6, shorter than a->b->d = 7
    }

    #[test]
    fn idiomatic_single_node() {
        let g = build_graph(&[("x", vec![])]);
        let dist = dijkstra_idiomatic(&g, "x");
        assert_eq!(dist.len(), 1);
        assert_eq!(dist["x"], 0);
    }

    #[test]
    fn idiomatic_disconnected_node() {
        let g = build_graph(&[("a", vec![("b", 5)]), ("b", vec![]), ("c", vec![])]);
        let dist = dijkstra_idiomatic(&g, "a");
        assert_eq!(dist["a"], 0);
        assert_eq!(dist["b"], 5);
        assert!(!dist.contains_key("c")); // c unreachable from a
    }

    #[test]
    fn idiomatic_multiple_paths_picks_shortest() {
        // a->b: 10, a->c: 1, c->b: 2 — shortest to b is a->c->b = 3
        let g = build_graph(&[
            ("a", vec![("b", 10), ("c", 1)]),
            ("c", vec![("b", 2)]),
            ("b", vec![]),
        ]);
        let dist = dijkstra_idiomatic(&g, "a");
        assert_eq!(dist["b"], 3);
    }

    // -- Functional --

    #[test]
    fn functional_basic_shortest_paths() {
        let g = sample_graph();
        let dist = dijkstra_functional(&g, "a");
        assert_eq!(dist["a"], 0);
        assert_eq!(dist["b"], 1);
        assert_eq!(dist["c"], 3);
        assert_eq!(dist["d"], 6);
    }

    #[test]
    fn functional_single_node() {
        let g = build_graph(&[("x", vec![])]);
        let dist = dijkstra_functional(&g, "x");
        assert_eq!(dist.len(), 1);
        assert_eq!(dist["x"], 0);
    }

    #[test]
    fn functional_disconnected_node() {
        let g = build_graph(&[("a", vec![("b", 5)]), ("b", vec![]), ("c", vec![])]);
        let dist = dijkstra_functional(&g, "a");
        assert_eq!(dist["a"], 0);
        assert_eq!(dist["b"], 5);
        assert!(!dist.contains_key("c"));
    }

    #[test]
    fn functional_matches_idiomatic() {
        let g = sample_graph();
        let d1 = dijkstra_idiomatic(&g, "a");
        let d2 = dijkstra_functional(&g, "a");
        assert_eq!(d1, d2);
    }

    // -- Visited --

    #[test]
    fn visited_basic_shortest_paths() {
        let g = sample_graph();
        let dist = dijkstra_visited(&g, "a");
        assert_eq!(dist["a"], 0);
        assert_eq!(dist["b"], 1);
        assert_eq!(dist["c"], 3);
        assert_eq!(dist["d"], 6);
    }

    #[test]
    fn visited_matches_idiomatic() {
        let g = sample_graph();
        let d1 = dijkstra_idiomatic(&g, "a");
        let d2 = dijkstra_visited(&g, "a");
        assert_eq!(d1, d2);
    }

    #[test]
    fn all_implementations_agree_on_linear_chain() {
        // a->b->c->d->e, each edge weight 1
        let g = build_graph(&[
            ("a", vec![("b", 1)]),
            ("b", vec![("c", 1)]),
            ("c", vec![("d", 1)]),
            ("d", vec![("e", 1)]),
            ("e", vec![]),
        ]);
        let d1 = dijkstra_idiomatic(&g, "a");
        let d2 = dijkstra_functional(&g, "a");
        let d3 = dijkstra_visited(&g, "a");
        assert_eq!(d1, d2);
        assert_eq!(d2, d3);
        assert_eq!(d1["e"], 4);
    }

    #[test]
    fn empty_graph_start_only() {
        let g: Graph = HashMap::new();
        let dist = dijkstra_idiomatic(&g, "a");
        assert_eq!(dist.len(), 1);
        assert_eq!(dist["a"], 0);
    }
}
