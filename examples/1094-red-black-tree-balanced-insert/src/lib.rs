// Red-Black Tree — Okasaki's Functional Balanced Insert
//
// A persistent (immutable) red-black tree following Okasaki's elegant
// functional approach. The `balance` function captures all four rotation
// cases in a single pattern match — a direct translation of the OCaml original.

/// Node color: Red or Black
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    Red,
    Black,
}

/// A persistent red-black tree.
///
/// Uses `Box` for heap-allocated children — Rust's closest analog to
/// OCaml's garbage-collected recursive variants.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RBTree<T> {
    Empty,
    Node(Color, Box<RBTree<T>>, T, Box<RBTree<T>>),
}

use Color::{Black, Red};
use RBTree::{Empty, Node};

impl<T: Ord + Clone> RBTree<T> {
    /// Creates an empty red-black tree.
    pub fn new() -> Self {
        Empty
    }

    // ── Solution 1: Idiomatic Rust — Okasaki balance via pattern matching ──

    /// Restores red-black invariants after insertion.
    ///
    /// Detects all four red-red violations and rotates to a uniform shape:
    /// ```text
    ///        y(R)
    ///       /    \
    ///     x(B)   z(B)
    ///    / \     / \
    ///   a   b   c   d
    /// ```
    fn balance(color: Color, left: RBTree<T>, value: T, right: RBTree<T>) -> Self {
        // Each arm matches one of the four red-red violation patterns.
        // Destructuring nested enums mirrors OCaml's nested pattern match exactly.
        match (color, &left, &right) {
            // Case 1: Left-left red violation
            (Black, Node(Red, ref ll, _, _), _) if matches!(ll.as_ref(), Node(Red, _, _, _)) => {
                if let Node(Red, ll, y, c) = left {
                    if let Node(Red, a, x, b) = *ll {
                        return Node(
                            Red,
                            Box::new(Node(Black, a, x, b)),
                            y,
                            Box::new(Node(Black, c, value, Box::new(right))),
                        );
                    }
                }
                unreachable!()
            }

            // Case 2: Left-right red violation
            (Black, Node(Red, _, _, ref lr), _) if matches!(lr.as_ref(), Node(Red, _, _, _)) => {
                if let Node(Red, a, x, lr) = left {
                    if let Node(Red, b, y, c) = *lr {
                        return Node(
                            Red,
                            Box::new(Node(Black, a, x, b)),
                            y,
                            Box::new(Node(Black, c, value, Box::new(right))),
                        );
                    }
                }
                unreachable!()
            }

            // Case 3: Right-left red violation
            (Black, _, Node(Red, ref rl, _, _)) if matches!(rl.as_ref(), Node(Red, _, _, _)) => {
                if let Node(Red, rl, z, d) = right {
                    if let Node(Red, b, y, c) = *rl {
                        return Node(
                            Red,
                            Box::new(Node(Black, Box::new(left), value, b)),
                            y,
                            Box::new(Node(Black, c, z, d)),
                        );
                    }
                }
                unreachable!()
            }

            // Case 4: Right-right red violation
            (Black, _, Node(Red, _, _, ref rr)) if matches!(rr.as_ref(), Node(Red, _, _, _)) => {
                if let Node(Red, b, y, rr) = right {
                    if let Node(Red, c, z, d) = *rr {
                        return Node(
                            Red,
                            Box::new(Node(Black, Box::new(left), value, b)),
                            y,
                            Box::new(Node(Black, c, z, d)),
                        );
                    }
                }
                unreachable!()
            }

            // No violation — reconstruct node as-is
            _ => Node(color, Box::new(left), value, Box::new(right)),
        }
    }

    /// Inserts a value into the tree, maintaining red-black invariants.
    ///
    /// Returns a new tree (persistent — original is not modified).
    /// The root is always painted black after insertion.
    pub fn insert(&self, x: T) -> Self {
        // Inner recursive helper mirrors OCaml's `ins` local function
        fn ins<T: Ord + Clone>(tree: &RBTree<T>, x: T) -> RBTree<T> {
            match tree {
                Empty => Node(Red, Box::new(Empty), x, Box::new(Empty)),
                Node(color, a, y, b) => {
                    if x < *y {
                        RBTree::balance(*color, ins(a, x), y.clone(), b.as_ref().clone())
                    } else if x > *y {
                        RBTree::balance(*color, a.as_ref().clone(), y.clone(), ins(b, x))
                    } else {
                        // Element already present — return unchanged
                        tree.clone()
                    }
                }
            }
        }

        // Paint root black — guarantees invariant 2
        match ins(self, x) {
            Node(_, a, y, b) => Node(Black, a, y, b),
            Empty => Empty,
        }
    }

    // ── Solution 2: Membership check — recursive, mirrors OCaml `mem` ──

    /// Checks whether a value exists in the tree.
    ///
    /// Recursive descent following BST ordering — O(log n) for balanced trees.
    pub fn contains(&self, x: &T) -> bool {
        match self {
            Empty => false,
            Node(_, a, y, b) => {
                if *x == *y {
                    true
                } else if *x < *y {
                    a.contains(x)
                } else {
                    b.contains(x)
                }
            }
        }
    }

    // ── Solution 3: In-order traversal — collects to sorted Vec ──

    /// Collects elements in sorted order (in-order traversal).
    pub fn to_sorted_vec(&self) -> Vec<&T> {
        match self {
            Empty => vec![],
            Node(_, a, v, b) => {
                let mut result = a.to_sorted_vec();
                result.push(v);
                result.extend(b.to_sorted_vec());
                result
            }
        }
    }

    /// Returns the number of elements in the tree.
    pub fn len(&self) -> usize {
        match self {
            Empty => 0,
            Node(_, a, _, b) => 1 + a.len() + b.len(),
        }
    }

    /// Returns true if the tree contains no elements.
    pub fn is_empty(&self) -> bool {
        matches!(self, Empty)
    }

    /// Returns the color of the root node, or None for empty trees.
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

/// Builds a tree from an iterator (fold-based, mirrors OCaml's `List.fold_left`).
impl<T: Ord + Clone> FromIterator<T> for RBTree<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        iter.into_iter().fold(RBTree::new(), |t, x| t.insert(x))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Validates that a tree satisfies all red-black invariants.
    /// Returns the black-height if valid, or an error message.
    fn validate_rb_invariants<T: Ord + Clone>(tree: &RBTree<T>) -> Result<usize, String> {
        match tree {
            Empty => Ok(1), // Empty nodes count as black
            Node(color, left, _, right) => {
                // Invariant 3: Red nodes must have black children
                if *color == Red {
                    if let Node(Red, _, _, _) = left.as_ref() {
                        return Err("Red node has red left child".to_string());
                    }
                    if let Node(Red, _, _, _) = right.as_ref() {
                        return Err("Red node has red right child".to_string());
                    }
                }

                let lh = validate_rb_invariants(left)?;
                let rh = validate_rb_invariants(right)?;

                // Invariant 4: Every path has same number of black nodes
                if lh != rh {
                    return Err(format!("Black-height mismatch: left={lh}, right={rh}"));
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
        // Root must be black
        assert_eq!(tree.root_color(), Some(Black));
    }

    #[test]
    fn test_multiple_inserts_sorted_output() {
        // Mirrors the OCaml example: insert [5;3;7;1;4;6;8;2;9]
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
                "Root not black after inserting 1..={n}"
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
                validate_rb_invariants(&tree).is_ok(),
                "RB invariants violated for insertion order {order:?}: {}",
                validate_rb_invariants(&tree).unwrap_err()
            );
        }
    }

    #[test]
    fn test_persistence() {
        // Inserting into a tree returns a new tree; original is unchanged
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
        let sorted: Vec<&str> = tree
            .to_sorted_vec()
            .into_iter()
            .map(|s| s.as_str())
            .collect();
        assert_eq!(sorted, vec!["apple", "banana", "cherry", "date"]);
    }

    #[test]
    fn test_worst_case_ascending_insert() {
        // Ascending insertion is the worst case for naive BSTs,
        // but RB tree stays balanced
        let tree: RBTree<i32> = (1..=100).collect();
        assert_eq!(tree.len(), 100);
        assert!(validate_rb_invariants(&tree).is_ok());
        assert_eq!(tree.to_sorted_vec().len(), 100);
    }
}
