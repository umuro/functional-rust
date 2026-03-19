// 965: Segment Tree for Range Sum Queries
// 1-indexed internal nodes; O(log n) point update and range sum

pub struct SegmentTree {
    n: usize,
    tree: Vec<i64>,
}

impl SegmentTree {
    pub fn new(n: usize) -> Self {
        SegmentTree {
            n,
            tree: vec![0i64; 4 * n],
        }
    }

    pub fn build(&mut self, arr: &[i64]) {
        self.build_rec(1, 0, self.n - 1, arr);
    }

    fn build_rec(&mut self, node: usize, lo: usize, hi: usize, arr: &[i64]) {
        if lo == hi {
            self.tree[node] = arr[lo];
        } else {
            let mid = (lo + hi) / 2;
            self.build_rec(2 * node, lo, mid, arr);
            self.build_rec(2 * node + 1, mid + 1, hi, arr);
            self.tree[node] = self.tree[2 * node] + self.tree[2 * node + 1];
        }
    }

    /// Point update: set position `pos` to `value`
    pub fn update(&mut self, pos: usize, value: i64) {
        self.update_rec(1, 0, self.n - 1, pos, value);
    }

    fn update_rec(&mut self, node: usize, lo: usize, hi: usize, pos: usize, value: i64) {
        if lo == hi {
            self.tree[node] = value;
        } else {
            let mid = (lo + hi) / 2;
            if pos <= mid {
                self.update_rec(2 * node, lo, mid, pos, value);
            } else {
                self.update_rec(2 * node + 1, mid + 1, hi, pos, value);
            }
            self.tree[node] = self.tree[2 * node] + self.tree[2 * node + 1];
        }
    }

    /// Range sum query [l, r] (inclusive, 0-indexed)
    pub fn query(&self, l: usize, r: usize) -> i64 {
        self.query_rec(1, 0, self.n - 1, l, r)
    }

    fn query_rec(&self, node: usize, lo: usize, hi: usize, l: usize, r: usize) -> i64 {
        if r < lo || hi < l {
            0
        } else if l <= lo && hi <= r {
            self.tree[node]
        } else {
            let mid = (lo + hi) / 2;
            self.query_rec(2 * node, lo, mid, l, r)
                + self.query_rec(2 * node + 1, mid + 1, hi, l, r)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_tree() -> SegmentTree {
        let arr = vec![1i64, 3, 5, 7, 9, 11];
        let mut st = SegmentTree::new(arr.len());
        st.build(&arr);
        st
    }

    #[test]
    fn test_total_sum() {
        let st = make_tree();
        assert_eq!(st.query(0, 5), 36);
    }

    #[test]
    fn test_range_queries() {
        let st = make_tree();
        assert_eq!(st.query(0, 2), 9); // 1+3+5
        assert_eq!(st.query(2, 4), 21); // 5+7+9
        assert_eq!(st.query(1, 3), 15); // 3+5+7
        assert_eq!(st.query(5, 5), 11); // single element
    }

    #[test]
    fn test_point_update() {
        let mut st = make_tree();
        st.update(2, 10); // replace 5 with 10
        assert_eq!(st.query(0, 5), 41); // 36 - 5 + 10
        assert_eq!(st.query(0, 2), 14); // 1+3+10
        assert_eq!(st.query(2, 4), 26); // 10+7+9
    }

    #[test]
    fn test_single_element() {
        let arr = vec![42i64];
        let mut st = SegmentTree::new(1);
        st.build(&arr);
        assert_eq!(st.query(0, 0), 42);
        st.update(0, 100);
        assert_eq!(st.query(0, 0), 100);
    }

    #[test]
    fn test_multiple_updates() {
        let mut st = make_tree();
        st.update(0, 0);
        st.update(5, 0);
        assert_eq!(st.query(0, 5), 24); // 0+3+5+7+9+0
    }
}
