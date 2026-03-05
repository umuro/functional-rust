//! # Maximum Flow
//! Ford-Fulkerson with BFS (Edmonds-Karp)

use std::collections::VecDeque;

pub fn max_flow(capacity: &mut Vec<Vec<i32>>, source: usize, sink: usize) -> i32 {
    let n = capacity.len();
    let mut flow = 0;
    
    loop {
        let mut parent = vec![None; n];
        let mut visited = vec![false; n];
        let mut queue = VecDeque::new();
        
        queue.push_back(source);
        visited[source] = true;
        
        while let Some(u) = queue.pop_front() {
            for v in 0..n {
                if !visited[v] && capacity[u][v] > 0 {
                    visited[v] = true;
                    parent[v] = Some(u);
                    queue.push_back(v);
                }
            }
        }
        
        if !visited[sink] { break; }
        
        let mut path_flow = i32::MAX;
        let mut v = sink;
        while let Some(u) = parent[v] {
            path_flow = path_flow.min(capacity[u][v]);
            v = u;
        }
        
        v = sink;
        while let Some(u) = parent[v] {
            capacity[u][v] -= path_flow;
            capacity[v][u] += path_flow;
            v = u;
        }
        flow += path_flow;
    }
    flow
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_max_flow() {
        let mut cap = vec![
            vec![0, 16, 13, 0, 0, 0],
            vec![0, 0, 10, 12, 0, 0],
            vec![0, 4, 0, 0, 14, 0],
            vec![0, 0, 9, 0, 0, 20],
            vec![0, 0, 0, 7, 0, 4],
            vec![0, 0, 0, 0, 0, 0],
        ];
        assert_eq!(max_flow(&mut cap, 0, 5), 23);
    }
}
