#[derive(Debug, Clone, PartialEq)]
pub enum Color {
    Red,
    Black,
}

#[derive(Debug, Clone, PartialEq)]
pub enum RBTree<T> {
    E,
    T(Color, Box<RBTree<T>>, T, Box<RBTree<T>>),
}

use Color::{Black, Red};
use RBTree::{E, T};

/// Balance function from Okasaki's "Purely Functional Data Structures".
/// Handles all 4 cases of red-red violations after insertion.
/// Takes ownership of color, left, value, right and returns a balanced tree.
pub fn balance<T>(color: Color, left: RBTree<T>, value: T, right: RBTree<T>) -> RBTree<T> {
    match (color, left, value, right) {
        // Case 1: red-red on left-left
        (Black, T(Red, a, x, b), y, d) if matches!(*a, T(Red, ..)) => {
            let T(Red, aa, xx, bb) = *a else {
                unreachable!()
            };
            T(
                Red,
                Box::new(T(Black, aa, xx, bb)),
                x,
                Box::new(T(Black, b, y, Box::new(d))),
            )
        }
        // Case 2: red-red on left-right
        (Black, T(Red, a, x, b), y, d) if matches!(*b, T(Red, ..)) => {
            let T(Red, bb, yy, cc) = *b else {
                unreachable!()
            };
            T(
                Red,
                Box::new(T(Black, a, x, bb)),
                yy,
                Box::new(T(Black, cc, y, Box::new(d))),
            )
        }
        // Case 3: red-red on right-left
        (Black, a, x, T(Red, b, y, d)) if matches!(*b, T(Red, ..)) => {
            let T(Red, bb, yy, cc) = *b else {
                unreachable!()
            };
            T(
                Red,
                Box::new(T(Black, Box::new(a), x, bb)),
                yy,
                Box::new(T(Black, cc, y, d)),
            )
        }
        // Case 4: red-red on right-right
        (Black, a, x, T(Red, b, y, d)) if matches!(*d, T(Red, ..)) => {
            let T(Red, cc, zz, dd) = *d else {
                unreachable!()
            };
            T(
                Red,
                Box::new(T(Black, Box::new(a), x, b)),
                y,
                Box::new(T(Black, cc, zz, dd)),
            )
        }
        // Default: no rebalancing needed
        (color, a, x, b) => T(color, Box::new(a), x, Box::new(b)),
    }
}

/// Insert a value into the red-black tree, maintaining balance invariants.
/// Functional and persistent — returns a new tree, original is consumed.
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
                    // Duplicate: return unchanged
                    T(color, left, v, right)
                }
            }
        }
    }
    // Root is always black
    match ins(value, tree) {
        T(_, left, v, right) => T(Black, left, v, right),
        E => E,
    }
}

/// Check whether a value is a member of the tree.
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

/// In-order traversal: returns all values in sorted order.
pub fn to_list<T: Clone>(tree: &RBTree<T>) -> Vec<T> {
    match tree {
        E => vec![],
        T(_, left, v, right) => {
            let mut acc = to_list(left);
            acc.push(v.clone());
            acc.extend(to_list(right));
            acc
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_tree(values: &[i32]) -> RBTree<i32> {
        values.iter().fold(RBTree::E, |t, &x| insert(x, t))
    }

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
    fn test_sorted_output() {
        let t = build_tree(&[5, 3, 7, 1, 4, 6, 8, 2, 9]);
        assert_eq!(to_list(&t), vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }

    #[test]
    fn test_membership_after_inserts() {
        let t = build_tree(&[5, 3, 7, 1, 4, 6, 8, 2, 9]);
        for i in 1..=9 {
            assert!(mem(&i, &t), "expected {i} in tree");
        }
        assert!(!mem(&0, &t));
        assert!(!mem(&10, &t));
    }

    #[test]
    fn test_duplicate_insert() {
        let t = build_tree(&[3, 3, 3]);
        assert_eq!(to_list(&t), vec![3]);
    }

    #[test]
    fn test_red_black_root_is_black() {
        let t = build_tree(&[1, 2, 3]);
        match &t {
            T(Color::Black, _, _, _) => (),
            _ => panic!("root must be black"),
        }
    }
}
