// 1069: Graph Coloring — Backtracking

// Approach 1: Adjacency matrix
fn graph_coloring(adj: &[Vec<i32>], num_colors: usize) -> Option<Vec<usize>> {
    let n = adj.len();
    let mut colors = vec![0usize; n];

    fn is_safe(node: usize, color: usize, adj: &[Vec<i32>], colors: &[usize]) -> bool {
        for i in 0..adj.len() {
            if adj[node][i] == 1 && colors[i] == color {
                return false;
            }
        }
        true
    }

    fn solve(node: usize, adj: &[Vec<i32>], colors: &mut Vec<usize>, num_colors: usize) -> bool {
        if node == adj.len() {
            return true;
        }
        for c in 1..=num_colors {
            if is_safe(node, c, adj, colors) {
                colors[node] = c;
                if solve(node + 1, adj, colors, num_colors) {
                    return true;
                }
                colors[node] = 0;
            }
        }
        false
    }

    if solve(0, adj, &mut colors, num_colors) {
        Some(colors)
    } else {
        None
    }
}

// Approach 2: Adjacency list
fn graph_coloring_list(adj: &[Vec<usize>], num_colors: usize) -> Option<Vec<usize>> {
    let n = adj.len();
    let mut colors = vec![0usize; n];

    fn is_safe(node: usize, color: usize, adj: &[Vec<usize>], colors: &[usize]) -> bool {
        adj[node].iter().all(|&neighbor| colors[neighbor] != color)
    }

    fn solve(node: usize, adj: &[Vec<usize>], colors: &mut Vec<usize>, num_colors: usize) -> bool {
        if node == adj.len() {
            return true;
        }
        for c in 1..=num_colors {
            if is_safe(node, c, adj, colors) {
                colors[node] = c;
                if solve(node + 1, adj, colors, num_colors) {
                    return true;
                }
                colors[node] = 0;
            }
        }
        false
    }

    if solve(0, adj, &mut colors, num_colors) {
        Some(colors)
    } else {
        None
    }
}

// Approach 3: Greedy coloring (not optimal but fast)
fn greedy_coloring(adj: &[Vec<usize>]) -> Vec<usize> {
    let n = adj.len();
    let mut colors = vec![0usize; n];
    for node in 0..n {
        let used: std::collections::HashSet<usize> = adj[node]
            .iter()
            .filter(|&&nb| colors[nb] > 0)
            .map(|&nb| colors[nb])
            .collect();
        colors[node] = (1..).find(|c| !used.contains(c)).unwrap();
    }
    colors
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_graph_coloring_possible() {
        let adj = vec![
            vec![0, 1, 1, 1],
            vec![1, 0, 1, 0],
            vec![1, 1, 0, 1],
            vec![1, 0, 1, 0],
        ];
        let colors = graph_coloring(&adj, 3).unwrap();
        assert_eq!(colors.len(), 4);
        // Verify no adjacent nodes share color
        for i in 0..4 {
            for j in 0..4 {
                if adj[i][j] == 1 {
                    assert_ne!(colors[i], colors[j]);
                }
            }
        }
    }

    #[test]
    fn test_graph_coloring_impossible() {
        let adj = vec![
            vec![0, 1, 1, 1],
            vec![1, 0, 1, 0],
            vec![1, 1, 0, 1],
            vec![1, 0, 1, 0],
        ];
        assert!(graph_coloring(&adj, 2).is_none());
    }

    #[test]
    fn test_graph_coloring_list() {
        let adj = vec![vec![1, 2, 3], vec![0, 2], vec![0, 1, 3], vec![0, 2]];
        let colors = graph_coloring_list(&adj, 3).unwrap();
        assert_eq!(colors.len(), 4);
    }

    #[test]
    fn test_greedy() {
        let adj = vec![vec![1, 2, 3], vec![0, 2], vec![0, 1, 3], vec![0, 2]];
        let colors = greedy_coloring(&adj);
        for (node, neighbors) in adj.iter().enumerate() {
            for &nb in neighbors {
                assert_ne!(colors[node], colors[nb]);
            }
        }
    }
}
