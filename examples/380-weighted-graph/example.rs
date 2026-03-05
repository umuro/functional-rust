// Weighted graph and Dijkstra in Rust
use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn dijkstra(adj: &[Vec<(usize, u64)>], n: usize, src: usize) -> Vec<u64> {
    let mut dist = vec![u64::MAX; n];
    dist[src] = 0;
    // Min-heap: (distance, vertex)
    let mut heap = BinaryHeap::new();
    heap.push(Reverse((0u64, src)));

    while let Some(Reverse((d, u))) = heap.pop() {
        if d > dist[u] { continue; } // stale entry
        for &(v, w) in &adj[u] {
            let nd = dist[u].saturating_add(w);
            if nd < dist[v] {
                dist[v] = nd;
                heap.push(Reverse((nd, v)));
            }
        }
    }
    dist
}

fn main() {
    let n = 5;
    let mut adj = vec![vec![]; n];
    let edges = [(0,1,10u64),(0,2,3),(1,3,2),(2,1,4),(2,3,8),(2,4,2),(3,4,5),(4,3,7)];
    for (u, v, w) in edges {
        adj[u].push((v, w));
    }
    let dist = dijkstra(&adj, n, 0);
    println!("Shortest distances from vertex 0:");
    for (v, d) in dist.iter().enumerate() {
        if *d == u64::MAX {
            println!("  to {}: inf", v);
        } else {
            println!("  to {}: {}", v, d);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dijkstra() {
        let n = 4;
        let mut adj = vec![vec![]; n];
        adj[0].push((1, 1u64));
        adj[0].push((2, 4));
        adj[1].push((2, 2));
        adj[1].push((3, 5));
        adj[2].push((3, 1));
        let dist = dijkstra(&adj, n, 0);
        assert_eq!(dist[0], 0);
        assert_eq!(dist[1], 1);
        assert_eq!(dist[2], 3);
        assert_eq!(dist[3], 4);
    }
}
