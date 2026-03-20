use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap};
use std::cmp::Reverse;

// Idiomatic Rust: min-heap (BinaryHeap + Reverse) + HashMap.
pub fn dijkstra(graph: &HashMap<String, Vec<(String, u32)>>, start: &str) -> HashMap<String, u32> {
    let mut dist: HashMap<String, u32> = HashMap::new();
    let mut heap: BinaryHeap<Reverse<(u32, String)>> = BinaryHeap::new();

    dist.insert(start.to_string(), 0);
    heap.push(Reverse((0, start.to_string())));

    while let Some(Reverse((d, u))) = heap.pop() {
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

// Functional/recursive — mirrors OCaml's Set.Make + Map.Make + tail-recursive go.
pub fn dijkstra_functional(
    graph: &BTreeMap<String, Vec<(String, u32)>>,
    start: &str,
) -> BTreeMap<String, u32> {
    let dist = BTreeMap::from([(start.to_string(), 0u32)]);
    let pq = BTreeSet::from([(0u32, start.to_string())]);
    go(graph, pq, dist)
}

fn go(
    graph: &BTreeMap<String, Vec<(String, u32)>>,
    mut pq: BTreeSet<(u32, String)>,
    dist: BTreeMap<String, u32>,
) -> BTreeMap<String, u32> {
    match pq.iter().next().cloned() {
        None => dist,
        Some((d, u)) => {
            pq.remove(&(d, u.clone()));
            let empty = vec![];
            let neighbors = graph.get(&u).unwrap_or(&empty);
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

fn main() {
    // Build graph: a->b:1, a->c:4, b->c:2, b->d:6, c->d:3
    let mut graph: HashMap<String, Vec<(String, u32)>> = HashMap::new();
    graph.insert("a".into(), vec![("b".into(), 1), ("c".into(), 4)]);
    graph.insert("b".into(), vec![("c".into(), 2), ("d".into(), 6)]);
    graph.insert("c".into(), vec![("d".into(), 3)]);
    graph.insert("d".into(), vec![]);

    let dist = dijkstra(&graph, "a");
    let mut pairs: Vec<_> = dist.iter().collect();
    pairs.sort();
    println!("=== Idiomatic (BinaryHeap + HashMap) ===");
    for (node, d) in &pairs {
        println!("{node}: {d}");
    }

    // Functional version with BTreeMap/BTreeSet
    let mut fgraph: BTreeMap<String, Vec<(String, u32)>> = BTreeMap::new();
    fgraph.insert("a".into(), vec![("b".into(), 1), ("c".into(), 4)]);
    fgraph.insert("b".into(), vec![("c".into(), 2), ("d".into(), 6)]);
    fgraph.insert("c".into(), vec![("d".into(), 3)]);
    fgraph.insert("d".into(), vec![]);

    let dist2 = dijkstra_functional(&fgraph, "a");
    println!("\n=== Functional/recursive (BTreeSet + BTreeMap) ===");
    for (node, d) in &dist2 {
        println!("{node}: {d}");
    }
}

/* Output:
   === Idiomatic (BinaryHeap + HashMap) ===
   a: 0
   b: 1
   c: 3
   d: 6

   === Functional/recursive (BTreeSet + BTreeMap) ===
   a: 0
   b: 1
   c: 3
   d: 6
*/
