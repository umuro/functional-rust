// Bellman-Ford
fn bellman_ford(edges: &[(usize, usize, i32)], n: usize, start: usize) -> Option<Vec<i32>> {
    let mut dist = vec![i32::MAX; n];
    dist[start] = 0;
    for _ in 0..n-1 {
        for &(u, v, w) in edges {
            if dist[u] != i32::MAX && dist[u] + w < dist[v] { dist[v] = dist[u] + w; }
        }
    }
    for &(u, v, w) in edges {
        if dist[u] != i32::MAX && dist[u] + w < dist[v] { return None; }
    }
    Some(dist)
}

fn main() {
    let edges = vec![(0, 1, 4), (0, 2, 5), (1, 2, -3)];
    if let Some(dist) = bellman_ford(&edges, 3, 0) {
        println!("Distances: {:?}", dist);
    }
}
