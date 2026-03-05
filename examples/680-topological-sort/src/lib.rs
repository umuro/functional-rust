//! # Topological Sort
//!
//! Linear ordering of vertices in a DAG. Time: O(V+E)

use std::collections::{HashMap, HashSet, VecDeque};

/// Kahn's algorithm (BFS-based)
pub fn topological_sort_kahn(graph: &HashMap<usize, Vec<usize>>, n: usize) -> Option<Vec<usize>> {
    let mut in_degree = vec![0; n];
    for neighbors in graph.values() {
        for &v in neighbors { in_degree[v] += 1; }
    }
    
    let mut queue: VecDeque<usize> = (0..n).filter(|&i| in_degree[i] == 0).collect();
    let mut result = Vec::new();
    
    while let Some(u) = queue.pop_front() {
        result.push(u);
        if let Some(neighbors) = graph.get(&u) {
            for &v in neighbors {
                in_degree[v] -= 1;
                if in_degree[v] == 0 { queue.push_back(v); }
            }
        }
    }
    
    if result.len() == n { Some(result) } else { None }
}

/// DFS-based topological sort
pub fn topological_sort_dfs(graph: &HashMap<usize, Vec<usize>>, n: usize) -> Option<Vec<usize>> {
    let mut visited = vec![0u8; n]; // 0=white, 1=gray, 2=black
    let mut result = Vec::new();
    
    fn dfs(v: usize, graph: &HashMap<usize, Vec<usize>>, visited: &mut [u8], result: &mut Vec<usize>) -> bool {
        visited[v] = 1;
        if let Some(neighbors) = graph.get(&v) {
            for &u in neighbors {
                if visited[u] == 1 { return false; } // cycle
                if visited[u] == 0 && !dfs(u, graph, visited, result) { return false; }
            }
        }
        visited[v] = 2;
        result.push(v);
        true
    }
    
    for i in 0..n {
        if visited[i] == 0 && !dfs(i, graph, &mut visited, &mut result) {
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
    fn test_kahn() {
        let mut g = HashMap::new();
        g.insert(0, vec![1, 2]);
        g.insert(1, vec![3]);
        g.insert(2, vec![3]);
        g.insert(3, vec![]);
        let order = topological_sort_kahn(&g, 4).unwrap();
        assert_eq!(order[0], 0);
        assert_eq!(order[3], 3);
    }

    #[test]
    fn test_cycle() {
        let mut g = HashMap::new();
        g.insert(0, vec![1]);
        g.insert(1, vec![0]);
        assert!(topological_sort_kahn(&g, 2).is_none());
    }
}
