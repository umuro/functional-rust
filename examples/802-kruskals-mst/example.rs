// Kruskal's MST — sort + Union-Find O(E log E)

struct UnionFind {
    parent: Vec<usize>,
    rank:   Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind { parent: (0..n).collect(), rank: vec![0; n] }
    }

    fn find(&mut self, mut v: usize) -> usize {
        while self.parent[v] != v {
            self.parent[v] = self.parent[self.parent[v]]; // path halving
            v = self.parent[v];
        }
        v
    }

    fn union(&mut self, u: usize, v: usize) -> bool {
        let (pu, pv) = (self.find(u), self.find(v));
        if pu == pv { return false; }
        if self.rank[pu] < self.rank[pv] {
            self.parent[pu] = pv;
        } else if self.rank[pu] > self.rank[pv] {
            self.parent[pv] = pu;
        } else {
            self.parent[pv] = pu;
            self.rank[pu] += 1;
        }
        true
    }
}

fn kruskal(n: usize, edges: &mut Vec<(i64, usize, usize)>) -> (i64, Vec<(usize, usize, i64)>) {
    edges.sort_unstable_by_key(|&(w, _, _)| w);
    let mut uf    = UnionFind::new(n);
    let mut total = 0i64;
    let mut mst   = Vec::new();
    for &(w, u, v) in edges.iter() {
        if uf.union(u, v) {
            total += w;
            mst.push((u, v, w));
        }
    }
    (total, mst)
}

fn main() {
    let mut edges = vec![
        (2i64, 0, 1), (6, 0, 3),
        (3, 1, 2),    (8, 1, 3), (5, 1, 4),
        (7, 2, 4),    (9, 3, 4),
    ];
    let (total, mst) = kruskal(5, &mut edges);
    println!("MST total weight: {total}");
    for (u, v, w) in &mst {
        println!("  edge {u}-{v}  weight={w}");
    }
}
