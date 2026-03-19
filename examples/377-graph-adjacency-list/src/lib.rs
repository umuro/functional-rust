#![allow(clippy::all)]
//! Graph - Adjacency List Representation
//!
//! Space-efficient graph representation with O(V+E) storage.

use std::collections::{HashMap, HashSet, VecDeque};

/// Graph with adjacency list representation
pub struct Graph {
    adj: HashMap<usize, Vec<usize>>,
    directed: bool,
}

impl Graph {
    pub fn new(directed: bool) -> Self {
        Self {
            adj: HashMap::new(),
            directed,
        }
    }

    pub fn add_vertex(&mut self, v: usize) {
        self.adj.entry(v).or_default();
    }

    pub fn add_edge(&mut self, u: usize, v: usize) {
        self.adj.entry(u).or_default().push(v);
        if !self.directed {
            self.adj.entry(v).or_default().push(u);
        }
        self.adj.entry(v).or_default();
    }

    pub fn neighbors(&self, u: usize) -> &[usize] {
        self.adj.get(&u).map_or(&[], Vec::as_slice)
    }

    pub fn vertices(&self) -> impl Iterator<Item = &usize> {
        self.adj.keys()
    }

    pub fn vertex_count(&self) -> usize {
        self.adj.len()
    }

    pub fn bfs(&self, start: usize) -> Vec<usize> {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        let mut order = Vec::new();
        queue.push_back(start);
        visited.insert(start);
        while let Some(node) = queue.pop_front() {
            order.push(node);
            let mut neighbors: Vec<_> = self.neighbors(node).to_vec();
            neighbors.sort();
            for nb in neighbors {
                if visited.insert(nb) {
                    queue.push_back(nb);
                }
            }
        }
        order
    }

    pub fn dfs(&self, start: usize) -> Vec<usize> {
        let mut visited = HashSet::new();
        let mut stack = vec![start];
        let mut order = Vec::new();
        while let Some(node) = stack.pop() {
            if visited.insert(node) {
                order.push(node);
                let mut neighbors: Vec<_> = self.neighbors(node).to_vec();
                neighbors.sort_by(|a, b| b.cmp(a));
                for nb in neighbors {
                    if !visited.contains(&nb) {
                        stack.push(nb);
                    }
                }
            }
        }
        order
    }

    pub fn has_path(&self, from: usize, to: usize) -> bool {
        self.bfs(from).contains(&to)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_edge() {
        let mut g = Graph::new(false);
        g.add_edge(0, 1);
        g.add_edge(0, 2);
        assert!(g.neighbors(0).contains(&1));
        assert!(g.neighbors(0).contains(&2));
        assert!(g.neighbors(1).contains(&0));
    }

    #[test]
    fn test_directed() {
        let mut g = Graph::new(true);
        g.add_edge(0, 1);
        assert!(g.neighbors(0).contains(&1));
        assert!(!g.neighbors(1).contains(&0));
    }

    #[test]
    fn test_bfs() {
        let mut g = Graph::new(false);
        g.add_edge(0, 1);
        g.add_edge(0, 2);
        g.add_edge(1, 3);
        g.add_edge(2, 3);
        let order = g.bfs(0);
        assert_eq!(order[0], 0);
        assert!(order.contains(&1));
        assert!(order.contains(&2));
        assert!(order.contains(&3));
    }

    #[test]
    fn test_dfs() {
        let mut g = Graph::new(false);
        g.add_edge(0, 1);
        g.add_edge(0, 2);
        g.add_edge(1, 3);
        let order = g.dfs(0);
        assert_eq!(order[0], 0);
        assert_eq!(order.len(), 4);
    }

    #[test]
    fn test_has_path() {
        let mut g = Graph::new(false);
        g.add_edge(0, 1);
        g.add_edge(2, 3);
        assert!(g.has_path(0, 1));
        assert!(!g.has_path(0, 2));
    }
}
