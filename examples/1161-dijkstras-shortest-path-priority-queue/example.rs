use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

/// Graph: adjacency list mapping node name → list of (neighbor, weight)
pub type Graph = HashMap<String, Vec<(String, u32)>>;

/// Solution 1: Idiomatic Rust — BinaryHeap as min-heap via Reverse
///
/// BinaryHeap is a max-heap; wrapping entries in Reverse<(dist, node)>
/// turns it into a min-heap so we always expand the closest node first.
/// HashMap tracks best-known distance; we skip stale queue entries.
pub fn dijkstra(graph: &Graph, start: &str) -> HashMap<String, u32> {
    let mut dist: HashMap<String, u32> = HashMap::new();
    dist.insert(start.to_string(), 0);

    // Reverse makes BinaryHeap behave as a min-heap
    let mut heap: BinaryHeap<Reverse<(u32, String)>> = BinaryHeap::new();
    heap.push(Reverse((0, start.to_string())));

    while let Some(Reverse((d, u))) = heap.pop() {
        // Stale entry: a shorter path to `u` was already found
        if dist.get(&u).copied().unwrap_or(u32::MAX) < d {
            continue;
        }
        let neighbors = graph.get(&u).map(Vec::as_slice).unwrap_or(&[]);
        for (v, w) in neighbors {
            let alt = d + w;
            if alt < dist.get(v).copied().unwrap_or(u32::MAX) {
                dist.insert(v.clone(), alt); // clone: String is not Copy
                heap.push(Reverse((alt, v.clone())));
            }
        }
    }

    dist
}

/// Solution 2: Functional-style — recursive `go` mirroring the OCaml structure
///
/// The OCaml code uses a recursive `go pq dist` function with immutable maps.
/// Here we mirror that shape: pop from the priority queue, fold over neighbors,
/// then tail-recurse. Rust has no TCO, so this is illustrative for small graphs.
pub fn dijkstra_functional(graph: &Graph, start: &str) -> HashMap<String, u32> {
    let mut dist: HashMap<String, u32> = HashMap::new();
    dist.insert(start.to_string(), 0);

    let mut pq: BinaryHeap<Reverse<(u32, String)>> = BinaryHeap::new();
    pq.push(Reverse((0, start.to_string())));

    go(graph, pq, dist)
}

// Mirrors OCaml: `let rec go pq dist = if PQ.is_empty pq then dist else ...`
fn go(
    graph: &Graph,
    mut pq: BinaryHeap<Reverse<(u32, String)>>,
    dist: HashMap<String, u32>,
) -> HashMap<String, u32> {
    match pq.pop() {
        None => dist,
        Some(Reverse((d, u))) => {
            // Skip stale entries, just like the OCaml version ignores outdated dist
            if dist.get(&u).copied().unwrap_or(u32::MAX) < d {
                return go(graph, pq, dist);
            }
            let neighbors = graph.get(&u).map(Vec::as_slice).unwrap_or(&[]);
            // fold mirrors OCaml: List.fold_left (fun (dist, pq) (v, w) -> ...) ...
            let (dist, pq) = neighbors
                .iter()
                .fold((dist, pq), |(mut dist, mut pq), (v, w)| {
                    let alt = d + w;
                    if alt < dist.get(v).copied().unwrap_or(u32::MAX) {
                        dist.insert(v.clone(), alt);
                        pq.push(Reverse((alt, v.clone())));
                    }
                    (dist, pq)
                });
            go(graph, pq, dist)
        }
    }
}

fn make_graph(edges: &[(&str, &[(&str, u32)])]) -> Graph {
    edges
        .iter()
        .map(|(node, neighbors)| {
            (
                node.to_string(),
                neighbors.iter().map(|(n, w)| (n.to_string(), *w)).collect(),
            )
        })
        .collect()
}

fn main() {
    // Graph from the OCaml original
    let g = make_graph(&[
        ("a", &[("b", 1), ("c", 4)]),
        ("b", &[("c", 2), ("d", 6)]),
        ("c", &[("d", 3)]),
        ("d", &[]),
    ]);

    println!("=== Idiomatic Dijkstra ===");
    let mut dist: Vec<_> = dijkstra(&g, "a").into_iter().collect();
    dist.sort_by_key(|(k, _)| k.clone());
    for (node, d) in &dist {
        println!("{}: {}", node, d);
    }

    println!("\n=== Functional-style Dijkstra ===");
    let mut dist2: Vec<_> = dijkstra_functional(&g, "a").into_iter().collect();
    dist2.sort_by_key(|(k, _)| k.clone());
    for (node, d) in &dist2 {
        println!("{}: {}", node, d);
    }
}

/* Output:
   === Idiomatic Dijkstra ===
   a: 0
   b: 1
   c: 3
   d: 6

   === Functional-style Dijkstra ===
   a: 0
   b: 1
   c: 3
   d: 6
*/
