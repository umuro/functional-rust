use std::cmp::Reverse;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap};

/// Graph keyed by node name; each node maps to its (neighbour, weight) list.
pub type Graph = HashMap<String, Vec<(String, usize)>>;

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

/// Dijkstra — idiomatic Rust using `BinaryHeap<Reverse<...>>` as a min-heap.
///
/// `BinaryHeap` is Rust's standard priority queue (max-heap by default).
/// Wrapping entries in `Reverse` turns it into a min-heap, giving O(log n)
/// push and pop — more cache-friendly than a `BTreeSet` for large graphs.
///
/// `HashMap` gives O(1) distance lookups; use `BTreeMap` if sorted output
/// is needed (see `dijkstra_sorted`).
pub fn dijkstra(graph: &Graph, start: &str) -> HashMap<String, usize> {
    let mut dist: HashMap<String, usize> = HashMap::from([(start.to_string(), 0)]);
    // Reverse so the heap acts as a min-heap: smallest distance = highest priority
    let mut heap: BinaryHeap<Reverse<(usize, String)>> =
        BinaryHeap::from([Reverse((0, start.to_string()))]);

    while let Some(Reverse((d, u))) = heap.pop() {
        // Stale entry: a shorter path to u was already finalised
        if dist.get(&u).is_some_and(|&best| d > best) {
            continue;
        }

        let empty: Vec<(String, usize)> = Vec::new();
        let neighbors = graph.get(&u).unwrap_or(&empty);
        for (v, w) in neighbors {
            let alt = d + w;
            if alt < *dist.get(v).unwrap_or(&usize::MAX) {
                dist.insert(v.clone(), alt);
                heap.push(Reverse((alt, v.clone())));
            }
        }
    }

    dist
}

/// Dijkstra — functional fold style, mirroring OCaml's `List.fold_left`.
///
/// Uses `BTreeSet<(usize, String)>` as the priority queue — an ordered set
/// that mirrors OCaml's `Set.Make`: `iter().next()` gives the minimum element,
/// just like OCaml's `PQ.min_elt`. `BTreeMap` mirrors `Map.Make(String)`.
///
/// The inner neighbour loop is expressed as an iterator fold, exactly as
/// OCaml's `List.fold_left (fun (dist, pq) (v, w) -> ...) (dist, pq) neighbors`.
pub fn dijkstra_functional(graph: &Graph, start: &str) -> BTreeMap<String, usize> {
    let mut dist: BTreeMap<String, usize> = BTreeMap::from([(start.to_string(), 0)]);
    let mut pq: BTreeSet<(usize, String)> = BTreeSet::from([(0, start.to_string())]);

    while let Some((d, u)) = pq.iter().next().cloned() {
        // Mirrors OCaml: let pq = PQ.remove (d, u) pq
        pq.remove(&(d, u.clone()));

        if dist.get(&u).is_some_and(|&best| d > best) {
            continue;
        }

        let empty: Vec<(String, usize)> = Vec::new();
        let neighbors = graph.get(&u).unwrap_or(&empty);

        // Mirrors OCaml: List.fold_left (fun (dist, pq) (v, w) -> ...) (dist, pq) neighbors
        (dist, pq) = neighbors
            .iter()
            .fold((dist, pq), |(mut dist, mut pq), (v, w)| {
                let alt = d + w;
                let current = *dist.get(v).unwrap_or(&usize::MAX);
                if alt < current {
                    dist.insert(v.clone(), alt);
                    // Mirrors OCaml: PQ.add (alt, v) pq
                    pq.insert((alt, v.clone()));
                }
                (dist, pq)
            });
    }

    dist
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
        assert_eq!(dist["c"], 3); // a->b->c = 1+2=3, shorter than a->c=4
        assert_eq!(dist["d"], 6); // a->b->c->d = 1+2+3=6
    }

    #[test]
    fn test_functional_matches_idiomatic() {
        let g = sample_graph();
        let h = dijkstra(&g, "a");
        let f = dijkstra_functional(&g, "a");
        // Compare as sorted key-value pairs
        let mut h_sorted: Vec<_> = h.iter().collect();
        h_sorted.sort();
        let f_sorted: Vec<_> = f.iter().collect();
        assert_eq!(h_sorted, f_sorted);
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
    fn test_functional_fold_diamond() {
        let g = build_graph(&[
            ("a", &[("b", 1), ("c", 10)]),
            ("b", &[("d", 1)]),
            ("c", &[("d", 1)]),
            ("d", &[]),
        ]);
        let dist = dijkstra_functional(&g, "a");
        assert_eq!(dist["d"], 2);
    }
}
