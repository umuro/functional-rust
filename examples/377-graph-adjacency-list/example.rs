use std::collections::{HashMap, VecDeque, HashSet};

struct Graph {
    adj: HashMap<usize, Vec<usize>>,
    directed: bool,
}

impl Graph {
    fn new(directed: bool) -> Self { Self { adj: HashMap::new(), directed } }

    fn add_edge(&mut self, u: usize, v: usize) {
        self.adj.entry(u).or_default().push(v);
        if !self.directed { self.adj.entry(v).or_default().push(u); }
        self.adj.entry(v).or_default(); // ensure v exists
        if self.directed { self.adj.entry(v).or_default(); }
    }

    fn neighbors(&self, u: usize) -> &[usize] {
        self.adj.get(&u).map_or(&[], Vec::as_slice)
    }

    fn bfs(&self, start: usize) -> Vec<usize> {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        let mut order = Vec::new();
        queue.push_back(start);
        visited.insert(start);
        while let Some(node) = queue.pop_front() {
            order.push(node);
            let mut neighbors: Vec<_> = self.neighbors(node).to_vec();
            neighbors.sort(); // deterministic order
            for &nb in &neighbors {
                if visited.insert(nb) { queue.push_back(nb); }
            }
        }
        order
    }

    fn dfs(&self, start: usize) -> Vec<usize> {
        let mut visited = HashSet::new();
        let mut stack = vec![start];
        let mut order = Vec::new();
        while let Some(node) = stack.pop() {
            if visited.insert(node) {
                order.push(node);
                let mut neighbors: Vec<_> = self.neighbors(node).to_vec();
                neighbors.sort_unstable_by(|a,b| b.cmp(a)); // push in reverse for correct order
                for nb in neighbors { if !visited.contains(&nb) { stack.push(nb); } }
            }
        }
        order
    }

    fn has_path(&self, from: usize, to: usize) -> bool {
        self.bfs(from).contains(&to)
    }

    fn connected_components(&self) -> Vec<Vec<usize>> {
        let mut visited = HashSet::new();
        let mut components = Vec::new();
        let mut nodes: Vec<_> = self.adj.keys().cloned().collect();
        nodes.sort();
        for &node in &nodes {
            if !visited.contains(&node) {
                let comp = self.bfs(node);
                for &n in &comp { visited.insert(n); }
                components.push(comp);
            }
        }
        components
    }
}

fn main() {
    let mut g = Graph::new(false);
    for (u,v) in [(0,1),(0,2),(1,3),(2,3),(3,4)] { g.add_edge(u,v); }

    println!("BFS from 0: {:?}", g.bfs(0));
    println!("DFS from 0: {:?}", g.dfs(0));
    println!("Path 0->4: {}", g.has_path(0,4));
    println!("Path 0->5: {}", g.has_path(0,5));

    // Disconnected graph
    let mut g2 = Graph::new(false);
    for (u,v) in [(0,1),(0,2),(3,4)] { g2.add_edge(u,v); }
    g2.adj.entry(5).or_default(); // isolated node
    println!("Components: {:?}", g2.connected_components());
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn bfs_order() {
        let mut g = Graph::new(false);
        for (u,v) in [(0,1),(0,2),(1,3),(2,3)] { g.add_edge(u,v); }
        assert_eq!(g.bfs(0), vec![0,1,2,3]);
    }
    #[test] fn has_path() {
        let mut g = Graph::new(false);
        g.add_edge(0,1); g.add_edge(1,2);
        assert!(g.has_path(0,2)); assert!(!g.has_path(0,3));
    }
    #[test] fn components() {
        let mut g = Graph::new(false);
        g.add_edge(0,1); g.add_edge(2,3);
        g.adj.entry(4).or_default();
        let comps = g.connected_components();
        assert_eq!(comps.len(), 3);
    }
}
