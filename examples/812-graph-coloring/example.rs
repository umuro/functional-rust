// Graph m-Colouring — backtracking O(m^V)

fn graph_color(adj: &[Vec<usize>], m: usize) -> Option<Vec<usize>> {
    let n = adj.len();
    let mut color = vec![usize::MAX; n];

    fn is_safe(v: usize, c: usize, adj: &[Vec<usize>], color: &[usize]) -> bool {
        adj[v].iter().all(|&u| color[u] != c)
    }

    fn solve(v: usize, n: usize, m: usize, adj: &[Vec<usize>], color: &mut Vec<usize>) -> bool {
        if v == n { return true; }
        for c in 0..m {
            if is_safe(v, c, adj, color) {
                color[v] = c;
                if solve(v + 1, n, m, adj, color) { return true; }
                color[v] = usize::MAX;
            }
        }
        false
    }

    if solve(0, n, m, adj, &mut color) { Some(color) } else { None }
}

fn chromatic_number(adj: &[Vec<usize>]) -> usize {
    let n = adj.len();
    (1..=n).find(|&m| graph_color(adj, m).is_some()).unwrap_or(n)
}

fn main() {
    // Petersen graph — chromatic number = 3
    let n = 10;
    let mut adj = vec![vec![]; n];
    let mut add = |u: usize, v: usize| { adj[u].push(v); adj[v].push(u); };
    add(0,1); add(1,2); add(2,3); add(3,4); add(4,0); // outer
    add(5,7); add(7,9); add(9,6); add(6,8); add(8,5); // inner
    add(0,5); add(1,6); add(2,7); add(3,8); add(4,9); // spokes

    let chi = chromatic_number(&adj);
    println!("Petersen chromatic number: {chi}  (expected 3)");
    match graph_color(&adj, 3) {
        None    => println!("No 3-colouring"),
        Some(c) => println!("3-colouring: {:?}", c),
    }

    // K5: chromatic number = 5
    let adj2: Vec<Vec<usize>> = (0..5).map(|u| (0..5usize).filter(|&v| v!=u).collect()).collect();
    println!("K5 chromatic number: {}", chromatic_number(&adj2));
}
