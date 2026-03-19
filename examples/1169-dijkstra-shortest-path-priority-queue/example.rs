use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

pub type Graph<'a> = HashMap<&'a str, Vec<(&'a str, u32)>>;

// Solution 1: Idiomatic Rust — `while let` loop with mutable state
pub fn dijkstra<'a>(graph: &Graph<'a>, start: &'a str) -> HashMap<&'a str, u32> {
    let mut dist: HashMap<&str, u32> = HashMap::new();
    dist.insert(start, 0);
    let mut heap: BinaryHeap<Reverse<(u32, &str)>> = BinaryHeap::new();
    heap.push(Reverse((0, start)));

    while let Some(Reverse((d, u))) = heap.pop() {
        if d > *dist.get(u).unwrap_or(&u32::MAX) {
            continue;
        }
        let neighbors = graph.get(u).map(Vec::as_slice).unwrap_or(&[]);
        for &(v, w) in neighbors {
            let alt = d + w;
            if alt < *dist.get(v).unwrap_or(&u32::MAX) {
                dist.insert(v, alt);
                heap.push(Reverse((alt, v)));
            }
        }
    }
    dist
}

// Solution 2: Functional style — immutable accumulator mirroring OCaml's `rec go`
pub fn dijkstra_functional<'a>(graph: &Graph<'a>, start: &'a str) -> HashMap<&'a str, u32> {
    let dist: HashMap<&str, u32> = std::iter::once((start, 0u32)).collect();
    let heap: BinaryHeap<Reverse<(u32, &str)>> =
        std::iter::once(Reverse((0u32, start))).collect();
    go(graph, heap, dist)
}

fn go<'a>(
    graph: &Graph<'a>,
    mut heap: BinaryHeap<Reverse<(u32, &'a str)>>,
    dist: HashMap<&'a str, u32>,
) -> HashMap<&'a str, u32> {
    let Some(Reverse((d, u))) = heap.pop() else {
        return dist;
    };
    if d > *dist.get(u).unwrap_or(&u32::MAX) {
        return go(graph, heap, dist);
    }
    let neighbors = graph.get(u).map(Vec::as_slice).unwrap_or(&[]);
    let (dist, heap) = neighbors.iter().fold((dist, heap), |(mut d_map, mut h), &(v, w)| {
        let alt = d + w;
        if alt < *d_map.get(v).unwrap_or(&u32::MAX) {
            d_map.insert(v, alt);
            h.push(Reverse((alt, v)));
        }
        (d_map, h)
    });
    go(graph, heap, dist)
}

fn main() {
    let mut g = Graph::new();
    g.insert("a", vec![("b", 1), ("c", 4)]);
    g.insert("b", vec![("c", 2), ("d", 6)]);
    g.insert("c", vec![("d", 3)]);
    g.insert("d", vec![]);

    let dist = dijkstra(&g, "a");
    // Sort output for deterministic display
    let mut entries: Vec<_> = dist.iter().collect();
    entries.sort_by_key(|&(k, _)| *k);
    for (node, d) in &entries {
        println!("{}: {}", node, d);
    }

    println!("---");
    let dist2 = dijkstra_functional(&g, "a");
    let mut entries2: Vec<_> = dist2.iter().collect();
    entries2.sort_by_key(|&(k, _)| *k);
    for (node, d) in &entries2 {
        println!("{}: {}", node, d);
    }
}

/* Output:
   a: 0
   b: 1
   c: 3
   d: 6
   ---
   a: 0
   b: 1
   c: 3
   d: 6
*/

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_graph<'a>() -> Graph<'a> {
        let mut g = Graph::new();
        g.insert("a", vec![("b", 1), ("c", 4)]);
        g.insert("b", vec![("c", 2), ("d", 6)]);
        g.insert("c", vec![("d", 3)]);
        g.insert("d", vec![]);
        g
    }

    #[test]
    fn test_single_node_start() {
        let mut g = Graph::new();
        g.insert("x", vec![]);
        let dist = dijkstra(&g, "x");
        assert_eq!(dist["x"], 0);
        assert_eq!(dist.len(), 1);
    }

    #[test]
    fn test_direct_edge() {
        let mut g = Graph::new();
        g.insert("a", vec![("b", 7)]);
        g.insert("b", vec![]);
        let dist = dijkstra(&g, "a");
        assert_eq!(dist["b"], 7);
    }

    #[test]
    fn test_shortest_path_prefers_indirect_route() {
        let g = sample_graph();
        let dist = dijkstra(&g, "a");
        assert_eq!(dist["c"], 3);
        assert_eq!(dist["d"], 6);
    }

    #[test]
    fn test_both_implementations_agree() {
        let g = sample_graph();
        assert_eq!(dijkstra(&g, "a"), dijkstra_functional(&g, "a"));
    }
}
