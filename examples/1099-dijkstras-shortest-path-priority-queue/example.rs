use std::cmp::Reverse;
use std::collections::{BinaryHeap, BTreeSet, HashMap};

/// Adjacency list representation: node -> [(neighbor, weight)]
type Graph<'a> = HashMap<&'a str, Vec<(&'a str, u64)>>;

// ---------------------------------------------------------------------------
// Solution 1: Idiomatic Rust — BinaryHeap as a min-priority queue
// ---------------------------------------------------------------------------
fn dijkstra_idiomatic<'a>(graph: &Graph<'a>, start: &'a str) -> HashMap<&'a str, u64> {
    let mut dist: HashMap<&'a str, u64> = HashMap::new();
    let mut heap: BinaryHeap<Reverse<(u64, &'a str)>> = BinaryHeap::new();

    dist.insert(start, 0);
    heap.push(Reverse((0, start)));

    while let Some(Reverse((d, u))) = heap.pop() {
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
// Uses BTreeSet as ordered set (like OCaml's Set.Make) with fold accumulation
// ---------------------------------------------------------------------------
fn dijkstra_functional<'a>(graph: &Graph<'a>, start: &'a str) -> HashMap<&'a str, u64> {
    type Pq<'a> = BTreeSet<(u64, &'a str)>;

    fn go<'a>(graph: &Graph<'a>, pq: Pq<'a>, dist: HashMap<&'a str, u64>) -> HashMap<&'a str, u64> {
        let &(d, u) = match pq.iter().next() {
            None => return dist,
            Some(min) => min,
        };

        let mut pq = pq;
        pq.remove(&(d, u));

        if d > *dist.get(u).unwrap_or(&u64::MAX) {
            return go(graph, pq, dist);
        }

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

fn build_graph<'a>(edges: &[(&'a str, Vec<(&'a str, u64)>)]) -> Graph<'a> {
    edges.iter().cloned().collect()
}

fn main() {
    let g = build_graph(&[
        ("a", vec![("b", 1), ("c", 4)]),
        ("b", vec![("c", 2), ("d", 6)]),
        ("c", vec![("d", 3)]),
        ("d", vec![]),
    ]);

    println!("=== Idiomatic (BinaryHeap) ===");
    let dist = dijkstra_idiomatic(&g, "a");
    let mut entries: Vec<_> = dist.iter().collect();
    entries.sort_by_key(|&(k, _)| *k);
    for (node, cost) in &entries {
        println!("  {}: {}", node, cost);
    }

    println!("\n=== Functional (BTreeSet + recursion) ===");
    let dist = dijkstra_functional(&g, "a");
    let mut entries: Vec<_> = dist.iter().collect();
    entries.sort_by_key(|&(k, _)| *k);
    for (node, cost) in &entries {
        println!("  {}: {}", node, cost);
    }
}

/* Output:
   === Idiomatic (BinaryHeap) ===
     a: 0
     b: 1
     c: 3
     d: 6

   === Functional (BTreeSet + recursion) ===
     a: 0
     b: 1
     c: 3
     d: 6
*/
