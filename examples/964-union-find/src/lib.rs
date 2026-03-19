// 964: Union-Find / Disjoint Set
// Path compression + union by rank
// OCaml: arrays + mutable fields; Rust: Vec + struct methods

pub struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
    components: usize,
}

impl UnionFind {
    pub fn new(n: usize) -> Self {
        UnionFind {
            parent: (0..n).collect(),
            rank: vec![0; n],
            components: n,
        }
    }

    // Find with path compression (iterative to avoid stack overflow)
    pub fn find(&mut self, mut i: usize) -> usize {
        // Two-pass path compression
        let mut root = i;
        while self.parent[root] != root {
            root = self.parent[root];
        }
        // Path compression: point all nodes directly to root
        while self.parent[i] != root {
            let next = self.parent[i];
            self.parent[i] = root;
            i = next;
        }
        root
    }

    // Union by rank; returns true if they were in different sets
    pub fn union(&mut self, a: usize, b: usize) -> bool {
        let ra = self.find(a);
        let rb = self.find(b);
        if ra == rb {
            return false;
        }
        self.components -= 1;
        match self.rank[ra].cmp(&self.rank[rb]) {
            std::cmp::Ordering::Less => self.parent[ra] = rb,
            std::cmp::Ordering::Greater => self.parent[rb] = ra,
            std::cmp::Ordering::Equal => {
                self.parent[rb] = ra;
                self.rank[ra] += 1;
            }
        }
        true
    }

    pub fn connected(&mut self, a: usize, b: usize) -> bool {
        self.find(a) == self.find(b)
    }

    pub fn num_components(&self) -> usize {
        self.components
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initial_state() {
        let mut uf = UnionFind::new(5);
        assert_eq!(uf.num_components(), 5);
        assert!(!uf.connected(0, 1));
        assert!(!uf.connected(2, 4));
    }

    #[test]
    fn test_union_and_connected() {
        let mut uf = UnionFind::new(6);
        assert!(uf.union(0, 1));
        assert!(uf.union(2, 3));
        assert!(uf.union(4, 5));
        assert_eq!(uf.num_components(), 3);

        assert!(uf.connected(0, 1));
        assert!(uf.connected(2, 3));
        assert!(!uf.connected(0, 2));
    }

    #[test]
    fn test_union_already_connected() {
        let mut uf = UnionFind::new(4);
        assert!(uf.union(0, 1));
        assert!(!uf.union(0, 1)); // already same set
        assert_eq!(uf.num_components(), 3);
    }

    #[test]
    fn test_transitivity() {
        let mut uf = UnionFind::new(6);
        uf.union(0, 1);
        uf.union(2, 3);
        uf.union(1, 2);
        assert!(uf.connected(0, 3)); // transitive
        assert_eq!(uf.num_components(), 3);
    }

    #[test]
    fn test_full_merge() {
        let mut uf = UnionFind::new(6);
        uf.union(0, 1);
        uf.union(2, 3);
        uf.union(4, 5);
        uf.union(1, 2);
        uf.union(3, 4);
        assert_eq!(uf.num_components(), 1);
        assert!(uf.connected(0, 5));
    }

    #[test]
    fn test_path_compression() {
        let mut uf = UnionFind::new(10);
        // Create a chain: 0-1-2-3-4-5-6-7-8-9
        for i in 0..9 {
            uf.union(i, i + 1);
        }
        assert_eq!(uf.num_components(), 1);
        // After find, path should be compressed
        let root = uf.find(9);
        assert_eq!(root, uf.find(0));
    }
}
