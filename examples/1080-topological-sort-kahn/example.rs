use std::collections::{HashMap, VecDeque};

fn kahn_sort(nodes: &[&str], edges: &[(&str, &str)]) -> Option<Vec<String>> {
    let mut in_degree: HashMap<&str, usize> = nodes.iter().map(|&n| (n, 0)).collect();
    let mut adj: HashMap<&str, Vec<&str>> = nodes.iter().map(|&n| (n, Vec::new())).collect();

    for &(from, to) in edges {
        *in_degree.entry(to).or_insert(0) += 1;
        adj.entry(from).or_default().push(to);
    }

    let mut queue: VecDeque<&str> = {
        let mut zeros: Vec<&str> = in_degree.iter()
            .filter(|(_, &d)| d == 0).map(|(&n, _)| n).collect();
        zeros.sort();
        zeros.into_iter().collect()
    };

    let mut result = Vec::new();

    while let Some(node) = queue.pop_front() {
        result.push(node.to_string());
        if let Some(neighbors) = adj.get(node) {
            let mut next = Vec::new();
            for &nb in neighbors {
                if let Some(deg) = in_degree.get_mut(nb) {
                    *deg -= 1;
                    if *deg == 0 { next.push(nb); }
                }
            }
            next.sort();
            queue.extend(next);
        }
    }

    if result.len() == nodes.len() { Some(result) } else { None }
}

fn main() {
    let nodes = vec!["a", "b", "c", "d", "e"];
    let edges = vec![("a","b"), ("a","c"), ("b","d"), ("c","d"), ("d","e")];
    match kahn_sort(&nodes, &edges) {
        Some(order) => println!("{}", order.join(" ")),
        None => println!("Cycle detected!"),
    }
}

/* Output:
   a b c d e
*/
