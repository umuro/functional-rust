//! # Depth-First Search
//!
//! Explores graph depth-first using recursion or stack. Time: O(V+E)

use std::collections::{HashMap, HashSet};

/// Recursive DFS
pub fn dfs_recursive<V: Eq + std::hash::Hash + Clone>(
    graph: &HashMap<V, Vec<V>>,
    start: V,
) -> Vec<V> {
    let mut visited = HashSet::new();
    let mut result = Vec::new();
    dfs_helper(graph, start, &mut visited, &mut result);
    result
}

fn dfs_helper<V: Eq + std::hash::Hash + Clone>(
    graph: &HashMap<V, Vec<V>>,
    v: V,
    visited: &mut HashSet<V>,
    result: &mut Vec<V>,
) {
    if visited.contains(&v) { return; }
    visited.insert(v.clone());
    result.push(v.clone());
    
    if let Some(neighbors) = graph.get(&v) {
        for n in neighbors {
            dfs_helper(graph, n.clone(), visited, result);
        }
    }
}

/// Iterative DFS using stack
pub fn dfs_iterative<V: Eq + std::hash::Hash + Clone>(
    graph: &HashMap<V, Vec<V>>,
    start: V,
) -> Vec<V> {
    let mut visited = HashSet::new();
    let mut stack = vec![start];
    let mut result = Vec::new();
    
    while let Some(v) = stack.pop() {
        if visited.contains(&v) { continue; }
        visited.insert(v.clone());
        result.push(v.clone());
        
        if let Some(neighbors) = graph.get(&v) {
            for n in neighbors.iter().rev() {
                if !visited.contains(n) { stack.push(n.clone()); }
            }
        }
    }
    result
}

/// Check if graph has cycle (directed)
pub fn has_cycle<V: Eq + std::hash::Hash + Clone>(graph: &HashMap<V, Vec<V>>) -> bool {
    let mut visited = HashSet::new();
    let mut rec_stack = HashSet::new();
    
    for start in graph.keys() {
        if has_cycle_helper(graph, start, &mut visited, &mut rec_stack) {
            return true;
        }
    }
    false
}

fn has_cycle_helper<V: Eq + std::hash::Hash + Clone>(
    graph: &HashMap<V, Vec<V>>,
    v: &V,
    visited: &mut HashSet<V>,
    rec_stack: &mut HashSet<V>,
) -> bool {
    if rec_stack.contains(v) { return true; }
    if visited.contains(v) { return false; }
    
    visited.insert(v.clone());
    rec_stack.insert(v.clone());
    
    if let Some(neighbors) = graph.get(v) {
        for n in neighbors {
            if has_cycle_helper(graph, n, visited, rec_stack) { return true; }
        }
    }
    
    rec_stack.remove(v);
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dfs_recursive() {
        let mut g = HashMap::new();
        g.insert(1, vec![2, 3]);
        g.insert(2, vec![4]);
        g.insert(3, vec![4]);
        g.insert(4, vec![]);
        let result = dfs_recursive(&g, 1);
        assert!(result.contains(&1) && result.contains(&4));
    }

    #[test]
    fn test_cycle_detection() {
        let mut g = HashMap::new();
        g.insert(1, vec![2]);
        g.insert(2, vec![3]);
        g.insert(3, vec![1]);
        assert!(has_cycle(&g));
    }
}
