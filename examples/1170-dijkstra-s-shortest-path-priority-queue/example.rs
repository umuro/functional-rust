use std::cmp::Reverse;
use std::collections::{BTreeMap, BinaryHeap};

type Graph = BTreeMap<String, Vec<(String, u32)>>;

/// Solution 1: Idiomatic Rust — BinaryHeap with Reverse for min-heap behaviour.
pub fn dijkstra(graph: &Graph, start: &str) -> BTreeMap<String, u32> {
    let mut dist: BTreeMap<String, u32> = BTreeMap::new();
    dist.insert(start.to_owned(), 0);

    let mut heap: BinaryHeap<(Reverse<u32>, String)> = BinaryHeap::new();
    heap.push((Reverse(0), start.to_owned()));

    while let Some((Reverse(d), u)) = heap.pop() {
        if dist.get(&u).is_some_and(|&best| d > best) {
            continue;
        }
        let neighbors = graph.get(&u).map(Vec::as_slice).unwrap_or(&[]);
        for (v, w) in neighbors {
            let alt = d + w;
            let current = dist.get(v).copied().unwrap_or(u32::MAX);
            if alt < current {
                dist.insert(v.clone(), alt);
                heap.push((Reverse(alt), v.clone()));
            }
        }
    }

    dist
}

/// Solution 2: Functional/recursive — mirrors OCaml's `go pq dist` tail recursion.
pub fn dijkstra_recursive(graph: &Graph, start: &str) -> BTreeMap<String, u32> {
    let mut dist = BTreeMap::new();
    dist.insert(start.to_owned(), 0u32);

    let mut pq: BTreeMap<(u32, String), ()> = BTreeMap::new();
    pq.insert((0, start.to_owned()), ());

    go(graph, pq, dist)
}

fn go(
    graph: &Graph,
    mut pq: BTreeMap<(u32, String), ()>,
    dist: BTreeMap<String, u32>,
) -> BTreeMap<String, u32> {
    let Some(((d, u), _)) = pq.pop_first() else {
        return dist;
    };

    if dist.get(&u).is_some_and(|&best| d > best) {
        return go(graph, pq, dist);
    }

    let neighbors = graph.get(&u).map(Vec::as_slice).unwrap_or(&[]);
    let (dist, pq) = neighbors
        .iter()
        .fold((dist, pq), |(mut d_map, mut q), (v, w)| {
            let alt = d + w;
            let current = d_map.get(v).copied().unwrap_or(u32::MAX);
            if alt < current {
                d_map.insert(v.clone(), alt);
                q.insert((alt, v.clone()), ());
            }
            (d_map, q)
        });

    go(graph, pq, dist)
}

fn build_graph(edges: &[(&str, &[(&str, u32)])]) -> Graph {
    edges
        .iter()
        .map(|(node, neighbours)| {
            let ns = neighbours
                .iter()
                .map(|(n, w)| ((*n).to_owned(), *w))
                .collect();
            ((*node).to_owned(), ns)
        })
        .collect()
}

fn main() {
    let g = build_graph(&[
        ("a", &[("b", 1), ("c", 4)]),
        ("b", &[("c", 2), ("d", 6)]),
        ("c", &[("d", 3)]),
        ("d", &[]),
    ]);

    println!("=== Idiomatic (BinaryHeap) ===");
    let dist = dijkstra(&g, "a");
    for (node, d) in &dist {
        println!("{node}: {d}");
    }

    println!("\n=== Recursive (BTreeMap PQ) ===");
    let dist2 = dijkstra_recursive(&g, "a");
    for (node, d) in &dist2 {
        println!("{node}: {d}");
    }
}

/* Output:
   === Idiomatic (BinaryHeap) ===
   a: 0
   b: 1
   c: 3
   d: 6

   === Recursive (BTreeMap PQ) ===
   a: 0
   b: 1
   c: 3
   d: 6
*/
