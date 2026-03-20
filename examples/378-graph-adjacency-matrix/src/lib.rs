#![allow(clippy::all)]
//! Graph - Adjacency Matrix Representation
//!
//! O(1) edge lookup, O(V²) storage.

/// Graph with adjacency matrix representation
pub struct Graph {
    vertices: usize,
    matrix: Vec<Vec<bool>>,
}

impl Graph {
    pub fn new(n: usize) -> Self {
        Self {
            vertices: n,
            matrix: vec![vec![false; n]; n],
        }
    }

    pub fn add_edge(&mut self, u: usize, v: usize) {
        self.matrix[u][v] = true;
        self.matrix[v][u] = true;
    }

    pub fn add_directed_edge(&mut self, u: usize, v: usize) {
        self.matrix[u][v] = true;
    }

    pub fn has_edge(&self, u: usize, v: usize) -> bool {
        self.matrix[u][v]
    }

    pub fn neighbors(&self, u: usize) -> Vec<usize> {
        (0..self.vertices).filter(|&v| self.matrix[u][v]).collect()
    }

    pub fn degree(&self, u: usize) -> usize {
        self.neighbors(u).len()
    }

    pub fn vertex_count(&self) -> usize {
        self.vertices
    }

    pub fn edge_count(&self) -> usize {
        let mut count = 0;
        for i in 0..self.vertices {
            for j in i + 1..self.vertices {
                if self.matrix[i][j] {
                    count += 1;
                }
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_edge() {
        let mut g = Graph::new(4);
        g.add_edge(0, 1);
        g.add_edge(0, 2);
        assert!(g.has_edge(0, 1));
        assert!(g.has_edge(1, 0));
        assert!(!g.has_edge(0, 3));
    }

    #[test]
    fn test_neighbors() {
        let mut g = Graph::new(4);
        g.add_edge(0, 1);
        g.add_edge(0, 2);
        g.add_edge(0, 3);
        let n = g.neighbors(0);
        assert_eq!(n.len(), 3);
    }

    #[test]
    fn test_degree() {
        let mut g = Graph::new(4);
        g.add_edge(0, 1);
        g.add_edge(0, 2);
        assert_eq!(g.degree(0), 2);
        assert_eq!(g.degree(1), 1);
    }

    #[test]
    fn test_edge_count() {
        let mut g = Graph::new(4);
        g.add_edge(0, 1);
        g.add_edge(1, 2);
        g.add_edge(2, 3);
        assert_eq!(g.edge_count(), 3);
    }

    #[test]
    fn test_directed() {
        let mut g = Graph::new(3);
        g.add_directed_edge(0, 1);
        assert!(g.has_edge(0, 1));
        assert!(!g.has_edge(1, 0));
    }
}
