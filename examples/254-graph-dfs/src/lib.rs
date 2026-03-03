use std::collections::{HashMap, HashSet};

/// Idiomatic Rust DFS: iterative with an explicit stack and HashSet for visited tracking.
/// Returns nodes in DFS pre-order — the same traversal order as the OCaml recursive version.
///
/// Takes `&HashMap<&str, Vec<&str>>` — borrows the graph, no allocation of keys needed.
pub fn dfs<'a>(graph: &'a HashMap<&str, Vec<&str>>, start: &'a str) -> Vec<&'a str> {
    let mut visited: HashSet<&str> = HashSet::new();
    // Stack-based DFS: push neighbors in reverse so left-to-right order is preserved
    let mut stack: Vec<&str> = vec![start];
    let mut result: Vec<&str> = Vec::new();

    while let Some(node) = stack.pop() {
        if !visited.insert(node) {
            // insert returns false when node was already present
            continue;
        }
        result.push(node);
        if let Some(neighbors) = graph.get(node) {
            // Reverse so the first neighbor ends up on top of the stack
            for &neighbor in neighbors.iter().rev() {
                if !visited.contains(neighbor) {
                    stack.push(neighbor);
                }
            }
        }
    }

    result
}

/// Functional-style DFS using an assoc-list graph (mirrors OCaml's `List.assoc`).
/// Recursive inner function threads the visited set, paralleling the OCaml structure.
///
/// OCaml threads `visited` as a pure value through returns; here we use `&mut HashSet`
/// for the same logical effect without repeated allocation.
pub fn dfs_recursive<'a>(graph: &[(&'a str, Vec<&'a str>)], start: &'a str) -> Vec<&'a str> {
    fn go<'a>(
        graph: &[(&'a str, Vec<&'a str>)],
        visited: &mut HashSet<&'a str>,
        node: &'a str,
    ) -> Vec<&'a str> {
        if !visited.insert(node) {
            return vec![];
        }
        // Mirror OCaml's `List.assoc node graph`
        let neighbors = graph
            .iter()
            .find(|(n, _)| *n == node)
            .map(|(_, ns)| ns.as_slice())
            .unwrap_or(&[]);

        // node :: paths  (pre-order: emit node before recursing into neighbors)
        let mut path = vec![node];
        for &neighbor in neighbors {
            path.extend(go(graph, visited, neighbor));
        }
        path
    }

    let mut visited = HashSet::new();
    go(graph, &mut visited, start)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn diamond_hashmap() -> HashMap<&'static str, Vec<&'static str>> {
        let mut g = HashMap::new();
        g.insert("a", vec!["b", "c"]);
        g.insert("b", vec!["d"]);
        g.insert("c", vec!["d"]);
        g.insert("d", vec![]);
        g
    }

    fn diamond_assoc() -> Vec<(&'static str, Vec<&'static str>)> {
        vec![
            ("a", vec!["b", "c"]),
            ("b", vec!["d"]),
            ("c", vec!["d"]),
            ("d", vec![]),
        ]
    }

    // --- dfs (HashMap / iterative) ---

    #[test]
    fn test_dfs_visits_all_nodes() {
        let graph = diamond_hashmap();
        let order = dfs(&graph, "a");
        assert_eq!(order.len(), 4);
        let mut sorted = order.clone();
        sorted.sort_unstable();
        assert_eq!(sorted, vec!["a", "b", "c", "d"]);
    }

    #[test]
    fn test_dfs_start_is_first() {
        let graph = diamond_hashmap();
        let order = dfs(&graph, "a");
        assert_eq!(order[0], "a");
    }

    #[test]
    fn test_dfs_no_duplicate_visits() {
        // Diamond: d is reachable via a→b→d and a→c→d — must appear exactly once
        let graph = diamond_hashmap();
        let order = dfs(&graph, "a");
        let d_count = order.iter().filter(|&&n| n == "d").count();
        assert_eq!(d_count, 1);
    }

    #[test]
    fn test_dfs_single_node() {
        let mut graph = HashMap::new();
        graph.insert("x", vec![]);
        let order = dfs(&graph, "x");
        assert_eq!(order, vec!["x"]);
    }

    #[test]
    fn test_dfs_linear_chain() {
        let mut graph = HashMap::new();
        graph.insert("1", vec!["2"]);
        graph.insert("2", vec!["3"]);
        graph.insert("3", vec![]);
        let order = dfs(&graph, "1");
        assert_eq!(order, vec!["1", "2", "3"]);
    }

    #[test]
    fn test_dfs_pre_order() {
        // For the diamond graph with neighbors visited left-to-right,
        // DFS must reach "b" before "c" (b is the first neighbor of a)
        let graph = diamond_hashmap();
        let order = dfs(&graph, "a");
        let pos: HashMap<&str, usize> = order.iter().enumerate().map(|(i, &n)| (n, i)).collect();
        assert!(pos["a"] < pos["b"]);
        assert!(pos["b"] < pos["c"], "b must be explored before c in DFS");
        assert!(pos["b"] < pos["d"]);
    }

    // --- dfs_recursive (assoc-list / functional) ---

    #[test]
    fn test_recursive_visits_all_nodes() {
        let graph = diamond_assoc();
        let order = dfs_recursive(&graph, "a");
        assert_eq!(order.len(), 4);
        let mut sorted = order.clone();
        sorted.sort_unstable();
        assert_eq!(sorted, vec!["a", "b", "c", "d"]);
    }

    #[test]
    fn test_recursive_no_duplicate_visits() {
        let graph = diamond_assoc();
        let order = dfs_recursive(&graph, "a");
        let d_count = order.iter().filter(|&&n| n == "d").count();
        assert_eq!(d_count, 1);
    }

    #[test]
    fn test_recursive_matches_ocaml_order() {
        // OCaml output for the diamond graph is: a b d c
        let graph = diamond_assoc();
        let order = dfs_recursive(&graph, "a");
        assert_eq!(order, vec!["a", "b", "d", "c"]);
    }

    #[test]
    fn test_recursive_unknown_start_not_in_graph() {
        // Node with no entry in the assoc-list has no neighbors — treated as a leaf
        let graph = diamond_assoc();
        let order = dfs_recursive(&graph, "z");
        assert_eq!(order, vec!["z"]);
    }
}
