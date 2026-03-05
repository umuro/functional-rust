//! # Prim's MST Algorithm
use std::collections::BinaryHeap;
use std::cmp::Reverse;

pub fn prims_mst(n: usize, edges: &[(usize, usize, i32)]) -> i32 {
    let mut adj = vec![vec![]; n];
    for &(u, v, w) in edges { adj[u].push((v, w)); adj[v].push((u, w)); }
    let mut visited = vec![false; n];
    let mut heap = BinaryHeap::new();
    heap.push(Reverse((0, 0)));
    let mut total = 0;
    
    while let Some(Reverse((w, u))) = heap.pop() {
        if visited[u] { continue; }
        visited[u] = true;
        total += w;
        for &(v, wt) in &adj[u] {
            if !visited[v] { heap.push(Reverse((wt, v))); }
        }
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn test_prims() {
        let edges = [(0, 1, 4), (0, 7, 8), (1, 2, 8), (1, 7, 11)];
        assert!(prims_mst(8, &edges) > 0);
    }
}
