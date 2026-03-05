// A* Pathfinding on a 2D grid — Manhattan heuristic, O(E log V)
use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn a_star(
    grid: &[Vec<u8>],
    start: (usize, usize),
    goal:  (usize, usize),
) -> Option<Vec<(usize, usize)>> {
    let rows = grid.len();
    let cols = grid[0].len();
    let inf  = u64::MAX / 2;

    let mut g_cost = vec![vec![inf; cols]; rows];
    let mut came: Vec<Vec<Option<(usize, usize)>>> = vec![vec![None; cols]; rows];
    // heap: Reverse((f, g, r, c))
    let mut heap = BinaryHeap::new();

    let h = |r: usize, c: usize| -> u64 {
        ((r as i64 - goal.0 as i64).abs() + (c as i64 - goal.1 as i64).abs()) as u64
    };

    g_cost[start.0][start.1] = 0;
    heap.push(Reverse((h(start.0, start.1), 0u64, start.0, start.1)));

    while let Some(Reverse((_, g, r, c))) = heap.pop() {
        if g > g_cost[r][c] { continue; } // stale
        if (r, c) == goal {
            // Reconstruct path
            let mut path = Vec::new();
            let mut pos = goal;
            loop {
                path.push(pos);
                if pos == start { break; }
                pos = came[pos.0][pos.1]?;
            }
            path.reverse();
            return Some(path);
        }
        for (dr, dc) in [(-1i64,0),(1,0),(0,-1),(0,1)] {
            let nr = r as i64 + dr;
            let nc = c as i64 + dc;
            if nr < 0 || nr >= rows as i64 || nc < 0 || nc >= cols as i64 { continue; }
            let (nr, nc) = (nr as usize, nc as usize);
            if grid[nr][nc] == b'#' { continue; }
            let ng = g + 1;
            if ng < g_cost[nr][nc] {
                g_cost[nr][nc] = ng;
                came[nr][nc]   = Some((r, c));
                heap.push(Reverse((ng + h(nr, nc), ng, nr, nc)));
            }
        }
    }
    None
}

fn main() {
    let grid: Vec<Vec<u8>> = vec![
        b"...#.".to_vec(),
        b".#.#.".to_vec(),
        b".#...".to_vec(),
        b".##..".to_vec(),  // Fixed: was ".##.." corrected
        b"#....".to_vec(),
    ];
    match a_star(&grid, (0,0), (4,4)) {
        None    => println!("No path found"),
        Some(p) => {
            println!("Path length: {}", p.len() - 1);
            println!("Path: {:?}", p);
        }
    }
}
