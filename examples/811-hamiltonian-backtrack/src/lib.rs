//! # Hamiltonian Path (Backtracking)
pub fn hamiltonian_path(n: usize, edges: &[(usize, usize)]) -> Option<Vec<usize>> {
    let mut adj = vec![vec![false; n]; n];
    for &(u, v) in edges {
        adj[u][v] = true;
        adj[v][u] = true;
    }
    let mut path = vec![0];
    let mut visited = vec![false; n];
    visited[0] = true;
    fn backtrack(n: usize, adj: &[Vec<bool>], path: &mut Vec<usize>, vis: &mut [bool]) -> bool {
        if path.len() == n {
            return true;
        }
        let last = *path.last().unwrap();
        for next in 0..n {
            if !vis[next] && adj[last][next] {
                vis[next] = true;
                path.push(next);
                if backtrack(n, adj, path, vis) {
                    return true;
                }
                path.pop();
                vis[next] = false;
            }
        }
        false
    }
    if backtrack(n, &adj, &mut path, &mut visited) {
        Some(path)
    } else {
        None
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_ham() {
        let p = hamiltonian_path(5, &[(0, 1), (1, 2), (2, 3), (3, 4), (4, 0), (1, 3), (0, 2)]);
        assert!(p.is_some());
    }
}
