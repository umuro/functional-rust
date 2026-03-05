// Graph as adjacency matrix in Rust
use std::fmt;

struct Graph {
    vertices: usize,
    matrix: Vec<Vec<bool>>,
}

impl Graph {
    fn new(n: usize) -> Self {
        Graph {
            vertices: n,
            matrix: vec![vec![false; n]; n],
        }
    }

    fn add_edge(&mut self, u: usize, v: usize) {
        self.matrix[u][v] = true;
        self.matrix[v][u] = true; // undirected
    }

    fn has_edge(&self, u: usize, v: usize) -> bool {
        self.matrix[u][v]
    }

    fn neighbors(&self, u: usize) -> Vec<usize> {
        (0..self.vertices)
            .filter(|&v| self.matrix[u][v])
            .collect()
    }

    fn degree(&self, u: usize) -> usize {
        self.neighbors(u).len()
    }
}

impl fmt::Display for Graph {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "  ")?;
        for i in 0..self.vertices {
            write!(f, "{} ", i)?;
        }
        writeln!(f)?;
        for i in 0..self.vertices {
            write!(f, "{} ", i)?;
            for j in 0..self.vertices {
                write!(f, "{} ", if self.matrix[i][j] { 1 } else { 0 })?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn main() {
    let mut g = Graph::new(5);
    g.add_edge(0, 1);
    g.add_edge(0, 2);
    g.add_edge(1, 3);
    g.add_edge(2, 3);
    g.add_edge(3, 4);

    println!("Graph adjacency matrix (5 vertices):");
    print!("{}", g);

    println!("Neighbors of vertex 3: {:?}", g.neighbors(3));
    println!("Has edge 0-1: {}", g.has_edge(0, 1));
    println!("Has edge 0-4: {}", g.has_edge(0, 4));
    println!("Degree of vertex 3: {}", g.degree(3));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_and_query_edge() {
        let mut g = Graph::new(4);
        g.add_edge(0, 1);
        assert!(g.has_edge(0, 1));
        assert!(g.has_edge(1, 0)); // undirected
        assert!(!g.has_edge(0, 2));
    }

    #[test]
    fn test_neighbors() {
        let mut g = Graph::new(5);
        g.add_edge(2, 0);
        g.add_edge(2, 3);
        g.add_edge(2, 4);
        let mut n = g.neighbors(2);
        n.sort();
        assert_eq!(n, vec![0, 3, 4]);
    }

    #[test]
    fn test_degree() {
        let mut g = Graph::new(4);
        g.add_edge(1, 0);
        g.add_edge(1, 2);
        g.add_edge(1, 3);
        assert_eq!(g.degree(1), 3);
    }
}
