//! # Graph Coloring
//! Vertex coloring with greedy algorithm

use std::collections::HashMap;

pub fn greedy_coloring(graph: &HashMap<usize, Vec<usize>>, n: usize) -> Vec<usize> {
    let mut colors = vec![usize::MAX; n];
    
    for v in 0..n {
        let mut used = vec![false; n];
        if let Some(neighbors) = graph.get(&v) {
            for &u in neighbors {
                if colors[u] != usize::MAX { used[colors[u]] = true; }
            }
        }
        colors[v] = used.iter().position(|&x| !x).unwrap_or(0);
    }
    colors
}

pub fn chromatic_number(colors: &[usize]) -> usize {
    colors.iter().max().map(|&m| m + 1).unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_coloring() {
        let mut g = HashMap::new();
        g.insert(0, vec![1, 2]);
        g.insert(1, vec![0, 2]);
        g.insert(2, vec![0, 1]);
        let colors = greedy_coloring(&g, 3);
        assert_eq!(chromatic_number(&colors), 3);
    }
}
