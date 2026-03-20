use std::cmp::Reverse;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap};

/// Idiomatic Rust: min-heap (BinaryHeap + Reverse) + HashMap.
/// Returns shortest distances from `start` to all reachable nodes.
pub fn dijkstra(graph: &HashMap<String, Vec<(String, u32)>>, start: &str) -> HashMap<String, u32> {
    let mut dist: HashMap<String, u32> = HashMap::new();
    let mut heap: BinaryHeap<Reverse<(u32, String)>> = BinaryHeap::new();

    dist.insert(start.to_string(), 0);
    heap.push(Reverse((0, start.to_string())));

    while let Some(Reverse((d, u))) = heap.pop() {
        // Stale entry: a shorter path was already found and recorded.
        if dist.get(&u).is_some_and(|&cur| d > cur) {
            continue;
        }
        if let Some(neighbors) = graph.get(&u) {
            for (v, w) in neighbors {
                let alt = d.saturating_add(*w);
                if alt < dist.get(v).copied().unwrap_or(u32::MAX) {
                    dist.insert(v.clone(), alt);
                    heap.push(Reverse((alt, v.clone())));
                }
            }
        }
    }
    dist
}

/// Functional/recursive style — mirrors the OCaml implementation.
/// Uses BTreeSet as ordered priority queue (like OCaml's Set.Make)
/// and BTreeMap as the distance map (like OCaml's Map.Make(String)).
pub fn dijkstra_functional(
    graph: &BTreeMap<String, Vec<(String, u32)>>,
    start: &str,
) -> BTreeMap<String, u32> {
    let dist = BTreeMap::from([(start.to_string(), 0u32)]);
    let pq = BTreeSet::from([(0u32, start.to_string())]);
    go(graph, pq, dist)
}

// Tail-recursive worker: take ownership of pq and dist, pass them forward.
// Mirrors OCaml's `let rec go pq dist = ...` with the same structure.
fn go(
    graph: &BTreeMap<String, Vec<(String, u32)>>,
    mut pq: BTreeSet<(u32, String)>,
    dist: BTreeMap<String, u32>,
) -> BTreeMap<String, u32> {
    match pq.iter().next().cloned() {
        None => dist,
        Some((d, u)) => {
            // Remove min element — OCaml does `PQ.remove (d, u) pq`
            pq.remove(&(d, u.clone()));
            let empty = vec![];
            let neighbors = graph.get(&u).unwrap_or(&empty);
            // fold over neighbors, threading (dist, pq) through — mirrors List.fold_left
            let (dist, pq) = neighbors
                .iter()
                .fold((dist, pq), |(mut dist, mut pq), (v, w)| {
                    let alt = d.saturating_add(*w);
                    if alt < dist.get(v).copied().unwrap_or(u32::MAX) {
                        dist.insert(v.clone(), alt);
                        pq.insert((alt, v.clone()));
                    }
                    (dist, pq)
                });
            go(graph, pq, dist)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_graph(edges: &[(&str, &[(&str, u32)])]) -> HashMap<String, Vec<(String, u32)>> {
        edges
            .iter()
            .map(|(node, neighbors)| {
                (
                    node.to_string(),
                    neighbors.iter().map(|(v, w)| (v.to_string(), *w)).collect(),
                )
            })
            .collect()
    }

    fn make_fgraph(edges: &[(&str, &[(&str, u32)])]) -> BTreeMap<String, Vec<(String, u32)>> {
        edges
            .iter()
            .map(|(node, neighbors)| {
                (
                    node.to_string(),
                    neighbors.iter().map(|(v, w)| (v.to_string(), *w)).collect(),
                )
            })
            .collect()
    }

    #[test]
    fn test_single_node_no_edges() {
        let graph = make_graph(&[("a", &[])]);
        let dist = dijkstra(&graph, "a");
        assert_eq!(dist["a"], 0);
        assert_eq!(dist.len(), 1);
    }

    #[test]
    fn test_linear_path() {
        let graph = make_graph(&[("a", &[("b", 1)]), ("b", &[("c", 2)]), ("c", &[])]);
        let dist = dijkstra(&graph, "a");
        assert_eq!(dist["a"], 0);
        assert_eq!(dist["b"], 1);
        assert_eq!(dist["c"], 3);
    }

    #[test]
    fn test_multiple_paths_prefers_shorter() {
        // OCaml example: a->b:1, a->c:4, b->c:2, b->d:6, c->d:3
        // Shortest: a=0, b=1, c=3 (a->b->c), d=6 (a->b->c->d)
        let graph = make_graph(&[
            ("a", &[("b", 1), ("c", 4)]),
            ("b", &[("c", 2), ("d", 6)]),
            ("c", &[("d", 3)]),
            ("d", &[]),
        ]);
        let dist = dijkstra(&graph, "a");
        assert_eq!(dist["a"], 0);
        assert_eq!(dist["b"], 1);
        assert_eq!(dist["c"], 3); // a->b->c=1+2, not a->c=4
        assert_eq!(dist["d"], 6); // a->b->c->d=1+2+3, not a->b->d=1+6=7
    }

    #[test]
    fn test_disconnected_nodes_not_reachable() {
        let graph = make_graph(&[
            ("a", &[("b", 5)]),
            ("b", &[]),
            ("c", &[("d", 1)]), // disconnected component
            ("d", &[]),
        ]);
        let dist = dijkstra(&graph, "a");
        assert_eq!(dist["a"], 0);
        assert_eq!(dist["b"], 5);
        assert!(!dist.contains_key("c")); // unreachable from a
        assert!(!dist.contains_key("d")); // unreachable from a
    }

    #[test]
    fn test_functional_produces_same_result() {
        let graph = make_graph(&[
            ("a", &[("b", 1), ("c", 4)]),
            ("b", &[("c", 2), ("d", 6)]),
            ("c", &[("d", 3)]),
            ("d", &[]),
        ]);
        let fgraph = make_fgraph(&[
            ("a", &[("b", 1), ("c", 4)]),
            ("b", &[("c", 2), ("d", 6)]),
            ("c", &[("d", 3)]),
            ("d", &[]),
        ]);
        let d1 = dijkstra(&graph, "a");
        let d2 = dijkstra_functional(&fgraph, "a");
        for (k, v) in &d2 {
            assert_eq!(d1[k], *v, "mismatch at node {k}");
        }
    }

    #[test]
    fn test_functional_single_node() {
        let fgraph = make_fgraph(&[("start", &[])]);
        let dist = dijkstra_functional(&fgraph, "start");
        assert_eq!(dist["start"], 0);
        assert_eq!(dist.len(), 1);
    }
}
