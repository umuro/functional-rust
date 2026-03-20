#![allow(clippy::all)]
//! # Articulation Points
pub fn articulation_points(n: usize, edges: &[(usize, usize)]) -> Vec<usize> {
    let mut adj = vec![vec![]; n];
    for &(u, v) in edges {
        adj[u].push(v);
        adj[v].push(u);
    }
    let mut disc = vec![0; n];
    let mut low = vec![0; n];
    let mut parent = vec![None; n];
    let mut ap = vec![false; n];
    let mut time = 0;
    let mut visited = vec![false; n];
    fn dfs(
        u: usize,
        adj: &[Vec<usize>],
        disc: &mut [usize],
        low: &mut [usize],
        parent: &mut [Option<usize>],
        ap: &mut [bool],
        time: &mut usize,
        vis: &mut [bool],
    ) {
        vis[u] = true;
        disc[u] = *time;
        low[u] = *time;
        *time += 1;
        let mut children = 0;
        for &v in &adj[u] {
            if !vis[v] {
                children += 1;
                parent[v] = Some(u);
                dfs(v, adj, disc, low, parent, ap, time, vis);
                low[u] = low[u].min(low[v]);
                if parent[u].is_none() && children > 1 {
                    ap[u] = true;
                }
                if parent[u].is_some() && low[v] >= disc[u] {
                    ap[u] = true;
                }
            } else if parent[u] != Some(v) {
                low[u] = low[u].min(disc[v]);
            }
        }
    }
    for v in 0..n {
        if !visited[v] {
            dfs(
                v,
                &adj,
                &mut disc,
                &mut low,
                &mut parent,
                &mut ap,
                &mut time,
                &mut visited,
            );
        }
    }
    (0..n).filter(|&i| ap[i]).collect()
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_ap() {
        let ap = articulation_points(5, &[(0, 1), (1, 2), (2, 0), (1, 3), (3, 4)]);
        assert!(!ap.is_empty());
    }
}
