//! # Graph Basics
//!
//! Fundamental graph representations and operations.

use std::collections::{HashMap, HashSet};

/// Adjacency list representation
#[derive(Debug, Clone)]
pub struct Graph<V: Eq + std::hash::Hash + Clone> {
    pub adj: HashMap<V, Vec<V>>,
    pub directed: bool,
}

impl<V: Eq + std::hash::Hash + Clone> Graph<V> {
    pub fn new(directed: bool) -> Self {
        Graph { adj: HashMap::new(), directed }
    }
    
    pub fn add_vertex(&mut self, v: V) {
        self.adj.entry(v).or_insert_with(Vec::new);
    }
    
    pub fn add_edge(&mut self, from: V, to: V) {
        self.adj.entry(from.clone()).or_default().push(to.clone());
        if !self.directed {
            self.adj.entry(to).or_default().push(from);
        }
    }
    
    pub fn neighbors(&self, v: &V) -> Option<&Vec<V>> {
        self.adj.get(v)
    }
    
    pub fn vertex_count(&self) -> usize {
        self.adj.len()
    }
    
    pub fn edge_count(&self) -> usize {
        let count: usize = self.adj.values().map(|v| v.len()).sum();
        if self.directed { count } else { count / 2 }
    }
}

/// Weighted graph
#[derive(Debug, Clone)]
pub struct WeightedGraph<V: Eq + std::hash::Hash + Clone> {
    pub adj: HashMap<V, Vec<(V, i32)>>,
}

impl<V: Eq + std::hash::Hash + Clone> WeightedGraph<V> {
    pub fn new() -> Self {
        WeightedGraph { adj: HashMap::new() }
    }
    
    pub fn add_edge(&mut self, from: V, to: V, weight: i32) {
        self.adj.entry(from).or_default().push((to, weight));
    }
}

impl<V: Eq + std::hash::Hash + Clone> Default for WeightedGraph<V> {
    fn default() -> Self { Self::new() }
}

/// Adjacency matrix representation
pub struct AdjMatrix {
    pub matrix: Vec<Vec<bool>>,
    pub n: usize,
}

impl AdjMatrix {
    pub fn new(n: usize) -> Self {
        AdjMatrix { matrix: vec![vec![false; n]; n], n }
    }
    
    pub fn add_edge(&mut self, from: usize, to: usize) {
        self.matrix[from][to] = true;
    }
    
    pub fn has_edge(&self, from: usize, to: usize) -> bool {
        self.matrix[from][to]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_graph() {
        let mut g: Graph<i32> = Graph::new(false);
        g.add_edge(1, 2);
        g.add_edge(2, 3);
        assert_eq!(g.vertex_count(), 3);
        assert_eq!(g.edge_count(), 2);
    }

    #[test]
    fn test_directed() {
        let mut g: Graph<i32> = Graph::new(true);
        g.add_edge(1, 2);
        g.add_edge(2, 3);
        assert_eq!(g.edge_count(), 2);
    }

    #[test]
    fn test_weighted() {
        let mut g: WeightedGraph<i32> = WeightedGraph::new();
        g.add_edge(1, 2, 5);
        assert_eq!(g.adj[&1][0], (2, 5));
    }

    #[test]
    fn test_matrix() {
        let mut m = AdjMatrix::new(3);
        m.add_edge(0, 1);
        assert!(m.has_edge(0, 1));
        assert!(!m.has_edge(1, 0));
    }
}
