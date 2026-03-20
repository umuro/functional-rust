#![allow(clippy::all)]
/// Color of a red-black tree node.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Color {
    Red,
    Black,
}

/// A functional, immutable red-black tree.
///
/// `E` is the empty leaf; `T` holds a color, left subtree, value, and right subtree.
/// Subtrees are heap-allocated via `Box` to allow recursive types.
#[derive(Debug, Clone, PartialEq, Eq)]
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
    ///
    /// Clone is semantically required: path copying builds a persistent tree.
    pub fn insert(&self, x: T) -> Self {
        let result = self.ins(&x);
        // Always paint the root black to maintain RB invariant 2.
        match result {
            RbTree::T(_, a, y, b) => RbTree::T(Color::Black, a, y, b),
            RbTree::E => RbTree::E,
        }
    }

    fn ins(&self, x: &T) -> Self {
        match self {
            RbTree::E => RbTree::T(
                Color::Red,
                Box::new(RbTree::E),
                x.clone(),
                Box::new(RbTree::E),
            ),
            RbTree::T(color, a, y, b) => {
                if x < y {
                    balance(color.clone(), a.ins(x), y.clone(), *b.clone())
                } else if x > y {
                    balance(color.clone(), *a.clone(), y.clone(), b.ins(x))
                } else {
                    self.clone() // duplicate — unchanged
                }
            }
        }
    }

    /// Check membership — O(log n) average.
    pub fn mem(&self, x: &T) -> bool {
        match self {
            RbTree::E => false,
            RbTree::T(_, a, y, b) => x == y || if x < y { a.mem(x) } else { b.mem(x) },
        }
    }

    /// Collect all elements in sorted order (in-order traversal).
    pub fn to_vec(&self) -> Vec<T> {
        match self {
            RbTree::E => vec![],
            RbTree::T(_, a, v, b) => {
                let mut out = a.to_vec();
                out.push(v.clone());
                out.extend(b.to_vec());
                out
            }
        }
    }

    /// Build a tree from any iterator.
    pub fn build(iter: impl IntoIterator<Item = T>) -> Self {
        iter.into_iter()
            .fold(RbTree::empty(), |tree, x| tree.insert(x))
    }
}

/// Okasaki's balance — restores red-black invariant after a red-red violation.
///
/// All four cases transform into the same canonical shape:
///   `Red( Black(a,x,b), y, Black(c,z,d) )`
///
/// We borrow `left` / `right` to detect which case applies, then clone only the
/// pieces that become shared roots of new subtrees.
fn balance<T: Clone>(color: Color, left: RbTree<T>, z: T, right: RbTree<T>) -> RbTree<T> {
    use Color::{Black, Red};
    use RbTree::T;

    if color != Black {
        return RbTree::T(color, Box::new(left), z, Box::new(right));
    }

    // Case 1: Black( Red( Red(a,x,b), y, c ), z, d )  — left-left red grandchild
    if let T(Red, ll, y_val, c_val) = &left {
        if let T(Red, a, x_val, b) = ll.as_ref() {
            return RbTree::T(
                Red,
                Box::new(T(Black, a.clone(), x_val.clone(), b.clone())),
                y_val.clone(),
                Box::new(T(Black, c_val.clone(), z, Box::new(right))),
            );
        }
        // Case 2: Black( Red( a, x, Red(b,y,c) ), z, d )  — left-right red grandchild
        if let T(Red, b, y2_val, c2_val) = c_val.as_ref() {
            return RbTree::T(
                Red,
                Box::new(T(Black, ll.clone(), y_val.clone(), b.clone())),
                y2_val.clone(),
                Box::new(T(Black, c2_val.clone(), z, Box::new(right))),
            );
        }
    }

    // Case 3: Black( a, x, Red( Red(b,y,c), z, d ) )  — right-left red grandchild
    if let T(Red, rl, y_val, d_val) = &right {
        if let T(Red, b, y2_val, c_val) = rl.as_ref() {
            return RbTree::T(
                Red,
                Box::new(T(Black, Box::new(left), z, b.clone())),
                y2_val.clone(),
                Box::new(T(Black, c_val.clone(), y_val.clone(), d_val.clone())),
            );
        }
        // Case 4: Black( a, x, Red( b, y, Red(c,z,d) ) )  — right-right red grandchild
        if let T(Red, c_val, z2_val, d2_val) = d_val.as_ref() {
            return RbTree::T(
                Red,
                Box::new(T(Black, Box::new(left), z, rl.clone())),
                y_val.clone(),
                Box::new(T(Black, c_val.clone(), z2_val.clone(), d2_val.clone())),
            );
        }
    }

    // Default: already balanced
    RbTree::T(color, Box::new(left), z, Box::new(right))
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
        let t = RbTree::build([5, 3, 7, 1, 4, 6, 8, 2, 9]);
        assert_eq!(t.to_vec(), vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }

    #[test]
    fn test_membership() {
        let t = RbTree::build([5, 3, 7, 1, 4, 6, 8, 2, 9]);
        for i in 1..=9 {
            assert!(t.mem(&i), "expected {i} to be in tree");
        }
        assert!(!t.mem(&0));
        assert!(!t.mem(&10));
    }

    #[test]
    fn test_duplicate_insert() {
        let t = RbTree::build([3, 3, 3, 1, 2]);
        assert_eq!(t.to_vec(), vec![1, 2, 3]);
    }

    #[test]
    fn test_root_is_black() {
        let t = RbTree::build([1, 2, 3, 4, 5]);
        assert!(
            matches!(t, RbTree::T(Color::Black, _, _, _)),
            "root must be black"
        );
    }

    #[test]
    fn test_ascending_insert_stays_sorted() {
        // Ascending insertions stress-test right-rotation balancing.
        let t = RbTree::build(1..=20i32);
        assert_eq!(t.to_vec(), (1..=20).collect::<Vec<_>>());
    }

    #[test]
    fn test_descending_insert_stays_sorted() {
        let t = RbTree::build((1..=20i32).rev());
        assert_eq!(t.to_vec(), (1..=20).collect::<Vec<_>>());
    }
}
