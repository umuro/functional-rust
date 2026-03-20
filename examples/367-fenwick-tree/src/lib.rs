#![allow(clippy::all)]
//! Fenwick Tree (Binary Indexed Tree)
//!
//! O(log n) prefix sums and point updates with minimal memory.

/// A Fenwick tree for prefix sum queries
pub struct FenwickTree {
    tree: Vec<i64>,
    n: usize,
}

impl FenwickTree {
    // === Approach 1: Basic construction ===

    /// Create a new Fenwick tree with n elements (all zeros)
    pub fn new(n: usize) -> Self {
        Self {
            tree: vec![0; n + 1],
            n,
        }
    }

    /// Build from a slice
    pub fn from_slice(arr: &[i64]) -> Self {
        let mut ft = Self::new(arr.len());
        for (i, &v) in arr.iter().enumerate() {
            ft.update(i + 1, v);
        }
        ft
    }

    // === Approach 2: Update and query ===

    /// Add delta to position i (1-indexed)
    pub fn update(&mut self, mut i: usize, delta: i64) {
        while i <= self.n {
            self.tree[i] += delta;
            i += i & i.wrapping_neg(); // lowbit
        }
    }

    /// Get prefix sum [1, i]
    pub fn prefix_sum(&self, mut i: usize) -> i64 {
        let mut sum = 0;
        while i > 0 {
            sum += self.tree[i];
            i -= i & i.wrapping_neg(); // lowbit
        }
        sum
    }

    /// Get range sum [l, r] (1-indexed)
    pub fn range_sum(&self, l: usize, r: usize) -> i64 {
        self.prefix_sum(r) - if l > 1 { self.prefix_sum(l - 1) } else { 0 }
    }

    /// Get single element value at position i
    pub fn point_query(&self, i: usize) -> i64 {
        self.range_sum(i, i)
    }

    // === Approach 3: Additional utilities ===

    /// Set position i to value (1-indexed)
    pub fn set(&mut self, i: usize, val: i64) {
        let current = self.point_query(i);
        self.update(i, val - current);
    }

    /// Get the size
    pub fn len(&self) -> usize {
        self.n
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.n == 0
    }

    /// Find first position where prefix sum >= target (binary search)
    pub fn lower_bound(&self, mut target: i64) -> usize {
        if target <= 0 {
            return 0;
        }
        let mut pos = 0;
        let mut bit = 1usize << (63 - self.n.leading_zeros());

        while bit > 0 {
            if pos + bit <= self.n && self.tree[pos + bit] < target {
                target -= self.tree[pos + bit];
                pos += bit;
            }
            bit >>= 1;
        }
        pos + 1
    }
}

/// 2D Fenwick Tree for 2D range queries
pub struct FenwickTree2D {
    tree: Vec<Vec<i64>>,
    rows: usize,
    cols: usize,
}

impl FenwickTree2D {
    /// Create a new 2D Fenwick tree
    pub fn new(rows: usize, cols: usize) -> Self {
        Self {
            tree: vec![vec![0; cols + 1]; rows + 1],
            rows,
            cols,
        }
    }

    /// Update position (r, c) by delta
    pub fn update(&mut self, mut r: usize, c: usize, delta: i64) {
        while r <= self.rows {
            let mut cc = c;
            while cc <= self.cols {
                self.tree[r][cc] += delta;
                cc += cc & cc.wrapping_neg();
            }
            r += r & r.wrapping_neg();
        }
    }

    /// Get sum of rectangle [(1,1), (r, c)]
    pub fn prefix_sum(&self, mut r: usize, c: usize) -> i64 {
        let mut sum = 0;
        while r > 0 {
            let mut cc = c;
            while cc > 0 {
                sum += self.tree[r][cc];
                cc -= cc & cc.wrapping_neg();
            }
            r -= r & r.wrapping_neg();
        }
        sum
    }

    /// Get sum of rectangle [(r1,c1), (r2,c2)]
    pub fn range_sum(&self, r1: usize, c1: usize, r2: usize, c2: usize) -> i64 {
        self.prefix_sum(r2, c2) - self.prefix_sum(r1 - 1, c2) - self.prefix_sum(r2, c1 - 1)
            + self.prefix_sum(r1 - 1, c1 - 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prefix_sum() {
        let ft = FenwickTree::from_slice(&[1, 2, 3, 4, 5]);
        assert_eq!(ft.prefix_sum(3), 6); // 1+2+3
        assert_eq!(ft.prefix_sum(5), 15); // 1+2+3+4+5
    }

    #[test]
    fn test_range_sum() {
        let ft = FenwickTree::from_slice(&[1, 2, 3, 4, 5]);
        assert_eq!(ft.range_sum(2, 4), 9); // 2+3+4
        assert_eq!(ft.range_sum(1, 1), 1);
    }

    #[test]
    fn test_point_update() {
        let mut ft = FenwickTree::from_slice(&[1, 2, 3, 4, 5]);
        ft.update(3, 7); // add 7 to position 3
        assert_eq!(ft.point_query(3), 10);
        assert_eq!(ft.prefix_sum(5), 22);
    }

    #[test]
    fn test_set() {
        let mut ft = FenwickTree::from_slice(&[1, 2, 3, 4, 5]);
        ft.set(3, 10);
        assert_eq!(ft.point_query(3), 10);
    }

    #[test]
    fn test_empty() {
        let ft = FenwickTree::new(0);
        assert!(ft.is_empty());
    }

    #[test]
    fn test_lower_bound() {
        let ft = FenwickTree::from_slice(&[1, 2, 3, 4, 5]);
        assert_eq!(ft.lower_bound(6), 3); // prefix[3] = 6
        assert_eq!(ft.lower_bound(10), 4); // prefix[4] = 10
    }

    #[test]
    fn test_2d_basic() {
        let mut ft = FenwickTree2D::new(3, 3);
        ft.update(1, 1, 1);
        ft.update(2, 2, 2);
        ft.update(3, 3, 3);
        assert_eq!(ft.prefix_sum(3, 3), 6);
    }

    #[test]
    fn test_2d_range() {
        let mut ft = FenwickTree2D::new(3, 3);
        for r in 1..=3 {
            for c in 1..=3 {
                ft.update(r, c, 1);
            }
        }
        assert_eq!(ft.range_sum(2, 2, 3, 3), 4);
    }
}
