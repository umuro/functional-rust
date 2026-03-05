// 966: Fenwick Tree (Binary Indexed Tree)
// O(log n) point update and prefix sum
// Key trick: lowbit(i) = i & (-i) traverses the tree

pub struct FenwickTree {
    n: usize,
    tree: Vec<i64>, // 1-indexed internally
}

impl FenwickTree {
    pub fn new(n: usize) -> Self {
        FenwickTree {
            n,
            tree: vec![0i64; n + 1],
        }
    }

    pub fn from_slice(arr: &[i64]) -> Self {
        let mut fw = FenwickTree::new(arr.len());
        for (i, &v) in arr.iter().enumerate() {
            fw.update(i, v);
        }
        fw
    }

    /// Point update: add `delta` to position `i` (0-indexed)
    pub fn update(&mut self, i: usize, delta: i64) {
        let mut idx = (i + 1) as i64; // convert to 1-indexed
        while idx <= self.n as i64 {
            self.tree[idx as usize] += delta;
            idx += idx & (-idx); // idx += lowbit(idx)
        }
    }

    /// Set position `i` to `value` (requires knowing current value)
    pub fn set(&mut self, i: usize, value: i64) {
        let current = self.range_sum(i, i);
        self.update(i, value - current);
    }

    /// Prefix sum [0, i] inclusive (0-indexed)
    pub fn prefix_sum(&self, i: usize) -> i64 {
        let mut idx = (i + 1) as i64; // convert to 1-indexed
        let mut sum = 0i64;
        while idx > 0 {
            sum += self.tree[idx as usize];
            idx -= idx & (-idx); // idx -= lowbit(idx)
        }
        sum
    }

    /// Range sum [l, r] inclusive (0-indexed)
    pub fn range_sum(&self, l: usize, r: usize) -> i64 {
        if l == 0 {
            self.prefix_sum(r)
        } else {
            self.prefix_sum(r) - self.prefix_sum(l - 1)
        }
    }

    pub fn len(&self) -> usize {
        self.n
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    fn make_fw() -> FenwickTree {
        FenwickTree::from_slice(&[1, 3, 5, 7, 9, 11])
    }

    #[test]
    fn test_prefix_sums() {
        let fw = make_fw();
        assert_eq!(fw.prefix_sum(0), 1);
        assert_eq!(fw.prefix_sum(2), 9);  // 1+3+5
        assert_eq!(fw.prefix_sum(5), 36); // total
    }

    #[test]
    fn test_range_sums() {
        let fw = make_fw();
        assert_eq!(fw.range_sum(0, 2), 9);  // 1+3+5
        assert_eq!(fw.range_sum(2, 4), 21); // 5+7+9
        assert_eq!(fw.range_sum(1, 3), 15); // 3+5+7
        assert_eq!(fw.range_sum(5, 5), 11); // single element
    }

    #[test]
    fn test_update() {
        let mut fw = make_fw();
        fw.update(2, 5); // arr[2] += 5 → 10
        assert_eq!(fw.prefix_sum(5), 41);
        assert_eq!(fw.range_sum(0, 2), 14);
        assert_eq!(fw.range_sum(2, 4), 26);
    }

    #[test]
    fn test_single_element() {
        let mut fw = FenwickTree::new(1);
        fw.update(0, 42);
        assert_eq!(fw.prefix_sum(0), 42);
        assert_eq!(fw.range_sum(0, 0), 42);
    }

    #[test]
    fn test_set() {
        let mut fw = make_fw();
        fw.set(2, 10); // set arr[2] = 10 (was 5)
        assert_eq!(fw.range_sum(2, 2), 10);
        assert_eq!(fw.prefix_sum(5), 41);
    }
}
