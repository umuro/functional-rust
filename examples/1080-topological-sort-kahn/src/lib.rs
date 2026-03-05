//! Topological Sort via Kahn's Algorithm
//!
//! Iterative topological sort using in-degree counting.
//! OCaml uses `Map` and `Set` from the standard library.
//! Rust uses `HashMap`, `HashSet`, and `VecDeque` for the queue.

use std::collections::{HashMap, HashSet, VecDeque};

// ── Solution 1: Idiomatic Rust — HashMap + VecDeque ──

/// Perform topological sort on a directed acyclic graph using Kahn's algorithm.
///
/// Takes a list of nodes and directed edges (from, to).
/// Returns nodes in topological order, or `None` if a cycle is detected.
///
/// OCaml: `val kahn_sort : string list -> (string * string) list -> string list`
pub fn kahn_sort(nodes: &[&str], edges: &[(&str, &str)]) -> Option<Vec<String>> {
    // Build in-degree map: count incoming edges for each node
    let mut in_degree: HashMap<&str, usize> = nodes.iter().map(|&n| (n, 0)).collect();

    // Build adjacency list for outgoing edges
    let mut adj: HashMap<&str, Vec<&str>> = nodes.iter().map(|&n| (n, Vec::new())).collect();

    for &(from, to) in edges {
        *in_degree.entry(to).or_insert(0) += 1;
        adj.entry(from).or_default().push(to);
    }

    // Start with all nodes that have zero in-degree
    let mut queue: VecDeque<&str> = in_degree
        .iter()
        .filter(|(_, &deg)| deg == 0)
        .map(|(&node, _)| node)
        .collect();

    // Sort the initial queue for deterministic output
    let mut sorted_queue: Vec<&str> = queue.drain(..).collect();
    sorted_queue.sort();
    queue.extend(sorted_queue);

    let mut result = Vec::new();

    while let Some(node) = queue.pop_front() {
        result.push(node.to_string());

        // Collect and sort neighbors for deterministic ordering
        if let Some(neighbors) = adj.get(node) {
            let mut next_nodes = Vec::new();
            for &neighbor in neighbors {
                if let Some(deg) = in_degree.get_mut(neighbor) {
                    *deg -= 1;
                    if *deg == 0 {
                        next_nodes.push(neighbor);
                    }
                }
            }
            next_nodes.sort();
            queue.extend(next_nodes);
        }
    }

    // If we processed all nodes, the sort succeeded
    if result.len() == nodes.len() {
        Some(result)
    } else {
        None // Cycle detected
    }
}

// ── Solution 2: Recursive (DFS-based) topological sort ──
//
// Uses depth-first search with coloring to detect cycles.

#[derive(Clone, Copy, PartialEq)]
enum Color {
    White, // Unvisited
    Gray,  // In progress (on current path)
    Black, // Finished
}

/// DFS-based topological sort. Returns `None` if a cycle is detected.
pub fn topo_sort_dfs(nodes: &[&str], edges: &[(&str, &str)]) -> Option<Vec<String>> {
    let adj: HashMap<&str, Vec<&str>> = {
        let mut map: HashMap<&str, Vec<&str>> = nodes.iter().map(|&n| (n, Vec::new())).collect();
        for &(from, to) in edges {
            map.entry(from).or_default().push(to);
        }
        // Sort adjacency lists for deterministic output
        for list in map.values_mut() {
            list.sort();
        }
        map
    };

    let mut color: HashMap<&str, Color> = nodes.iter().map(|&n| (n, Color::White)).collect();
    let mut result = Vec::new();

    // Sort nodes for deterministic starting order
    let mut sorted_nodes: Vec<&str> = nodes.to_vec();
    sorted_nodes.sort();

    for &node in &sorted_nodes {
        if color[node] == Color::White && !dfs_visit(node, &adj, &mut color, &mut result) {
            return None; // Cycle
        }
    }

    result.reverse();
    Some(result)
}

fn dfs_visit<'a>(
    node: &'a str,
    adj: &HashMap<&'a str, Vec<&'a str>>,
    color: &mut HashMap<&'a str, Color>,
    result: &mut Vec<String>,
) -> bool {
    color.insert(node, Color::Gray);

    if let Some(neighbors) = adj.get(node) {
        for &neighbor in neighbors {
            match color.get(neighbor) {
                Some(Color::Gray) => return false, // Back edge → cycle
                Some(Color::White) => {
                    if !dfs_visit(neighbor, adj, color, result) {
                        return false;
                    }
                }
                _ => {} // Black: already finished
            }
        }
    }

    color.insert(node, Color::Black);
    result.push(node.to_string());
    true
}

// ── Solution 3: Functional-style with iterators ──

/// Kahn's algorithm expressed more functionally using iterators and fold.
pub fn kahn_functional(nodes: &[&str], edges: &[(&str, &str)]) -> Option<Vec<String>> {
    let node_set: HashSet<&str> = nodes.iter().copied().collect();

    // Build in-degree via fold
    let in_degree: HashMap<&str, usize> = edges.iter().fold(
        nodes
            .iter()
            .map(|&n| (n, 0_usize))
            .collect::<HashMap<_, _>>(),
        |mut acc, &(_, to)| {
            *acc.entry(to).or_insert(0) += 1;
            acc
        },
    );

    // Build adjacency via fold
    let adj: HashMap<&str, Vec<&str>> = edges.iter().fold(
        nodes
            .iter()
            .map(|&n| (n, Vec::new()))
            .collect::<HashMap<_, _>>(),
        |mut acc, &(from, to)| {
            acc.entry(from).or_default().push(to);
            acc
        },
    );

    // Iterative process using a mutable state tuple
    let mut deg = in_degree;
    let mut queue: Vec<&str> = deg
        .iter()
        .filter(|(_, &d)| d == 0)
        .map(|(&n, _)| n)
        .collect();
    queue.sort();
    let mut result = Vec::new();

    while let Some(node) = queue.first().copied() {
        queue.remove(0);
        if !node_set.contains(node) {
            continue;
        }
        result.push(node.to_string());

        if let Some(neighbors) = adj.get(node) {
            let mut new_zeros = Vec::new();
            for &nb in neighbors {
                if let Some(d) = deg.get_mut(nb) {
                    *d -= 1;
                    if *d == 0 {
                        new_zeros.push(nb);
                    }
                }
            }
            new_zeros.sort();
            queue.extend(new_zeros);
        }
    }

    if result.len() == nodes.len() {
        Some(result)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_linear_chain() {
        let nodes = vec!["a", "b", "c"];
        let edges = vec![("a", "b"), ("b", "c")];
        assert_eq!(
            kahn_sort(&nodes, &edges),
            Some(vec!["a".to_string(), "b".to_string(), "c".to_string()])
        );
    }

    #[test]
    fn test_diamond_graph() {
        let nodes = vec!["a", "b", "c", "d", "e"];
        let edges = vec![("a", "b"), ("a", "c"), ("b", "d"), ("c", "d"), ("d", "e")];
        let result = kahn_sort(&nodes, &edges).unwrap();
        // a must come before b, c; b and c before d; d before e
        assert_eq!(result[0], "a");
        assert_eq!(result[3], "d");
        assert_eq!(result[4], "e");
    }

    #[test]
    fn test_cycle_detection() {
        let nodes = vec!["a", "b", "c"];
        let edges = vec![("a", "b"), ("b", "c"), ("c", "a")];
        assert_eq!(kahn_sort(&nodes, &edges), None);
    }

    #[test]
    fn test_single_node() {
        let nodes = vec!["x"];
        let edges: Vec<(&str, &str)> = vec![];
        assert_eq!(kahn_sort(&nodes, &edges), Some(vec!["x".to_string()]));
    }

    #[test]
    fn test_dfs_matches_kahn() {
        let nodes = vec!["a", "b", "c", "d", "e"];
        let edges = vec![("a", "b"), ("a", "c"), ("b", "d"), ("c", "d"), ("d", "e")];
        let kahn = kahn_sort(&nodes, &edges).unwrap();
        let dfs = topo_sort_dfs(&nodes, &edges).unwrap();
        // Both should produce valid topological orderings
        assert_eq!(kahn[0], "a");
        assert_eq!(dfs[0], "a");
        assert_eq!(*kahn.last().unwrap(), "e");
        assert_eq!(*dfs.last().unwrap(), "e");
    }

    #[test]
    fn test_dfs_cycle_detection() {
        let nodes = vec!["a", "b", "c"];
        let edges = vec![("a", "b"), ("b", "c"), ("c", "a")];
        assert_eq!(topo_sort_dfs(&nodes, &edges), None);
    }

    #[test]
    fn test_functional_kahn() {
        let nodes = vec!["a", "b", "c", "d", "e"];
        let edges = vec![("a", "b"), ("a", "c"), ("b", "d"), ("c", "d"), ("d", "e")];
        let result = kahn_functional(&nodes, &edges).unwrap();
        assert_eq!(result[0], "a");
        assert_eq!(*result.last().unwrap(), "e");
    }

    #[test]
    fn test_disconnected_nodes() {
        let nodes = vec!["a", "b", "c"];
        let edges: Vec<(&str, &str)> = vec![];
        let result = kahn_sort(&nodes, &edges).unwrap();
        assert_eq!(result.len(), 3);
        // All nodes present, sorted alphabetically (deterministic)
        assert_eq!(result, vec!["a", "b", "c"]);
    }
}
