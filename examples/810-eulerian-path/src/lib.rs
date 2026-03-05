//! # Eulerian Path
pub fn eulerian_path(n: usize, edges: &[(usize, usize)]) -> Option<Vec<usize>> {
    let mut adj: Vec<Vec<usize>> = vec![vec![]; n];
    let mut deg = vec![[0, 0]; n];
    for &(u, v) in edges { adj[u].push(v); deg[u][0] += 1; deg[v][1] += 1; }
    let (mut start, mut end) = (None, None);
    for v in 0..n {
        let diff = deg[v][0] as i32 - deg[v][1] as i32;
        match diff { 1 => if start.is_some() { return None; } else { start = Some(v); },
            -1 => if end.is_some() { return None; } else { end = Some(v); },
            0 => {}, _ => return None }
    }
    let start = start.unwrap_or(0);
    let mut path = vec![]; let mut stack = vec![start];
    while let Some(&v) = stack.last() {
        if adj[v].is_empty() { path.push(v); stack.pop(); }
        else { let u = adj[v].pop().unwrap(); stack.push(u); }
    }
    path.reverse(); if path.len() != edges.len() + 1 { None } else { Some(path) }
}
#[cfg(test)] mod tests { use super::*;
    #[test] fn test_euler() { let p = eulerian_path(4, &[(0,1),(1,2),(2,0),(0,3),(3,0)]); assert!(p.is_some()); }
}
