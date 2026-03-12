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

use Color::*;
use RBTree::*;

/// Balance function from Okasaki's paper.
/// Takes a tuple (color, left, value, right) and returns a balanced tree.
pub fn balance<T>(color: Color, left: RBTree<T>, value: T, right: RBTree<T>) -> RBTree<T> {
    match (color, left, value, right) {
        (Black, T(Red, T(Red, a, x, b), y, c), z, d) |
        (Black, T(Red, a, x, T(Red, b, y, c)), z, d) |
        (Black, a, x, T(Red, T(Red, b, y, c), z, d)) |
        (Black, a, x, T(Red, b, y, T(Red, c, z, d))) => {
            T(Red, Box::new(T(Black, a, x, b)), y, Box::new(T(Black, c, z, d)))
        }
        (color, a, x, b) => T(color, Box::new(a), x, Box::new(b)),
    }
}

/// Insert a value into the tree (functional, persistent).
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
    match ins(value, tree) {
        T(_, left, v, right) => T(Black, left, v, right),
        E => E,
    }
}

/// Membership test.
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

/// Convert tree to sorted list (in-order traversal).
pub fn to_list<T: Clone>(tree: &RBTree<T>) -> Vec<T> {
    match tree {
        E => vec![],
        T(_, left, v, right) => {
            let mut list = to_list(left);
            list.push(v.clone());
            list.extend(to_list(right));
            list
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_tree() {
        let t = RBTree::<i32>::E;
        assert!(!mem(&5, &t));
        assert_eq!(to_list(&t), vec![]);
    }

    #[test]
    fn test_single_insert() {
        let t = insert(5, RBTree::E);
        assert!(mem(&5, &t));
        assert!(!mem(&3, &t));
        assert_eq!(to_list(&t), vec![5]);
    }

    #[test]
    fn test_multiple_inserts() {
        let t = insert(5, RBTree::E);
        let t = insert(3, t);
        let t = insert(7, t);
        let t = insert(1, t);
        assert!(mem(&5, &t));
        assert!(mem(&3, &t));
        assert!(mem(&7, &t));
        assert!(mem(&1, &t));
        assert!(!mem(&9, &t));
        let list = to_list(&t);
        assert_eq!(list, vec![1, 3, 5, 7]);
    }

    #[test]
    fn test_balance_red_red_left() {
        // Build a tree that triggers balance case
        let t = T(Black,
            Box::new(T(Red,
                Box::new(T(Red,
                    Box::new(E),
                    1,
                    Box::new(E))),
                2,
                Box::new(E))),
            3,
            Box::new(E));
        let balanced = balance(Black, *Box::new(t), 4, E);
        // After balancing, root should be Red
        match balanced {
            T(Color::Red, _, _, _) => (),
            _ => panic!("Root should be Red after balance"),
        }
    }
}