#![allow(clippy::all)]
// 1037: Adjacency List — HashMap<usize, Vec<usize>>
// Classic graph representation with BFS and DFS

use std::collections::{HashMap, HashSet, VecDeque};

/// Adjacency list graph
struct Graph {
    adj: HashMap<usize, Vec<usize>>,
}

impl Graph {
    fn new() -> Self {
        Graph {
            adj: HashMap::new(),
        }
    }

    fn add_edge(&mut self, from: usize, to: usize) {
        self.adj.entry(from).or_default().push(to);
        // Ensure 'to' node exists in the map
        self.adj.entry(to).or_default();
    }

    fn neighbors(&self, node: usize) -> &[usize] {
        self.adj.get(&node).map_or(&[], |v| v.as_slice())
    }

    /// BFS traversal
    fn bfs(&self, start: usize) -> Vec<usize> {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        let mut order = Vec::new();

        visited.insert(start);
        queue.push_back(start);

        while let Some(node) = queue.pop_front() {
            order.push(node);
            for &neighbor in self.neighbors(node) {
                if visited.insert(neighbor) {
                    queue.push_back(neighbor);
                }
            }
        }
        order
    }

    /// DFS traversal (recursive)
    fn dfs(&self, start: usize) -> Vec<usize> {
        let mut visited = HashSet::new();
        let mut order = Vec::new();
        self.dfs_helper(start, &mut visited, &mut order);
        order
    }

    fn dfs_helper(&self, node: usize, visited: &mut HashSet<usize>, order: &mut Vec<usize>) {
        if !visited.insert(node) {
            return;
        }
        order.push(node);
        for &neighbor in self.neighbors(node) {
            self.dfs_helper(neighbor, visited, order);
        }
    }

    /// Find shortest path using BFS
    fn find_path(&self, start: usize, goal: usize) -> Option<Vec<usize>> {
        let mut visited = HashSet::new();
        let mut parent: HashMap<usize, usize> = HashMap::new();
        let mut queue = VecDeque::new();

        visited.insert(start);
        queue.push_back(start);

        while let Some(node) = queue.pop_front() {
            if node == goal {
                // Reconstruct path
                let mut path = vec![goal];
                let mut current = goal;
                while current != start {
                    current = parent[&current];
                    path.push(current);
                }
                path.reverse();
                return Some(path);
            }
            for &neighbor in self.neighbors(node) {
                if visited.insert(neighbor) {
                    parent.insert(neighbor, node);
                    queue.push_back(neighbor);
                }
            }
        }
        None
    }
}

fn test_bfs_dfs() {
    let mut g = Graph::new();
    g.add_edge(0, 1);
    g.add_edge(0, 2);
    g.add_edge(1, 3);
    g.add_edge(2, 3);
    g.add_edge(3, 4);

    let bfs_order = g.bfs(0);
    assert_eq!(bfs_order.len(), 5);
    assert_eq!(bfs_order[0], 0);
    assert!(bfs_order.contains(&4));

    let dfs_order = g.dfs(0);
    assert_eq!(dfs_order.len(), 5);
    assert_eq!(dfs_order[0], 0);
}

fn test_path_finding() {
    let mut g = Graph::new();
    g.add_edge(0, 1);
    g.add_edge(0, 2);
    g.add_edge(1, 3);
    g.add_edge(2, 3);
    g.add_edge(3, 4);

    let path = g.find_path(0, 4).unwrap();
    assert_eq!(*path.first().unwrap(), 0);
    assert_eq!(*path.last().unwrap(), 4);
    assert!(path.len() <= 4); // Shortest path is 3 hops

    assert!(g.find_path(4, 0).is_none()); // No path back (directed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bfs() {
        test_bfs_dfs();
    }

    #[test]
    fn test_paths() {
        test_path_finding();
    }

    #[test]
    fn test_disconnected() {
        let mut g = Graph::new();
        g.add_edge(0, 1);
        g.add_edge(2, 3);
        let reachable = g.bfs(0);
        assert_eq!(reachable.len(), 2); // Only 0 and 1
        assert!(!reachable.contains(&2));
    }

    #[test]
    fn test_self_loop() {
        let mut g = Graph::new();
        g.add_edge(0, 0);
        g.add_edge(0, 1);
        let order = g.bfs(0);
        assert_eq!(order.len(), 2);
    }
}
