//! # Max Flow (Ford-Fulkerson with BFS)
use std::collections::VecDeque;
pub fn max_flow(n: usize, edges: &[(usize, usize, i32)], source: usize, sink: usize) -> i32 {
    let mut cap = vec![vec![0i32; n]; n];
    for &(u, v, c) in edges {
        cap[u][v] += c;
    }
    let mut flow = 0;
    loop {
        let mut parent = vec![None; n];
        let mut q = VecDeque::new();
        q.push_back(source);
        parent[source] = Some(source);
        while let Some(u) = q.pop_front() {
            if u == sink {
                break;
            }
            for v in 0..n {
                if parent[v].is_none() && cap[u][v] > 0 {
                    parent[v] = Some(u);
                    q.push_back(v);
                }
            }
        }
        if parent[sink].is_none() {
            break;
        }
        let mut path_flow = i32::MAX;
        let mut v = sink;
        while v != source {
            let u = parent[v].unwrap();
            path_flow = path_flow.min(cap[u][v]);
            v = u;
        }
        v = sink;
        while v != source {
            let u = parent[v].unwrap();
            cap[u][v] -= path_flow;
            cap[v][u] += path_flow;
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
    fn test_flow() {
        assert_eq!(
            max_flow(
                4,
                &[(0, 1, 10), (0, 2, 10), (1, 2, 2), (1, 3, 4), (2, 3, 9)],
                0,
                3
            ),
            13
        );
    }
}
