struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<u32>,
    size: Vec<usize>,
    components: usize,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            rank: vec![0; n],
            size: vec![1; n],
            components: n,
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]); // path compression
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) -> bool {
        let rx = self.find(x);
        let ry = self.find(y);
        if rx == ry { return false; } // already connected
        // union by rank
        if self.rank[rx] < self.rank[ry] {
            self.parent[rx] = ry;
            self.size[ry] += self.size[rx];
        } else if self.rank[rx] > self.rank[ry] {
            self.parent[ry] = rx;
            self.size[rx] += self.size[ry];
        } else {
            self.parent[ry] = rx;
            self.size[rx] += self.size[ry];
            self.rank[rx] += 1;
        }
        self.components -= 1;
        true
    }

    fn connected(&mut self, x: usize, y: usize) -> bool { self.find(x) == self.find(y) }
    fn component_size(&mut self, x: usize) -> usize { let r = self.find(x); self.size[r] }
}

fn count_connected_components(n: usize, edges: &[(usize,usize)]) -> usize {
    let mut uf = UnionFind::new(n);
    for &(u,v) in edges { uf.union(u, v); }
    uf.components
}

fn main() {
    let mut uf = UnionFind::new(10);
    uf.union(0,1); uf.union(2,3); uf.union(0,2);
    println!("0-3 connected: {}", uf.connected(0,3));
    println!("0-4 connected: {}", uf.connected(0,4));
    println!("Component of 0 size: {}", uf.component_size(0));
    println!("Components: {}", uf.components);

    let edges = [(0,1),(1,2),(3,4)];
    println!("Connected components in 5-node graph: {}", count_connected_components(5, &edges));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn basic_union_find() {
        let mut uf = UnionFind::new(5);
        assert!(!uf.connected(0,1));
        uf.union(0,1); assert!(uf.connected(0,1));
    }
    #[test] fn transitive() {
        let mut uf = UnionFind::new(4);
        uf.union(0,1); uf.union(1,2);
        assert!(uf.connected(0,2));
        assert!(!uf.connected(0,3));
    }
    #[test] fn components() {
        assert_eq!(count_connected_components(5, &[(0,1),(2,3)]), 3);
    }
}
