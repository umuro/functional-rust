use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

pub type Graph = HashMap<String, Vec<(String, usize)>>;

/// Dijkstra's shortest path — idiomatic Rust.
/// Uses BinaryHeap<Reverse<...>> as a min-heap (OCaml uses Set.Make as ordered priority queue).
/// Stale-entry detection: if we pop a distance > what we already recorded, skip it.
pub fn dijkstra(graph: &Graph, start: &str) -> HashMap<String, usize> {
    let mut dist: HashMap<String, usize> = HashMap::from([(start.to_string(), 0)]);
    let mut heap: BinaryHeap<Reverse<(usize, String)>> =
        BinaryHeap::from([Reverse((0, start.to_string()))]);

    while let Some(Reverse((d, u))) = heap.pop() {
        // Skip stale entries — a shorter path was already found
        if dist.get(&u).is_some_and(|&best| d > best) {
            continue;
        }

        let Some(neighbors) = graph.get(&u) else {
            continue;
        };

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

/// Dijkstra's shortest path — functional style using iterator combinators.
/// Mirrors OCaml's List.fold_left over neighbors; collects relaxed edges before
/// mutating dist/heap to satisfy the borrow checker.
pub fn dijkstra_functional(graph: &Graph, start: &str) -> HashMap<String, usize> {
    let mut dist: HashMap<String, usize> = HashMap::from([(start.to_string(), 0)]);
    let mut heap: BinaryHeap<Reverse<(usize, String)>> =
        BinaryHeap::from([Reverse((0, start.to_string()))]);

    while let Some(Reverse((d, u))) = heap.pop() {
        if dist.get(&u).is_some_and(|&best| d > best) {
            continue;
        }

        // Compute all edge relaxations as (node, new_dist) pairs — mirrors OCaml fold_left
        let relaxed: Vec<(String, usize)> = graph
            .get(&u)
            .into_iter()
            .flatten()
            .filter_map(|(v, w)| {
                let alt = d + w;
                (alt < *dist.get(v).unwrap_or(&usize::MAX)).then_some((v.clone(), alt))
            })
            .collect();

        dist.extend(relaxed.iter().cloned());
        heap.extend(relaxed.into_iter().map(|(v, alt)| Reverse((alt, v))));
    }

    dist
}

/// Build a graph from a slice of (from, [(to, weight)]) pairs — convenience for tests.
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
        assert_eq!(dist["c"], 3); // a->b->c = 1+2 = 3, shorter than a->c = 4
        assert_eq!(dist["d"], 6); // a->b->c->d = 1+2+3 = 6
    }

    #[test]
    fn test_functional_matches_idiomatic() {
        let g = sample_graph();
        let d1 = dijkstra(&g, "a");
        let d2 = dijkstra_functional(&g, "a");
        assert_eq!(d1, d2);
    }

    #[test]
    fn test_single_node_graph() {
        let g = build_graph(&[("x", &[])]);
        let dist = dijkstra(&g, "x");
        assert_eq!(dist["x"], 0);
        assert_eq!(dist.len(), 1);
    }

    #[test]
    fn test_unreachable_nodes_not_in_result() {
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
        // Diamond graph: a->b (1), a->c (10), b->d (1), c->d (1)
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
}
