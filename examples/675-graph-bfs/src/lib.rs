//! # Breadth-First Search
//!
//! Explores graph level by level. Time: O(V+E), Space: O(V)

use std::collections::{HashMap, HashSet, VecDeque};

pub fn bfs<V: Eq + std::hash::Hash + Clone>(
    graph: &HashMap<V, Vec<V>>,
    start: V,
) -> Vec<V> {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    let mut result = Vec::new();
    
    queue.push_back(start.clone());
    visited.insert(start);
    
    while let Some(v) = queue.pop_front() {
        result.push(v.clone());
        if let Some(neighbors) = graph.get(&v) {
            for n in neighbors {
                if !visited.contains(n) {
                    visited.insert(n.clone());
                    queue.push_back(n.clone());
                }
            }
        }
    }
    result
}

/// BFS shortest path (unweighted)
pub fn bfs_shortest_path<V: Eq + std::hash::Hash + Clone>(
    graph: &HashMap<V, Vec<V>>,
    start: V,
    end: V,
) -> Option<Vec<V>> {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    let mut parent: HashMap<V, V> = HashMap::new();
    
    queue.push_back(start.clone());
    visited.insert(start.clone());
    
    while let Some(v) = queue.pop_front() {
        if v == end {
            let mut path = vec![end.clone()];
            let mut curr = &end;
            while let Some(p) = parent.get(curr) {
                path.push(p.clone());
                curr = p;
            }
            path.reverse();
            return Some(path);
        }
        
        if let Some(neighbors) = graph.get(&v) {
            for n in neighbors {
                if !visited.contains(n) {
                    visited.insert(n.clone());
                    parent.insert(n.clone(), v.clone());
                    queue.push_back(n.clone());
                }
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_graph() -> HashMap<i32, Vec<i32>> {
        let mut g = HashMap::new();
        g.insert(1, vec![2, 3]);
        g.insert(2, vec![4, 5]);
        g.insert(3, vec![6]);
        g.insert(4, vec![]);
        g.insert(5, vec![]);
        g.insert(6, vec![]);
        g
    }

    #[test]
    fn test_bfs() {
        let g = sample_graph();
        let result = bfs(&g, 1);
        assert_eq!(result, vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_shortest_path() {
        let g = sample_graph();
        let path = bfs_shortest_path(&g, 1, 6);
        assert_eq!(path, Some(vec![1, 3, 6]));
    }
}
