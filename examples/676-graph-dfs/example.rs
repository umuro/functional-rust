// DFS in Rust
use std::collections::{HashMap, HashSet};

fn dfs(graph: &HashMap<i32, Vec<i32>>, v: i32, visited: &mut HashSet<i32>) {
    if !visited.insert(v) { return; }
    print!("{} ", v);
    if let Some(ns) = graph.get(&v) {
        for &n in ns { dfs(graph, n, visited); }
    }
}

fn main() {
    let mut g = HashMap::new();
    g.insert(1, vec![2, 3]);
    g.insert(2, vec![4]);
    g.insert(3, vec![4]);
    g.insert(4, vec![]);
    print!("DFS: ");
    dfs(&g, 1, &mut HashSet::new());
    println!();
}
