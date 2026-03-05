// Prim's MST — O(E log V) with BinaryHeap<Reverse<>>
use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn prim(adj: &[Vec<(usize, i64)>]) -> (i64, Vec<(usize, usize, i64)>) {
    let n = adj.len();
    let mut key    = vec![i64::MAX; n];
    let mut parent = vec![usize::MAX; n];
    let mut in_mst = vec![false; n];
    let mut heap   = BinaryHeap::new();

    key[0] = 0;
    heap.push(Reverse((0i64, 0usize)));

    let mut total = 0i64;
    let mut mst   = Vec::new();

    while let Some(Reverse((w, u))) = heap.pop() {
        if in_mst[u] { continue; }
        in_mst[u] = true;
        if parent[u] != usize::MAX {
            total += w;
            mst.push((parent[u], u, w));
        }
        for &(v, wv) in &adj[u] {
            if !in_mst[v] && wv < key[v] {
                key[v]    = wv;
                parent[v] = u;
                heap.push(Reverse((wv, v)));
            }
        }
    }
    (total, mst)
}

fn main() {
    let n = 5;
    let mut adj: Vec<Vec<(usize, i64)>> = vec![vec![]; n];
    let mut add = |u: usize, v: usize, w: i64| {
        adj[u].push((v, w));
        adj[v].push((u, w));
    };
    add(0, 1, 2); add(0, 3, 6);
    add(1, 2, 3); add(1, 3, 8); add(1, 4, 5);
    add(2, 4, 7);
    add(3, 4, 9);

    let (total, mst) = prim(&adj);
    println!("MST total weight: {total}");
    for (u, v, w) in &mst {
        println!("  edge {u}-{v}  weight={w}");
    }
}
