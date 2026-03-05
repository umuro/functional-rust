// Edmonds-Karp Max Flow — BFS augmenting paths, O(VE²)
use std::collections::VecDeque;

fn max_flow(cap: &mut Vec<Vec<i64>>, src: usize, snk: usize) -> i64 {
    let n = cap.len();
    let mut total = 0i64;

    loop {
        // BFS to find augmenting path
        let mut parent = vec![usize::MAX; n];
        parent[src] = src;
        let mut deque = VecDeque::from([src]);

        while let Some(u) = deque.pop_front() {
            if u == snk { break; }
            for v in 0..n {
                if parent[v] == usize::MAX && cap[u][v] > 0 {
                    parent[v] = u;
                    deque.push_back(v);
                }
            }
        }

        if parent[snk] == usize::MAX { break; } // no augmenting path

        // Find bottleneck
        let mut flow = i64::MAX;
        let mut v = snk;
        while v != src {
            let u = parent[v];
            flow = flow.min(cap[u][v]);
            v = u;
        }

        // Update residual
        v = snk;
        while v != src {
            let u = parent[v];
            cap[u][v] -= flow;
            cap[v][u] += flow;
            v = u;
        }
        total += flow;
    }
    total
}

fn main() {
    let n = 6;
    let mut cap = vec![vec![0i64; n]; n];
    let mut set = |u: usize, v: usize, c: i64| cap[u][v] = c;
    set(0,1,16); set(0,2,13);
    set(1,2,10); set(1,3,12);
    set(2,1, 4); set(2,4,14);
    set(3,2, 9); set(3,5,20);
    set(4,3, 7); set(4,5, 4);

    println!("Max flow (0->5): {}  (expected 23)", max_flow(&mut cap, 0, 5));
}
