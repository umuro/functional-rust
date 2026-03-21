#![allow(dead_code)]
#![allow(clippy::all)]
// 1036: Graph with Arena Allocation
// Vec<Node> indexed by usize — the idiomatic Rust graph pattern

use std::collections::VecDeque;

/// A graph node stored in an arena (Vec)
#[derive(Debug)]
struct Node {
    label: String,
    edges: Vec<usize>, // indices into the arena
}

/// Arena-based graph
struct Graph {
    nodes: Vec<Node>,
}

impl Graph {
    fn new() -> Self {
        Graph { nodes: Vec::new() }
    }

    fn add_node(&mut self, label: &str) -> usize {
        let idx = self.nodes.len();
        self.nodes.push(Node {
            label: label.to_string(),
            edges: Vec::new(),
        });
        idx
    }

    fn add_edge(&mut self, from: usize, to: usize) {
        self.nodes[from].edges.push(to);
    }

    fn neighbors(&self, idx: usize) -> &[usize] {
        &self.nodes[idx].edges
    }

    fn label(&self, idx: usize) -> &str {
        &self.nodes[idx].label
    }

    fn bfs(&self, start: usize) -> Vec<usize> {
        let mut visited = vec![false; self.nodes.len()];
        let mut queue = VecDeque::new();
        let mut order = Vec::new();

        visited[start] = true;
        queue.push_back(start);

        while let Some(idx) = queue.pop_front() {
            order.push(idx);
            for &neighbor in self.neighbors(idx) {
                if !visited[neighbor] {
                    visited[neighbor] = true;
                    queue.push_back(neighbor);
                }
            }
        }
        order
    }

    fn dfs(&self, start: usize) -> Vec<usize> {
        let mut visited = vec![false; self.nodes.len()];
        let mut order = Vec::new();
        self.dfs_helper(start, &mut visited, &mut order);
        order
    }

    fn dfs_helper(&self, idx: usize, visited: &mut [bool], order: &mut Vec<usize>) {
        visited[idx] = true;
        order.push(idx);
        for &neighbor in self.neighbors(idx) {
            if !visited[neighbor] {
                self.dfs_helper(neighbor, visited, order);
            }
        }
    }
}

fn basic_arena_graph() {
    let mut g = Graph::new();
    let a = g.add_node("A");
    let b = g.add_node("B");
    let c = g.add_node("C");

    g.add_edge(a, b);
    g.add_edge(a, c);
    g.add_edge(b, c);
    g.add_edge(c, a);

    assert_eq!(g.label(a), "A");
    let neighbors: Vec<&str> = g.neighbors(a).iter().map(|&i| g.label(i)).collect();
    assert_eq!(neighbors, vec!["B", "C"]);
}

fn bfs_test() {
    let mut g = Graph::new();
    let start = g.add_node("start");
    let mid1 = g.add_node("mid1");
    let mid2 = g.add_node("mid2");
    let end_node = g.add_node("end");

    g.add_edge(start, mid1);
    g.add_edge(start, mid2);
    g.add_edge(mid1, end_node);
    g.add_edge(mid2, end_node);

    let order: Vec<&str> = g.bfs(start).iter().map(|&i| g.label(i)).collect();
    assert_eq!(order, vec!["start", "mid1", "mid2", "end"]);
}

fn weighted_arena() {
    struct WNode {
        label: &'static str,
        edges: Vec<(usize, f64)>,
    }

    let nodes = vec![
        WNode {
            label: "A",
            edges: vec![(1, 1.0), (2, 4.0)],
        },
        WNode {
            label: "B",
            edges: vec![(2, 2.0)],
        },
        WNode {
            label: "C",
            edges: vec![],
        },
    ];

    let total: f64 = nodes[0].edges.iter().map(|(_, w)| w).sum();
    assert!((total - 5.0).abs() < f64::EPSILON);
    assert_eq!(nodes[0].label, "A");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        basic_arena_graph();
    }

    #[test]
    fn test_bfs() {
        bfs_test();
    }

    #[test]
    fn test_weighted() {
        weighted_arena();
    }

    #[test]
    fn test_dfs() {
        let mut g = Graph::new();
        let a = g.add_node("A");
        let b = g.add_node("B");
        let c = g.add_node("C");
        let d = g.add_node("D");
        g.add_edge(a, b);
        g.add_edge(a, c);
        g.add_edge(b, d);
        let order: Vec<&str> = g.dfs(a).iter().map(|&i| g.label(i)).collect();
        assert_eq!(order, vec!["A", "B", "D", "C"]);
    }
}
