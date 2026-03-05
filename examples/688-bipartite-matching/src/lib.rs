//! # Bipartite Matching
//! Hungarian algorithm / maximum matching

pub fn max_bipartite_matching(graph: &[Vec<usize>], m: usize) -> usize {
    let mut match_r = vec![None; m];
    let mut result = 0;
    
    for u in 0..graph.len() {
        let mut seen = vec![false; m];
        if dfs(u, graph, &mut match_r, &mut seen) { result += 1; }
    }
    result
}

fn dfs(u: usize, graph: &[Vec<usize>], match_r: &mut [Option<usize>], seen: &mut [bool]) -> bool {
    for &v in &graph[u] {
        if seen[v] { continue; }
        seen[v] = true;
        if match_r[v].is_none() || dfs(match_r[v].unwrap(), graph, match_r, seen) {
            match_r[v] = Some(u);
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_matching() {
        let graph = vec![vec![0, 1], vec![0], vec![1, 2], vec![2]];
        assert_eq!(max_bipartite_matching(&graph, 3), 3);
    }
}
