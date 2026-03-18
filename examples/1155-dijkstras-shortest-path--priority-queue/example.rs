use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

pub type Graph<'a> = HashMap<&'a str, Vec<(&'a str, u32)>>;

pub fn dijkstra<'a>(graph: &'a Graph, start: &'a str) -> HashMap<&'a str, u32> {
    let mut dist: HashMap<&str, u32> = HashMap::new();
    dist.insert(start, 0);

    let mut heap: BinaryHeap<Reverse<(u32, &str)>> = BinaryHeap::new();
    heap.push(Reverse((0, start)));

    while let Some(Reverse((d, u))) = heap.pop() {
        if dist.get(u).copied().unwrap_or(u32::MAX) < d {
            continue;
        }

        let neighbors = graph.get(u).map(Vec::as_slice).unwrap_or(&[]);
        for &(v, w) in neighbors {
            let alt = d + w;
            let current = dist.get(v).copied().unwrap_or(u32::MAX);
            if alt < current {
                dist.insert(v, alt);
                heap.push(Reverse((alt, v)));
            }
        }
    }

    dist
}

fn main() {
    let mut g: Graph = HashMap::new();
    g.insert("a", vec![("b", 1), ("c", 4)]);
    g.insert("b", vec![("c", 2), ("d", 6)]);
    g.insert("c", vec![("d", 3)]);
    g.insert("d", vec![]);

    let dist = dijkstra(&g, "a");

    // Sort for deterministic output
    let mut entries: Vec<_> = dist.iter().collect();
    entries.sort_by_key(|(k, _)| **k);
    for (node, d) in entries {
        println!("{node}: {d}");
    }
}

/* Output:
   a: 0
   b: 1
   c: 3
   d: 6
*/
