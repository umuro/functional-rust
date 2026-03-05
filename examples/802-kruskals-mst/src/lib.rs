//! # Kruskal's MST Algorithm

pub struct UnionFind { parent: Vec<usize>, rank: Vec<usize> }

impl UnionFind {
    pub fn new(n: usize) -> Self { Self { parent: (0..n).collect(), rank: vec![0; n] } }
    pub fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x { self.parent[x] = self.find(self.parent[x]); }
        self.parent[x]
    }
    pub fn union(&mut self, x: usize, y: usize) -> bool {
        let (px, py) = (self.find(x), self.find(y));
        if px == py { return false; }
        match self.rank[px].cmp(&self.rank[py]) {
            std::cmp::Ordering::Less => self.parent[px] = py,
            std::cmp::Ordering::Greater => self.parent[py] = px,
            std::cmp::Ordering::Equal => { self.parent[py] = px; self.rank[px] += 1; }
        }
        true
    }
}

pub fn kruskals_mst(n: usize, edges: &mut [(usize, usize, i32)]) -> i32 {
    edges.sort_by_key(|e| e.2);
    let mut uf = UnionFind::new(n);
    let mut total = 0;
    for &(u, v, w) in edges.iter() {
        if uf.union(u, v) { total += w; }
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn test_kruskals() {
        let mut edges = [(0, 1, 10), (0, 2, 6), (0, 3, 5), (1, 3, 15), (2, 3, 4)];
        assert_eq!(kruskals_mst(4, &mut edges), 19);
    }
}
