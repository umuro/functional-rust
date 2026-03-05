// Topological Sort — iterative DFS with cycle detection O(V+E)

fn topo_sort(adj: &[Vec<usize>]) -> Result<Vec<usize>, usize> {
    let n = adj.len();
    let mut state  = vec![0u8; n]; // 0=white,1=grey,2=black
    let mut result = Vec::new();

    for start in 0..n {
        if state[start] != 0 { continue; }
        // Iterative DFS: (node, index)
        let mut stack: Vec<(usize, usize)> = vec![(start, 0)];
        state[start] = 1; // grey

        while let Some((u, idx)) = stack.last_mut() {
            let u = *u;
            if *idx < adj[u].len() {
                let v = adj[u][*idx]; *idx += 1;
                match state[v] {
                    1 => return Err(v), // back edge = cycle
                    0 => { state[v] = 1; stack.push((v, 0)); }
                    _ => {}
                }
            } else {
                state[u] = 2; // black
                result.push(u);
                stack.pop();
            }
        }
    }
    result.reverse();
    Ok(result)
}

fn main() {
    // DAG
    let mut adj = vec![vec![]; 6];
    let mut add = |u: usize, v: usize| adj[u].push(v);
    add(5,2); add(5,0); add(4,0); add(4,1); add(2,3); add(3,1);
    match topo_sort(&adj) {
        Err(c)  => println!("Cycle at node {c}"),
        Ok(ord) => println!("Topological order: {:?}", ord),
    }

    // Graph with cycle
    let adj2 = vec![vec![1], vec![2], vec![0]];
    match topo_sort(&adj2) {
        Err(_)  => println!("Cycle detected (correct)"),
        Ok(ord) => println!("Order: {:?}", ord),
    }
}
