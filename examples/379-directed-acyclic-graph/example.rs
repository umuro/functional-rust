// DAG and Topological Sort in Rust using Kahn's algorithm
use std::collections::VecDeque;

fn topological_sort(adj: &[Vec<usize>], n: usize) -> Option<Vec<usize>> {
    let mut in_degree = vec![0usize; n];
    for u in 0..n {
        for &v in &adj[u] {
            in_degree[v] += 1;
        }
    }
    let mut queue: VecDeque<usize> = (0..n)
        .filter(|&v| in_degree[v] == 0)
        .collect();
    let mut result = Vec::new();
    while let Some(u) = queue.pop_front() {
        result.push(u);
        for &v in &adj[u] {
            in_degree[v] -= 1;
            if in_degree[v] == 0 {
                queue.push_back(v);
            }
        }
    }
    if result.len() == n { Some(result) } else { None }
}

fn main() {
    let n = 6;
    let mut adj = vec![vec![]; n];
    let edges = [(5,2),(5,0),(4,0),(4,1),(2,0),(2,3),(3,1)];
    for (u, v) in edges {
        adj[u].push(v);
    }
    match topological_sort(&adj, n) {
        Some(order) => println!("Topological order: {:?}", order),
        None => println!("Graph has a cycle!"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_topo_sort() {
        let n = 4;
        let mut adj = vec![vec![]; n];
        // 0->1, 0->2, 1->3, 2->3
        adj[0].push(1); adj[0].push(2);
        adj[1].push(3); adj[2].push(3);
        let order = topological_sort(&adj, n).unwrap();
        assert_eq!(order[0], 0);
        assert_eq!(*order.last().unwrap(), 3);
    }

    #[test]
    fn test_cycle_detection() {
        let n = 3;
        let mut adj = vec![vec![]; n];
        adj[0].push(1); adj[1].push(2); adj[2].push(0); // cycle
        assert!(topological_sort(&adj, n).is_none());
    }
}
