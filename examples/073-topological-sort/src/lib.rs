/// # Topological Sort — DAG Ordering
///
/// Order nodes in a directed acyclic graph so every edge goes from earlier to later.
/// Uses DFS-based algorithm.
use std::collections::{HashMap, HashSet};

/// DFS-based topological sort.
/// Returns nodes in topological order (dependencies first).
pub fn topo_sort(edges: &[(&str, &str)]) -> Vec<String> {
    // Collect all nodes
    let mut all_nodes = HashSet::new();
    let mut adj: HashMap<&str, Vec<&str>> = HashMap::new();
    for &(from, to) in edges {
        all_nodes.insert(from);
        all_nodes.insert(to);
        adj.entry(from).or_default().push(to);
    }

    let mut visited = HashSet::new();
    let mut order = Vec::new();

    fn visit<'a>(
        node: &'a str,
        adj: &HashMap<&'a str, Vec<&'a str>>,
        visited: &mut HashSet<&'a str>,
        order: &mut Vec<String>,
    ) {
        if visited.contains(node) {
            return;
        }
        visited.insert(node);
        if let Some(neighbors) = adj.get(node) {
            for &neighbor in neighbors {
                visit(neighbor, adj, visited, order);
            }
        }
        // Post-order: add after all descendants visited
        order.push(node.to_string());
    }

    // Visit all nodes (handles disconnected components)
    let mut sorted_nodes: Vec<&str> = all_nodes.into_iter().collect();
    sorted_nodes.sort(); // deterministic ordering
    for node in sorted_nodes {
        visit(node, &adj, &mut visited, &mut order);
    }

    order.reverse(); // reverse post-order = topological order
    order
}

/// Kahn's algorithm (BFS-based) — alternative approach using in-degree counting.
pub fn topo_sort_kahn(edges: &[(&str, &str)]) -> Vec<String> {
    let mut all_nodes = HashSet::new();
    let mut adj: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut in_degree: HashMap<&str, usize> = HashMap::new();

    for &(from, to) in edges {
        all_nodes.insert(from);
        all_nodes.insert(to);
        adj.entry(from).or_default().push(to);
        *in_degree.entry(to).or_insert(0) += 1;
        in_degree.entry(from).or_insert(0);
    }

    // Start with nodes that have no incoming edges
    let mut queue: Vec<&str> = in_degree
        .iter()
        .filter(|(_, &deg)| deg == 0)
        .map(|(&node, _)| node)
        .collect();
    queue.sort(); // deterministic
    let mut result = Vec::new();

    while let Some(node) = queue.first().copied() {
        queue.remove(0);
        result.push(node.to_string());
        if let Some(neighbors) = adj.get(node) {
            for &neighbor in neighbors {
                let deg = in_degree.get_mut(neighbor).unwrap();
                *deg -= 1;
                if *deg == 0 {
                    queue.push(neighbor);
                    queue.sort();
                }
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let edges = vec![("a", "b"), ("a", "c"), ("b", "d"), ("c", "d"), ("d", "e")];
        let order = topo_sort(&edges);
        // a must come before b and c; b and c before d; d before e
        let pos = |s: &str| order.iter().position(|x| x == s).unwrap();
        assert!(pos("a") < pos("b"));
        assert!(pos("a") < pos("c"));
        assert!(pos("b") < pos("d"));
        assert!(pos("d") < pos("e"));
    }

    #[test]
    fn test_kahn() {
        let edges = vec![("a", "b"), ("a", "c"), ("b", "d"), ("c", "d"), ("d", "e")];
        let order = topo_sort_kahn(&edges);
        let pos = |s: &str| order.iter().position(|x| x == s).unwrap();
        assert!(pos("a") < pos("b"));
        assert!(pos("d") < pos("e"));
    }

    #[test]
    fn test_empty() {
        let order = topo_sort(&[]);
        assert!(order.is_empty());
    }

    #[test]
    fn test_single_edge() {
        let order = topo_sort(&[("a", "b")]);
        assert_eq!(order, vec!["a", "b"]);
    }

    #[test]
    fn test_linear_chain() {
        let edges = vec![("a", "b"), ("b", "c"), ("c", "d")];
        let order = topo_sort(&edges);
        assert_eq!(order, vec!["a", "b", "c", "d"]);
    }
}
