#![allow(clippy::all)]
//! Segment Tree for Range Queries
//!
//! O(log n) range queries and point updates.

/// A segment tree for range sum queries
pub struct SegmentTree {
    data: Vec<i64>,
    n: usize,
}

impl SegmentTree {
    // === Approach 1: Build from array ===

    /// Build a segment tree from an array
    pub fn new(arr: &[i64]) -> Self {
        let n = arr.len();
        let mut st = Self {
            data: vec![0; 4 * n],
            n,
        };
        if n > 0 {
            st.build(arr, 1, 0, n - 1);
        }
        st
    }

    fn build(&mut self, arr: &[i64], v: usize, l: usize, r: usize) {
        if l == r {
            self.data[v] = arr[l];
            return;
        }
        let m = (l + r) / 2;
        self.build(arr, 2 * v, l, m);
        self.build(arr, 2 * v + 1, m + 1, r);
        self.data[v] = self.data[2 * v] + self.data[2 * v + 1];
    }

    // === Approach 2: Range queries ===

    /// Query sum in range [ql, qr] - O(log n)
    pub fn query(&self, ql: usize, qr: usize) -> i64 {
        if self.n == 0 {
            return 0;
        }
        self.query_internal(1, 0, self.n - 1, ql, qr)
    }

    fn query_internal(&self, v: usize, l: usize, r: usize, ql: usize, qr: usize) -> i64 {
        if qr < l || r < ql {
            return 0;
        }
        if ql <= l && r <= qr {
            return self.data[v];
        }
        let m = (l + r) / 2;
        self.query_internal(2 * v, l, m, ql, qr) + self.query_internal(2 * v + 1, m + 1, r, ql, qr)
    }

    /// Alias for query - sum in range [l, r]
    pub fn sum(&self, l: usize, r: usize) -> i64 {
        self.query(l, r)
    }

    // === Approach 3: Point updates ===

    /// Set value at position pos - O(log n)
    pub fn set(&mut self, pos: usize, val: i64) {
        if self.n > 0 {
            self.update_internal(1, 0, self.n - 1, pos, val);
        }
    }

    fn update_internal(&mut self, v: usize, l: usize, r: usize, pos: usize, val: i64) {
        if l == r {
            self.data[v] = val;
            return;
        }
        let m = (l + r) / 2;
        if pos <= m {
            self.update_internal(2 * v, l, m, pos, val);
        } else {
            self.update_internal(2 * v + 1, m + 1, r, pos, val);
        }
        self.data[v] = self.data[2 * v] + self.data[2 * v + 1];
    }

    /// Add delta to value at position pos
    pub fn add(&mut self, pos: usize, delta: i64) {
        let current = self.query(pos, pos);
        self.set(pos, current + delta);
    }

    /// Get the size of the underlying array
    pub fn len(&self) -> usize {
        self.n
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.n == 0
    }
}

/// Generic segment tree with custom combine function
pub struct GenericSegmentTree<T, F>
where
    F: Fn(&T, &T) -> T,
{
    data: Vec<Option<T>>,
    n: usize,
    identity: T,
    combine: F,
}

impl<T: Clone, F: Fn(&T, &T) -> T> GenericSegmentTree<T, F> {
    /// Create a new generic segment tree
    pub fn new(arr: &[T], identity: T, combine: F) -> Self {
        let n = arr.len();
        let mut st = Self {
            data: vec![None; 4 * n.max(1)],
            n,
            identity,
            combine,
        };
        if n > 0 {
            st.build(arr, 1, 0, n - 1);
        }
        st
    }

    fn build(&mut self, arr: &[T], v: usize, l: usize, r: usize) {
        if l == r {
            self.data[v] = Some(arr[l].clone());
            return;
        }
        let m = (l + r) / 2;
        self.build(arr, 2 * v, l, m);
        self.build(arr, 2 * v + 1, m + 1, r);
        let left = self.data[2 * v].as_ref().unwrap();
        let right = self.data[2 * v + 1].as_ref().unwrap();
        self.data[v] = Some((self.combine)(left, right));
    }

    /// Query range [ql, qr]
    pub fn query(&self, ql: usize, qr: usize) -> T {
        if self.n == 0 {
            return self.identity.clone();
        }
        self.query_internal(1, 0, self.n - 1, ql, qr)
    }

    fn query_internal(&self, v: usize, l: usize, r: usize, ql: usize, qr: usize) -> T {
        if qr < l || r < ql {
            return self.identity.clone();
        }
        if ql <= l && r <= qr {
            return self.data[v].as_ref().unwrap().clone();
        }
        let m = (l + r) / 2;
        let left = self.query_internal(2 * v, l, m, ql, qr);
        let right = self.query_internal(2 * v + 1, m + 1, r, ql, qr);
        (self.combine)(&left, &right)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_range_sum() {
        let st = SegmentTree::new(&[1, 2, 3, 4, 5]);
        assert_eq!(st.sum(0, 4), 15);
        assert_eq!(st.sum(1, 3), 9);
        assert_eq!(st.sum(2, 2), 3);
    }

    #[test]
    fn test_point_update() {
        let mut st = SegmentTree::new(&[1, 2, 3, 4, 5]);
        st.set(2, 10);
        assert_eq!(st.sum(0, 4), 22); // 1+2+10+4+5
        assert_eq!(st.sum(2, 2), 10);
    }

    #[test]
    fn test_add() {
        let mut st = SegmentTree::new(&[1, 2, 3, 4, 5]);
        st.add(2, 7);
        assert_eq!(st.sum(2, 2), 10);
    }

    #[test]
    fn test_single_element() {
        let st = SegmentTree::new(&[42]);
        assert_eq!(st.sum(0, 0), 42);
    }

    #[test]
    fn test_empty() {
        let st = SegmentTree::new(&[]);
        assert!(st.is_empty());
    }

    #[test]
    fn test_generic_max() {
        let arr = vec![3, 1, 4, 1, 5, 9, 2, 6];
        let st = GenericSegmentTree::new(&arr, i32::MIN, |a, b| *a.max(b));
        assert_eq!(st.query(0, 7), 9);
        assert_eq!(st.query(0, 4), 5);
        assert_eq!(st.query(5, 5), 9);
    }

    #[test]
    fn test_generic_min() {
        let arr = vec![3, 1, 4, 1, 5, 9, 2, 6];
        let st = GenericSegmentTree::new(&arr, i32::MAX, |a, b| *a.min(b));
        assert_eq!(st.query(0, 7), 1);
        assert_eq!(st.query(4, 7), 2);
    }

    #[test]
    fn test_multiple_updates() {
        let mut st = SegmentTree::new(&[1, 1, 1, 1, 1]);
        st.set(0, 5);
        st.set(4, 5);
        assert_eq!(st.sum(0, 4), 13);
    }
}
