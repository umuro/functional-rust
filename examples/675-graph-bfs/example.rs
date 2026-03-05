// BFS in Rust
use std::collections::{HashMap, HashSet, VecDeque};

fn bfs(graph: &HashMap<i32, Vec<i32>>, start: i32) {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::from([start]);
    visited.insert(start);
    
    while let Some(v) = queue.pop_front() {
        print!("{} ", v);
        if let Some(ns) = graph.get(&v) {
            for &n in ns {
                if visited.insert(n) { queue.push_back(n); }
            }
        }
    }
}

fn main() {
    let mut g = HashMap::new();
    g.insert(1, vec![2, 3]);
    g.insert(2, vec![4]);
    g.insert(3, vec![4]);
    g.insert(4, vec![]);
    print!("BFS: "); bfs(&g, 1); println!();
}
