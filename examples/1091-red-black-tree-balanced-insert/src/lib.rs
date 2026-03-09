// Red-Black Tree — Balanced Insert (Iterator + free-function style)
//
// A persistent red-black tree following Okasaki's functional balancing.
// This version emphasizes:
// - Free functions (`balance`, `insert`) mirroring OCaml's module-level style
// - A proper `Iterator` implementation via stack-based in-order traversal
// - `Ordering::cmp` throughout (not chained if/else)

use std::cmp::Ordering;

/// Node color in a red-black tree.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    Red,
    Black,
}

/// A persistent red-black tree.
///
/// `Box` for heap-allocated children — Rust's closest analog to
/// OCaml's garbage-collected recursive variants.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RBTree<T> {
    Empty,
    Node(Color, Box<RBTree<T>>, T, Box<RBTree<T>>),
}

use Color::{Black, Red};
use RBTree::{Empty, Node};

// ── Solution 1: Free functions — mirrors OCaml's module-level `balance`/`insert` ──

/// Okasaki's balance — detects four red-red violations and rotates to:
/// ```text
///        y(R)
///       /    \
///     x(B)   z(B)
///    / \     / \
///   a   b   c   d
/// ```
///
/// Takes ownership of subtrees so rotated nodes reuse allocations.
/// Each arm corresponds to one of OCaml's four or-pattern cases.
pub fn balance<T: Ord + Clone>(
    color: Color,
    left: RBTree<T>,
    value: T,
    right: RBTree<T>,
) -> RBTree<T> {
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

/// Inserts a value into the tree, returning a new tree (persistent).
///
/// Free function mirroring OCaml's `let insert x t = ...`.
/// The root is always painted black after insertion.
pub fn insert<T: Ord + Clone>(x: T, tree: &RBTree<T>) -> RBTree<T> {
    fn ins<T: Ord + Clone>(x: &T, tree: &RBTree<T>) -> RBTree<T> {
        match tree {
            Empty => Node(Red, Box::new(Empty), x.clone(), Box::new(Empty)),
            Node(color, a, y, b) => match x.cmp(y) {
                Ordering::Less => balance(*color, ins(x, a), y.clone(), b.as_ref().clone()),
                Ordering::Greater => balance(*color, a.as_ref().clone(), y.clone(), ins(x, b)),
                Ordering::Equal => tree.clone(),
            },
        }
    }

    // Paint root black — guarantees invariant 2
    match ins(&x, tree) {
        Node(_, a, y, b) => Node(Black, a, y, b),
        Empty => Empty,
    }
}

/// Checks membership — mirrors OCaml's `let rec mem x = function ...`.
pub fn mem<T: Ord>(x: &T, tree: &RBTree<T>) -> bool {
    match tree {
        Empty => false,
        Node(_, a, y, b) => match x.cmp(y) {
            Ordering::Equal => true,
            Ordering::Less => mem(x, a),
            Ordering::Greater => mem(x, b),
        },
    }
}

// ── Solution 2: Method-based API — idiomatic Rust wrapper ──

impl<T: Ord + Clone> RBTree<T> {
    /// Creates an empty red-black tree.
    pub fn new() -> Self {
        Empty
    }

    /// Inserts a value, returning a new tree. Delegates to free `insert`.
    pub fn insert(&self, x: T) -> Self {
        insert(x, self)
    }

    /// Checks whether a value exists in the tree.
    pub fn contains(&self, x: &T) -> bool {
        mem(x, self)
    }

    /// Returns the number of elements.
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

    /// Returns the root's color, or `None` for empty.
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

/// Builds a tree from an iterator — mirrors OCaml's `List.fold_left`.
impl<T: Ord + Clone> FromIterator<T> for RBTree<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        iter.into_iter().fold(RBTree::new(), |t, x| t.insert(x))
    }
}

// ── Solution 3: Stack-based in-order iterator ──
//
// Unlike the recursive `to_list` in OCaml which builds a full list,
// this iterator yields elements lazily via an explicit stack —
// O(log n) space, suitable for large trees.

/// In-order iterator over a red-black tree.
pub struct InOrder<'a, T> {
    stack: Vec<&'a RBTree<T>>,
}

impl<'a, T> InOrder<'a, T> {
    fn new(tree: &'a RBTree<T>) -> Self {
        let mut iter = InOrder { stack: Vec::new() };
        iter.push_left_spine(tree);
        iter
    }

    /// Pushes all left children onto the stack (descend left spine).
    fn push_left_spine(&mut self, mut tree: &'a RBTree<T>) {
        while let Node(_, left, _, _) = tree {
            self.stack.push(tree);
            tree = left;
        }
    }
}

impl<'a, T> Iterator for InOrder<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.stack.pop()?;
        if let Node(_, _, value, right) = node {
            self.push_left_spine(right);
            Some(value)
        } else {
            None
        }
    }
}

impl<T: Ord + Clone> RBTree<T> {
    /// Returns a lazy in-order iterator over the tree's elements.
    pub fn iter(&self) -> InOrder<'_, T> {
        InOrder::new(self)
    }

    /// Collects elements in sorted order (convenience wrapper).
    pub fn to_sorted_vec(&self) -> Vec<&T> {
        self.iter().collect()
    }
}

impl<'a, T: Ord + Clone> IntoIterator for &'a RBTree<T> {
    type Item = &'a T;
    type IntoIter = InOrder<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
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
        // Root must be black after insert
        assert_eq!(tree.root_color(), Some(Black));
    }

    #[test]
    fn test_multiple_inserts_sorted_output() {
        // Mirrors OCaml: fold_left insert [5;3;7;1;4;6;8;2;9]
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
    fn test_membership_via_free_function() {
        let tree: RBTree<i32> = [10, 20, 30, 40, 50].into_iter().collect();
        // Free function `mem` — mirrors OCaml `mem x tree`
        assert!(mem(&10, &tree));
        assert!(mem(&30, &tree));
        assert!(mem(&50, &tree));
        assert!(!mem(&15, &tree));
        assert!(!mem(&0, &tree));
        assert!(!mem(&100, &tree));
    }

    #[test]
    fn test_free_function_insert() {
        // Free function `insert` — mirrors OCaml `insert x tree`
        let t0: RBTree<i32> = Empty;
        let t1 = insert(5, &t0);
        let t2 = insert(3, &t1);
        let t3 = insert(7, &t2);
        assert_eq!(t3.len(), 3);
        assert_eq!(t3.to_sorted_vec(), vec![&3, &5, &7]);
    }

    #[test]
    fn test_iterator_laziness() {
        let tree: RBTree<i32> = (1..=10).collect();
        let mut iter = tree.iter();
        // Iterator yields elements one at a time
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&3));
        // Can stop early without traversing the whole tree
    }

    #[test]
    fn test_into_iterator() {
        let tree: RBTree<i32> = [4, 2, 6, 1, 3].into_iter().collect();
        // Works in for loops via IntoIterator
        let collected: Vec<&i32> = (&tree).into_iter().collect();
        assert_eq!(collected, vec![&1, &2, &3, &4, &6]);
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
        // Inserting returns a new tree; original is unchanged
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
    fn test_ascending_worst_case() {
        // Ascending is worst case for naive BSTs; RB tree stays balanced
        let tree: RBTree<i32> = (1..=100).collect();
        assert_eq!(tree.len(), 100);
        assert!(validate_rb(&tree).is_ok());
        // Iterator should yield all 100 in order
        let items: Vec<&i32> = tree.iter().collect();
        assert_eq!(items.len(), 100);
        assert_eq!(*items[0], 1);
        assert_eq!(*items[99], 100);
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
}
