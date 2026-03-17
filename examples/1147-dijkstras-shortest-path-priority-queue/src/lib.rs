use std::collections::{BTreeMap, BTreeSet};

/// Graph keyed by node name; each node maps to its (neighbour, weight) list.
/// BTreeMap mirrors OCaml's `Map.Make(String)` — sorted, deterministic iteration.
pub type Graph = BTreeMap<String, Vec<(String, usize)>>;

/// Dijkstra using `BTreeSet<(usize, String)>` as a priority queue.
///
/// This mirrors OCaml's `Set.Make` idiom: an ordered set doubles as a min-heap.
/// `BTreeSet::iter().next()` gives the minimum element in O(log n), just like
/// OCaml's `PQ.min_elt`. `BTreeSet::remove` mirrors `PQ.remove`.
///
/// `BTreeMap` for distances gives sorted output like OCaml's `Map.Make(String)`.
pub fn dijkstra(graph: &Graph, start: &str) -> BTreeMap<String, usize> {
    let mut dist: BTreeMap<String, usize> = BTreeMap::from([(start.to_string(), 0)]);
    let mut pq: BTreeSet<(usize, String)> = BTreeSet::from([(0, start.to_string())]);

    while let Some((d, u)) = pq.iter().next().cloned() {
        // Mirrors OCaml: let pq = PQ.remove (d, u) pq
        pq.remove(&(d, u.clone()));

        // Stale entry: a shorter path to u was already finalised
        if dist.get(&u).is_some_and(|&best| d > best) {
            continue;
        }

        let neighbors = graph.get(&u).map(Vec::as_slice).unwrap_or(&[]);
        for (v, w) in neighbors {
            let alt = d + w;
            if alt < *dist.get(v).unwrap_or(&usize::MAX) {
                dist.insert(v.clone(), alt);
                // Mirrors OCaml: PQ.add (alt, v) pq
                pq.insert((alt, v.clone()));
            }
        }
    }

    dist
}

/// Dijkstra — recursive functional style.
///
/// Mirrors OCaml's `let rec go pq dist = if PQ.is_empty pq then dist else ...`
/// and `List.fold_left` over neighbours.
///
/// # Note
/// Rust does not guarantee tail-call optimisation; deep graphs may stack-overflow.
/// Use `dijkstra` for production use; this variant demonstrates the OCaml parallel.
pub fn dijkstra_recursive(graph: &Graph, start: &str) -> BTreeMap<String, usize> {
    let dist = BTreeMap::from([(start.to_string(), 0)]);
    let pq = BTreeSet::from([(0usize, start.to_string())]);
    go(graph, pq, dist)
}

/// Recursive worker — mirrors OCaml's `let rec go pq dist = ...`
fn go(
    graph: &Graph,
    mut pq: BTreeSet<(usize, String)>,
    dist: BTreeMap<String, usize>,
) -> BTreeMap<String, usize> {
    // if PQ.is_empty pq then dist
    let Some((d, u)) = pq.iter().next().cloned() else {
        return dist;
    };
    // let pq = PQ.remove (d, u) pq
    pq.remove(&(d, u.clone()));

    let neighbors = graph.get(&u).map(Vec::as_slice).unwrap_or(&[]);

    // Mirrors: List.fold_left (fun (dist, pq) (v, w) -> ...) (dist, pq) neighbors
    let (dist, pq) = neighbors
        .iter()
        .fold((dist, pq), |(mut dist, mut pq), (v, w)| {
            let alt = d + w;
            let current = *dist.get(v).unwrap_or(&usize::MAX);
            if alt < current {
                dist.insert(v.clone(), alt);
                pq.insert((alt, v.clone()));
            }
            (dist, pq)
        });

    go(graph, pq, dist)
}

/// Build a `Graph` from a slice of `(from, [(to, weight)])` pairs — convenience for tests.
pub fn build_graph(edges: &[(&str, &[(&str, usize)])]) -> Graph {
    edges
        .iter()
        .map(|(from, neighbors)| {
            let ns = neighbors
                .iter()
                .map(|(to, w)| ((*to).to_string(), *w))
                .collect();
            ((*from).to_string(), ns)
        })
        .collect()
}

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
        let dist = dijkstra(&sample_graph(), "a");
        assert_eq!(dist["a"], 0);
        assert_eq!(dist["b"], 1);
        assert_eq!(dist["c"], 3); // a->b->c = 1+2=3, shorter than a->c = 4
        assert_eq!(dist["d"], 6); // a->b->c->d = 1+2+3=6
    }

    #[test]
    fn test_recursive_matches_iterative() {
        let g = sample_graph();
        assert_eq!(dijkstra(&g, "a"), dijkstra_recursive(&g, "a"));
    }

    #[test]
    fn test_single_node_graph() {
        let g = build_graph(&[("x", &[])]);
        let dist = dijkstra(&g, "x");
        assert_eq!(dist["x"], 0);
        assert_eq!(dist.len(), 1);
    }

    #[test]
    fn test_unreachable_nodes_absent() {
        let g = build_graph(&[
            ("a", &[("b", 5)]),
            ("c", &[("d", 2)]), // disconnected component
            ("b", &[]),
            ("d", &[]),
        ]);
        let dist = dijkstra(&g, "a");
        assert_eq!(dist.get("a"), Some(&0));
        assert_eq!(dist.get("b"), Some(&5));
        assert_eq!(dist.get("c"), None); // unreachable from "a"
        assert_eq!(dist.get("d"), None);
    }

    #[test]
    fn test_multiple_paths_picks_shortest() {
        // Diamond: a->b (1), a->c (10), b->d (1), c->d (1)
        // Shortest to d: a->b->d = 2, not a->c->d = 11
        let g = build_graph(&[
            ("a", &[("b", 1), ("c", 10)]),
            ("b", &[("d", 1)]),
            ("c", &[("d", 1)]),
            ("d", &[]),
        ]);
        let dist = dijkstra(&g, "a");
        assert_eq!(dist["d"], 2);
    }

    #[test]
    fn test_btreemap_output_is_sorted() {
        // BTreeMap mirrors OCaml's Map.Make — iteration order is always sorted
        let dist = dijkstra(&sample_graph(), "a");
        let keys: Vec<_> = dist.keys().cloned().collect();
        let mut expected = keys.clone();
        expected.sort();
        assert_eq!(keys, expected);
    }
}
