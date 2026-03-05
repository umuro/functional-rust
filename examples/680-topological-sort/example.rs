// Topological Sort (Kahn's algorithm)
use std::collections::{HashMap, VecDeque};

fn topo_sort(graph: &HashMap<usize, Vec<usize>>, n: usize) -> Option<Vec<usize>> {
    let mut in_deg = vec![0; n];
    for ns in graph.values() { for &v in ns { in_deg[v] += 1; } }
    let mut queue: VecDeque<_> = (0..n).filter(|&i| in_deg[i] == 0).collect();
    let mut result = vec![];
    while let Some(u) = queue.pop_front() {
        result.push(u);
        if let Some(ns) = graph.get(&u) {
            for &v in ns { in_deg[v] -= 1; if in_deg[v] == 0 { queue.push_back(v); } }
        }
    }
    if result.len() == n { Some(result) } else { None }
}

fn main() {
    let mut g = HashMap::new();
    g.insert(0, vec![1, 2]);
    g.insert(1, vec![3]);
    g.insert(2, vec![3]);
    g.insert(3, vec![]);
    println!("Order: {:?}", topo_sort(&g, 4));
}
