use std::collections::{HashMap, HashSet};

/// Idiomatic Rust DFS: iterative with an explicit stack and HashSet for visited tracking.
pub fn dfs<'a>(graph: &'a HashMap<&str, Vec<&str>>, start: &'a str) -> Vec<&'a str> {
    let mut visited: HashSet<&str> = HashSet::new();
    let mut stack: Vec<&str> = vec![start];
    let mut result: Vec<&str> = Vec::new();

    while let Some(node) = stack.pop() {
        if !visited.insert(node) {
            continue;
        }
        result.push(node);
        if let Some(neighbors) = graph.get(node) {
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
pub fn dfs_recursive<'a>(graph: &[(&'a str, Vec<&'a str>)], start: &'a str) -> Vec<&'a str> {
    fn go<'a>(
        graph: &[(&'a str, Vec<&'a str>)],
        visited: &mut HashSet<&'a str>,
        node: &'a str,
    ) -> Vec<&'a str> {
        if !visited.insert(node) {
            return vec![];
        }
        let neighbors = graph
            .iter()
            .find(|(n, _)| *n == node)
            .map(|(_, ns)| ns.as_slice())
            .unwrap_or(&[]);

        let mut path = vec![node];
        for &neighbor in neighbors {
            path.extend(go(graph, visited, neighbor));
        }
        path
    }

    let mut visited = HashSet::new();
    go(graph, &mut visited, start)
}

fn main() {
    // Idiomatic DFS with HashMap
    let mut graph = HashMap::new();
    graph.insert("a", vec!["b", "c"]);
    graph.insert("b", vec!["d"]);
    graph.insert("c", vec!["d"]);
    graph.insert("d", vec![]);

    let order = dfs(&graph, "a");
    println!("dfs (iterative): {:?}", order);

    // Functional DFS with assoc-list — mirrors OCaml directly
    let assoc = vec![
        ("a", vec!["b", "c"]),
        ("b", vec!["d"]),
        ("c", vec!["d"]),
        ("d", vec![]),
    ];

    let order2 = dfs_recursive(&assoc, "a");
    println!("dfs_recursive:   {:?}", order2);
}

/* Output:
   dfs (iterative): ["a", "b", "d", "c"]
   dfs_recursive:   ["a", "b", "d", "c"]
*/
