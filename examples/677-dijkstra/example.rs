// Dijkstra's algorithm
use std::collections::{BinaryHeap, HashMap};
use std::cmp::Reverse;

fn dijkstra(graph: &HashMap<usize, Vec<(usize, i32)>>, start: usize, n: usize) -> Vec<i32> {
    let mut dist = vec![i32::MAX; n];
    let mut heap = BinaryHeap::new();
    dist[start] = 0;
    heap.push(Reverse((0, start)));
    while let Some(Reverse((d, u))) = heap.pop() {
        if d > dist[u] { continue; }
        if let Some(ns) = graph.get(&u) {
            for &(v, w) in ns {
                if d + w < dist[v] { dist[v] = d + w; heap.push(Reverse((dist[v], v))); }
            }
        }
    }
    dist
}

fn main() {
    let mut g = HashMap::new();
    g.insert(0, vec![(1, 4), (2, 1)]);
    g.insert(2, vec![(1, 2)]);
    let dist = dijkstra(&g, 0, 3);
    println!("Distances: {:?}", dist);
}
