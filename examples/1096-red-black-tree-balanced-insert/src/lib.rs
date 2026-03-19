#![allow(clippy::all)]
// Red-Black Tree with Okasaki's Functional Balancing (Method Style)
//
// A purely functional red-black tree using Okasaki's four-case balance rule,
// expressed with methods on the tree type rather than free functions.
// The tree is persistent: `insert` returns a new tree, leaving the original intact.

/// Node color in a red-black tree.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    Red,
    Black,
}

/// A persistent red-black tree.
///
/// `E` is the empty leaf. `T` holds a color, left subtree, value, and right
/// subtree — a direct mirror of the OCaml `'a rbtree` type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RBTree<V> {
    E,
    T(Color, Box<RBTree<V>>, V, Box<RBTree<V>>),
}

use Color::{Black, Red};
use RBTree::{E, T};

// ---------------------------------------------------------------------------
// Solution 1: Idiomatic Rust — methods with Okasaki balance
// ---------------------------------------------------------------------------

impl<V: Ord> RBTree<V> {
    /// Create an empty red-black tree.
    pub fn new() -> Self {
        E
    }

    /// Insert a value, returning a new balanced tree.
    ///
    /// The inner recursive `ins` creates red nodes, then `balance` fixes any
    /// red-red violations. The root is always repainted black.
    pub fn insert(self, value: V) -> Self {
        // Repaint root black after insertion
        match Self::ins(value, self) {
            T(_, left, v, right) => T(Black, left, v, right),
            E => E,
        }
    }

    /// Check whether `value` is present in the tree.
    ///
    /// Mirrors the OCaml `mem` function: recursively descend left or right
    /// based on comparison, returning `true` on an exact match.
    pub fn mem(&self, value: &V) -> bool {
        match self {
            E => false,
            T(_, left, v, right) => {
                if value == v {
                    true
                } else if value < v {
                    left.mem(value)
                } else {
                    right.mem(value)
                }
            }
        }
    }

    /// Internal recursive insert — colors new leaves red.
    fn ins(x: V, tree: Self) -> Self {
        match tree {
            E => T(Red, Box::new(E), x, Box::new(E)),
            T(color, left, y, right) => {
                if x < y {
                    Self::balance(color, Self::ins(x, *left), y, *right)
                } else if x > y {
                    Self::balance(color, *left, y, Self::ins(x, *right))
                } else {
                    // Duplicate — return tree unchanged
                    T(color, left, y, right)
                }
            }
        }
    }

    /// Okasaki's balance: four red-red violation cases → one balanced result.
    ///
    /// Each arm detects a different position of the red-red violation and
    /// rotates into the canonical form:
    ///   `T(Red, T(Black, a, x, b), y, T(Black, c, z, d))`
    fn balance(color: Color, left: Self, val: V, right: Self) -> Self {
        // Helper: produce the common rebalanced node
        let balanced = |a: Self, x: V, b: Self, y: V, c: Self, z: V, d: Self| -> Self {
            T(
                Red,
                Box::new(T(Black, Box::new(a), x, Box::new(b))),
                y,
                Box::new(T(Black, Box::new(c), z, Box::new(d))),
            )
        };

        match (color, left, val, right) {
            // Case 1: left-left red-red
            (Black, T(Red, ll, y, c), z, d) if matches!(*ll, T(Red, ..)) => {
                let T(_, a, x, b) = *ll else { unreachable!() };
                balanced(*a, x, *b, y, *c, z, d)
            }
            // Case 2: left-right red-red
            (Black, T(Red, a, x, lr), z, d) if matches!(*lr, T(Red, ..)) => {
                let T(_, b, y, c) = *lr else { unreachable!() };
                balanced(*a, x, *b, y, *c, z, d)
            }
            // Case 3: right-left red-red
            (Black, a, x, T(Red, rl, z, d)) if matches!(*rl, T(Red, ..)) => {
                let T(_, b, y, c) = *rl else { unreachable!() };
                balanced(a, x, *b, y, *c, z, *d)
            }
            // Case 4: right-right red-red
            (Black, a, x, T(Red, b, y, rr)) if matches!(*rr, T(Red, ..)) => {
                let T(_, c, z, d) = *rr else { unreachable!() };
                balanced(a, x, *b, y, *c, z, *d)
            }
            // No violation — reconstruct unchanged
            (col, a, x, b) => T(col, Box::new(a), x, Box::new(b)),
        }
    }
}

// ---------------------------------------------------------------------------
// Solution 2: Accumulator-based in-order traversal
// ---------------------------------------------------------------------------

impl<V: Clone> RBTree<V> {
    /// Collect all values in sorted (in-order) order.
    ///
    /// Uses a mutable accumulator for O(n) traversal, avoiding the O(n log n)
    /// cost of OCaml's repeated list append (`@`).
    pub fn to_sorted_vec(&self) -> Vec<V> {
        let mut result = Vec::new();
        self.collect_inorder(&mut result);
        result
    }

    fn collect_inorder(&self, acc: &mut Vec<V>) {
        if let T(_, left, v, right) = self {
            left.collect_inorder(acc);
            acc.push(v.clone()); // clone: tree retains ownership of values
            right.collect_inorder(acc);
        }
    }
}

// ---------------------------------------------------------------------------
// Solution 3: Recursive to_list — closer to OCaml style (allocates per level)
// ---------------------------------------------------------------------------

impl<V: Clone> RBTree<V> {
    /// In-order list, OCaml style: `to_list a @ [v] @ to_list b`.
    ///
    /// Allocates at every level, mirroring the OCaml original. Prefer
    /// `to_sorted_vec` for real use.
    pub fn to_list_recursive(&self) -> Vec<V> {
        match self {
            E => Vec::new(),
            T(_, left, v, right) => {
                let mut result = left.to_list_recursive();
                result.push(v.clone()); // clone: tree keeps ownership
                result.extend(right.to_list_recursive());
                result
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Invariant validation helpers
// ---------------------------------------------------------------------------

impl<V: Ord> RBTree<V> {
    /// Validate all red-black tree invariants:
    /// 1. Root is black
    /// 2. No red node has a red child
    /// 3. Every root-to-leaf path has the same black-height
    pub fn is_valid(&self) -> bool {
        let root_black = !matches!(self, T(Red, ..));
        root_black && self.no_red_red() && self.black_height().is_some()
    }

    /// Check that no red node has a red child.
    fn no_red_red(&self) -> bool {
        match self {
            E => true,
            T(Red, left, _, right) => {
                !matches!(left.as_ref(), T(Red, ..))
                    && !matches!(right.as_ref(), T(Red, ..))
                    && left.no_red_red()
                    && right.no_red_red()
            }
            T(Black, left, _, right) => left.no_red_red() && right.no_red_red(),
        }
    }

    /// Compute the black-height (uniform black count on every root-to-leaf path).
    /// Returns `None` if the invariant is violated.
    fn black_height(&self) -> Option<usize> {
        match self {
            E => Some(1), // leaves count as black
            T(color, left, _, right) => {
                let lh = left.black_height()?;
                let rh = right.black_height()?;
                if lh != rh {
                    return None;
                }
                Some(lh + usize::from(*color == Black))
            }
        }
    }
}

impl<V: Ord> Default for RBTree<V> {
    fn default() -> Self {
        Self::new()
    }
}

/// Build a tree from an iterator by folding `insert` over each element.
///
/// Direct analog of OCaml's `List.fold_left (fun t x -> insert x t) E xs`.
impl<V: Ord> FromIterator<V> for RBTree<V> {
    fn from_iter<I: IntoIterator<Item = V>>(iter: I) -> Self {
        iter.into_iter().fold(E, |tree, x| tree.insert(x))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_tree() {
        let tree: RBTree<i32> = RBTree::new();
        assert!(!tree.mem(&1));
        assert_eq!(tree.to_sorted_vec(), Vec::<i32>::new());
        assert!(tree.is_valid());
    }

    #[test]
    fn test_single_insert() {
        let tree = RBTree::new().insert(42);
        assert!(tree.mem(&42));
        assert!(!tree.mem(&0));
        assert_eq!(tree.to_sorted_vec(), vec![42]);
        // Root must be black after insert
        assert!(matches!(tree, T(Black, ..)));
        assert!(tree.is_valid());
    }

    #[test]
    fn test_multiple_inserts_sorted_output() {
        // Same sequence as the OCaml example
        let tree = RBTree::from_iter([5, 3, 7, 1, 4, 6, 8, 2, 9]);
        assert_eq!(tree.to_sorted_vec(), vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }

    #[test]
    fn test_membership() {
        let tree = RBTree::from_iter([5, 3, 7, 1, 4, 6, 8, 2, 9]);
        for v in 1..=9 {
            assert!(tree.mem(&v), "expected {v} in tree");
        }
        assert!(!tree.mem(&0));
        assert!(!tree.mem(&10));
    }

    #[test]
    fn test_duplicate_inserts_ignored() {
        let tree = RBTree::from_iter([3, 1, 2, 3, 1, 2, 3]);
        assert_eq!(tree.to_sorted_vec(), vec![1, 2, 3]);
    }

    #[test]
    fn test_invariants_after_scrambled_inserts() {
        let tree = RBTree::from_iter([
            10, 5, 15, 3, 7, 12, 18, 1, 4, 6, 8, 11, 13, 17, 20, 2, 9, 14, 16, 19,
        ]);
        assert!(tree.is_valid());
        assert_eq!(tree.to_sorted_vec(), (1..=20).collect::<Vec<_>>());
    }

    #[test]
    fn test_ascending_insert_stays_balanced() {
        let tree = RBTree::from_iter(1..=100);
        assert!(tree.is_valid());
        assert_eq!(tree.to_sorted_vec(), (1..=100).collect::<Vec<_>>());
    }

    #[test]
    fn test_descending_insert_stays_balanced() {
        let tree = RBTree::from_iter((1..=50).rev());
        assert!(tree.is_valid());
        assert_eq!(tree.to_sorted_vec(), (1..=50).collect::<Vec<_>>());
    }

    #[test]
    fn test_from_iter_empty() {
        let tree: RBTree<i32> = RBTree::from_iter(std::iter::empty());
        assert_eq!(tree, E);
        assert!(tree.is_valid());
    }

    #[test]
    fn test_to_list_recursive_matches_sorted_vec() {
        let tree = RBTree::from_iter([5, 3, 7, 1, 4, 6, 8, 2, 9]);
        assert_eq!(tree.to_list_recursive(), tree.to_sorted_vec());
    }

    #[test]
    fn test_string_keys() {
        let tree = RBTree::from_iter(["cherry", "apple", "banana", "date"]);
        assert_eq!(
            tree.to_sorted_vec(),
            vec!["apple", "banana", "cherry", "date"]
        );
        assert!(tree.mem(&"banana"));
        assert!(!tree.mem(&"fig"));
        assert!(tree.is_valid());
    }

    #[test]
    fn test_default_is_empty() {
        let tree: RBTree<i32> = RBTree::default();
        assert_eq!(tree, E);
    }
}
