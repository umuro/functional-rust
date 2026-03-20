#![allow(clippy::all)]
// Red-Black Tree with Okasaki's Functional Balancing
//
// A purely functional red-black tree implementation following Chris Okasaki's
// elegant balancing approach from "Purely Functional Data Structures."
// The key insight is that all four rotation cases collapse into a single
// rebalancing rule expressed via pattern matching.

/// Node color: Red or Black
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    Red,
    Black,
}

/// A persistent red-black tree. `E` is the empty tree; `T` holds a color,
/// left subtree, value, and right subtree — mirroring the OCaml type directly.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RBTree<T> {
    E,
    T(Color, Box<RBTree<T>>, T, Box<RBTree<T>>),
}

use Color::{Black, Red};
use RBTree::{E, T};

// ---------------------------------------------------------------------------
// Solution 1: Idiomatic Rust — pattern matching with Okasaki balance
// ---------------------------------------------------------------------------

/// Okasaki's balance function: four imbalanced cases map to the same result.
///
/// Takes ownership of all parts (functional rebuild), returns a new tree node.
/// Each pattern detects a red-red violation after insertion and rotates to fix it.
///
/// Since `box` patterns are nightly-only, we match on the outer structure first
/// and then destructure the inner `Box` contents with `if let` / nested matches.
fn balance<V: Ord>(color: Color, left: RBTree<V>, val: V, right: RBTree<V>) -> RBTree<V> {
    // Helper: build the common balanced result node
    //   T(Red, T(Black, a, x, b), y, T(Black, c, z, d))
    fn balanced<V>(
        a: RBTree<V>,
        x: V,
        b: RBTree<V>,
        y: V,
        c: RBTree<V>,
        z: V,
        d: RBTree<V>,
    ) -> RBTree<V> {
        T(
            Red,
            Box::new(T(Black, Box::new(a), x, Box::new(b))),
            y,
            Box::new(T(Black, Box::new(c), z, Box::new(d))),
        )
    }

    match (color, left, val, right) {
        // Case 1: left-left red-red
        // Black, T(Red, T(Red, a, x, b), y, c), z, d
        (Black, T(Red, ll, y, c), z, d) if matches!(*ll, T(Red, ..)) => {
            let T(_, a, x, b) = *ll else { unreachable!() };
            balanced(*a, x, *b, y, *c, z, d)
        }
        // Case 2: left-right red-red
        // Black, T(Red, a, x, T(Red, b, y, c)), z, d
        (Black, T(Red, a, x, lr), z, d) if matches!(*lr, T(Red, ..)) => {
            let T(_, b, y, c) = *lr else { unreachable!() };
            balanced(*a, x, *b, y, *c, z, d)
        }
        // Case 3: right-left red-red
        // Black, a, x, T(Red, T(Red, b, y, c), z, d)
        (Black, a, x, T(Red, rl, z, d)) if matches!(*rl, T(Red, ..)) => {
            let T(_, b, y, c) = *rl else { unreachable!() };
            balanced(a, x, *b, y, *c, z, *d)
        }
        // Case 4: right-right red-red
        // Black, a, x, T(Red, b, y, T(Red, c, z, d))
        (Black, a, x, T(Red, b, y, rr)) if matches!(*rr, T(Red, ..)) => {
            let T(_, c, z, d) = *rr else { unreachable!() };
            balanced(a, x, *b, y, *c, z, *d)
        }
        // No violation — reconstruct the node unchanged
        (color, a, x, b) => T(color, Box::new(a), x, Box::new(b)),
    }
}

/// Insert a value into the red-black tree, maintaining balance invariants.
///
/// The inner `ins` function inserts as in a normal BST (coloring new nodes red),
/// then `balance` fixes any red-red violations. The root is always repainted black.
pub fn insert<V: Ord>(value: V, tree: RBTree<V>) -> RBTree<V> {
    fn ins<V: Ord>(x: V, tree: RBTree<V>) -> RBTree<V> {
        match tree {
            E => T(Red, Box::new(E), x, Box::new(E)),
            T(color, left, y, right) => {
                if x < y {
                    balance(color, ins(x, *left), y, *right)
                } else if x > y {
                    balance(color, *left, y, ins(x, *right))
                } else {
                    // Duplicate — return tree unchanged
                    T(color, left, y, right)
                }
            }
        }
    }

    // Repaint root black after insertion
    match ins(value, tree) {
        T(_, left, y, right) => T(Black, left, y, right),
        E => E,
    }
}

/// Check membership in the tree (recursive search, mirrors OCaml `mem`).
pub fn mem<V: Ord>(value: &V, tree: &RBTree<V>) -> bool {
    match tree {
        E => false,
        T(_, left, y, right) => {
            if value == y {
                true
            } else if value < y {
                mem(value, left)
            } else {
                mem(value, right)
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Solution 2: Accumulator-based in-order traversal (avoids OCaml's `@` cost)
// ---------------------------------------------------------------------------

/// Collect tree values in sorted (in-order) order into a Vec.
///
/// Mirrors the OCaml `to_list` but uses a mutable accumulator instead of
/// list append (`@`), giving O(n) instead of O(n log n).
pub fn to_sorted_vec<V: Clone>(tree: &RBTree<V>) -> Vec<V> {
    fn collect<V: Clone>(tree: &RBTree<V>, acc: &mut Vec<V>) {
        match tree {
            E => {}
            T(_, left, v, right) => {
                collect(left, acc);
                acc.push(v.clone()); // clone needed: tree retains ownership
                collect(right, acc);
            }
        }
    }
    let mut result = Vec::new();
    collect(tree, &mut result);
    result
}

// ---------------------------------------------------------------------------
// Solution 3: Fold-based tree construction — mirrors OCaml List.fold_left
// ---------------------------------------------------------------------------

/// Build a tree from an iterator of values using a left fold over `insert`.
///
/// Direct analog of `List.fold_left (fun t x -> insert x t) E xs`.
pub fn from_iter<V: Ord>(iter: impl IntoIterator<Item = V>) -> RBTree<V> {
    iter.into_iter().fold(E, |tree, x| insert(x, tree))
}

/// Compute the black-height of the tree (number of black nodes on any
/// path from root to a leaf). Returns `None` if the tree violates the
/// invariant (different paths have different black-heights).
fn black_height<V>(tree: &RBTree<V>) -> Option<usize> {
    match tree {
        E => Some(1), // leaves count as black
        T(color, left, _, right) => {
            let lh = black_height(left)?;
            let rh = black_height(right)?;
            if lh != rh {
                return None;
            }
            Some(lh + usize::from(*color == Black))
        }
    }
}

/// Validate that no red node has a red child.
fn no_red_red<V>(tree: &RBTree<V>) -> bool {
    match tree {
        E => true,
        T(Red, left, _, right) => {
            let left_ok = !matches!(left.as_ref(), T(Red, ..));
            let right_ok = !matches!(right.as_ref(), T(Red, ..));
            left_ok && right_ok && no_red_red(left) && no_red_red(right)
        }
        T(Black, left, _, right) => no_red_red(left) && no_red_red(right),
    }
}

/// Validate all red-black tree invariants:
/// 1. Root is black
/// 2. No red node has a red child
/// 3. Every path from root to leaf has the same number of black nodes
pub fn is_valid_rbtree<V: Ord>(tree: &RBTree<V>) -> bool {
    let root_black = !matches!(tree, T(Red, ..));
    root_black && no_red_red(tree) && black_height(tree).is_some()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_tree() {
        let tree: RBTree<i32> = E;
        assert!(!mem(&1, &tree));
        assert_eq!(to_sorted_vec(&tree), Vec::<i32>::new());
        assert!(is_valid_rbtree(&tree));
    }

    #[test]
    fn test_single_insert() {
        let tree = insert(42, E);
        assert!(mem(&42, &tree));
        assert!(!mem(&0, &tree));
        assert_eq!(to_sorted_vec(&tree), vec![42]);
        assert!(matches!(tree, T(Black, ..)));
        assert!(is_valid_rbtree(&tree));
    }

    #[test]
    fn test_multiple_inserts_sorted_output() {
        // Same sequence as the OCaml example
        let tree = from_iter([5, 3, 7, 1, 4, 6, 8, 2, 9]);
        assert_eq!(to_sorted_vec(&tree), vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }

    #[test]
    fn test_membership() {
        let tree = from_iter([5, 3, 7, 1, 4, 6, 8, 2, 9]);
        for v in 1..=9 {
            assert!(mem(&v, &tree), "expected {v} to be in tree");
        }
        assert!(!mem(&0, &tree));
        assert!(!mem(&10, &tree));
    }

    #[test]
    fn test_duplicate_inserts_ignored() {
        let tree = from_iter([3, 1, 2, 3, 1, 2, 3]);
        assert_eq!(to_sorted_vec(&tree), vec![1, 2, 3]);
    }

    #[test]
    fn test_invariants_after_scrambled_inserts() {
        let tree = from_iter([
            10, 5, 15, 3, 7, 12, 18, 1, 4, 6, 8, 11, 13, 17, 20, 2, 9, 14, 16, 19,
        ]);
        assert!(is_valid_rbtree(&tree));
        assert_eq!(to_sorted_vec(&tree), (1..=20).collect::<Vec<_>>());
    }

    #[test]
    fn test_ascending_insert_stays_balanced() {
        // Ascending insertion is worst-case for naive BSTs; RB tree must stay valid
        let tree = from_iter(1..=100);
        assert!(is_valid_rbtree(&tree));
        assert_eq!(to_sorted_vec(&tree), (1..=100).collect::<Vec<_>>());
    }

    #[test]
    fn test_descending_insert_stays_balanced() {
        let tree = from_iter((1..=50).rev());
        assert!(is_valid_rbtree(&tree));
        assert_eq!(to_sorted_vec(&tree), (1..=50).collect::<Vec<_>>());
    }

    #[test]
    fn test_from_iter_empty() {
        let tree: RBTree<i32> = from_iter(std::iter::empty());
        assert_eq!(tree, E);
        assert!(is_valid_rbtree(&tree));
    }

    #[test]
    fn test_string_keys() {
        let tree = from_iter(["cherry", "apple", "banana", "date"]);
        assert_eq!(
            to_sorted_vec(&tree),
            vec!["apple", "banana", "cherry", "date"]
        );
        assert!(mem(&"banana", &tree));
        assert!(!mem(&"fig", &tree));
        assert!(is_valid_rbtree(&tree));
    }

    #[test]
    fn test_root_always_black_after_each_insert() {
        let mut tree: RBTree<i32> = E;
        for i in 1..=20 {
            tree = insert(i, tree);
            assert!(
                matches!(&tree, T(Black, ..)),
                "root must be black after inserting {i}"
            );
        }
    }
}
