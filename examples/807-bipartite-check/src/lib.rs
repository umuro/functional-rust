//! # Bipartite Check
pub fn is_bipartite(n: usize, edges: &[(usize, usize)]) -> bool {
    let mut adj = vec![vec![]; n];
    for &(u, v) in edges { adj[u].push(v); adj[v].push(u); }
    let mut color = vec![None; n];
    fn bfs(start: usize, adj: &[Vec<usize>], color: &mut [Option<bool>]) -> bool {
        use std::collections::VecDeque;
        let mut q = VecDeque::new(); q.push_back(start); color[start] = Some(true);
        while let Some(u) = q.pop_front() {
            for &v in &adj[u] {
                if color[v].is_none() { color[v] = Some(!color[u].unwrap()); q.push_back(v); }
                else if color[v] == color[u] { return false; }
            }
        }
        true
    }
    for v in 0..n { if color[v].is_none() && !bfs(v, &adj, &mut color) { return false; } }
    true
}
#[cfg(test)] mod tests { use super::*;
    #[test] fn test_bipartite() { assert!(is_bipartite(4, &[(0,1),(1,2),(2,3),(3,0)])); }
    #[test] fn test_not_bipartite() { assert!(!is_bipartite(3, &[(0,1),(1,2),(2,0)])); }
}
