#![allow(clippy::all)]
//! 1102: Red-Black Tree with Okasaki's Functional Balancing
//!
//! A purely functional red-black tree using Okasaki's elegant pattern-matching
//! balance function. Insert returns a new tree sharing structure with the old one.

#[derive(Debug, Clone, PartialEq)]
pub enum Color {
    Red,
    Black,
}

/// Purely functional red-black tree.
/// `E` = empty leaf; `T(color, left, value, right)` = internal node.
#[derive(Debug, Clone, PartialEq)]
pub enum RbTree<T> {
    E,
    T(Color, Box<RbTree<T>>, T, Box<RbTree<T>>),
}

impl<T: Ord + Clone> RbTree<T> {
    pub fn empty() -> Self {
        RbTree::E
    }

    /// Insert `x`, returning a new balanced tree. Root is always painted Black.
    pub fn insert(&self, x: T) -> Self {
        match self.ins(&x) {
            RbTree::T(_, a, y, b) => RbTree::T(Color::Black, a, y, b),
            RbTree::E => RbTree::E,
        }
    }

    /// Recursive insert that may return a Red root (fixed by `insert`).
    fn ins(&self, x: &T) -> Self {
        match self {
            RbTree::E => RbTree::T(
                Color::Red,
                Box::new(RbTree::E),
                x.clone(),
                Box::new(RbTree::E),
            ),
            // color, a, y, b are references (&Color, &Box<RbTree<T>>, &T, &Box<RbTree<T>>)
            RbTree::T(color, a, y, b) => {
                if x < y {
                    balance(color.clone(), a.ins(x), y.clone(), (**b).clone())
                } else if x > y {
                    balance(color.clone(), (**a).clone(), y.clone(), b.ins(x))
                } else {
                    self.clone() // duplicate: no change
                }
            }
        }
    }

    /// Test membership: O(log n).
    pub fn member(&self, x: &T) -> bool {
        match self {
            RbTree::E => false,
            RbTree::T(_, a, y, b) => {
                if x == y {
                    true
                } else if x < y {
                    a.member(x)
                } else {
                    b.member(x)
                }
            }
        }
    }

    /// In-order traversal — returns elements in sorted order.
    pub fn to_sorted_vec(&self) -> Vec<T> {
        match self {
            RbTree::E => vec![],
            RbTree::T(_, a, v, b) => {
                let mut out = a.to_sorted_vec();
                out.push(v.clone());
                out.extend(b.to_sorted_vec());
                out
            }
        }
    }

    /// Check that the root node is Black (RB invariant after insert).
    pub fn root_is_black(&self) -> bool {
        match self {
            RbTree::E => true,
            RbTree::T(color, _, _, _) => *color == Color::Black,
        }
    }
}

impl<T: Ord + Clone> FromIterator<T> for RbTree<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        iter.into_iter().fold(RbTree::empty(), |t, x| t.insert(x))
    }
}

/// Okasaki's balance: fix any red-red violation after insert into a Black node.
///
/// All four symmetric cases rewrite to:
///   `T(Red, T(Black, a, x, b), y, T(Black, c, z, d))`
///
/// Using `ref` bindings to borrow fields without consuming `left`/`right`,
/// so the owned values remain available for the default fallthrough.
fn balance<T: Clone>(color: Color, left: RbTree<T>, val: T, right: RbTree<T>) -> RbTree<T> {
    use Color::{Black, Red};

    if color == Black {
        // Case 1: T(Black, T(Red, T(Red,a,x,b), y,c), z, d)  — left-left
        if let RbTree::T(Red, ref ll, ref lv, ref lr) = left {
            if let RbTree::T(Red, ref a, ref x, ref b) = **ll {
                return RbTree::T(
                    Red,
                    Box::new(RbTree::T(Black, a.clone(), x.clone(), b.clone())),
                    lv.clone(),
                    Box::new(RbTree::T(Black, lr.clone(), val, Box::new(right))),
                );
            }
            // Case 2: T(Black, T(Red,a,x, T(Red,b,y,c)), z, d)  — left-right
            if let RbTree::T(Red, ref b, ref y, ref c) = **lr {
                return RbTree::T(
                    Red,
                    Box::new(RbTree::T(Black, ll.clone(), lv.clone(), b.clone())),
                    y.clone(),
                    Box::new(RbTree::T(Black, c.clone(), val, Box::new(right))),
                );
            }
        }
        // Case 3: T(Black, a, x, T(Red, T(Red,b,y,c), z,d))  — right-left
        if let RbTree::T(Red, ref rl, ref rv, ref rr) = right {
            if let RbTree::T(Red, ref b, ref y, ref c) = **rl {
                return RbTree::T(
                    Red,
                    Box::new(RbTree::T(Black, Box::new(left), val, b.clone())),
                    y.clone(),
                    Box::new(RbTree::T(Black, c.clone(), rv.clone(), rr.clone())),
                );
            }
            // Case 4: T(Black, a, x, T(Red, b, y, T(Red,c,z,d)))  — right-right
            if let RbTree::T(Red, ref c, ref z, ref d) = **rr {
                return RbTree::T(
                    Red,
                    Box::new(RbTree::T(Black, Box::new(left), val, rl.clone())),
                    rv.clone(),
                    Box::new(RbTree::T(Black, c.clone(), z.clone(), d.clone())),
                );
            }
        }
    }

    RbTree::T(color, Box::new(left), val, Box::new(right))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_tree() {
        let t: RbTree<i32> = RbTree::empty();
        assert!(!t.member(&1));
        assert_eq!(t.to_sorted_vec(), Vec::<i32>::new());
        assert!(t.root_is_black());
    }

    #[test]
    fn test_single_element() {
        let t = RbTree::empty().insert(42);
        assert!(t.member(&42));
        assert!(!t.member(&0));
        assert_eq!(t.to_sorted_vec(), vec![42]);
        assert!(t.root_is_black());
    }

    #[test]
    fn test_sorted_order_multiple_elements() {
        let t = RbTree::from_iter([5, 3, 7, 1, 4, 6, 8, 2, 9]);
        assert_eq!(t.to_sorted_vec(), vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }

    #[test]
    fn test_duplicates_ignored() {
        let t = RbTree::from_iter([3, 1, 4, 1, 5, 9, 2, 6, 5]);
        assert_eq!(t.to_sorted_vec(), vec![1, 2, 3, 4, 5, 6, 9]);
    }

    #[test]
    fn test_root_always_black() {
        let t = RbTree::from_iter([5, 3, 7, 1, 4, 6, 8]);
        assert!(t.root_is_black());
    }

    #[test]
    fn test_member_all_inserted_values() {
        let values = [5, 3, 7, 1, 4, 6, 8, 2, 9];
        let t = RbTree::from_iter(values);
        for v in values {
            assert!(t.member(&v));
        }
        assert!(!t.member(&10));
        assert!(!t.member(&0));
    }

    #[test]
    fn test_ascending_insert_stays_balanced() {
        // Worst case for naive BST: ascending order. RB tree must stay balanced.
        let t = RbTree::from_iter(1..=16);
        assert_eq!(t.to_sorted_vec(), (1..=16).collect::<Vec<_>>());
        assert!(t.root_is_black());
    }

    #[test]
    fn test_string_tree() {
        let t = RbTree::from_iter(["banana", "apple", "cherry", "date"]);
        assert_eq!(t.to_sorted_vec(), vec!["apple", "banana", "cherry", "date"]);
        assert!(t.member(&"apple"));
        assert!(!t.member(&"elderberry"));
    }
}
