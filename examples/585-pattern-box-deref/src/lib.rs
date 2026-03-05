//! # Box Deref Patterns
//!
//! Pattern matching through Box with automatic dereferencing.

/// Binary search tree using Box for recursive structure.
#[derive(Debug, Clone, PartialEq)]
pub enum Tree {
    Leaf,
    Node {
        val: i32,
        left: Box<Tree>,
        right: Box<Tree>,
    },
}

impl Tree {
    /// Create a leaf node.
    pub fn leaf() -> Box<Self> {
        Box::new(Tree::Leaf)
    }

    /// Create an internal node.
    pub fn node(val: i32, left: Box<Tree>, right: Box<Tree>) -> Box<Self> {
        Box::new(Tree::Node { val, left, right })
    }

    /// Create a single-node tree.
    pub fn singleton(val: i32) -> Box<Self> {
        Self::node(val, Self::leaf(), Self::leaf())
    }
}

/// Calculate tree depth - Rust auto-derefs through Box in patterns.
pub fn depth(t: &Tree) -> usize {
    match t {
        Tree::Leaf => 0,
        Tree::Node { left, right, .. } => 1 + depth(left).max(depth(right)),
    }
}

/// Alternative using explicit deref.
pub fn depth_explicit(t: &Tree) -> usize {
    match t {
        Tree::Leaf => 0,
        Tree::Node { left, right, .. } => {
            let l = depth_explicit(left.as_ref());
            let r = depth_explicit(right.as_ref());
            1 + l.max(r)
        }
    }
}

/// Check if tree contains a value (BST search).
pub fn contains(t: &Tree, v: i32) -> bool {
    match t {
        Tree::Leaf => false,
        Tree::Node { val, left, right } => match v.cmp(val) {
            std::cmp::Ordering::Equal => true,
            std::cmp::Ordering::Less => contains(left, v),
            std::cmp::Ordering::Greater => contains(right, v),
        },
    }
}

/// Insert a value into BST.
pub fn insert(t: Box<Tree>, v: i32) -> Box<Tree> {
    match *t {
        Tree::Leaf => Tree::singleton(v),
        Tree::Node { val, left, right } => {
            if v < val {
                Tree::node(val, insert(left, v), right)
            } else if v > val {
                Tree::node(val, left, insert(right, v))
            } else {
                Tree::node(val, left, right)
            }
        }
    }
}

/// Count total nodes in tree.
pub fn count(t: &Tree) -> usize {
    match t {
        Tree::Leaf => 0,
        Tree::Node { left, right, .. } => 1 + count(left) + count(right),
    }
}

/// Sum all values in tree.
pub fn sum(t: &Tree) -> i32 {
    match t {
        Tree::Leaf => 0,
        Tree::Node { val, left, right } => val + sum(left) + sum(right),
    }
}

/// Collect values in order (inorder traversal).
pub fn inorder(t: &Tree) -> Vec<i32> {
    match t {
        Tree::Leaf => vec![],
        Tree::Node { val, left, right } => {
            let mut result = inorder(left);
            result.push(*val);
            result.extend(inorder(right));
            result
        }
    }
}

/// Find minimum value (leftmost).
pub fn min_value(t: &Tree) -> Option<i32> {
    match t {
        Tree::Leaf => None,
        Tree::Node { val, left, .. } => match left.as_ref() {
            Tree::Leaf => Some(*val),
            _ => min_value(left),
        },
    }
}

/// Find maximum value (rightmost).
pub fn max_value(t: &Tree) -> Option<i32> {
    match t {
        Tree::Leaf => None,
        Tree::Node { val, right, .. } => match right.as_ref() {
            Tree::Leaf => Some(*val),
            _ => max_value(right),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_tree(values: &[i32]) -> Box<Tree> {
        values
            .iter()
            .fold(Tree::leaf(), |acc, &v| insert(acc, v))
    }

    #[test]
    fn test_depth_leaf() {
        assert_eq!(depth(&Tree::Leaf), 0);
    }

    #[test]
    fn test_depth_single() {
        let t = Tree::singleton(1);
        assert_eq!(depth(&t), 1);
    }

    #[test]
    fn test_depth_balanced() {
        let t = build_tree(&[5, 3, 7, 1, 4, 6, 8]);
        assert_eq!(depth(&t), 3);
    }

    #[test]
    fn test_depth_approaches_equivalent() {
        let t = build_tree(&[5, 3, 7, 1, 4]);
        assert_eq!(depth(&t), depth_explicit(&t));
    }

    #[test]
    fn test_contains() {
        let t = build_tree(&[5, 3, 7]);
        assert!(contains(&t, 3));
        assert!(contains(&t, 5));
        assert!(contains(&t, 7));
        assert!(!contains(&t, 6));
    }

    #[test]
    fn test_insert_maintains_bst() {
        let t = build_tree(&[5, 3, 7, 1, 4, 6, 8]);
        let values = inorder(&t);
        assert_eq!(values, vec![1, 3, 4, 5, 6, 7, 8]);
    }

    #[test]
    fn test_count() {
        let t = build_tree(&[5, 3, 7, 1, 4]);
        assert_eq!(count(&t), 5);
    }

    #[test]
    fn test_sum() {
        let t = build_tree(&[5, 3, 7, 1, 4]);
        assert_eq!(sum(&t), 20);
    }

    #[test]
    fn test_inorder() {
        let t = build_tree(&[5, 3, 7, 1, 4]);
        assert_eq!(inorder(&t), vec![1, 3, 4, 5, 7]);
    }

    #[test]
    fn test_min_max() {
        let t = build_tree(&[5, 3, 7, 1, 4, 6, 8]);
        assert_eq!(min_value(&t), Some(1));
        assert_eq!(max_value(&t), Some(8));
    }

    #[test]
    fn test_min_max_empty() {
        assert_eq!(min_value(&Tree::Leaf), None);
        assert_eq!(max_value(&Tree::Leaf), None);
    }
}
