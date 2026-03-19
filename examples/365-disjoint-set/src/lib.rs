//! Union-Find (Disjoint Set) Data Structure
//!
//! Track which elements belong to the same group and merge groups in O(α(n)) amortized.

/// Union-Find with path compression and union by rank
#[derive(Debug, Clone)]
pub struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<u32>,
    size: Vec<usize>,
    components: usize,
}

impl UnionFind {
    // === Approach 1: Basic API ===

    /// Create a new Union-Find with n elements (0..n)
    pub fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            rank: vec![0; n],
            size: vec![1; n],
            components: n,
        }
    }

    /// Find the root of element x with path compression - O(α(n)) amortized
    pub fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]); // path compression
        }
        self.parent[x]
    }

    /// Union two sets - returns true if they were different sets
    pub fn union(&mut self, x: usize, y: usize) -> bool {
        let rx = self.find(x);
        let ry = self.find(y);
        if rx == ry {
            return false; // already connected
        }
        // Union by rank
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

    /// Check if two elements are in the same set
    pub fn connected(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }

    // === Approach 2: Query methods ===

    /// Get the size of the component containing x
    pub fn component_size(&mut self, x: usize) -> usize {
        let root = self.find(x);
        self.size[root]
    }

    /// Get the number of distinct components
    pub fn num_components(&self) -> usize {
        self.components
    }

    /// Get the total number of elements
    pub fn len(&self) -> usize {
        self.parent.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.parent.is_empty()
    }

    // === Approach 3: Immutable find (for read-only queries) ===

    /// Find root without path compression (for immutable access)
    pub fn find_immut(&self, mut x: usize) -> usize {
        while self.parent[x] != x {
            x = self.parent[x];
        }
        x
    }

    /// Check connectivity without mutation
    pub fn connected_immut(&self, x: usize, y: usize) -> bool {
        self.find_immut(x) == self.find_immut(y)
    }

    /// Get all elements in the same component as x
    pub fn component_members(&self, x: usize) -> Vec<usize> {
        let root = self.find_immut(x);
        (0..self.len())
            .filter(|&i| self.find_immut(i) == root)
            .collect()
    }

    /// Get all roots (one per component)
    pub fn roots(&self) -> Vec<usize> {
        (0..self.len()).filter(|&i| self.parent[i] == i).collect()
    }
}

/// Count connected components in a graph
pub fn count_connected_components(n: usize, edges: &[(usize, usize)]) -> usize {
    let mut uf = UnionFind::new(n);
    for &(u, v) in edges {
        uf.union(u, v);
    }
    uf.num_components()
}

/// Check if a graph has a cycle (for undirected graphs)
pub fn has_cycle(n: usize, edges: &[(usize, usize)]) -> bool {
    let mut uf = UnionFind::new(n);
    for &(u, v) in edges {
        if !uf.union(u, v) {
            return true; // already connected = cycle
        }
    }
    false
}

/// Kruskal's MST - returns edges in MST and total weight
pub fn kruskal_mst(n: usize, mut edges: Vec<(i64, usize, usize)>) -> (Vec<(usize, usize)>, i64) {
    edges.sort_by_key(|e| e.0);
    let mut uf = UnionFind::new(n);
    let mut mst = Vec::new();
    let mut total_weight = 0;

    for (weight, u, v) in edges {
        if uf.union(u, v) {
            mst.push((u, v));
            total_weight += weight;
        }
    }

    (mst, total_weight)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_union_find() {
        let mut uf = UnionFind::new(5);
        assert!(!uf.connected(0, 1));
        uf.union(0, 1);
        assert!(uf.connected(0, 1));
    }

    #[test]
    fn test_transitive_connectivity() {
        let mut uf = UnionFind::new(4);
        uf.union(0, 1);
        uf.union(1, 2);
        assert!(uf.connected(0, 2));
        assert!(!uf.connected(0, 3));
    }

    #[test]
    fn test_component_count() {
        let mut uf = UnionFind::new(5);
        assert_eq!(uf.num_components(), 5);
        uf.union(0, 1);
        assert_eq!(uf.num_components(), 4);
        uf.union(2, 3);
        assert_eq!(uf.num_components(), 3);
        uf.union(0, 2);
        assert_eq!(uf.num_components(), 2);
    }

    #[test]
    fn test_component_size() {
        let mut uf = UnionFind::new(5);
        assert_eq!(uf.component_size(0), 1);
        uf.union(0, 1);
        uf.union(1, 2);
        assert_eq!(uf.component_size(0), 3);
        assert_eq!(uf.component_size(1), 3);
        assert_eq!(uf.component_size(2), 3);
    }

    #[test]
    fn test_count_connected_components() {
        assert_eq!(count_connected_components(5, &[(0, 1), (2, 3)]), 3);
        assert_eq!(count_connected_components(4, &[(0, 1), (1, 2), (2, 3)]), 1);
    }

    #[test]
    fn test_has_cycle() {
        assert!(!has_cycle(3, &[(0, 1), (1, 2)]));
        assert!(has_cycle(3, &[(0, 1), (1, 2), (2, 0)]));
    }

    #[test]
    fn test_kruskal_mst() {
        // Triangle with weights 1, 2, 3
        let edges = vec![(1, 0, 1), (2, 1, 2), (3, 0, 2)];
        let (mst, weight) = kruskal_mst(3, edges);
        assert_eq!(mst.len(), 2);
        assert_eq!(weight, 3); // edges with weight 1 and 2
    }

    #[test]
    fn test_component_members() {
        let mut uf = UnionFind::new(5);
        uf.union(0, 1);
        uf.union(1, 2);

        let mut members = uf.component_members(0);
        members.sort();
        assert_eq!(members, vec![0, 1, 2]);
    }

    #[test]
    fn test_roots() {
        let mut uf = UnionFind::new(4);
        uf.union(0, 1);
        uf.union(2, 3);

        let roots = uf.roots();
        assert_eq!(roots.len(), 2);
    }

    #[test]
    fn test_immutable_find() {
        let mut uf = UnionFind::new(3);
        uf.union(0, 1);

        // Immutable find should work
        assert!(uf.connected_immut(0, 1));
        assert!(!uf.connected_immut(0, 2));
    }
}
