//! # Kosaraju's SCC Algorithm
pub fn kosaraju(n: usize, edges: &[(usize, usize)]) -> Vec<Vec<usize>> {
    let mut adj = vec![vec![]; n];
    let mut radj = vec![vec![]; n];
    for &(u, v) in edges {
        adj[u].push(v);
        radj[v].push(u);
    }
    let mut visited = vec![false; n];
    let mut order = vec![];
    fn dfs1(v: usize, adj: &[Vec<usize>], vis: &mut [bool], ord: &mut Vec<usize>) {
        vis[v] = true;
        for &u in &adj[v] {
            if !vis[u] {
                dfs1(u, adj, vis, ord);
            }
        }
        ord.push(v);
    }
    fn dfs2(v: usize, radj: &[Vec<usize>], vis: &mut [bool], comp: &mut Vec<usize>) {
        vis[v] = true;
        comp.push(v);
        for &u in &radj[v] {
            if !vis[u] {
                dfs2(u, radj, vis, comp);
            }
        }
    }
    for v in 0..n {
        if !visited[v] {
            dfs1(v, &adj, &mut visited, &mut order);
        }
    }
    visited.fill(false);
    let mut sccs = vec![];
    for &v in order.iter().rev() {
        if !visited[v] {
            let mut comp = vec![];
            dfs2(v, &radj, &mut visited, &mut comp);
            sccs.push(comp);
        }
    }
    sccs
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_kosaraju() {
        let sccs = kosaraju(5, &[(0, 1), (1, 2), (2, 0), (1, 3), (3, 4)]);
        assert!(!sccs.is_empty());
    }
}
