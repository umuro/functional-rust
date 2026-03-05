//! # Minimum Vertex Cover (Trees)
pub fn min_vertex_cover_tree(n: usize, edges: &[(usize, usize)]) -> usize {
    if n == 0 { return 0; }
    let mut adj = vec![vec![]; n];
    for &(u, v) in edges { adj[u].push(v); adj[v].push(u); }
    let mut dp = vec![[0, 0]; n]; let mut visited = vec![false; n];
    fn dfs(v: usize, adj: &[Vec<usize>], dp: &mut [[usize; 2]], vis: &mut [bool]) {
        vis[v] = true; dp[v][0] = 0; dp[v][1] = 1;
        for &u in &adj[v] { if !vis[u] { dfs(u, adj, dp, vis);
            dp[v][0] += dp[u][1]; dp[v][1] += dp[u][0].min(dp[u][1]); } }
    }
    dfs(0, &adj, &mut dp, &mut visited);
    dp[0][0].min(dp[0][1])
}
#[cfg(test)] mod tests { use super::*;
    #[test] fn test_vc() { assert_eq!(min_vertex_cover_tree(4, &[(0,1),(1,2),(1,3)]), 1); }
}
