//! # Prim's Algorithm
//! MST using priority queue. Time: O(E log V)

use std::collections::{BinaryHeap, HashMap};
use std::cmp::Reverse;

pub fn prim(graph: &HashMap<usize, Vec<(usize, i32)>>, n: usize) -> i32 {
    let mut visited = vec![false; n];
    let mut heap = BinaryHeap::new();
    let mut cost = 0;
    
    heap.push(Reverse((0, 0usize)));
    
    while let Some(Reverse((w, u))) = heap.pop() {
        if visited[u] { continue; }
        visited[u] = true;
        cost += w;
        
        if let Some(neighbors) = graph.get(&u) {
            for &(v, weight) in neighbors {
                if !visited[v] { heap.push(Reverse((weight, v))); }
            }
        }
    }
    cost
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_prim() {
        let mut g = HashMap::new();
        g.insert(0, vec![(1, 4), (2, 3)]);
        g.insert(1, vec![(0, 4), (2, 1), (3, 2)]);
        g.insert(2, vec![(0, 3), (1, 1), (3, 4)]);
        g.insert(3, vec![(1, 2), (2, 4)]);
        assert_eq!(prim(&g, 4), 6);
    }
}
