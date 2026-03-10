//! Red-Black Tree with Okasaki's Functional Balancing
//!
//! Implements an immutable, persistent red-black tree using Okasaki's elegant
//! pattern-matching balance function. The tree is purely functional — every
//! insert returns a new tree sharing structure with the old one.

/// Node color in the red-black tree.
#[derive(Debug, Clone, PartialEq)]
pub enum Color {
    Red,
    Black,
}

/// A persistent, immutable red-black tree.
///
/// `E` is the empty leaf. `T` holds color, left subtree, value, right subtree.
/// Uses `Box` for heap allocation of recursive nodes.
#[derive(Debug, Clone, PartialEq)]
pub enum RbTree<T> {
    E,
    T(Color, Box<RbTree<T>>, T, Box<RbTree<T>>),
}

impl<T: Ord + Clone> RbTree<T> {
    /// Creates an empty red-black tree.
    pub fn empty() -> Self {
        RbTree::E
    }

    /// Inserts a value into the tree, returning a new balanced tree.
    ///
    /// Uses Okasaki's functional balancing: insert red, then rebalance bottom-up.
    /// The root is always colored Black after insertion.
    pub fn insert(&self, x: T) -> Self {
        match self.ins(x) {
            RbTree::T(_, a, y, b) => RbTree::T(Color::Black, a, y, b),
            RbTree::E => RbTree::E,
        }
    }

    /// Returns true if `x` is a member of the tree.
    pub fn mem(&self, x: &T) -> bool {
        match self {
            RbTree::E => false,
            RbTree::T(_, a, y, b) => {
                if x == y {
                    true
                } else if x < y {
                    a.mem(x)
                } else {
                    b.mem(x)
                }
            }
        }
    }

    /// Returns a sorted Vec of all elements in the tree (in-order traversal).
    pub fn to_vec(&self) -> Vec<&T> {
        match self {
            RbTree::E => vec![],
            RbTree::T(_, a, v, b) => {
                let mut result = a.to_vec();
                result.push(v);
                result.extend(b.to_vec());
                result
            }
        }
    }

    /// Recursive insert — inserts as Red, relies on `balance` to fix violations.
    fn ins(&self, x: T) -> Self {
        match self {
            RbTree::E => RbTree::T(Color::Red, Box::new(RbTree::E), x, Box::new(RbTree::E)),
            RbTree::T(color, a, y, b) => {
                if x < *y {
                    balance(color.clone(), a.ins(x), y.clone(), *b.clone())
                } else if x > *y {
                    balance(color.clone(), *a.clone(), y.clone(), b.ins(x))
                } else {
                    // x == y: already present, return unchanged
                    RbTree::T(color.clone(), a.clone(), y.clone(), b.clone())
                }
            }
        }
    }
}

/// Okasaki's balance function — fixes red-red violations after insertion.
///
/// Recognizes the four cases where a black node has a red child with a red
/// grandchild, and rotates them all into a single canonical form:
/// a red node with two black children.
///
/// All four cases produce the same output:
///   T(Red, T(Black,a,x,b), y, T(Black,c,z,d))
fn balance<T: Clone>(color: Color, left: RbTree<T>, val: T, right: RbTree<T>) -> RbTree<T> {
    use Color::{Black, Red};
    use RbTree::T;

    // Case 1: Black, (Red (Red a x b) y c), z, d
    if let (Black, T(Red, ll, lv, lr), T(Red, lrl, lrv, lrr)) =
        (&color, &left, lr.as_ref().map(|_| ()).ok_or(()).ok())
    {
        let _ = (ll, lv, lr, lrl, lrv, lrr); // suppress unused
    }

    // Use a clean tuple-match approach mirroring OCaml's elegant pattern
    match (color, left, right) {
        // Case 1: left-left red violation
        (Black, T(Red, ll, lv, lr), right) if matches!(*ll, T(Red, _, _, _)) => {
            if let T(Red, a, x, b) = *ll {
                let c = lr;
                let d = Box::new(right);
                return T(
                    Red,
                    Box::new(T(Black, a, x, b)),
                    lv,
                    Box::new(T(Black, c, val, d)),
                );
            }
            unreachable!()
        }

        // Case 2: left-right red violation
        (Black, T(Red, ll, lv, lr), right) if matches!(*lr, T(Red, _, _, _)) => {
            if let T(Red, b, y, c) = *lr {
                let a = ll;
                let d = Box::new(right);
                return T(
                    Red,
                    Box::new(T(Black, a, lv, b)),
                    y,
                    Box::new(T(Black, c, val, d)),
                );
            }
            unreachable!()
        }

        // Case 3: right-left red violation
        (Black, left, T(Red, rl, rv, rr)) if matches!(*rl, T(Red, _, _, _)) => {
            if let T(Red, b, y, c) = *rl {
                let a = Box::new(left);
                let d = rr;
                return T(
                    Red,
                    Box::new(T(Black, a, val, b)),
                    y,
                    Box::new(T(Black, c, rv, d)),
                );
            }
            unreachable!()
        }

        // Case 4: right-right red violation
        (Black, left, T(Red, rl, rv, rr)) if matches!(*rr, T(Red, _, _, _)) => {
            if let T(Red, c, z, d) = *rr {
                let a = Box::new(left);
                let b = rl;
                return T(
                    Red,
                    Box::new(T(Black, a, val, b)),
                    rv,
                    Box::new(T(Black, c, z, d)),
                );
            }
            unreachable!()
        }

        // Default: no violation, just wrap
        (color, left, right) => T(color, Box::new(left), val, Box::new(right)),
    }
}

/// Builds a red-black tree from an iterator.
pub fn from_iter<T: Ord + Clone>(iter: impl IntoIterator<Item = T>) -> RbTree<T> {
    iter.into_iter()
        .fold(RbTree::empty(), |tree, x| tree.insert(x))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_tree() {
        let t: RbTree<i32> = RbTree::empty();
        assert_eq!(t.to_vec(), Vec::<&i32>::new());
        assert!(!t.mem(&42));
    }

    #[test]
    fn test_single_element() {
        let t = RbTree::empty().insert(5);
        assert_eq!(t.to_vec(), vec![&5]);
        assert!(t.mem(&5));
        assert!(!t.mem(&3));
    }

    #[test]
    fn test_sorted_output() {
        // Okasaki's example: insert in arbitrary order, to_vec gives sorted
        let t = from_iter([5, 3, 7, 1, 4, 6, 8, 2, 9]);
        assert_eq!(t.to_vec(), vec![&1, &2, &3, &4, &5, &6, &7, &8, &9]);
    }

    #[test]
    fn test_membership() {
        let t = from_iter([5, 3, 7, 1, 4, 6, 8, 2, 9]);
        for i in 1..=9 {
            assert!(t.mem(&i), "expected {i} to be in tree");
        }
        assert!(!t.mem(&0));
        assert!(!t.mem(&10));
    }

    #[test]
    fn test_duplicate_insert() {
        let t = from_iter([3, 3, 3, 1, 2]);
        // Duplicates are ignored — set semantics
        assert_eq!(t.to_vec(), vec![&1, &2, &3]);
    }

    #[test]
    fn test_root_is_black() {
        // Red-black invariant: root must be Black
        let t = from_iter([10, 5, 15]);
        match &t {
            RbTree::T(Color::Black, _, _, _) => {}
            _ => panic!("root must be Black"),
        }
    }

    #[test]
    fn test_left_left_balance() {
        // Triggers case 1: inserting in descending order forces left-left rotation
        let t = from_iter([3, 2, 1]);
        assert_eq!(t.to_vec(), vec![&1, &2, &3]);
        match &t {
            RbTree::T(Color::Black, _, _, _) => {}
            _ => panic!("root must be Black"),
        }
    }

    #[test]
    fn test_right_right_balance() {
        // Triggers case 4: inserting in ascending order forces right-right rotation
        let t = from_iter([1, 2, 3]);
        assert_eq!(t.to_vec(), vec![&1, &2, &3]);
    }

    #[test]
    fn test_large_insert() {
        let t = from_iter(0..100);
        let expected: Vec<i32> = (0..100).collect();
        let got: Vec<i32> = t.to_vec().into_iter().copied().collect();
        assert_eq!(got, expected);
    }

    #[test]
    fn test_immutability() {
        // Each insert returns a new tree; old tree is unchanged
        let t0 = RbTree::empty();
        let t1 = t0.insert(5);
        let t2 = t1.insert(3);
        assert_eq!(t0.to_vec(), Vec::<&i32>::new());
        assert_eq!(t1.to_vec(), vec![&5]);
        assert_eq!(t2.to_vec(), vec![&3, &5]);
    }
}
