//! # A* Pathfinding
use std::collections::{BinaryHeap, HashMap};
use std::cmp::Reverse;

pub fn astar(start: (i32, i32), goal: (i32, i32), obstacles: &[(i32, i32)]) -> Option<Vec<(i32, i32)>> {
    let obs: std::collections::HashSet<_> = obstacles.iter().copied().collect();
    let heuristic = |p: (i32, i32)| ((p.0 - goal.0).abs() + (p.1 - goal.1).abs()) as usize;
    let dirs = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    
    let mut open = BinaryHeap::new();
    let mut g_score = HashMap::new();
    let mut came_from = HashMap::new();
    
    g_score.insert(start, 0usize);
    open.push(Reverse((heuristic(start), start)));
    
    while let Some(Reverse((_, current))) = open.pop() {
        if current == goal {
            let mut path = vec![current];
            let mut c = current;
            while let Some(&p) = came_from.get(&c) { path.push(p); c = p; }
            path.reverse();
            return Some(path);
        }
        
        for &(dx, dy) in &dirs {
            let next = (current.0 + dx, current.1 + dy);
            if obs.contains(&next) { continue; }
            let new_g = g_score[&current] + 1;
            if !g_score.contains_key(&next) || new_g < g_score[&next] {
                g_score.insert(next, new_g);
                came_from.insert(next, current);
                open.push(Reverse((new_g + heuristic(next), next)));
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn test_astar() {
        let path = astar((0, 0), (2, 2), &[(1, 1)]);
        assert!(path.is_some());
    }
}
