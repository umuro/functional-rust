//! # Strongly Connected Components
//! Kosaraju's algorithm for finding SCCs. Time: O(V+E)

use std::collections::HashMap;

pub fn kosaraju(graph: &HashMap<usize, Vec<usize>>, n: usize) -> Vec<Vec<usize>> {
    let mut visited = vec![false; n];
    let mut order = Vec::new();
    
    // First DFS - fill order
    for i in 0..n {
        if !visited[i] { dfs1(i, graph, &mut visited, &mut order); }
    }
    
    // Build reverse graph
    let mut rev = HashMap::new();
    for (&u, neighbors) in graph {
        for &v in neighbors { rev.entry(v).or_insert_with(Vec::new).push(u); }
    }
    
    // Second DFS on reverse in reverse order
    visited.fill(false);
    let mut sccs = Vec::new();
    for &v in order.iter().rev() {
        if !visited[v] {
            let mut scc = Vec::new();
            dfs2(v, &rev, &mut visited, &mut scc);
            sccs.push(scc);
        }
    }
    sccs
}

fn dfs1(v: usize, g: &HashMap<usize, Vec<usize>>, vis: &mut [bool], ord: &mut Vec<usize>) {
    vis[v] = true;
    if let Some(ns) = g.get(&v) { for &u in ns { if !vis[u] { dfs1(u, g, vis, ord); } } }
    ord.push(v);
}

fn dfs2(v: usize, g: &HashMap<usize, Vec<usize>>, vis: &mut [bool], scc: &mut Vec<usize>) {
    vis[v] = true;
    scc.push(v);
    if let Some(ns) = g.get(&v) { for &u in ns { if !vis[u] { dfs2(u, g, vis, scc); } } }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_scc() {
        let mut g = HashMap::new();
        g.insert(0, vec![1]);
        g.insert(1, vec![2]);
        g.insert(2, vec![0, 3]);
        g.insert(3, vec![4]);
        g.insert(4, vec![5]);
        g.insert(5, vec![3]);
        let sccs = kosaraju(&g, 6);
        assert_eq!(sccs.len(), 2);
    }
}
