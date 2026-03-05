// 1047: Flat Binary Tree in Vec
// Children of node i: left = 2*i+1, right = 2*i+2, parent = (i-1)/2

struct FlatTree<T> {
    data: Vec<T>,
}

impl<T: std::fmt::Debug + Clone + Ord> FlatTree<T> {
    fn new(data: Vec<T>) -> Self {
        FlatTree { data }
    }

    fn left_child(i: usize) -> usize { 2 * i + 1 }
    fn right_child(i: usize) -> usize { 2 * i + 2 }
    fn parent(i: usize) -> usize { (i - 1) / 2 }

    fn get(&self, i: usize) -> Option<&T> {
        self.data.get(i)
    }

    fn is_leaf(&self, i: usize) -> bool {
        Self::left_child(i) >= self.data.len()
    }

    fn left(&self, i: usize) -> Option<&T> {
        self.data.get(Self::left_child(i))
    }

    fn right(&self, i: usize) -> Option<&T> {
        self.data.get(Self::right_child(i))
    }

    /// Level-order traversal (returns levels)
    fn levels(&self) -> Vec<Vec<&T>> {
        let mut result = Vec::new();
        let mut start = 0;
        let mut level_size = 1;

        while start < self.data.len() {
            let end = (start + level_size).min(self.data.len());
            result.push(self.data[start..end].iter().collect());
            start = end;
            level_size *= 2;
        }
        result
    }

    /// Heapify: build max-heap in place
    fn heapify(&mut self) {
        let n = self.data.len();
        for i in (0..n / 2).rev() {
            self.sift_down(i);
        }
    }

    fn sift_down(&mut self, mut i: usize) {
        let n = self.data.len();
        loop {
            let mut largest = i;
            let l = Self::left_child(i);
            let r = Self::right_child(i);

            if l < n && self.data[l] > self.data[largest] {
                largest = l;
            }
            if r < n && self.data[r] > self.data[largest] {
                largest = r;
            }

            if largest == i {
                break;
            }
            self.data.swap(i, largest);
            i = largest;
        }
    }

    /// Depth of the tree
    fn depth(&self) -> usize {
        if self.data.is_empty() {
            0
        } else {
            (self.data.len() as f64).log2().floor() as usize + 1
        }
    }
}

fn basic_tree() {
    //       1
    //      / \
    //     2   3
    //    / \ /
    //   4  5 6
    let tree = FlatTree::new(vec![1, 2, 3, 4, 5, 6]);

    assert_eq!(tree.get(0), Some(&1)); // Root
    assert_eq!(tree.left(0), Some(&2));
    assert_eq!(tree.right(0), Some(&3));
    assert_eq!(tree.left(1), Some(&4));
    assert_eq!(tree.right(1), Some(&5));
    assert_eq!(tree.left(2), Some(&6));
    assert_eq!(tree.right(2), None); // No right child for node 2

    assert!(tree.is_leaf(3));
    assert!(!tree.is_leaf(0));
}

fn level_order_test() {
    let tree = FlatTree::new(vec![1, 2, 3, 4, 5, 6, 7]);
    let levels = tree.levels();
    assert_eq!(levels.len(), 3);
    assert_eq!(levels[0], vec![&1]);
    assert_eq!(levels[1], vec![&2, &3]);
    assert_eq!(levels[2], vec![&4, &5, &6, &7]);
}

fn heap_test() {
    let mut tree = FlatTree::new(vec![3, 1, 4, 1, 5, 9, 2, 6]);
    tree.heapify();

    // Root should be max
    assert_eq!(tree.get(0), Some(&9));

    // Heap property: parent >= children
    for i in 1..tree.data.len() {
        let p = FlatTree::<i32>::parent(i);
        assert!(tree.data[p] >= tree.data[i],
            "Heap violation: parent[{}]={:?} < child[{}]={:?}",
            p, tree.data[p], i, tree.data[i]);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() { basic_tree(); }

    #[test]
    fn test_levels() { level_order_test(); }

    #[test]
    fn test_heap() { heap_test(); }

    #[test]
    fn test_depth() {
        assert_eq!(FlatTree::new(vec![1]).depth(), 1);
        assert_eq!(FlatTree::new(vec![1, 2, 3]).depth(), 2);
        assert_eq!(FlatTree::new(vec![1, 2, 3, 4, 5, 6, 7]).depth(), 3);
    }

    #[test]
    fn test_empty() {
        let tree: FlatTree<i32> = FlatTree::new(vec![]);
        assert_eq!(tree.depth(), 0);
        assert_eq!(tree.get(0), None);
    }
}
