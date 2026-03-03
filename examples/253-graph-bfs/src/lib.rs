use std::collections::{HashMap, HashSet, VecDeque};

/// Idiomatic Rust BFS: uses HashMap for adjacency list, VecDeque as queue,
/// HashSet for visited tracking. Returns nodes in BFS order.
pub fn bfs<'a>(graph: &'a HashMap<&str, Vec<&str>>, start: &'a str) -> Vec<&'a str> {
    let mut visited: HashSet<&str> = HashSet::new();
    let mut queue: VecDeque<&str> = VecDeque::new();
    let mut result: Vec<&str> = Vec::new();

    queue.push_back(start);
    visited.insert(start);

    while let Some(node) = queue.pop_front() {
        result.push(node);
        if let Some(neighbors) = graph.get(node) {
            for &neighbor in neighbors {
                if visited.insert(neighbor) {
                    // insert returns true if the value was not already present
                    queue.push_back(neighbor);
                }
            }
        }
    }

    result
}

/// Functional-style BFS using slice-based adjacency list (closer to OCaml).
/// The graph is a slice of (node, neighbors) pairs — mirrors `List.assoc`.
pub fn bfs_assoc<'a>(graph: &[(&'a str, Vec<&'a str>)], start: &'a str) -> Vec<&'a str> {
    let mut visited: HashSet<&str> = HashSet::new();
    let mut queue: VecDeque<&str> = VecDeque::new();
    let mut result: Vec<&str> = Vec::new();

    queue.push_back(start);
    visited.insert(start);

    while let Some(node) = queue.pop_front() {
        result.push(node);
        // Mirror OCaml's List.assoc: find neighbors for this node
        if let Some((_, neighbors)) = graph.iter().find(|(n, _)| *n == node) {
            for &neighbor in neighbors {
                if visited.insert(neighbor) {
                    queue.push_back(neighbor);
                }
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_graph() -> HashMap<&'static str, Vec<&'static str>> {
        let mut g = HashMap::new();
        g.insert("a", vec!["b", "c"]);
        g.insert("b", vec!["d"]);
        g.insert("c", vec!["d"]);
        g.insert("d", vec![]);
        g
    }

    #[test]
    fn test_bfs_visits_all_nodes() {
        let graph = make_graph();
        let order = bfs(&graph, "a");
        // All four nodes must appear exactly once
        assert_eq!(order.len(), 4);
        let mut sorted = order.clone();
        sorted.sort_unstable();
        assert_eq!(sorted, vec!["a", "b", "c", "d"]);
    }

    #[test]
    fn test_bfs_start_node_is_first() {
        let graph = make_graph();
        let order = bfs(&graph, "a");
        assert_eq!(order[0], "a");
    }

    #[test]
    fn test_bfs_level_order() {
        let graph = make_graph();
        let order = bfs(&graph, "a");
        // "a" must come before "b" and "c"; "b" and "c" before "d"
        let pos: HashMap<&str, usize> = order.iter().enumerate().map(|(i, &n)| (n, i)).collect();
        assert!(pos["a"] < pos["b"]);
        assert!(pos["a"] < pos["c"]);
        assert!(pos["b"] < pos["d"]);
        assert!(pos["c"] < pos["d"]);
    }

    #[test]
    fn test_bfs_single_node() {
        let mut graph = HashMap::new();
        graph.insert("x", vec![]);
        let order = bfs(&graph, "x");
        assert_eq!(order, vec!["x"]);
    }

    #[test]
    fn test_bfs_linear_chain() {
        let mut graph = HashMap::new();
        graph.insert("1", vec!["2"]);
        graph.insert("2", vec!["3"]);
        graph.insert("3", vec![]);
        let order = bfs(&graph, "1");
        assert_eq!(order, vec!["1", "2", "3"]);
    }

    #[test]
    fn test_bfs_assoc_matches_ocaml() {
        let graph = vec![
            ("a", vec!["b", "c"]),
            ("b", vec!["d"]),
            ("c", vec!["d"]),
            ("d", vec![]),
        ];
        let order = bfs_assoc(&graph, "a");
        // Same reachability guarantee as OCaml version
        assert_eq!(order[0], "a");
        assert_eq!(order.len(), 4);
        let mut sorted = order.clone();
        sorted.sort_unstable();
        assert_eq!(sorted, vec!["a", "b", "c", "d"]);
    }

    #[test]
    fn test_bfs_no_duplicate_visits() {
        // Diamond graph: a->b, a->c, b->d, c->d — d reachable via two paths
        let mut graph = HashMap::new();
        graph.insert("a", vec!["b", "c"]);
        graph.insert("b", vec!["d"]);
        graph.insert("c", vec!["d"]);
        graph.insert("d", vec![]);
        let order = bfs(&graph, "a");
        // d must appear exactly once despite two paths leading to it
        let d_count = order.iter().filter(|&&n| n == "d").count();
        assert_eq!(d_count, 1);
    }
}
