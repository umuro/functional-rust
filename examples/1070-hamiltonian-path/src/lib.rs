// 1070: Hamiltonian Path — Backtracking

// Approach 1: Adjacency matrix backtracking (fixed start = 0)
fn hamiltonian_path(adj: &[Vec<i32>]) -> Option<Vec<usize>> {
    let n = adj.len();
    let mut path = vec![0usize; n];
    let mut visited = vec![false; n];
    path[0] = 0;
    visited[0] = true;

    fn solve(pos: usize, adj: &[Vec<i32>], path: &mut Vec<usize>, visited: &mut Vec<bool>) -> bool {
        let n = adj.len();
        if pos == n {
            return true;
        }
        for v in 0..n {
            if !visited[v] && adj[path[pos - 1]][v] == 1 {
                path[pos] = v;
                visited[v] = true;
                if solve(pos + 1, adj, path, visited) {
                    return true;
                }
                visited[v] = false;
            }
        }
        false
    }

    if solve(1, adj, &mut path, &mut visited) {
        Some(path)
    } else {
        None
    }
}

// Approach 2: Try all starting vertices
fn hamiltonian_path_any(adj: &[Vec<i32>]) -> Option<Vec<usize>> {
    let n = adj.len();
    for start in 0..n {
        let mut path = vec![0usize; n];
        let mut visited = vec![false; n];
        path[0] = start;
        visited[start] = true;

        fn solve(
            pos: usize,
            adj: &[Vec<i32>],
            path: &mut Vec<usize>,
            visited: &mut Vec<bool>,
        ) -> bool {
            let n = adj.len();
            if pos == n {
                return true;
            }
            for v in 0..n {
                if !visited[v] && adj[path[pos - 1]][v] == 1 {
                    path[pos] = v;
                    visited[v] = true;
                    if solve(pos + 1, adj, path, visited) {
                        return true;
                    }
                    visited[v] = false;
                }
            }
            false
        }

        if solve(1, adj, &mut path, &mut visited) {
            return Some(path);
        }
    }
    None
}

// Approach 3: Bitmask DP for Hamiltonian path existence (O(2^n * n^2))
fn hamiltonian_exists_dp(adj: &[Vec<i32>]) -> bool {
    let n = adj.len();
    if n == 0 {
        return true;
    }
    // dp[mask][i] = can we reach node i having visited exactly the nodes in mask?
    let mut dp = vec![vec![false; n]; 1 << n];
    for i in 0..n {
        dp[1 << i][i] = true;
    }
    for mask in 1..(1 << n) {
        for u in 0..n {
            if !dp[mask][u] {
                continue;
            }
            if mask & (1 << u) == 0 {
                continue;
            }
            for v in 0..n {
                if mask & (1 << v) == 0 && adj[u][v] == 1 {
                    dp[mask | (1 << v)][v] = true;
                }
            }
        }
    }
    let full = (1 << n) - 1;
    (0..n).any(|i| dp[full][i])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_complete_graph() {
        let adj = vec![
            vec![0, 1, 1, 1],
            vec![1, 0, 1, 1],
            vec![1, 1, 0, 1],
            vec![1, 1, 1, 0],
        ];
        let path = hamiltonian_path(&adj).unwrap();
        assert_eq!(path.len(), 4);
    }

    #[test]
    fn test_path_graph() {
        let adj = vec![
            vec![0, 1, 0, 0],
            vec![1, 0, 1, 0],
            vec![0, 1, 0, 1],
            vec![0, 0, 1, 0],
        ];
        let path = hamiltonian_path(&adj).unwrap();
        assert_eq!(path.len(), 4);
    }

    #[test]
    fn test_any_start() {
        let adj = vec![
            vec![0, 1, 1, 1],
            vec![1, 0, 1, 1],
            vec![1, 1, 0, 1],
            vec![1, 1, 1, 0],
        ];
        assert!(hamiltonian_path_any(&adj).is_some());
    }

    #[test]
    fn test_bitmask_dp() {
        let adj = vec![
            vec![0, 1, 1, 1],
            vec![1, 0, 1, 1],
            vec![1, 1, 0, 1],
            vec![1, 1, 1, 0],
        ];
        assert!(hamiltonian_exists_dp(&adj));

        // Disconnected graph
        let adj2 = vec![vec![0, 0, 0], vec![0, 0, 0], vec![0, 0, 0]];
        assert!(!hamiltonian_exists_dp(&adj2));
    }
}
