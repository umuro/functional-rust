#![allow(clippy::all)]
use std::collections::{HashMap, HashSet};

/// Topological sort using DFS
///
/// Ownership insight: edges are borrowed as slices of string slices.
/// The visited set and result vector are owned locally.
pub fn topo_sort(edges: &[(&str, &str)]) -> Vec<String> {
    let mut all_nodes = HashSet::new();
    let mut adj: HashMap<&str, Vec<&str>> = HashMap::new();
    for &(a, b) in edges {
        all_nodes.insert(a);
        all_nodes.insert(b);
        adj.entry(a).or_default().push(b);
    }

    let mut visited = HashSet::new();
    let mut order = Vec::new();

    fn visit<'a>(
        node: &'a str,
        adj: &HashMap<&str, Vec<&'a str>>,
        visited: &mut HashSet<&'a str>,
        order: &mut Vec<String>,
    ) {
        if visited.contains(node) {
            return;
        }
        visited.insert(node);
        if let Some(neighbors) = adj.get(node) {
            for &n in neighbors {
                visit(n, adj, visited, order);
            }
        }
        order.push(node.to_string());
    }

    for &node in &all_nodes {
        visit(node, &adj, &mut visited, &mut order);
    }
    order
}

/// Version using owned Strings
pub fn topo_sort_owned(edges: Vec<(String, String)>) -> Vec<String> {
    let str_edges: Vec<(&str, &str)> = edges
        .iter()
        .map(|(a, b)| (a.as_str(), b.as_str()))
        .collect();
    topo_sort(&str_edges)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linear_chain() {
        let edges = vec![("a", "b"), ("b", "c"), ("c", "d")];
        let result = topo_sort(&edges);
        // d should come before c, c before b, b before a
        let pos = |s: &str| result.iter().position(|x| x == s).unwrap();
        assert!(pos("d") < pos("c"));
        assert!(pos("c") < pos("b"));
    }

    #[test]
    fn test_diamond() {
        let edges = vec![("a", "b"), ("a", "c"), ("b", "d"), ("c", "d")];
        let result = topo_sort(&edges);
        let pos = |s: &str| result.iter().position(|x| x == s).unwrap();
        assert!(pos("d") < pos("b"));
        assert!(pos("d") < pos("c"));
    }

    #[test]
    fn test_empty() {
        let result = topo_sort(&[]);
        assert!(result.is_empty());
    }

    #[test]
    fn test_single_edge() {
        let result = topo_sort(&[("x", "y")]);
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn test_owned_version() {
        let edges = vec![("a".into(), "b".into()), ("b".into(), "c".into())];
        let result = topo_sort_owned(edges);
        assert_eq!(result.len(), 3);
    }
}
