// Minimum Vertex Cover — 2-approximation O(V+E) + exact backtracking
use std::collections::HashSet;

fn vertex_cover_2approx(n: usize, edges: &[(usize, usize)]) -> HashSet<usize> {
    let mut covered = vec![false; n];
    let mut cover   = HashSet::new();
    for &(u, v) in edges {
        if !covered[u] && !covered[v] {
            covered[u] = true;
            covered[v] = true;
            cover.insert(u);
            cover.insert(v);
        }
    }
    cover
}

fn vertex_cover_exact(n: usize, edges: &[(usize, usize)]) -> Vec<usize> {
    let mut best     = n + 1;
    let mut best_mask = 0u64;

    fn is_cover(mask: u64, edges: &[(usize, usize)]) -> bool {
        edges.iter().all(|&(u, v)| mask >> u & 1 == 1 || mask >> v & 1 == 1)
    }

    fn solve(i: usize, n: usize, mask: u64, size: usize,
             best: &mut usize, best_mask: &mut u64, edges: &[(usize, usize)]) {
        if size >= *best { return; }
        if i == n {
            if is_cover(mask, edges) && size < *best {
                *best = size;
                *best_mask = mask;
            }
            return;
        }
        solve(i + 1, n, mask, size, best, best_mask, edges);
        solve(i + 1, n, mask | (1 << i), size + 1, best, best_mask, edges);
    }

    solve(0, n, 0, 0, &mut best, &mut best_mask, edges);
    (0..n).filter(|&i| best_mask >> i & 1 == 1).collect()
}

fn main() {
    let n     = 7;
    let edges = vec![(0,1),(0,2),(1,3),(2,3),(3,4),(4,5),(4,6)];
    let cover2 = vertex_cover_2approx(n, &edges);
    println!("2-approx cover ({} vertices): {:?}", cover2.len(), {
        let mut v: Vec<_> = cover2.into_iter().collect(); v.sort(); v
    });
    let mut exact = vertex_cover_exact(n, &edges);
    exact.sort();
    println!("Exact cover ({} vertices):    {:?}", exact.len(), exact);
}
