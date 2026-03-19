#![allow(clippy::all)]
// Red-Black Tree — Balanced Insert (Okasaki)
//
// A persistent (immutable) red-black tree following Okasaki's functional
// balancing. Four rotation cases collapse into one rebalanced shape.
// This version takes ownership in `balance` to avoid unnecessary cloning
// in the restructured subtrees.

/// Node color in a red-black tree.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    Red,
    Black,
}

/// A persistent red-black tree parameterized over `T`.
///
/// `Box` gives heap-allocated children — Rust's analog to OCaml's
/// garbage-collected recursive variant.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RBTree<T> {
    Empty,
    Node(Color, Box<RBTree<T>>, T, Box<RBTree<T>>),
}

use Color::{Black, Red};
use RBTree::{Empty, Node};

// ── Solution 1: Idiomatic Rust — Okasaki balance via ownership transfer ──

impl<T: Ord + Clone> RBTree<T> {
    /// Creates an empty red-black tree.
    pub fn new() -> Self {
        Empty
    }

    /// Okasaki's balance — detects four red-red violations and rotates to:
    /// ```text
    ///        y(R)
    ///       /    \
    ///     x(B)   z(B)
    ///    / \     / \
    ///   a   b   c   d
    /// ```
    ///
    /// Takes ownership of subtrees so the restructured tree reuses allocations
    /// rather than cloning.
    fn balance(color: Color, left: Self, value: T, right: Self) -> Self {
        match (color, left, right) {
            // Case 1: left-left red — Black(Red(Red(a,x,b),y,c),z,d)
            (Black, Node(Red, ll, y, c), d) if matches!(*ll, Node(Red, _, _, _)) => {
                let Node(Red, a, x, b) = *ll else {
                    unreachable!()
                };
                Node(
                    Red,
                    Box::new(Node(Black, a, x, b)),
                    y,
                    Box::new(Node(Black, c, value, Box::new(d))),
                )
            }

            // Case 2: left-right red — Black(Red(a,x,Red(b,y,c)),z,d)
            (Black, Node(Red, a, x, lr), d) if matches!(*lr, Node(Red, _, _, _)) => {
                let Node(Red, b, y, c) = *lr else {
                    unreachable!()
                };
                Node(
                    Red,
                    Box::new(Node(Black, a, x, b)),
                    y,
                    Box::new(Node(Black, c, value, Box::new(d))),
                )
            }

            // Case 3: right-left red — Black(a,x,Red(Red(b,y,c),z,d))
            (Black, a, Node(Red, rl, z, d)) if matches!(*rl, Node(Red, _, _, _)) => {
                let Node(Red, b, y, c) = *rl else {
                    unreachable!()
                };
                Node(
                    Red,
                    Box::new(Node(Black, Box::new(a), value, b)),
                    y,
                    Box::new(Node(Black, c, z, d)),
                )
            }

            // Case 4: right-right red — Black(a,x,Red(b,y,Red(c,z,d)))
            (Black, a, Node(Red, b, y, rr)) if matches!(*rr, Node(Red, _, _, _)) => {
                let Node(Red, c, z, d) = *rr else {
                    unreachable!()
                };
                Node(
                    Red,
                    Box::new(Node(Black, Box::new(a), value, b)),
                    y,
                    Box::new(Node(Black, c, z, d)),
                )
            }

            // No violation — reassemble as-is
            (c, left, right) => Node(c, Box::new(left), value, Box::new(right)),
        }
    }

    /// Inserts a value, returning a new tree (persistent — original unchanged).
    ///
    /// The root is always painted black after insertion.
    pub fn insert(&self, x: T) -> Self {
        // Recursive helper — may produce a red root
        fn ins<T: Ord + Clone>(tree: &RBTree<T>, x: T) -> RBTree<T> {
            match tree {
                Empty => Node(Red, Box::new(Empty), x, Box::new(Empty)),
                Node(color, a, y, b) => match x.cmp(y) {
                    std::cmp::Ordering::Less => {
                        RBTree::balance(*color, ins(a, x), y.clone(), b.as_ref().clone())
                    }
                    std::cmp::Ordering::Greater => {
                        RBTree::balance(*color, a.as_ref().clone(), y.clone(), ins(b, x))
                    }
                    // Duplicate — return tree unchanged
                    std::cmp::Ordering::Equal => tree.clone(),
                },
            }
        }

        // Paint root black (invariant: root is always black)
        match ins(self, x) {
            Node(_, a, y, b) => Node(Black, a, y, b),
            Empty => Empty,
        }
    }

    // ── Solution 2: Membership — recursive BST search, mirrors OCaml `mem` ──

    /// Checks whether a value exists in the tree — O(log n).
    pub fn contains(&self, x: &T) -> bool {
        match self {
            Empty => false,
            Node(_, a, y, b) => match x.cmp(y) {
                std::cmp::Ordering::Equal => true,
                std::cmp::Ordering::Less => a.contains(x),
                std::cmp::Ordering::Greater => b.contains(x),
            },
        }
    }

    // ── Solution 3: In-order traversal — collects sorted elements ──

    /// Collects elements in sorted order via in-order traversal.
    pub fn to_sorted_vec(&self) -> Vec<&T> {
        fn collect<'a, T>(tree: &'a RBTree<T>, acc: &mut Vec<&'a T>) {
            if let Node(_, a, v, b) = tree {
                collect(a, acc);
                acc.push(v);
                collect(b, acc);
            }
        }
        let mut result = Vec::new();
        collect(self, &mut result);
        result
    }

    /// Returns the number of elements in the tree.
    pub fn len(&self) -> usize {
        match self {
            Empty => 0,
            Node(_, a, _, b) => 1 + a.len() + b.len(),
        }
    }

    /// Returns true if the tree is empty.
    pub fn is_empty(&self) -> bool {
        matches!(self, Empty)
    }

    /// Returns the color of the root, or `None` for an empty tree.
    pub fn root_color(&self) -> Option<Color> {
        match self {
            Empty => None,
            Node(c, _, _, _) => Some(*c),
        }
    }
}

impl<T: Ord + Clone> Default for RBTree<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// Builds a tree from an iterator — mirrors OCaml's `List.fold_left insert`.
impl<T: Ord + Clone> FromIterator<T> for RBTree<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        iter.into_iter().fold(RBTree::new(), |t, x| t.insert(x))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Validates red-black invariants, returning the black-height or an error.
    fn validate_rb<T: Ord + Clone>(tree: &RBTree<T>) -> Result<usize, String> {
        match tree {
            Empty => Ok(1), // empty nodes count as black
            Node(color, left, _, right) => {
                // Red nodes must not have red children
                if *color == Red {
                    if matches!(left.as_ref(), Node(Red, _, _, _)) {
                        return Err("red node has red left child".into());
                    }
                    if matches!(right.as_ref(), Node(Red, _, _, _)) {
                        return Err("red node has red right child".into());
                    }
                }
                let lh = validate_rb(left)?;
                let rh = validate_rb(right)?;
                if lh != rh {
                    return Err(format!("black-height mismatch: left={lh}, right={rh}"));
                }
                Ok(if *color == Black { lh + 1 } else { lh })
            }
        }
    }

    #[test]
    fn test_empty_tree() {
        let tree: RBTree<i32> = RBTree::new();
        assert!(tree.is_empty());
        assert_eq!(tree.len(), 0);
        assert_eq!(tree.to_sorted_vec(), Vec::<&i32>::new());
        assert!(!tree.contains(&1));
    }

    #[test]
    fn test_single_insert() {
        let tree = RBTree::new().insert(42);
        assert!(!tree.is_empty());
        assert_eq!(tree.len(), 1);
        assert!(tree.contains(&42));
        assert!(!tree.contains(&0));
        assert_eq!(tree.root_color(), Some(Black));
    }

    #[test]
    fn test_multiple_inserts_sorted_output() {
        // Mirrors OCaml: insert [5;3;7;1;4;6;8;2;9]
        let tree: RBTree<i32> = [5, 3, 7, 1, 4, 6, 8, 2, 9].into_iter().collect();
        assert_eq!(tree.len(), 9);
        assert_eq!(
            tree.to_sorted_vec(),
            vec![&1, &2, &3, &4, &5, &6, &7, &8, &9]
        );
    }

    #[test]
    fn test_duplicate_inserts_ignored() {
        let tree: RBTree<i32> = [3, 1, 2, 1, 3, 2].into_iter().collect();
        assert_eq!(tree.len(), 3);
        assert_eq!(tree.to_sorted_vec(), vec![&1, &2, &3]);
    }

    #[test]
    fn test_membership() {
        let tree: RBTree<i32> = [10, 20, 30, 40, 50].into_iter().collect();
        assert!(tree.contains(&10));
        assert!(tree.contains(&30));
        assert!(tree.contains(&50));
        assert!(!tree.contains(&15));
        assert!(!tree.contains(&0));
        assert!(!tree.contains(&100));
    }

    #[test]
    fn test_root_always_black() {
        for n in 1..=20 {
            let tree: RBTree<i32> = (1..=n).collect();
            assert_eq!(
                tree.root_color(),
                Some(Black),
                "root not black after inserting 1..={n}"
            );
        }
    }

    #[test]
    fn test_rb_invariants_hold() {
        let orders: Vec<Vec<i32>> = vec![
            (1..=15).collect(),
            (1..=15).rev().collect(),
            vec![8, 4, 12, 2, 6, 10, 14, 1, 3, 5, 7, 9, 11, 13, 15],
            vec![5, 3, 7, 1, 4, 6, 8, 2, 9],
        ];
        for order in &orders {
            let tree: RBTree<i32> = order.iter().copied().collect();
            assert!(
                validate_rb(&tree).is_ok(),
                "invariants violated for {order:?}: {}",
                validate_rb(&tree).unwrap_err()
            );
        }
    }

    #[test]
    fn test_persistence() {
        let t1: RBTree<i32> = [3, 1, 5].into_iter().collect();
        let t2 = t1.insert(2);
        let t3 = t1.insert(4);

        assert_eq!(t1.len(), 3);
        assert_eq!(t2.len(), 4);
        assert_eq!(t3.len(), 4);

        assert!(!t1.contains(&2));
        assert!(t2.contains(&2));
        assert!(!t2.contains(&4));
        assert!(t3.contains(&4));
        assert!(!t3.contains(&2));
    }

    #[test]
    fn test_string_keys() {
        let tree: RBTree<String> = ["cherry", "apple", "banana", "date"]
            .into_iter()
            .map(String::from)
            .collect();
        assert_eq!(tree.len(), 4);
        assert!(tree.contains(&"banana".to_string()));
        assert!(!tree.contains(&"elderberry".to_string()));
        let sorted: Vec<&str> = tree.to_sorted_vec().iter().map(|s| s.as_str()).collect();
        assert_eq!(sorted, vec!["apple", "banana", "cherry", "date"]);
    }

    #[test]
    fn test_ascending_worst_case() {
        // Ascending insertion is worst case for naive BSTs;
        // RB tree stays balanced
        let tree: RBTree<i32> = (1..=100).collect();
        assert_eq!(tree.len(), 100);
        assert!(validate_rb(&tree).is_ok());
    }

    #[test]
    fn test_default_is_empty() {
        let tree: RBTree<i32> = RBTree::default();
        assert!(tree.is_empty());
    }
}
