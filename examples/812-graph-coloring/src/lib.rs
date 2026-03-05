//! # Graph Coloring
pub fn graph_coloring(n: usize, edges: &[(usize, usize)], m: usize) -> Option<Vec<usize>> {
    let mut adj = vec![vec![]; n];
    for &(u, v) in edges { adj[u].push(v); adj[v].push(u); }
    let mut colors = vec![0; n];
    fn is_safe(v: usize, c: usize, adj: &[Vec<usize>], colors: &[usize]) -> bool {
        adj[v].iter().all(|&u| colors[u] != c)
    }
    fn solve(v: usize, n: usize, m: usize, adj: &[Vec<usize>], colors: &mut [usize]) -> bool {
        if v == n { return true; }
        for c in 1..=m { if is_safe(v, c, adj, colors) { colors[v] = c;
            if solve(v + 1, n, m, adj, colors) { return true; } colors[v] = 0; } }
        false
    }
    if solve(0, n, m, &adj, &mut colors) { Some(colors) } else { None }
}
#[cfg(test)] mod tests { use super::*;
    #[test] fn test_color() { let c = graph_coloring(4, &[(0,1),(1,2),(2,3),(3,0),(0,2)], 3); assert!(c.is_some()); }
}
