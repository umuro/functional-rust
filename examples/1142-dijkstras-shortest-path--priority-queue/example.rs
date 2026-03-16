use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

pub type Graph<'a> = HashMap<&'a str, Vec<(&'a str, u32)>>;

/// Idiomatic Rust: BinaryHeap with Reverse for min-heap behaviour.
pub fn dijkstra<'a>(graph: &'a Graph, start: &'a str) -> HashMap<&'a str, u32> {
    let mut dist: HashMap<&str, u32> = HashMap::new();
    dist.insert(start, 0);

    let mut heap: BinaryHeap<(Reverse<u32>, &str)> = BinaryHeap::new();
    heap.push((Reverse(0), start));

    while let Some((Reverse(d), u)) = heap.pop() {
        if dist.get(u).is_some_and(|&best| d > best) {
            continue;
        }
        let neighbors = graph.get(u).map(Vec::as_slice).unwrap_or(&[]);
        for &(v, w) in neighbors {
            let alt = d + w;
            let current = dist.get(v).copied().unwrap_or(u32::MAX);
            if alt < current {
                dist.insert(v, alt);
                heap.push((Reverse(alt), v));
            }
        }
    }

    dist
}

/// Functional Rust: mirrors OCaml's tail-recursive `go` with fold over neighbours.
pub fn dijkstra_functional<'a>(graph: &'a Graph, start: &'a str) -> HashMap<&'a str, u32> {
    let dist: HashMap<&str, u32> = std::iter::once((start, 0)).collect();
    let mut heap: BinaryHeap<(Reverse<u32>, &str)> = BinaryHeap::new();
    heap.push((Reverse(0), start));
    go(graph, heap, dist)
}

fn go<'a>(
    graph: &'a Graph,
    mut heap: BinaryHeap<(Reverse<u32>, &'a str)>,
    dist: HashMap<&'a str, u32>,
) -> HashMap<&'a str, u32> {
    match heap.pop() {
        None => dist,
        Some((Reverse(d), u)) => {
            if dist.get(u).is_some_and(|&best| d > best) {
                return go(graph, heap, dist);
            }
            let neighbors = graph.get(u).map(Vec::as_slice).unwrap_or(&[]);
            let (dist, heap) =
                neighbors
                    .iter()
                    .fold((dist, heap), |(mut dist, mut heap), &(v, w)| {
                        let alt = d + w;
                        let current = dist.get(v).copied().unwrap_or(u32::MAX);
                        if alt < current {
                            dist.insert(v, alt);
                            heap.push((Reverse(alt), v));
                        }
                        (dist, heap)
                    });
            go(graph, heap, dist)
        }
    }
}

fn main() {
    let mut g: Graph = HashMap::new();
    g.insert("a", vec![("b", 1), ("c", 4)]);
    g.insert("b", vec![("c", 2), ("d", 6)]);
    g.insert("c", vec![("d", 3)]);
    g.insert("d", vec![]);

    let dist = dijkstra(&g, "a");
    let mut sorted: Vec<_> = dist.iter().collect();
    sorted.sort_by_key(|&(k, _)| k);
    println!("=== dijkstra (imperative) ===");
    for (node, d) in &sorted {
        println!("{}: {}", node, d);
    }

    let dist2 = dijkstra_functional(&g, "a");
    let mut sorted2: Vec<_> = dist2.iter().collect();
    sorted2.sort_by_key(|&(k, _)| k);
    println!("\n=== dijkstra_functional (recursive/OCaml-style) ===");
    for (node, d) in &sorted2 {
        println!("{}: {}", node, d);
    }
}

/* Output:
   === dijkstra (imperative) ===
   a: 0
   b: 1
   c: 3
   d: 6

   === dijkstra_functional (recursive/OCaml-style) ===
   a: 0
   b: 1
   c: 3
   d: 6
*/
