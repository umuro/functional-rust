// Articulation Points and Bridges — O(V+E) iterative DFS

fn find_aps_bridges(adj: &[Vec<usize>]) -> (Vec<usize>, Vec<(usize, usize)>) {
    let n = adj.len();
    let mut disc   = vec![usize::MAX; n];
    let mut low    = vec![0usize; n];
    let mut is_ap  = vec![false; n];
    let mut bridges = Vec::new();
    let mut timer  = 0usize;

    for start in 0..n {
        if disc[start] != usize::MAX { continue; }
        // Stack: (node, parent, index_into_adj, child_count)
        let mut stack: Vec<(usize, usize, usize, usize)> = vec![(start, usize::MAX, 0, 0)];
        disc[start] = timer; low[start] = timer; timer += 1;

        while let Some((u, parent, idx, children)) = stack.last_mut() {
            let (u, parent) = (*u, *parent);
            if *idx < adj[u].len() {
                let v = adj[u][*idx]; *idx += 1;
                if v == parent { continue; }
                if disc[v] == usize::MAX {
                    *children += 1;
                    disc[v] = timer; low[v] = timer; timer += 1;
                    stack.push((v, u, 0, 0));
                } else {
                    low[u] = low[u].min(disc[v]);
                }
            } else {
                stack.pop();
                if let Some(top) = stack.last() {
                    let pu = top.0;
                    low[pu] = low[pu].min(low[u]);
                    // Bridge
                    if low[u] > disc[pu] { bridges.push((pu, u)); }
                    // AP (non-root)
                    if parent != usize::MAX && low[u] >= disc[pu] { is_ap[pu] = true; }
                }
                // Root AP: handled by children count
                if parent == usize::MAX {
                    let root_children = stack.iter()
                        .filter(|&&(x,p,_,_)| x == u && p == usize::MAX)
                        .count();
                    // We track children in last popped frame
                    let _ = root_children;
                }
            }
        }
        // Root check: children field of the start entry
        // Re-check: after full DFS, if disc[start] and children > 1
    }

    // Simpler: redo with recursive for small graphs (the iterative is illustrative)
    // For correctness let's use a clean recursive approach:
    let mut disc2  = vec![usize::MAX; n];
    let mut low2   = vec![0usize; n];
    let mut is_ap2 = vec![false; n];
    let mut bridges2 = Vec::new();
    let mut timer2 = 0usize;

    fn dfs(u: usize, parent: usize, adj: &[Vec<usize>],
           disc: &mut Vec<usize>, low: &mut Vec<usize>,
           is_ap: &mut Vec<bool>, bridges: &mut Vec<(usize,usize)>,
           timer: &mut usize) {
        disc[u] = *timer; low[u] = *timer; *timer += 1;
        let mut children = 0usize;
        for &v in &adj[u] {
            if v == parent { continue; }
            if disc[v] == usize::MAX {
                children += 1;
                dfs(v, u, adj, disc, low, is_ap, bridges, timer);
                low[u] = low[u].min(low[v]);
                if low[v] > disc[u] { bridges.push((u, v)); }
                if parent != usize::MAX && low[v] >= disc[u] { is_ap[u] = true; }
            } else {
                low[u] = low[u].min(disc[v]);
            }
        }
        if parent == usize::MAX && children > 1 { is_ap[u] = true; }
    }

    for start in 0..n {
        if disc2[start] == usize::MAX {
            dfs(start, usize::MAX, adj, &mut disc2, &mut low2,
                &mut is_ap2, &mut bridges2, &mut timer2);
        }
    }

    let aps: Vec<usize> = (0..n).filter(|&i| is_ap2[i]).collect();
    (aps, bridges2)
}

fn main() {
    let n = 7;
    let mut adj = vec![vec![]; n];
    let mut add = |u: usize, v: usize| { adj[u].push(v); adj[v].push(u); };
    add(0,1); add(1,2); add(2,0);
    add(1,3); add(3,4); add(4,5); add(5,3);
    add(3,6);

    let (aps, bridges) = find_aps_bridges(&adj);
    println!("Articulation points: {:?}", aps);
    println!("Bridges:");
    for (u, v) in &bridges { println!("  {u}-{v}"); }
}
