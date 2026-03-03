use std::collections::{HashMap, HashSet, VecDeque};

/// Idiomatic Rust BFS using HashMap adjacency list and VecDeque queue.
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
                    queue.push_back(neighbor);
                }
            }
        }
    }

    result
}

/// Functional-style BFS using slice-based adjacency list (mirrors OCaml's List.assoc).
pub fn bfs_assoc<'a>(graph: &[(&'a str, Vec<&'a str>)], start: &'a str) -> Vec<&'a str> {
    let mut visited: HashSet<&str> = HashSet::new();
    let mut queue: VecDeque<&str> = VecDeque::new();
    let mut result: Vec<&str> = Vec::new();

    queue.push_back(start);
    visited.insert(start);

    while let Some(node) = queue.pop_front() {
        result.push(node);
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

fn main() {
    // HashMap-based graph (idiomatic Rust)
    let mut graph = HashMap::new();
    graph.insert("a", vec!["b", "c"]);
    graph.insert("b", vec!["d"]);
    graph.insert("c", vec!["d"]);
    graph.insert("d", vec![]);

    let order = bfs(&graph, "a");
    println!("BFS from 'a': {:?}", order);

    // Slice-based graph (mirrors OCaml List.assoc style)
    let graph_assoc = vec![
        ("a", vec!["b", "c"]),
        ("b", vec!["d"]),
        ("c", vec!["d"]),
        ("d", vec![]),
    ];
    let order2 = bfs_assoc(&graph_assoc, "a");
    println!("BFS (assoc) from 'a': {:?}", order2);

    // Larger example: tree-shaped graph
    let mut tree = HashMap::new();
    tree.insert("root", vec!["L1a", "L1b"]);
    tree.insert("L1a", vec!["L2a", "L2b"]);
    tree.insert("L1b", vec!["L2c"]);
    tree.insert("L2a", vec![]);
    tree.insert("L2b", vec![]);
    tree.insert("L2c", vec![]);
    let order3 = bfs(&tree, "root");
    println!("BFS tree: {:?}", order3);
}

/* Output:
   BFS from 'a': ["a", "b", "c", "d"]
   BFS (assoc) from 'a': ["a", "b", "c", "d"]
   BFS tree: ["root", "L1a", "L1b", "L2a", "L2b", "L2c"]
*/
