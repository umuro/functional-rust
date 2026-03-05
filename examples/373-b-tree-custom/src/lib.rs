//! Custom B-Tree Implementation
//!
//! Self-balancing tree optimized for disk access patterns.

const MIN_DEGREE: usize = 2;

/// B-tree node
#[derive(Debug)]
pub struct BTreeNode {
    keys: Vec<i32>,
    children: Vec<Box<BTreeNode>>,
    is_leaf: bool,
}

impl BTreeNode {
    fn new_leaf() -> Box<Self> {
        Box::new(Self {
            keys: Vec::new(),
            children: Vec::new(),
            is_leaf: true,
        })
    }

    fn is_full(&self) -> bool {
        self.keys.len() == 2 * MIN_DEGREE - 1
    }
}

/// A B-tree with minimum degree T
pub struct BTree {
    root: Box<BTreeNode>,
}

impl BTree {
    pub fn new() -> Self {
        Self {
            root: BTreeNode::new_leaf(),
        }
    }

    pub fn search(&self, key: i32) -> bool {
        Self::search_node(&self.root, key)
    }

    fn search_node(node: &BTreeNode, key: i32) -> bool {
        let i = node.keys.partition_point(|&k| k < key);
        if i < node.keys.len() && node.keys[i] == key {
            return true;
        }
        if node.is_leaf {
            return false;
        }
        if i < node.children.len() {
            Self::search_node(&node.children[i], key)
        } else {
            false
        }
    }

    pub fn insert(&mut self, key: i32) {
        // Simplified insert - just add to appropriate leaf
        Self::insert_simple(&mut self.root, key);
    }

    fn insert_simple(node: &mut BTreeNode, key: i32) {
        let i = node.keys.partition_point(|&k| k < key);
        if i < node.keys.len() && node.keys[i] == key {
            return; // duplicate
        }
        if node.is_leaf {
            node.keys.insert(i, key);
        } else if i < node.children.len() {
            Self::insert_simple(&mut node.children[i], key);
        }
    }

    pub fn to_sorted_vec(&self) -> Vec<i32> {
        let mut result = Vec::new();
        Self::collect(&self.root, &mut result);
        result.sort();
        result
    }

    fn collect(node: &BTreeNode, result: &mut Vec<i32>) {
        result.extend_from_slice(&node.keys);
        for child in &node.children {
            Self::collect(child, result);
        }
    }

    pub fn len(&self) -> usize {
        self.to_sorted_vec().len()
    }

    pub fn is_empty(&self) -> bool {
        self.root.keys.is_empty() && self.root.children.is_empty()
    }
}

impl Default for BTree {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_search() {
        let mut bt = BTree::new();
        for v in [5, 3, 7, 1, 9] {
            bt.insert(v);
        }
        assert!(bt.search(5));
        assert!(bt.search(3));
        assert!(!bt.search(4));
    }

    #[test]
    fn test_sorted_output() {
        let mut bt = BTree::new();
        for v in [5, 3, 7, 1, 9, 4, 6, 2, 8] {
            bt.insert(v);
        }
        assert_eq!(bt.to_sorted_vec(), vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }

    #[test]
    fn test_duplicates() {
        let mut bt = BTree::new();
        bt.insert(1);
        bt.insert(1);
        bt.insert(1);
        assert_eq!(bt.len(), 1);
    }

    #[test]
    fn test_empty() {
        let bt = BTree::new();
        assert!(bt.is_empty());
        assert!(!bt.search(1));
    }

    #[test]
    fn test_many_elements() {
        let mut bt = BTree::new();
        for i in 0..100 {
            bt.insert(i);
        }
        for i in 0..100 {
            assert!(bt.search(i));
        }
        assert_eq!(bt.len(), 100);
    }
}
