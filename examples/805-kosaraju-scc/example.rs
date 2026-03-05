// Kosaraju's SCC — two iterative DFS passes, O(V+E)

fn kosaraju(adj: &[Vec<usize>]) -> Vec<Vec<usize>> {
    let n = adj.len();

    // Pass 1: DFS on original, record finish order
    let mut visited = vec![false; n];
    let mut finish  = Vec::new();
    for start in 0..n {
        if visited[start] { continue; }
        let mut stack = vec![(start, 0usize)];
        visited[start] = true;
        while let Some((u, idx)) = stack.last_mut() {
            let u = *u;
            if *idx < adj[u].len() {
                let v = adj[u][*idx]; *idx += 1;
                if !visited[v] { visited[v] = true; stack.push((v, 0)); }
            } else {
                finish.push(u);
                stack.pop();
            }
        }
    }

    // Build transposed graph
    let mut radj = vec![vec![]; n];
    for u in 0..n {
        for &v in &adj[u] { radj[v].push(u); }
    }

    // Pass 2: DFS on transposed in reverse finish order
    let mut visited2 = vec![false; n];
    let mut sccs = Vec::new();
    for &start in finish.iter().rev() {
        if visited2[start] { continue; }
        let mut scc   = Vec::new();
        let mut stack = vec![start];
        visited2[start] = true;
        while let Some(u) = stack.pop() {
            scc.push(u);
            for &v in &radj[u] {
                if !visited2[v] { visited2[v] = true; stack.push(v); }
            }
        }
        sccs.push(scc);
    }
    sccs
}

fn main() {
    let n = 8;
    let mut adj = vec![vec![]; n];
    let mut add = |u: usize, v: usize| adj[u].push(v);
    add(0,1); add(1,2); add(2,0); add(2,3);
    add(3,4); add(4,5); add(5,3);
    add(6,5); add(6,7); add(7,6);

    let sccs = kosaraju(&adj);
    println!("SCCs ({} total):", sccs.len());
    for (i, scc) in sccs.iter().enumerate() {
        println!("  SCC {}: {:?}", i + 1, scc);
    }
}
