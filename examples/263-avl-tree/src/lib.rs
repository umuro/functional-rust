#![allow(clippy::all)]
//! AVL Tree — Self-Balancing BST
//!
//! OCaml: `type 'a avl = Empty | Node of 'a avl * 'a * 'a avl * int`
//! Rust: `enum Avl<T> { Empty, Node { left, value, right, height } }`
//!
//! An AVL tree maintains a balance invariant: for every node, the height
//! difference between left and right subtrees is at most 1. Rotations
//! restore balance after each insert.
use std::cmp::{max, Ordering};

/// Persistent AVL tree — each insert returns a new balanced tree.
#[derive(Debug, Clone, PartialEq)]
pub enum Avl<T> {
    Empty,
    Node {
        left: Box<Avl<T>>,
        value: T,
        right: Box<Avl<T>>,
        height: i32,
    },
}

impl<T: Ord + Clone> Avl<T> {
    /// Creates an empty AVL tree.
    pub fn empty() -> Self {
        Avl::Empty
    }

    /// Returns the height of the tree.
    pub fn height(&self) -> i32 {
        match self {
            Avl::Empty => 0,
            Avl::Node { height, .. } => *height,
        }
    }

    /// Creates a node, computing height from children.
    /// OCaml: `let node l v r = Node (l, v, r, 1 + max (height l) (height r))`
    fn node(left: Avl<T>, value: T, right: Avl<T>) -> Self {
        let h = 1 + max(left.height(), right.height());
        Avl::Node {
            left: Box::new(left),
            value,
            right: Box::new(right),
            height: h,
        }
    }

    /// Computes the balance factor (left height - right height).
    fn balance_factor(&self) -> i32 {
        match self {
            Avl::Empty => 0,
            Avl::Node { left, right, .. } => left.height() - right.height(),
        }
    }

    /// Right rotation for left-heavy trees.
    ///
    /// ```text
    ///     v              lv
    ///    / \            /  \
    ///   lv  r   =>    ll   v
    ///  / \                / \
    /// ll  lr             lr  r
    /// ```
    fn rotate_right(self) -> Self {
        match self {
            Avl::Node {
                left, value, right, ..
            } => match *left {
                Avl::Node {
                    left: ll,
                    value: lv,
                    right: lr,
                    ..
                } => Self::node(*ll, lv, Self::node(*lr, value, *right)),
                _ => Self::node(*left, value, *right),
            },
            other => other,
        }
    }

    /// Left rotation for right-heavy trees.
    ///
    /// ```text
    ///   v                rv
    ///  / \              /  \
    /// l   rv    =>     v    rr
    ///    / \          / \
    ///   rl  rr      l   rl
    /// ```
    fn rotate_left(self) -> Self {
        match self {
            Avl::Node {
                left, value, right, ..
            } => match *right {
                Avl::Node {
                    left: rl,
                    value: rv,
                    right: rr,
                    ..
                } => Self::node(Self::node(*left, value, *rl), rv, *rr),
                _ => Self::node(*left, value, *right),
            },
            other => other,
        }
    }

    /// Rebalances a node after insertion.
    /// Handles four cases: left-left, left-right, right-right, right-left.
    fn rebalance(self) -> Self {
        let bf = self.balance_factor();
        if bf > 1 {
            // Left-heavy: check if left child is right-heavy (left-right case)
            match self {
                Avl::Node {
                    ref left,
                    ref value,
                    ref right,
                    ..
                } if left.balance_factor() < 0 => {
                    // Left-right case: rotate left child left, then rotate right
                    let new_left = (**left).clone().rotate_left();
                    Self::node(new_left, value.clone(), (**right).clone()).rotate_right()
                }
                _ => self.rotate_right(),
            }
        } else if bf < -1 {
            // Right-heavy: check if right child is left-heavy (right-left case)
            match self {
                Avl::Node {
                    ref left,
                    ref value,
                    ref right,
                    ..
                } if right.balance_factor() > 0 => {
                    // Right-left case: rotate right child right, then rotate left
                    let new_right = (**right).clone().rotate_right();
                    Self::node((**left).clone(), value.clone(), new_right).rotate_left()
                }
                _ => self.rotate_left(),
            }
        } else {
            self
        }
    }

    /// Inserts a value, returning a new balanced AVL tree.
    /// Duplicates are ignored.
    pub fn insert(&self, x: T) -> Self {
        match self {
            Avl::Empty => Self::node(Avl::Empty, x, Avl::Empty),
            Avl::Node {
                left, value, right, ..
            } => match x.cmp(value) {
                Ordering::Less => {
                    Self::node(left.insert(x), value.clone(), (**right).clone()).rebalance()
                }
                Ordering::Greater => {
                    Self::node((**left).clone(), value.clone(), right.insert(x)).rebalance()
                }
                Ordering::Equal => self.clone(),
            },
        }
    }

    /// In-order traversal returns sorted elements.
    pub fn inorder(&self) -> Vec<T> {
        match self {
            Avl::Empty => vec![],
            Avl::Node {
                left, value, right, ..
            } => {
                let mut result = left.inorder();
                result.push(value.clone());
                result.extend(right.inorder());
                result
            }
        }
    }

    /// Builds an AVL tree from an iterator.
    pub fn build(items: impl IntoIterator<Item = T>) -> Self {
        items
            .into_iter()
            .fold(Avl::empty(), |tree, x| tree.insert(x))
    }

    /// Checks if the tree satisfies the AVL balance invariant.
    pub fn is_balanced(&self) -> bool {
        match self {
            Avl::Empty => true,
            Avl::Node { left, right, .. } => {
                let bf = self.balance_factor();
                (-1..=1).contains(&bf) && left.is_balanced() && right.is_balanced()
            }
        }
    }
}

impl<T: Ord + Clone> Default for Avl<T> {
    fn default() -> Self {
        Self::empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_tree() {
        let tree: Avl<i32> = Avl::empty();
        assert_eq!(tree.inorder(), Vec::<i32>::new());
        assert_eq!(tree.height(), 0);
        assert!(tree.is_balanced());
    }

    #[test]
    fn test_single_element() {
        let tree = Avl::empty().insert(42);
        assert_eq!(tree.inorder(), vec![42]);
        assert_eq!(tree.height(), 1);
        assert!(tree.is_balanced());
    }

    #[test]
    fn test_sorted_insertion_stays_balanced() {
        // Inserting in sorted order would degrade a plain BST to a linked list.
        // AVL rotations keep it balanced.
        let tree = Avl::build(1..=7);
        assert_eq!(tree.inorder(), vec![1, 2, 3, 4, 5, 6, 7]);
        assert!(tree.is_balanced());
        assert!(tree.height() <= 4); // log2(7) + 1 ≈ 3.8
    }

    #[test]
    fn test_reverse_insertion_stays_balanced() {
        let tree = Avl::build((1..=8).rev());
        assert_eq!(tree.inorder(), vec![1, 2, 3, 4, 5, 6, 7, 8]);
        assert!(tree.is_balanced());
    }

    #[test]
    fn test_ocaml_example() {
        let tree = Avl::build([7, 3, 9, 1, 5, 8, 10, 2]);
        assert_eq!(tree.inorder(), vec![1, 2, 3, 5, 7, 8, 9, 10]);
        assert!(tree.is_balanced());
    }

    #[test]
    fn test_duplicate_insert() {
        let tree = Avl::build([3, 1, 3, 2, 1]);
        assert_eq!(tree.inorder(), vec![1, 2, 3]);
        assert!(tree.is_balanced());
    }
}
