/// Color of a red-black tree node.
#[derive(Debug, Clone, PartialEq)]
pub enum Color {
    Red,
    Black,
}

/// A functional, immutable red-black tree.
///
/// `E` is the empty leaf; `T` holds a color, left subtree, value, and right subtree.
/// Ownership is managed via `Box` to heap-allocate child nodes.
#[derive(Debug, Clone, PartialEq)]
pub enum RbTree<T> {
    E,
    T(Color, Box<RbTree<T>>, T, Box<RbTree<T>>),
}

impl<T: Ord + Clone> RbTree<T> {
    /// Create an empty red-black tree.
    pub fn empty() -> Self {
        RbTree::E
    }

    /// Insert a value, returning a new tree with the black-root invariant restored.
    pub fn insert(&self, x: T) -> Self {
        // Inner recursive insertion returns a possibly-red-root tree.
        let result = self.ins(&x);
        // Always paint the root black.
        match result {
            RbTree::T(_, a, y, b) => RbTree::T(Color::Black, a, y, b),
            RbTree::E => RbTree::E,
        }
    }

    fn ins(&self, x: &T) -> Self {
        match self {
            RbTree::E => RbTree::T(Color::Red, Box::new(RbTree::E), x.clone(), Box::new(RbTree::E)),
            RbTree::T(color, a, y, b) => {
                if x < y {
                    balance(color.clone(), a.ins(x), y.clone(), *b.clone())
                } else if x > y {
                    balance(color.clone(), *a.clone(), y.clone(), b.ins(x))
                } else {
                    // Duplicate — return unchanged
                    self.clone()
                }
            }
        }
    }

    /// Check membership.
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

    /// Collect elements in sorted order (in-order traversal).
    pub fn to_vec(&self) -> Vec<T> {
        match self {
            RbTree::E => vec![],
            RbTree::T(_, a, v, b) => {
                let mut result = a.to_vec();
                result.push(v.clone());
                result.extend(b.to_vec());
                result
            }
        }
    }

    /// Build a tree from an iterator.
    pub fn from_iter(iter: impl IntoIterator<Item = T>) -> Self {
        iter.into_iter().fold(RbTree::empty(), |tree, x| tree.insert(x))
    }
}

/// Okasaki's balance function — restores the red-black invariant after insertion.
///
/// Matches all four cases where a black node has a red grandchild,
/// and rewrites them into a balanced subtree with a red root.
fn balance<T: Clone>(color: Color, left: RbTree<T>, z: T, right: RbTree<T>) -> RbTree<T> {
    use Color::{Black, Red};
    use RbTree::{T, E};

    // Case 1: left-left red grandchild
    if let (Black, T(Red, ref a, ref x, ref b_inner), ref y, ref d) = (&color, &left, &z, &right) {
        if let T(Red, ref a2, ref x2, ref b2) = **a {
            return T(
                Red,
                Box::new(T(Black, a2.clone(), x2.clone(), b2.clone())),
                x.clone(),
                Box::new(T(Black, b_inner.clone(), y.clone(), Box::new(d.clone()))),
            );
        }
    }

    // Case 2: left-right red grandchild
    if let (Black, T(Red, ref a, ref x, ref b_inner), ref y, ref d) = (&color, &left, &z, &right) {
        if let T(Red, ref b2, ref y2, ref c2) = **b_inner {
            return T(
                Red,
                Box::new(T(Black, a.clone(), x.clone(), b2.clone())),
                y2.clone(),
                Box::new(T(Black, c2.clone(), y.clone(), Box::new(d.clone()))),
            );
        }
    }

    // Case 3: right-left red grandchild
    if let (Black, ref a, ref x, T(Red, ref b_inner, ref y, ref d)) = (&color, &left, &z, &right) {
        if let T(Red, ref b2, ref y2, ref c2) = **b_inner {
            return T(
                Red,
                Box::new(T(Black, Box::new(a.clone()), x.clone(), b2.clone())),
                y2.clone(),
                Box::new(T(Black, c2.clone(), y.clone(), d.clone())),
            );
        }
    }

    // Case 4: right-right red grandchild
    if let (Black, ref a, ref x, T(Red, ref b_inner, ref y, ref d)) = (&color, &left, &z, &right) {
        if let T(Red, ref c2, ref z2, ref d2) = **d {
            return T(
                Red,
                Box::new(T(Black, Box::new(a.clone()), x.clone(), b_inner.clone())),
                y.clone(),
                Box::new(T(Black, c2.clone(), z2.clone(), d2.clone())),
            );
        }
    }

    // Default: no rebalancing needed
    T(color, Box::new(left), z, Box::new(right))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_tree() {
        let t: RbTree<i32> = RbTree::empty();
        assert_eq!(t.to_vec(), vec![]);
        assert!(!t.mem(&1));
    }

    #[test]
    fn test_single_insert() {
        let t = RbTree::empty().insert(42);
        assert_eq!(t.to_vec(), vec![42]);
        assert!(t.mem(&42));
        assert!(!t.mem(&0));
    }

    #[test]
    fn test_sorted_order() {
        let t = RbTree::from_iter([5, 3, 7, 1, 4, 6, 8, 2, 9]);
        assert_eq!(t.to_vec(), vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }

    #[test]
    fn test_membership() {
        let t = RbTree::from_iter([5, 3, 7, 1, 4, 6, 8, 2, 9]);
        for i in 1..=9 {
            assert!(t.mem(&i), "expected {i} to be in tree");
        }
        assert!(!t.mem(&0));
        assert!(!t.mem(&10));
    }

    #[test]
    fn test_duplicate_insert() {
        let t = RbTree::from_iter([3, 3, 3, 1, 2]);
        assert_eq!(t.to_vec(), vec![1, 2, 3]);
    }

    #[test]
    fn test_root_is_black() {
        let t = RbTree::from_iter([1, 2, 3, 4, 5]);
        match &t {
            RbTree::T(Color::Black, _, _, _) => {}
            _ => panic!("root must be black"),
        }
    }

    #[test]
    fn test_ascending_insert_stays_sorted() {
        // Ascending insertions stress-test left-rotation balancing.
        let t = RbTree::from_iter(1..=20i32);
        assert_eq!(t.to_vec(), (1..=20).collect::<Vec<_>>());
    }

    #[test]
    fn test_descending_insert_stays_sorted() {
        let t = RbTree::from_iter((1..=20i32).rev());
        assert_eq!(t.to_vec(), (1..=20).collect::<Vec<_>>());
    }
}
