#![allow(clippy::all)]
//! # Topological Sort (DFS)
pub fn topological_sort(n: usize, edges: &[(usize, usize)]) -> Option<Vec<usize>> {
    let mut adj = vec![vec![]; n];
    for &(u, v) in edges {
        adj[u].push(v);
    }
    let mut visited = vec![0u8; n];
    let mut result = vec![];
    fn dfs(v: usize, adj: &[Vec<usize>], vis: &mut [u8], res: &mut Vec<usize>) -> bool {
        vis[v] = 1;
        for &u in &adj[v] {
            if vis[u] == 1 {
                return false;
            }
            if vis[u] == 0 && !dfs(u, adj, vis, res) {
                return false;
            }
        }
        vis[v] = 2;
        res.push(v);
        true
    }
    for v in 0..n {
        if visited[v] == 0 && !dfs(v, &adj, &mut visited, &mut result) {
            return None;
        }
    }
    result.reverse();
    Some(result)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_topo() {
        let r = topological_sort(4, &[(0, 1), (0, 2), (1, 3), (2, 3)]);
        assert!(r.is_some());
    }
}
