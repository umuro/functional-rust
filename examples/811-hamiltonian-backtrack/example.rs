// Hamiltonian Cycle — backtracking O(n!) worst case

fn hamiltonian_cycle(adj: &[Vec<usize>]) -> Option<Vec<usize>> {
    let n = adj.len();
    let mut path    = vec![0usize; n];
    let mut visited = vec![false; n];
    visited[0] = true;

    fn backtrack(
        pos: usize,
        n: usize,
        adj: &[Vec<usize>],
        path: &mut Vec<usize>,
        visited: &mut Vec<bool>,
    ) -> bool {
        if pos == n {
            // Check closing edge
            return adj[path[n - 1]].contains(&0);
        }
        for v in 0..n {
            if !visited[v] && adj[path[pos - 1]].contains(&v) {
                path[pos]    = v;
                visited[v]   = true;
                if backtrack(pos + 1, n, adj, path, visited) { return true; }
                visited[v]   = false;
            }
        }
        false
    }

    if backtrack(1, n, adj, &mut path, &mut visited) {
        let mut result = path.clone();
        result.push(0);
        Some(result)
    } else {
        None
    }
}

fn main() {
    // 5-cycle
    let n = 5;
    let mut adj = vec![vec![]; n];
    let mut add = |u: usize, v: usize| { adj[u].push(v); adj[v].push(u); };
    add(0,1); add(1,2); add(2,3); add(3,4); add(4,0);
    println!("5-cycle Ham: {:?}", hamiltonian_cycle(&adj));

    // K4
    let adj2: Vec<Vec<usize>> = (0..4)
        .map(|u| (0..4).filter(|&v| v != u).collect())
        .collect();
    println!("K4 Ham:      {:?}", hamiltonian_cycle(&adj2));

    // Path graph P4 — no Hamiltonian cycle (only path)
    let adj3 = vec![vec![1usize], vec![0,2], vec![1,3], vec![2usize]];
    println!("P4 Ham:      {:?}", hamiltonian_cycle(&adj3));
}
