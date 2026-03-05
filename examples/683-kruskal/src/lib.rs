//! # Kruskal's Algorithm
//! MST using Union-Find. Time: O(E log E)

pub struct UnionFind { parent: Vec<usize>, rank: Vec<usize> }

impl UnionFind {
    pub fn new(n: usize) -> Self { UnionFind { parent: (0..n).collect(), rank: vec![0; n] } }
    
    pub fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x { self.parent[x] = self.find(self.parent[x]); }
        self.parent[x]
    }
    
    pub fn union(&mut self, x: usize, y: usize) -> bool {
        let px = self.find(x);
        let py = self.find(y);
        if px == py { return false; }
        if self.rank[px] < self.rank[py] { self.parent[px] = py; }
        else if self.rank[px] > self.rank[py] { self.parent[py] = px; }
        else { self.parent[py] = px; self.rank[px] += 1; }
        true
    }
}

pub fn kruskal(mut edges: Vec<(usize, usize, i32)>, n: usize) -> (i32, Vec<(usize, usize, i32)>) {
    edges.sort_by_key(|e| e.2);
    let mut uf = UnionFind::new(n);
    let mut mst = Vec::new();
    let mut cost = 0;
    
    for (u, v, w) in edges {
        if uf.union(u, v) { mst.push((u, v, w)); cost += w; }
    }
    (cost, mst)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_kruskal() {
        let edges = vec![(0,1,4), (0,2,3), (1,2,1), (1,3,2), (2,3,4)];
        let (cost, _) = kruskal(edges, 4);
        assert_eq!(cost, 6);
    }
}
