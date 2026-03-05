// Tarjan's SCC — O(V+E), iterative DFS to avoid stack overflow

fn tarjan_scc(adj: &[Vec<usize>]) -> Vec<Vec<usize>> {
    let n = adj.len();
    let mut disc     = vec![usize::MAX; n];
    let mut low      = vec![0usize; n];
    let mut on_stack = vec![false; n];
    let mut stack    = Vec::new();
    let mut timer    = 0usize;
    let mut sccs     = Vec::new();

    for start in 0..n {
        if disc[start] != usize::MAX { continue; }
        // Iterative DFS: (node, index into adj list)
        let mut call_stack: Vec<(usize, usize)> = vec![(start, 0)];
        disc[start] = timer; low[start] = timer; timer += 1;
        stack.push(start); on_stack[start] = true;

        while let Some((u, idx)) = call_stack.last_mut() {
            let u = *u;
            if *idx < adj[u].len() {
                let v = adj[u][*idx];
                *idx += 1;
                if disc[v] == usize::MAX {
                    disc[v] = timer; low[v] = timer; timer += 1;
                    stack.push(v); on_stack[v] = true;
                    call_stack.push((v, 0));
                } else if on_stack[v] {
                    low[u] = low[u].min(disc[v]);
                }
            } else {
                call_stack.pop();
                if let Some(&(parent, _)) = call_stack.last() {
                    low[parent] = low[parent].min(low[u]);
                }
                if low[u] == disc[u] {
                    let mut scc = Vec::new();
                    loop {
                        let w = stack.pop().unwrap();
                        on_stack[w] = false;
                        scc.push(w);
                        if w == u { break; }
                    }
                    sccs.push(scc);
                }
            }
        }
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

    let sccs = tarjan_scc(&adj);
    println!("Number of SCCs: {}", sccs.len());
    for (i, scc) in sccs.iter().enumerate() {
        println!("  SCC {}: {:?}", i + 1, scc);
    }
}
