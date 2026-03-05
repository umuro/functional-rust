//! # A* Search
//! Heuristic-guided pathfinding. Time: O(E) with good heuristic

use std::collections::{BinaryHeap, HashMap, HashSet};
use std::cmp::Reverse;

pub fn astar<N: Eq + std::hash::Hash + Clone>(
    start: N, goal: N,
    neighbors: impl Fn(&N) -> Vec<(N, i32)>,
    heuristic: impl Fn(&N) -> i32,
) -> Option<i32> {
    let mut open = BinaryHeap::new();
    let mut g_score: HashMap<N, i32> = HashMap::new();
    
    g_score.insert(start.clone(), 0);
    open.push(Reverse((heuristic(&start), 0, start)));
    
    while let Some(Reverse((_, g, current))) = open.pop() {
        if current == goal { return Some(g); }
        if g > *g_score.get(&current).unwrap_or(&i32::MAX) { continue; }
        
        for (next, cost) in neighbors(&current) {
            let new_g = g + cost;
            if new_g < *g_score.get(&next).unwrap_or(&i32::MAX) {
                g_score.insert(next.clone(), new_g);
                let f = new_g + heuristic(&next);
                open.push(Reverse((f, new_g, next)));
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_astar() {
        let graph: HashMap<i32, Vec<(i32, i32)>> = [(0, vec![(1, 1), (2, 4)]), (1, vec![(2, 2)]), (2, vec![])].into();
        let result = astar(0, 2, |n| graph.get(n).cloned().unwrap_or_default(), |n| (2 - n).abs());
        assert_eq!(result, Some(3));
    }
}
