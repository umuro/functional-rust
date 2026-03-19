// 1038: Adjacency Matrix — Dense Graph Operations
// Vec<Vec<bool>> for O(1) edge lookup

/// Adjacency matrix graph
struct MatrixGraph {
    matrix: Vec<Vec<bool>>,
    size: usize,
}

impl MatrixGraph {
    fn new(n: usize) -> Self {
        MatrixGraph {
            matrix: vec![vec![false; n]; n],
            size: n,
        }
    }

    fn add_edge(&mut self, from: usize, to: usize) {
        self.matrix[from][to] = true;
    }

    fn has_edge(&self, from: usize, to: usize) -> bool {
        self.matrix[from][to]
    }

    fn neighbors(&self, node: usize) -> Vec<usize> {
        self.matrix[node]
            .iter()
            .enumerate()
            .filter_map(|(i, &connected)| if connected { Some(i) } else { None })
            .collect()
    }

    fn out_degree(&self, node: usize) -> usize {
        self.matrix[node].iter().filter(|&&c| c).count()
    }

    fn in_degree(&self, node: usize) -> usize {
        (0..self.size).filter(|&i| self.matrix[i][node]).count()
    }

    /// Warshall's transitive closure
    fn transitive_closure(&self) -> Vec<Vec<bool>> {
        let n = self.size;
        let mut tc: Vec<Vec<bool>> = self.matrix.clone();

        for k in 0..n {
            for i in 0..n {
                for j in 0..n {
                    if tc[i][k] && tc[k][j] {
                        tc[i][j] = true;
                    }
                }
            }
        }
        tc
    }
}

fn basic_matrix() {
    let mut g = MatrixGraph::new(4);
    g.add_edge(0, 1);
    g.add_edge(0, 2);
    g.add_edge(1, 2);
    g.add_edge(2, 3);

    assert!(g.has_edge(0, 1));
    assert!(!g.has_edge(0, 3));
    assert!(g.has_edge(2, 3));
}

fn degree_and_neighbors() {
    let mut g = MatrixGraph::new(4);
    g.add_edge(0, 1);
    g.add_edge(0, 2);
    g.add_edge(1, 2);
    g.add_edge(2, 3);

    assert_eq!(g.out_degree(0), 2);
    assert_eq!(g.out_degree(2), 1);
    assert_eq!(g.in_degree(2), 2); // from 0 and 1
    assert_eq!(g.neighbors(0), vec![1, 2]);
}

fn transitive_closure() {
    let mut g = MatrixGraph::new(4);
    g.add_edge(0, 1);
    g.add_edge(1, 2);
    g.add_edge(2, 3);

    assert!(!g.has_edge(0, 3)); // No direct edge

    let tc = g.transitive_closure();
    assert!(tc[0][3]); // Reachable transitively
    assert!(tc[0][2]); // 0 -> 1 -> 2
    assert!(tc[1][3]); // 1 -> 2 -> 3
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        basic_matrix();
    }

    #[test]
    fn test_degree() {
        degree_and_neighbors();
    }

    #[test]
    fn test_transitive() {
        transitive_closure();
    }

    #[test]
    fn test_undirected() {
        let mut g = MatrixGraph::new(3);
        // Undirected: add both directions
        g.add_edge(0, 1);
        g.add_edge(1, 0);
        assert!(g.has_edge(0, 1));
        assert!(g.has_edge(1, 0));
    }

    #[test]
    fn test_self_loop() {
        let mut g = MatrixGraph::new(3);
        g.add_edge(0, 0);
        assert!(g.has_edge(0, 0));
        assert_eq!(g.out_degree(0), 1);
    }
}
