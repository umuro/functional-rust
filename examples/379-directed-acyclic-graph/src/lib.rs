//! Directed Acyclic Graph (DAG) and Topological Sort
//!
//! Kahn's algorithm for topological ordering.

use std::collections::VecDeque;

/// Topological sort using Kahn's algorithm
/// Returns None if graph has a cycle
pub fn topological_sort(adj: &[Vec<usize>], n: usize) -> Option<Vec<usize>> {
    let mut in_degree = vec![0usize; n];
    for u in 0..n {
        for &v in &adj[u] {
            in_degree[v] += 1;
        }
    }
    let mut queue: VecDeque<usize> = (0..n).filter(|&v| in_degree[v] == 0).collect();
    let mut result = Vec::new();
    while let Some(u) = queue.pop_front() {
        result.push(u);
        for &v in &adj[u] {
            in_degree[v] -= 1;
            if in_degree[v] == 0 {
                queue.push_back(v);
            }
        }
    }
    if result.len() == n {
        Some(result)
    } else {
        None
    }
}

/// Check if a directed graph has a cycle
pub fn has_cycle(adj: &[Vec<usize>], n: usize) -> bool {
    topological_sort(adj, n).is_none()
}

/// DFS-based topological sort (returns reverse postorder)
pub fn topological_sort_dfs(adj: &[Vec<usize>], n: usize) -> Option<Vec<usize>> {
    let mut visited = vec![0u8; n]; // 0=unvisited, 1=in progress, 2=done
    let mut result = Vec::new();

    fn dfs(
        u: usize,
        adj: &[Vec<usize>],
        visited: &mut [u8],
        result: &mut Vec<usize>,
    ) -> bool {
        if visited[u] == 1 {
            return false;
        } // cycle
        if visited[u] == 2 {
            return true;
        }
        visited[u] = 1;
        for &v in &adj[u] {
            if !dfs(v, adj, visited, result) {
                return false;
            }
        }
        visited[u] = 2;
        result.push(u);
        true
    }

    for u in 0..n {
        if visited[u] == 0 && !dfs(u, adj, &mut visited, &mut result) {
            return None;
        }
    }
    result.reverse();
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_topo_sort_kahn() {
        let mut adj = vec![vec![]; 4];
        adj[0].push(1);
        adj[0].push(2);
        adj[1].push(3);
        adj[2].push(3);
        let order = topological_sort(&adj, 4).unwrap();
        assert_eq!(order[0], 0);
        assert_eq!(*order.last().unwrap(), 3);
    }

    #[test]
    fn test_cycle_detection() {
        let mut adj = vec![vec![]; 3];
        adj[0].push(1);
        adj[1].push(2);
        adj[2].push(0);
        assert!(has_cycle(&adj, 3));
    }

    #[test]
    fn test_no_cycle() {
        let mut adj = vec![vec![]; 3];
        adj[0].push(1);
        adj[1].push(2);
        assert!(!has_cycle(&adj, 3));
    }

    #[test]
    fn test_dfs_sort() {
        let mut adj = vec![vec![]; 4];
        adj[0].push(1);
        adj[0].push(2);
        adj[1].push(3);
        adj[2].push(3);
        let order = topological_sort_dfs(&adj, 4).unwrap();
        assert_eq!(order[0], 0);
    }

    #[test]
    fn test_empty_graph() {
        let adj: Vec<Vec<usize>> = vec![vec![]; 3];
        let order = topological_sort(&adj, 3).unwrap();
        assert_eq!(order.len(), 3);
    }
}
