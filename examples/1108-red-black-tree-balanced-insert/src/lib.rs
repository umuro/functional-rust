#![allow(clippy::all)]
/// Red-Black Tree with Okasaki's functional balancing.
///
/// Uses persistent, immutable nodes via `Box`. Each insert creates a new path
/// from root to insertion point; unchanged subtrees are shared via `Clone`.
/// The `balance` function implements Okasaki's elegant 4-case rotation.
#[derive(Debug, Clone, PartialEq)]
pub enum Color {
    Red,
    Black,
}

/// A persistent red-black tree node.
///
/// `Box` is used so the recursive type has known size. We clone subtrees only
/// when restructuring during balance — commented at each site.
#[derive(Debug, Clone, PartialEq)]
pub enum RbTree<T> {
    Empty,
    Node(Color, Box<RbTree<T>>, T, Box<RbTree<T>>),
}

use Color::{Black, Red};
use RbTree::{Empty, Node};

/// Okasaki's 4-case balance: detects any double-red violation and fixes it.
///
/// All four cases produce the same output structure:
///   Red( Black(a,x,b), y, Black(c,z,d) )
/// This is purely structural — no mutation, returns a new tree.
fn balance<T: Clone>(color: Color, left: RbTree<T>, val: T, right: RbTree<T>) -> RbTree<T> {
    // Case 1: left-left red violation
    // Ergonomics: ll,lr: &Box<RbTree<T>>; lv,v: &T; r: &RbTree<T>
    if let (Black, Node(Red, ll, lv, lr), v, r) = (&color, &left, &val, &right) {
        if let Node(Red, a, x, b) = ll.as_ref() {
            return Node(
                Red,
                Box::new(Node(Black, a.clone(), (*x).clone(), b.clone())), // clone: new path node
                (*lv).clone(),
                Box::new(Node(Black, lr.clone(), (*v).clone(), Box::new(r.clone()))), // clone: new path node
            );
        }
    }
    // Case 2: left-right red violation
    if let (Black, Node(Red, la, lv, lr), v, r) = (&color, &left, &val, &right) {
        if let Node(Red, b, y, c) = lr.as_ref() {
            return Node(
                Red,
                Box::new(Node(Black, la.clone(), (*lv).clone(), b.clone())), // clone: new path node
                (*y).clone(),
                Box::new(Node(Black, c.clone(), (*v).clone(), Box::new(r.clone()))), // clone: new path node
            );
        }
    }
    // Case 3: right-left red violation
    // Ergonomics: l: &RbTree<T>; rl,rr: &Box<RbTree<T>>; rv,v: &T
    if let (Black, l, v, Node(Red, rl, rv, rr)) = (&color, &left, &val, &right) {
        if let Node(Red, b, y, c) = rl.as_ref() {
            return Node(
                Red,
                Box::new(Node(Black, Box::new(l.clone()), (*v).clone(), b.clone())), // clone: new path node
                (*y).clone(),
                Box::new(Node(Black, c.clone(), (*rv).clone(), rr.clone())), // clone: new path node
            );
        }
    }
    // Case 4: right-right red violation
    if let (Black, l, v, Node(Red, rl, rv, rr)) = (&color, &left, &val, &right) {
        if let Node(Red, c, z, d) = rr.as_ref() {
            return Node(
                Red,
                Box::new(Node(Black, Box::new(l.clone()), (*v).clone(), rl.clone())), // clone: new path node
                (*rv).clone(),
                Box::new(Node(Black, c.clone(), (*z).clone(), d.clone())), // clone: new path node
            );
        }
    }
    // No violation — wrap as-is
    Node(color, Box::new(left), val, Box::new(right))
}

/// Insert `val` into a red-black tree, returning a new balanced tree.
///
/// The root is always painted black after insertion (Okasaki's invariant).
/// Returns the original tree unchanged if `val` is already present.
pub fn insert<T: Ord + Clone>(val: T, tree: &RbTree<T>) -> RbTree<T> {
    fn ins<T: Ord + Clone>(val: &T, tree: &RbTree<T>) -> RbTree<T> {
        match tree {
            Empty => Node(Red, Box::new(Empty), val.clone(), Box::new(Empty)),
            Node(color, left, y, right) => {
                if val < y {
                    balance(color.clone(), ins(val, left), y.clone(), *right.clone())
                } else if val > y {
                    balance(color.clone(), *left.clone(), y.clone(), ins(val, right))
                } else {
                    // Duplicate: return unchanged (clone the current node)
                    tree.clone()
                }
            }
        }
    }

    // Paint root black
    match ins(&val, tree) {
        Node(_, left, v, right) => Node(Black, left, v, right),
        Empty => Empty,
    }
}

/// Build a red-black tree from an iterator.
pub fn from_iter<T: Ord + Clone>(iter: impl IntoIterator<Item = T>) -> RbTree<T> {
    iter.into_iter().fold(Empty, |tree, val| insert(val, &tree))
}

/// Check if `val` is a member of the tree.
pub fn member<T: Ord>(val: &T, tree: &RbTree<T>) -> bool {
    match tree {
        Empty => false,
        Node(_, left, y, right) => {
            if val == y {
                true
            } else if val < y {
                member(val, left)
            } else {
                member(val, right)
            }
        }
    }
}

/// Collect tree values in sorted order (in-order traversal).
pub fn to_sorted_vec<T: Clone>(tree: &RbTree<T>) -> Vec<T> {
    match tree {
        Empty => vec![],
        Node(_, left, v, right) => {
            let mut result = to_sorted_vec(left);
            result.push(v.clone());
            result.extend(to_sorted_vec(right));
            result
        }
    }
}

/// Verify red-black invariants: no consecutive reds, equal black height on all paths.
///
/// Returns `Ok(black_height)` or `Err(description)`.
pub fn check_invariants<T: std::fmt::Debug>(tree: &RbTree<T>) -> Result<usize, String> {
    match tree {
        Empty => Ok(1), // Empty counts as black
        Node(color, left, _, right) => {
            // Red node must not have red children
            if *color == Red {
                if let Node(Red, _, _, _) = left.as_ref() {
                    return Err("Red node has red left child".into());
                }
                if let Node(Red, _, _, _) = right.as_ref() {
                    return Err("Red node has red right child".into());
                }
            }
            let lh = check_invariants(left)?;
            let rh = check_invariants(right)?;
            if lh != rh {
                return Err(format!("Black height mismatch: left={lh}, right={rh}"));
            }
            Ok(lh + usize::from(*color == Black))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_tree() {
        let tree: RbTree<i32> = Empty;
        assert_eq!(to_sorted_vec(&tree), Vec::<i32>::new());
        assert!(!member(&1, &tree));
    }

    #[test]
    fn test_single_insert() {
        let tree = insert(42, &Empty);
        assert_eq!(to_sorted_vec(&tree), vec![42]);
        assert!(member(&42, &tree));
        assert!(!member(&0, &tree));
    }

    #[test]
    fn test_sorted_output_after_inserts() {
        // Insert in arbitrary order; to_sorted_vec must return sorted
        let tree = from_iter([5, 3, 7, 1, 4, 6, 8, 2, 9]);
        assert_eq!(to_sorted_vec(&tree), vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }

    #[test]
    fn test_member_all_inserted() {
        let vals = [5, 3, 7, 1, 4, 6, 8, 2, 9];
        let tree = from_iter(vals);
        assert!(vals.iter().all(|v| member(v, &tree)));
        assert!(!member(&0, &tree));
        assert!(!member(&10, &tree));
    }

    #[test]
    fn test_duplicate_insert_no_duplicates() {
        let tree = from_iter([3, 3, 3, 1, 2, 1]);
        assert_eq!(to_sorted_vec(&tree), vec![1, 2, 3]);
    }

    #[test]
    fn test_rbtree_invariants_after_inserts() {
        let tree = from_iter([5, 3, 7, 1, 4, 6, 8, 2, 9]);
        assert!(check_invariants(&tree).is_ok(), "RB invariants must hold");
    }

    #[test]
    fn test_rbtree_invariants_ascending_input() {
        // Ascending insertion is the worst case for naive BSTs — triggers all 4 balance cases
        let tree = from_iter(1..=20);
        assert!(check_invariants(&tree).is_ok());
        assert_eq!(to_sorted_vec(&tree), (1..=20).collect::<Vec<_>>());
    }

    #[test]
    fn test_root_is_always_black() {
        let tree = insert(1, &Empty);
        match &tree {
            Node(color, _, _, _) => assert_eq!(*color, Black),
            Empty => panic!("expected a node"),
        }
    }
}
