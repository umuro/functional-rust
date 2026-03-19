#![allow(clippy::all)]
//! Red-Black Tree with balanced insertion.
//!
//! A red-black tree is a self-balancing binary search tree where each node
//! carries a color (Red or Black) and the tree satisfies invariants that
//! guarantee O(log n) operations:
//!   1. No red node has a red child.
//!   2. Every path from root to leaf has the same number of black nodes.
//!
//! The `balance` function restores these invariants after insertion by
//! pattern-matching on the four possible red-red violation cases and
//! rotating them into a single canonical form.

/// Node color — mirrors OCaml's `type color = Red | Black`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    Red,
    Black,
}

/// Red-black tree — mirrors OCaml's `type 'a rbtree = E | T of color * 'a rbtree * 'a * 'a rbtree`
///
/// Uses `Box` for recursive ownership. Each `Node` owns its children,
/// analogous to OCaml's garbage-collected algebraic data type, but with
/// explicit heap allocation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RBTree<T> {
    Empty,
    Node(Color, Box<RBTree<T>>, T, Box<RBTree<T>>),
}

use Color::{Black, Red};
use RBTree::{Empty, Node};

impl<T: Ord + Clone> Default for RBTree<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Ord + Clone> RBTree<T> {
    // -- Solution 1: Idiomatic Rust API --
    // Wraps the functional core with a clean public interface.

    /// Create an empty red-black tree.
    pub fn new() -> Self {
        Empty
    }

    /// Insert a value, returning a new tree with the value added.
    /// Duplicate values are ignored (set semantics).
    ///
    /// Mirrors OCaml's `let insert x t = ...` — the inner recursive `ins`
    /// creates red nodes, then the root is forced to Black.
    pub fn insert(self, value: T) -> Self {
        // The recursive helper that may create red-red violations
        fn ins<T: Ord + Clone>(tree: RBTree<T>, value: &T) -> RBTree<T> {
            match tree {
                Empty => Node(Red, Box::new(Empty), value.clone(), Box::new(Empty)),
                Node(color, left, v, right) => {
                    if *value < v {
                        balance(color, ins(*left, value), v, *right)
                    } else if *value > v {
                        balance(color, *left, v, ins(*right, value))
                    } else {
                        // Value already present — preserve existing node
                        Node(color, left, v, right)
                    }
                }
            }
        }

        // Force root to Black — ensures invariant 1 at the root
        match ins(self, &value) {
            Node(_, left, v, right) => Node(Black, left, v, right),
            Empty => Empty,
        }
    }

    /// Check membership — mirrors OCaml's `let rec mem x = function ...`
    pub fn mem(&self, value: &T) -> bool {
        match self {
            Empty => false,
            Node(_, left, v, right) => {
                if *value == *v {
                    true
                } else if *value < *v {
                    left.mem(value)
                } else {
                    right.mem(value)
                }
            }
        }
    }

    // -- Solution 2: Iterator-based in-order traversal --
    // Collects elements into a sorted Vec using an explicit stack,
    // avoiding the OCaml-style `to_list a @ [v] @ to_list b` which
    // would be O(n²) due to repeated `@` (append).

    /// Collect all elements in sorted (in-order) order.
    pub fn to_sorted_vec(&self) -> Vec<T> {
        let mut result = Vec::new();
        let mut stack: Vec<&RBTree<T>> = Vec::new();
        let mut current = self;

        loop {
            match current {
                Node(_, left, _, _) => {
                    stack.push(current);
                    current = left;
                }
                Empty => match stack.pop() {
                    Some(Node(_, _, v, right)) => {
                        result.push(v.clone());
                        current = right;
                    }
                    _ => break,
                },
            }
        }
        result
    }

    // -- Solution 3: Recursive to_sorted_vec (closer to OCaml style) --
    // Direct translation of OCaml's `to_list`, but collects into a Vec
    // passed by mutable reference to avoid O(n²) appends.

    /// Collect all elements in sorted order (recursive approach).
    pub fn to_sorted_vec_recursive(&self) -> Vec<T> {
        let mut result = Vec::new();
        self.collect_inorder(&mut result);
        result
    }

    fn collect_inorder(&self, acc: &mut Vec<T>) {
        if let Node(_, left, v, right) = self {
            left.collect_inorder(acc);
            acc.push(v.clone());
            right.collect_inorder(acc);
        }
    }

    /// Returns the number of elements in the tree.
    pub fn len(&self) -> usize {
        match self {
            Empty => 0,
            Node(_, left, _, right) => 1 + left.len() + right.len(),
        }
    }

    /// Returns true if the tree is empty.
    pub fn is_empty(&self) -> bool {
        matches!(self, Empty)
    }
}

/// Build a tree from an iterator of values.
/// Mirrors OCaml's `List.fold_left (fun t x -> insert x t) E [...]`
impl<T: Ord + Clone> FromIterator<T> for RBTree<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        iter.into_iter().fold(Self::new(), |tree, v| tree.insert(v))
    }
}

/// Balance restores red-black invariants after insertion.
///
/// Matches the four cases where a black node has a red child with a red
/// grandchild (the "red-red violation") and rotates them into a single
/// canonical form:
///
/// ```text
///        y(Red)
///       / \
///    x(B)  z(B)
///   / \   / \
///  a   b c   d
/// ```
///
/// This is a direct translation of OCaml's 4-pattern `balance` function.
fn balance<T>(color: Color, left: RBTree<T>, value: T, right: RBTree<T>) -> RBTree<T> {
    match (color, left, value, right) {
        // Case 1: Left-Left — red child's red left grandchild
        (Black, Node(Red, ll, lv, lr), v, r) if matches!(ll.as_ref(), Node(Red, _, _, _)) => {
            let Node(Red, a, x, b) = *ll else {
                unreachable!()
            };
            Node(
                Red,
                Box::new(Node(Black, a, x, b)),
                lv,
                Box::new(Node(Black, lr, v, Box::new(r))),
            )
        }
        // Case 2: Left-Right — red child's red right grandchild
        (Black, Node(Red, a, x, lr), v, r) if matches!(lr.as_ref(), Node(Red, _, _, _)) => {
            let Node(Red, b, y, c) = *lr else {
                unreachable!()
            };
            Node(
                Red,
                Box::new(Node(Black, a, x, b)),
                y,
                Box::new(Node(Black, c, v, Box::new(r))),
            )
        }
        // Case 3: Right-Left — red child's red left grandchild
        (Black, l, v, Node(Red, rl, rv, rr)) if matches!(rl.as_ref(), Node(Red, _, _, _)) => {
            let Node(Red, b, y, c) = *rl else {
                unreachable!()
            };
            Node(
                Red,
                Box::new(Node(Black, Box::new(l), v, b)),
                y,
                Box::new(Node(Black, c, rv, rr)),
            )
        }
        // Case 4: Right-Right — red child's red right grandchild
        (Black, l, v, Node(Red, rl, rv, rr)) if matches!(rr.as_ref(), Node(Red, _, _, _)) => {
            let Node(Red, c, z, d) = *rr else {
                unreachable!()
            };
            Node(
                Red,
                Box::new(Node(Black, Box::new(l), v, rl)),
                rv,
                Box::new(Node(Black, c, z, d)),
            )
        }
        // No violation — return unchanged
        (color, left, value, right) => Node(color, Box::new(left), value, Box::new(right)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_tree() {
        let tree: RBTree<i32> = RBTree::new();
        assert!(tree.is_empty());
        assert_eq!(tree.len(), 0);
        assert_eq!(tree.to_sorted_vec(), Vec::<i32>::new());
        assert!(!tree.mem(&1));
    }

    #[test]
    fn test_single_insert() {
        let tree = RBTree::new().insert(42);
        assert!(!tree.is_empty());
        assert_eq!(tree.len(), 1);
        assert!(tree.mem(&42));
        assert!(!tree.mem(&0));
        assert_eq!(tree.to_sorted_vec(), vec![42]);
        // Root must be Black
        assert!(matches!(tree, Node(Black, _, _, _)));
    }

    #[test]
    fn test_multiple_inserts_sorted_output() {
        // Mirrors the OCaml test: insert [5;3;7;1;4;6;8;2;9]
        let tree = RBTree::from_iter([5, 3, 7, 1, 4, 6, 8, 2, 9]);
        assert_eq!(tree.to_sorted_vec(), vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
        assert_eq!(tree.len(), 9);
    }

    #[test]
    fn test_duplicate_ignored() {
        let tree = RBTree::from_iter([3, 1, 2, 3, 1, 2]);
        assert_eq!(tree.to_sorted_vec(), vec![1, 2, 3]);
        assert_eq!(tree.len(), 3);
    }

    #[test]
    fn test_membership() {
        let tree = RBTree::from_iter([5, 3, 7, 1, 4, 6, 8, 2, 9]);
        for v in 1..=9 {
            assert!(tree.mem(&v), "expected tree to contain {v}");
        }
        assert!(!tree.mem(&0));
        assert!(!tree.mem(&10));
    }

    #[test]
    fn test_ascending_insertion() {
        // Worst case for naive BST — red-black tree should stay balanced
        let tree = RBTree::from_iter(1..=15);
        assert_eq!(tree.to_sorted_vec(), (1..=15).collect::<Vec<_>>());
        assert_eq!(tree.len(), 15);
        // Root must be Black
        assert!(matches!(tree, Node(Black, _, _, _)));
    }

    #[test]
    fn test_descending_insertion() {
        let tree = RBTree::from_iter((1..=10).rev());
        assert_eq!(tree.to_sorted_vec(), (1..=10).collect::<Vec<_>>());
    }

    #[test]
    fn test_recursive_to_sorted_vec_matches_iterative() {
        let tree = RBTree::from_iter([5, 3, 7, 1, 4, 6, 8, 2, 9]);
        assert_eq!(tree.to_sorted_vec(), tree.to_sorted_vec_recursive());
    }

    #[test]
    fn test_root_always_black() {
        // After every insertion, the root should be Black
        let mut tree = RBTree::new();
        for v in [5, 3, 7, 1, 4, 6, 8, 2, 9] {
            tree = tree.insert(v);
            assert!(
                matches!(tree, Node(Black, _, _, _)),
                "root must be Black after inserting {v}"
            );
        }
    }

    #[test]
    fn test_string_keys() {
        let tree = RBTree::from_iter(["cherry", "apple", "banana", "date"]);
        assert_eq!(
            tree.to_sorted_vec(),
            vec!["apple", "banana", "cherry", "date"]
        );
        assert!(tree.mem(&"banana"));
        assert!(!tree.mem(&"elderberry"));
    }

    #[test]
    fn test_black_height_invariant() {
        // Every path from root to leaf must have the same number of black nodes
        fn black_height<T>(tree: &RBTree<T>) -> Option<usize> {
            match tree {
                Empty => Some(0),
                Node(color, left, _, right) => {
                    let lh = black_height(left)?;
                    let rh = black_height(right)?;
                    if lh != rh {
                        return None;
                    }
                    Some(lh + if *color == Black { 1 } else { 0 })
                }
            }
        }

        let tree = RBTree::from_iter([5, 3, 7, 1, 4, 6, 8, 2, 9]);
        assert!(
            black_height(&tree).is_some(),
            "black height invariant violated"
        );

        // Also check ascending insertion (stress test for balancing)
        let tree2 = RBTree::from_iter(1..=31);
        assert!(
            black_height(&tree2).is_some(),
            "black height invariant violated for ascending insertion"
        );
    }

    #[test]
    fn test_no_red_red_violation() {
        // No red node should have a red child
        fn no_red_red<T>(tree: &RBTree<T>) -> bool {
            match tree {
                Empty => true,
                Node(Red, left, _, right) => {
                    let left_ok = !matches!(left.as_ref(), Node(Red, _, _, _));
                    let right_ok = !matches!(right.as_ref(), Node(Red, _, _, _));
                    left_ok && right_ok && no_red_red(left) && no_red_red(right)
                }
                Node(Black, left, _, right) => no_red_red(left) && no_red_red(right),
            }
        }

        let tree = RBTree::from_iter([5, 3, 7, 1, 4, 6, 8, 2, 9]);
        assert!(no_red_red(&tree), "red-red violation detected");

        let tree2 = RBTree::from_iter(1..=31);
        assert!(
            no_red_red(&tree2),
            "red-red violation for ascending insertion"
        );
    }
}
