#![allow(clippy::all)]
/// Red-Black Tree — Okasaki's purely functional balanced BST
///
/// A red-black tree maintains balance through color invariants:
/// 1. No red node has a red child
/// 2. Every path from root to leaf has the same number of black nodes
///
/// Okasaki's insight: all four rotation cases collapse into a single balance function
/// that pattern-matches on the four "red-red violation" shapes and rewrites them
/// into one canonical balanced form.
use std::cmp::Ordering;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    Red,
    Black,
}

/// A purely functional red-black tree.
/// Uses `Box` for heap-allocated children — no `Rc` needed because the tree is rebuilt
/// on each insert (persistent data structure via path copying).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RBTree<T> {
    Empty,
    Node {
        color: Color,
        left: Box<RBTree<T>>,
        value: T,
        right: Box<RBTree<T>>,
    },
}

use Color::{Black, Red};
use RBTree::{Empty, Node};

impl<T> RBTree<T> {
    fn node(color: Color, left: RBTree<T>, value: T, right: RBTree<T>) -> Self {
        Node {
            color,
            left: Box::new(left),
            value,
            right: Box::new(right),
        }
    }

    fn is_red_node(&self) -> bool {
        matches!(self, Node { color: Red, .. })
    }
}

impl<T: Ord> RBTree<T> {
    /// Creates an empty red-black tree.
    pub fn new() -> Self {
        Empty
    }

    /// Checks membership — O(log n).
    ///
    /// Direct translation of OCaml's `mem`:
    /// ```ocaml
    /// let rec mem x = function
    ///   | E -> false
    ///   | T (_, a, y, b) -> x = y || (if x < y then mem x a else mem x b)
    /// ```
    pub fn mem(&self, x: &T) -> bool {
        match self {
            Empty => false,
            Node {
                left, value, right, ..
            } => match x.cmp(value) {
                Ordering::Equal => true,
                Ordering::Less => left.mem(x),
                Ordering::Greater => right.mem(x),
            },
        }
    }

    /// Inserts a value, returning a new tree (functional/persistent).
    ///
    /// The inner `ins` builds a potentially unbalanced tree with a red-red violation,
    /// then `balance` fixes it. The root is always repainted black.
    pub fn insert(self, x: T) -> Self
    where
        T: Clone,
    {
        fn ins<T: Ord + Clone>(tree: RBTree<T>, x: &T) -> RBTree<T> {
            match tree {
                Empty => RBTree::node(Red, Empty, x.clone(), Empty),
                Node {
                    color,
                    left,
                    value,
                    right,
                } => match x.cmp(&value) {
                    Ordering::Less => balance(color, ins(*left, x), value, *right),
                    Ordering::Greater => balance(color, *left, value, ins(*right, x)),
                    Ordering::Equal => RBTree::node(color, *left, value, *right),
                },
            }
        }

        // Root is always black
        match ins(self, &x) {
            Node {
                left, value, right, ..
            } => Node {
                color: Black,
                left,
                value,
                right,
            },
            Empty => Empty,
        }
    }

    /// In-order traversal producing a sorted vector — recursive (OCaml style).
    ///
    /// Mirrors OCaml's `to_list`:
    /// ```ocaml
    /// let rec to_list = function
    ///   | E -> [] | T (_, a, v, b) -> to_list a @ [v] @ to_list b
    /// ```
    pub fn to_sorted_vec(&self) -> Vec<&T> {
        match self {
            Empty => vec![],
            Node {
                left, value, right, ..
            } => {
                let mut result = left.to_sorted_vec();
                result.push(value);
                result.extend(right.to_sorted_vec());
                result
            }
        }
    }

    /// In-order traversal — iterative with explicit stack (idiomatic Rust).
    pub fn to_sorted_vec_iter(&self) -> Vec<&T> {
        let mut stack = vec![];
        let mut result = vec![];
        let mut current = self;

        loop {
            match current {
                Node {
                    left, value, right, ..
                } => {
                    stack.push((value, right.as_ref()));
                    current = left.as_ref();
                }
                Empty => match stack.pop() {
                    Some((value, right)) => {
                        result.push(value);
                        current = right;
                    }
                    None => break,
                },
            }
        }

        result
    }

    /// Returns the number of elements in the tree.
    pub fn len(&self) -> usize {
        match self {
            Empty => 0,
            Node { left, right, .. } => 1 + left.len() + right.len(),
        }
    }

    /// Returns true if the tree is empty.
    pub fn is_empty(&self) -> bool {
        matches!(self, Empty)
    }

    /// Returns the height (longest path from root to leaf).
    pub fn height(&self) -> usize {
        match self {
            Empty => 0,
            Node { left, right, .. } => 1 + left.height().max(right.height()),
        }
    }

    /// Validates red-black tree invariants:
    /// 1. No red node has a red child
    /// 2. Every root-to-leaf path has the same black depth
    ///
    /// Returns `Some(black_depth)` if valid, `None` if violated.
    pub fn validate(&self) -> Option<usize> {
        match self {
            Empty => Some(1), // leaves count as black
            Node {
                color, left, right, ..
            } => {
                if *color == Red
                    && (matches!(left.as_ref(), Node { color: Red, .. })
                        || matches!(right.as_ref(), Node { color: Red, .. }))
                {
                    return None;
                }

                let left_depth = left.validate()?;
                let right_depth = right.validate()?;

                if left_depth != right_depth {
                    return None;
                }

                Some(if *color == Black {
                    left_depth + 1
                } else {
                    left_depth
                })
            }
        }
    }
}

impl<T: Ord> Default for RBTree<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// Okasaki's balance function — the heart of the algorithm.
///
/// Pattern-matches on four cases where a black node has a red child
/// with a red grandchild (red-red violation), and rewrites all four
/// into one canonical balanced form:
///
/// ```text
///        y(R)
///       /    \
///     x(B)   z(B)
///    / \     / \
///   a   b   c   d
/// ```
///
/// This is a direct translation of the OCaml:
/// ```ocaml
/// let balance = function
///   | Black, T(Red, T(Red,a,x,b), y, c), z, d      (* left-left *)
///   | Black, T(Red, a, x, T(Red,b,y,c)), z, d      (* left-right *)
///   | Black, a, x, T(Red, T(Red,b,y,c), z, d)      (* right-left *)
///   | Black, a, x, T(Red, b, y, T(Red,c,z,d))      (* right-right *)
///     -> T(Red, T(Black,a,x,b), y, T(Black,c,z,d))
///   | color, a, x, b -> T(color, a, x, b)
/// ```
///
/// In Rust we can't match four nested patterns in one arm like OCaml, so we
/// peek at the structure via references first, then destructure by move.
fn balance<T>(color: Color, left: RBTree<T>, value: T, right: RBTree<T>) -> RBTree<T> {
    if color != Black {
        return RBTree::node(color, left, value, right);
    }

    // Peek at two levels of nesting to determine which (if any) case applies.
    // All four cases rewrite to: Red(Black(a,x,b), y, Black(c,z,d))

    // Case 1: left-left — Black(Red(Red(a,x,b),y,c), z, d)
    if left.is_red_node() {
        if let Node { left: ref ll, .. } = left {
            if ll.is_red_node() {
                // Destructure by move now that we've confirmed the shape
                if let Node {
                    left: ll_box,
                    value: y,
                    right: c,
                    ..
                } = left
                {
                    if let Node {
                        left: a,
                        value: x,
                        right: b,
                        ..
                    } = *ll_box
                    {
                        return RBTree::node(
                            Red,
                            RBTree::node(Black, *a, x, *b),
                            y,
                            RBTree::node(Black, *c, value, right),
                        );
                    }
                }
                unreachable!();
            }
        }
    }

    // Case 2: left-right — Black(Red(a,x,Red(b,y,c)), z, d)
    if left.is_red_node() {
        if let Node { right: ref lr, .. } = left {
            if lr.is_red_node() {
                if let Node {
                    left: a,
                    value: x,
                    right: lr_box,
                    ..
                } = left
                {
                    if let Node {
                        left: b,
                        value: y,
                        right: c,
                        ..
                    } = *lr_box
                    {
                        return RBTree::node(
                            Red,
                            RBTree::node(Black, *a, x, *b),
                            y,
                            RBTree::node(Black, *c, value, right),
                        );
                    }
                }
                unreachable!();
            }
        }
    }

    // Case 3: right-left — Black(a, x, Red(Red(b,y,c),z,d))
    if right.is_red_node() {
        if let Node { left: ref rl, .. } = right {
            if rl.is_red_node() {
                if let Node {
                    left: rl_box,
                    value: z,
                    right: d,
                    ..
                } = right
                {
                    if let Node {
                        left: b,
                        value: y,
                        right: c,
                        ..
                    } = *rl_box
                    {
                        return RBTree::node(
                            Red,
                            RBTree::node(Black, left, value, *b),
                            y,
                            RBTree::node(Black, *c, z, *d),
                        );
                    }
                }
                unreachable!();
            }
        }
    }

    // Case 4: right-right — Black(a, x, Red(b,y,Red(c,z,d)))
    if right.is_red_node() {
        if let Node { right: ref rr, .. } = right {
            if rr.is_red_node() {
                if let Node {
                    left: b,
                    value: y,
                    right: rr_box,
                    ..
                } = right
                {
                    if let Node {
                        left: c,
                        value: z,
                        right: d,
                        ..
                    } = *rr_box
                    {
                        return RBTree::node(
                            Red,
                            RBTree::node(Black, left, value, *b),
                            y,
                            RBTree::node(Black, *c, z, *d),
                        );
                    }
                }
                unreachable!();
            }
        }
    }

    // Default: no rebalancing needed
    RBTree::node(Black, left, value, right)
}

/// Convenience: build a tree from an iterator (functional fold).
pub fn from_iter<T: Ord + Clone>(iter: impl IntoIterator<Item = T>) -> RBTree<T> {
    iter.into_iter().fold(RBTree::new(), |t, x| t.insert(x))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_tree() {
        let tree: RBTree<i32> = RBTree::new();
        assert!(tree.is_empty());
        assert_eq!(tree.len(), 0);
        assert!(!tree.mem(&1));
        assert_eq!(tree.to_sorted_vec(), Vec::<&i32>::new());
    }

    #[test]
    fn test_single_insert() {
        let tree = RBTree::new().insert(42);
        assert!(!tree.is_empty());
        assert_eq!(tree.len(), 1);
        assert!(tree.mem(&42));
        assert!(!tree.mem(&0));
        assert_eq!(tree.to_sorted_vec(), vec![&42]);
        // Root must be black
        assert!(matches!(tree, Node { color: Black, .. }));
    }

    #[test]
    fn test_multiple_inserts_sorted_output() {
        // Same sequence as the OCaml example
        let tree = from_iter([5, 3, 7, 1, 4, 6, 8, 2, 9]);
        let sorted = tree.to_sorted_vec();
        assert_eq!(sorted, vec![&1, &2, &3, &4, &5, &6, &7, &8, &9]);
    }

    #[test]
    fn test_membership() {
        let tree = from_iter([5, 3, 7, 1, 4, 6, 8, 2, 9]);
        for x in 1..=9 {
            assert!(tree.mem(&x), "expected {x} to be in the tree");
        }
        assert!(!tree.mem(&0));
        assert!(!tree.mem(&10));
        assert!(!tree.mem(&100));
    }

    #[test]
    fn test_duplicate_insert() {
        let tree = from_iter([3, 1, 3, 2, 1, 3]);
        assert_eq!(tree.len(), 3);
        assert_eq!(tree.to_sorted_vec(), vec![&1, &2, &3]);
    }

    #[test]
    fn test_red_black_invariants() {
        let tree = from_iter([5, 3, 7, 1, 4, 6, 8, 2, 9]);
        assert!(matches!(tree, Node { color: Black, .. }));
        assert!(tree.validate().is_some(), "red-black invariants violated");
    }

    #[test]
    fn test_invariants_ascending_insert() {
        // Ascending order stresses right-rotation paths
        let tree = from_iter(1..=20);
        assert!(
            tree.validate().is_some(),
            "invariants violated on ascending insert"
        );
        assert_eq!(tree.len(), 20);
        let result = tree.to_sorted_vec();
        assert_eq!(result.len(), 20);
        assert_eq!(*result[0], 1);
        assert_eq!(*result[19], 20);
    }

    #[test]
    fn test_invariants_descending_insert() {
        // Descending order stresses left-rotation paths
        let tree = from_iter((1..=20).rev());
        assert!(
            tree.validate().is_some(),
            "invariants violated on descending insert"
        );
        assert_eq!(tree.len(), 20);
    }

    #[test]
    fn test_height_is_logarithmic() {
        let tree = from_iter(1..=100);
        let h = tree.height();
        // Red-black tree height <= 2 * log2(n+1)
        // For n=100: 2 * log2(101) ≈ 13.3, so height should be <= 14
        assert!(h <= 14, "height {h} exceeds 2*log2(101) bound");
    }

    #[test]
    fn test_iterator_traversal_matches_recursive() {
        let tree = from_iter([5, 3, 7, 1, 4, 6, 8, 2, 9]);
        assert_eq!(tree.to_sorted_vec(), tree.to_sorted_vec_iter());
    }

    #[test]
    fn test_string_keys() {
        let tree = from_iter(["delta", "alpha", "charlie", "bravo"].map(String::from));
        let sorted = tree.to_sorted_vec();
        assert_eq!(
            sorted.iter().map(|s| s.as_str()).collect::<Vec<_>>(),
            vec!["alpha", "bravo", "charlie", "delta"]
        );
    }
}
