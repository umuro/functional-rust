// Floyd-Warshall — all-pairs shortest paths O(V³)
const INF: i64 = i64::MAX / 2;

fn floyd_warshall(n: usize, edges: &[(usize, usize, i64)]) -> (Vec<Vec<i64>>, Vec<Vec<Option<usize>>>, bool) {
    let mut dist = vec![vec![INF; n]; n];
    let mut next: Vec<Vec<Option<usize>>> = vec![vec![None; n]; n];

    for i in 0..n { dist[i][i] = 0; }
    for &(u, v, w) in edges {
        if w < dist[u][v] {
            dist[u][v] = w;
            next[u][v] = Some(v);
        }
    }

    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                if dist[i][k] < INF && dist[k][j] < INF {
                    let via = dist[i][k] + dist[k][j];
                    if via < dist[i][j] {
                        dist[i][j] = via;
                        next[i][j] = next[i][k];
                    }
                }
            }
        }
    }

    let neg_cycle = (0..n).any(|i| dist[i][i] < 0);
    (dist, next, neg_cycle)
}

fn reconstruct(next: &Vec<Vec<Option<usize>>>, src: usize, dst: usize) -> Option<Vec<usize>> {
    if next[src][dst].is_none() { return None; }
    let mut path = vec![src];
    let mut v = src;
    while v != dst {
        v = next[v][dst]?;
        path.push(v);
    }
    Some(path)
}

fn main() {
    let edges = vec![(0,1,3i64),(0,3,7),(1,0,8),(1,2,2),(2,0,5),(2,3,1),(3,0,2)];
    let n = 4;
    let (dist, next, nc) = floyd_warshall(n, &edges);
    println!("Negative cycle: {nc}");
    for i in 0..n {
        for j in 0..n {
            if dist[i][j] >= INF {
                println!("  {i}->{j}: INF");
            } else {
                println!("  {i}->{j}: {}  {:?}", dist[i][j], reconstruct(&next, i, j));
            }
        }
    }
}
