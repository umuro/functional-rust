// Bellman-Ford — O(V×E) shortest paths with negative-edge support

const INF: i64 = i64::MAX / 2;

fn bellman_ford(
    n: usize,
    edges: &[(usize, usize, i64)],
    src: usize,
) -> (Vec<i64>, Vec<Option<usize>>, bool) {
    let mut dist = vec![INF; n];
    let mut prev = vec![None; n];
    dist[src] = 0;

    for _ in 0..n - 1 {
        for &(u, v, w) in edges {
            if dist[u] < INF && dist[u] + w < dist[v] {
                dist[v] = dist[u] + w;
                prev[v] = Some(u);
            }
        }
    }

    // Negative cycle detection
    let neg_cycle = edges.iter().any(|&(u, v, w)| {
        dist[u] < INF && dist[u] + w < dist[v]
    });

    (dist, prev, neg_cycle)
}

fn reconstruct(prev: &[Option<usize>], dst: usize) -> Vec<usize> {
    let mut path = Vec::new();
    let mut v = dst;
    loop {
        path.push(v);
        match prev[v] {
            Some(p) => v = p,
            None    => break,
        }
    }
    path.reverse();
    path
}

fn main() {
    let edges = vec![
        (0, 1, -1i64), (0, 2,  4),
        (1, 2,  3),    (1, 3,  2), (1, 4, 2),
        (3, 2,  5),    (3, 1,  1),
        (4, 3, -3),
    ];
    let n = 5;
    let (dist, prev, nc) = bellman_ford(n, &edges, 0);
    println!("Source: 0  |  Negative cycle: {nc}");
    for i in 0..n {
        if dist[i] >= INF {
            println!("  0 -> {i}: unreachable");
        } else {
            println!("  0 -> {i}: dist={}, path={:?}", dist[i], reconstruct(&prev, i));
        }
    }

    // Negative cycle
    let edges2 = vec![(0usize,1,1i64),(1,2,-1),(2,0,-1)];
    let (_, _, nc2) = bellman_ford(3, &edges2, 0);
    println!("\nNegative cycle in [0->1->2->0]: {nc2}");
}
