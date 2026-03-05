// Eulerian Path/Circuit — Hierholzer's algorithm O(V+E)

fn eulerian_path(adj: &[Vec<usize>]) -> Option<Vec<usize>> {
    let n = adj.len();
    let degree: Vec<usize> = adj.iter().map(|v| v.len()).collect();

    let odd_verts: Vec<usize> = (0..n).filter(|&i| degree[i] % 2 != 0).collect();
    let start = match odd_verts.len() {
        0 => 0,
        2 => odd_verts[0],
        _ => return None, // No Eulerian path
    };

    // Mutable index into each adjacency list (pointer to next unused edge)
    let mut idx = vec![0usize; n];
    // For undirected: we need to track edge usage. Use adjacency list copy.
    let mut adj_mut: Vec<Vec<usize>> = adj.to_vec();

    let mut stack   = vec![start];
    let mut circuit = Vec::new();

    while let Some(&v) = stack.last() {
        if idx[v] < adj_mut[v].len() {
            let u = adj_mut[v][idx[v]];
            idx[v] += 1;
            // Remove v from u's adjacency list (mark used)
            if let Some(pos) = adj_mut[u].iter().position(|&x| x == v) {
                adj_mut[u].swap_remove(pos);
                // Adjust idx[u] if needed
                if pos < idx[u] && idx[u] > 0 { idx[u] -= 1; }
            }
            stack.push(u);
        } else {
            circuit.push(stack.pop().unwrap());
        }
    }
    circuit.reverse();
    Some(circuit)
}

fn main() {
    // Triangle circuit
    let adj1 = vec![vec![1usize,2], vec![0,2], vec![0,1]];
    println!("Triangle: {:?}", eulerian_path(&adj1));

    // Euler path: 0-1-2-3-4-1-3 (two odd vertices: 0 and 4)
    let mut adj2 = vec![vec![]; 5];
    let mut add = |u: usize, v: usize| { adj2[u].push(v); adj2[v].push(u); };
    add(0,1); add(1,2); add(2,3); add(3,4); add(4,1); add(1,3);
    println!("Path:     {:?}", eulerian_path(&adj2));

    // No Eulerian path (4 odd-degree vertices)
    let mut adj3 = vec![vec![]; 4];
    let mut addx = |u: usize, v: usize| { adj3[u].push(v); adj3[v].push(u); };
    addx(0,1); addx(0,2); addx(1,2); addx(1,3);
    println!("No path:  {:?}", eulerian_path(&adj3));
}
