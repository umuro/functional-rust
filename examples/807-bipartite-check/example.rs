// Bipartite Check — BFS 2-colouring O(V+E)
use std::collections::VecDeque;

fn is_bipartite(adj: &[Vec<usize>]) -> Option<Vec<i8>> {
    let n = adj.len();
    let mut color = vec![-1i8; n];

    for start in 0..n {
        if color[start] != -1 { continue; }
        color[start] = 0;
        let mut deque = VecDeque::from([start]);
        while let Some(u) = deque.pop_front() {
            for &v in &adj[u] {
                if color[v] == -1 {
                    color[v] = 1 - color[u];
                    deque.push_back(v);
                } else if color[v] == color[u] {
                    return None; // odd cycle
                }
            }
        }
    }
    Some(color)
}

fn main() {
    // 4-cycle: bipartite
    let mut adj1 = vec![vec![]; 4];
    let mut add1 = |u: usize, v: usize| { adj1[u].push(v); adj1[v].push(u); };
    add1(0,1); add1(1,2); add1(2,3); add1(3,0);
    match is_bipartite(&adj1) {
        None    => println!("4-cycle: NOT bipartite"),
        Some(c) => println!("4-cycle: bipartite, colors={:?}", c),
    }

    // Triangle: not bipartite
    let mut adj2 = vec![vec![]; 3];
    let mut add2 = |u: usize, v: usize| { adj2[u].push(v); adj2[v].push(u); };
    add2(0,1); add2(1,2); add2(2,0);
    match is_bipartite(&adj2) {
        None    => println!("Triangle: NOT bipartite"),
        Some(c) => println!("Triangle: bipartite, colors={:?}", c),
    }

    // Complete bipartite K3,3
    let mut adj3 = vec![vec![]; 6];
    let mut add3 = |u: usize, v: usize| { adj3[u].push(v); adj3[v].push(u); };
    for u in 0..3 { for v in 3..6 { add3(u, v); } }
    match is_bipartite(&adj3) {
        None    => println!("K3,3: NOT bipartite"),
        Some(c) => println!("K3,3: bipartite, colors={:?}", c),
    }
}
