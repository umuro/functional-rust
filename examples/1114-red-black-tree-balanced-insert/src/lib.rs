#![allow(clippy::all)]
#[derive(Debug, Clone, PartialEq)]
pub enum Color {
    Red,
    Black,
}

/// A persistent, functional red-black tree using Okasaki's balancing scheme.
#[derive(Debug, Clone, PartialEq)]
pub enum RBTree<T> {
    E,
    /// T(color, left, value, right)
    T(Color, Box<RBTree<T>>, T, Box<RBTree<T>>),
}

use Color::{Black, Red};
use RBTree::{E, T};

/// Balance a black node with a double-red violation in one of its four grandchild positions.
///
/// Implements Okasaki's elegant 4-case functional balancing. Rust cannot pattern-match
/// through `Box` in a single arm, so we use nested `match *box_value` to unbox and inspect.
///
/// All four cases produce the same shape: `T(Red, T(Black,a,x,b), y, T(Black,c,z,d))`
pub fn balance<T>(color: Color, left: RBTree<T>, value: T, right: RBTree<T>) -> RBTree<T> {
    match (color, left, value, right) {
        // Cases 1 & 2: left child is Red — look for double-red grandchild on the left side
        (Black, T(Red, ll, lv, lr), v, r) => match *ll {
            // Case 1: left-left grandchild is Red (left-left rotation)
            T(Red, a, x, b) => T(
                Red,
                Box::new(T(Black, a, x, b)),
                lv,
                Box::new(T(Black, lr, v, Box::new(r))),
            ),
            // Case 2: left-right grandchild may be Red (left-right rotation)
            ll => match *lr {
                T(Red, b, y, c) => T(
                    Red,
                    Box::new(T(Black, Box::new(ll), lv, b)),
                    y,
                    Box::new(T(Black, c, v, Box::new(r))),
                ),
                lr => T(
                    Black,
                    Box::new(T(Red, Box::new(ll), lv, Box::new(lr))),
                    v,
                    Box::new(r),
                ),
            },
        },

        // Cases 3 & 4: right child is Red — look for double-red grandchild on the right side
        (Black, l, v, T(Red, rl, rv, rr)) => match *rl {
            // Case 3: right-left grandchild is Red (right-left rotation)
            T(Red, b, y, c) => T(
                Red,
                Box::new(T(Black, Box::new(l), v, b)),
                y,
                Box::new(T(Black, c, rv, rr)),
            ),
            // Case 4: right-right grandchild may be Red (right-right rotation)
            rl => match *rr {
                T(Red, c, z, d) => T(
                    Red,
                    Box::new(T(Black, Box::new(l), v, Box::new(rl))),
                    rv,
                    Box::new(T(Black, c, z, d)),
                ),
                rr => T(
                    Black,
                    Box::new(l),
                    v,
                    Box::new(T(Red, Box::new(rl), rv, Box::new(rr))),
                ),
            },
        },

        // Default: no double-red violation, wrap as-is
        (color, l, v, r) => T(color, Box::new(l), v, Box::new(r)),
    }
}

/// Insert a value into the tree (functional, returns a new tree).
///
/// The inner `ins` returns a potentially-red root; the outer match forces it black,
/// maintaining the red-black invariant that the root is always black.
pub fn insert<T: Ord>(value: T, tree: RBTree<T>) -> RBTree<T> {
    fn ins<T: Ord>(value: T, tree: RBTree<T>) -> RBTree<T> {
        match tree {
            E => T(Red, Box::new(E), value, Box::new(E)),
            T(color, left, v, right) => {
                if value < v {
                    balance(color, ins(value, *left), v, *right)
                } else if value > v {
                    balance(color, *left, v, ins(value, *right))
                } else {
                    T(color, left, v, right)
                }
            }
        }
    }
    // Force the root black after insertion
    match ins(value, tree) {
        T(_, left, v, right) => T(Black, left, v, right),
        E => E,
    }
}

/// O(log n) membership test.
pub fn mem<T: Ord>(value: &T, tree: &RBTree<T>) -> bool {
    match tree {
        E => false,
        T(_, left, v, right) => {
            if value == v {
                true
            } else if value < v {
                mem(value, left)
            } else {
                mem(value, right)
            }
        }
    }
}

/// In-order traversal — returns all elements in sorted order.
///
/// Recursive translation: mirrors the OCaml `to_list` directly.
pub fn to_list<T: Clone>(tree: &RBTree<T>) -> Vec<T> {
    match tree {
        E => vec![],
        T(_, left, v, right) => {
            let mut result = to_list(left);
            result.push(v.clone());
            result.extend(to_list(right));
            result
        }
    }
}

/// Build a tree from an iterator of values.
///
/// Idiomatic Rust: uses `fold` over the iterator, mirrors OCaml's `List.fold_left`.
pub fn from_iter<T: Ord>(iter: impl Iterator<Item = T>) -> RBTree<T> {
    iter.fold(E, |tree, v| insert(v, tree))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_tree() {
        let t = RBTree::<i32>::E;
        assert!(!mem(&5, &t));
        assert_eq!(to_list(&t), Vec::<i32>::new());
    }

    #[test]
    fn test_single_insert() {
        let t = insert(42, RBTree::E);
        assert!(mem(&42, &t));
        assert!(!mem(&0, &t));
        assert_eq!(to_list(&t), vec![42]);
    }

    #[test]
    fn test_multiple_inserts_produce_sorted_output() {
        let t = from_iter([5, 3, 7, 1, 4, 6, 8, 2, 9].into_iter());
        assert_eq!(to_list(&t), vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }

    #[test]
    fn test_membership_after_bulk_insert() {
        let t = from_iter([5, 3, 7, 1, 4, 6, 8, 2, 9].into_iter());
        for i in 1..=9 {
            assert!(mem(&i, &t), "expected {i} to be in tree");
        }
        assert!(!mem(&0, &t));
        assert!(!mem(&10, &t));
    }

    #[test]
    fn test_root_is_always_black() {
        let values = [5, 3, 7, 1, 4, 6, 8, 2, 9];
        let mut t = RBTree::E;
        for v in values {
            t = insert(v, t);
            match &t {
                RBTree::T(Color::Black, _, _, _) => (),
                _ => panic!("root must be Black after every insert"),
            }
        }
    }

    #[test]
    fn test_duplicate_insert_ignored() {
        let t = from_iter([5, 3, 7, 5, 3].into_iter());
        assert_eq!(to_list(&t), vec![3, 5, 7]);
    }

    #[test]
    fn test_balance_case1_left_left_red() {
        // Manually construct Black(Red(Red(E,1,E), 2, E), 3, E) and balance it
        let inner = T(Red, Box::new(E), 1, Box::new(E));
        let left = T(Red, Box::new(inner), 2, Box::new(E));
        let result = balance(Black, left, 3, E);
        // Should become Red(Black(E,1,E), 2, Black(E,3,E))
        assert_eq!(
            result,
            T(
                Red,
                Box::new(T(Black, Box::new(E), 1, Box::new(E))),
                2,
                Box::new(T(Black, Box::new(E), 3, Box::new(E)))
            )
        );
    }

    #[test]
    fn test_balance_case4_right_right_red() {
        // Manually construct Black(E, 1, Red(E, 2, Red(E, 3, E))) and balance it
        let inner = T(Red, Box::new(E), 3, Box::new(E));
        let right = T(Red, Box::new(E), 2, Box::new(inner));
        let result = balance(Black, E, 1, right);
        // Should become Red(Black(E,1,E), 2, Black(E,3,E))
        assert_eq!(
            result,
            T(
                Red,
                Box::new(T(Black, Box::new(E), 1, Box::new(E))),
                2,
                Box::new(T(Black, Box::new(E), 3, Box::new(E)))
            )
        );
    }
}
