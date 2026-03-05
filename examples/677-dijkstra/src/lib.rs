//! # Dijkstra's Algorithm
//!
//! Shortest path from single source in weighted graph.
//! Time: O((V+E) log V) with binary heap

use std::collections::{BinaryHeap, HashMap};
use std::cmp::Reverse;

/// Dijkstra's shortest path
pub fn dijkstra(
    graph: &HashMap<usize, Vec<(usize, i32)>>,
    start: usize,
    n: usize,
) -> Vec<i32> {
    let mut dist = vec![i32::MAX; n];
    let mut heap = BinaryHeap::new();
    
    dist[start] = 0;
    heap.push(Reverse((0, start)));
    
    while let Some(Reverse((d, u))) = heap.pop() {
        if d > dist[u] { continue; }
        
        if let Some(neighbors) = graph.get(&u) {
            for &(v, w) in neighbors {
                let nd = d + w;
                if nd < dist[v] {
                    dist[v] = nd;
                    heap.push(Reverse((nd, v)));
                }
            }
        }
    }
    dist
}

/// Dijkstra with path reconstruction
pub fn dijkstra_path(
    graph: &HashMap<usize, Vec<(usize, i32)>>,
    start: usize,
    end: usize,
    n: usize,
) -> Option<(i32, Vec<usize>)> {
    let mut dist = vec![i32::MAX; n];
    let mut prev = vec![None; n];
    let mut heap = BinaryHeap::new();
    
    dist[start] = 0;
    heap.push(Reverse((0, start)));
    
    while let Some(Reverse((d, u))) = heap.pop() {
        if u == end { break; }
        if d > dist[u] { continue; }
        
        if let Some(neighbors) = graph.get(&u) {
            for &(v, w) in neighbors {
                let nd = d + w;
                if nd < dist[v] {
                    dist[v] = nd;
                    prev[v] = Some(u);
                    heap.push(Reverse((nd, v)));
                }
            }
        }
    }
    
    if dist[end] == i32::MAX { return None; }
    
    let mut path = vec![];
    let mut curr = Some(end);
    while let Some(c) = curr {
        path.push(c);
        curr = prev[c];
    }
    path.reverse();
    Some((dist[end], path))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dijkstra() {
        let mut g = HashMap::new();
        g.insert(0, vec![(1, 4), (2, 1)]);
        g.insert(1, vec![(3, 1)]);
        g.insert(2, vec![(1, 2), (3, 5)]);
        g.insert(3, vec![]);
        
        let dist = dijkstra(&g, 0, 4);
        assert_eq!(dist[3], 4); // 0->2->1->3
    }

    #[test]
    fn test_dijkstra_path() {
        let mut g = HashMap::new();
        g.insert(0, vec![(1, 4), (2, 1)]);
        g.insert(1, vec![(3, 1)]);
        g.insert(2, vec![(1, 2), (3, 5)]);
        g.insert(3, vec![]);
        
        let (d, path) = dijkstra_path(&g, 0, 3, 4).unwrap();
        assert_eq!(d, 4);
        assert_eq!(path, vec![0, 2, 1, 3]);
    }
}
