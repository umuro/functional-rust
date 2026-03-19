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

fn main() {
    // Build the sample graph from the OCaml example
    let mut graph: Graph = HashMap::new();
    graph.insert("a".into(), vec![("b".into(), 1), ("c".into(), 4)]);
    graph.insert("b".into(), vec![("c".into(), 2), ("d".into(), 6)]);
    graph.insert("c".into(), vec![("d".into(), 3)]);
    graph.insert("d".into(), vec![]);

    println!("=== Idiomatic Rust (BinaryHeap) ===");
    let dist = dijkstra(&graph, "a");
    let mut results: Vec<_> = dist.iter().collect();
    results.sort();
    for (node, d) in &results {
        println!("{node}: {d}");
    }

    // Functional version with BTreeMap (deterministic ordering)
    let mut fg: FunctionalGraph = BTreeMap::new();
    fg.insert("a".into(), vec![("b".into(), 1), ("c".into(), 4)]);
    fg.insert("b".into(), vec![("c".into(), 2), ("d".into(), 6)]);
    fg.insert("c".into(), vec![("d".into(), 3)]);
    fg.insert("d".into(), vec![]);

    println!("\n=== Functional style (BTreeSet PQ, mirrors OCaml Set.Make) ===");
    let dist_f = dijkstra_functional(&fg, "a");
    for (node, d) in &dist_f {
        println!("{node}: {d}");
    }
}

/* Output:
   === Idiomatic Rust (BinaryHeap) ===
   a: 0
   b: 1
   c: 3
   d: 6

   === Functional style (BTreeSet PQ, mirrors OCaml Set.Make) ===
   a: 0
   b: 1
   c: 3
   d: 6
*/
