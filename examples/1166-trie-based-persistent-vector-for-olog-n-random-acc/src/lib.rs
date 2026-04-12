#![allow(dead_code)]
//! Trie-based Persistent Vector for O(log n) Random Access
//! See example.ml for OCaml reference
//!
//! A purely functional persistent vector backed by a balanced binary tree.
//! Structural sharing via `Rc` means `set` only clones the O(log n) path nodes.

use std::rc::Rc;

/// Persistent vector backed by a balanced binary tree.
///
/// Each `set` shares unchanged subtrees via `Rc`, so old versions remain fully
/// accessible after an update — purely functional update semantics.
#[derive(Debug, Clone, PartialEq)]
pub enum PVec<T> {
    Nil,
    One(T),
    Two(Rc<PVec<T>>, Rc<PVec<T>>),
}

impl<T: Clone> PVec<T> {
    /// Create an empty persistent vector.
    pub fn empty() -> Self {
        PVec::Nil
    }

    /// Number of elements in the vector (traverses the tree: O(n)).
    pub fn size(&self) -> usize {
        match self {
            PVec::Nil => 0,
            PVec::One(_) => 1,
            PVec::Two(l, r) => l.size() + r.size(),
        }
    }

    /// Get the element at index `i`. Returns `None` if out of bounds.
    pub fn get(&self, i: usize) -> Option<&T> {
        match self {
            PVec::Nil => None,
            PVec::One(x) => (i == 0).then_some(x),
            PVec::Two(l, r) => {
                let ls = l.size();
                if i < ls {
                    l.get(i)
                } else {
                    r.get(i - ls)
                }
            }
        }
    }

    /// Return a new vector with the element at index `i` replaced by `v`.
    /// Unchanged subtrees are shared with the original via `Rc` (structural sharing).
    /// Returns `None` if `i` is out of bounds.
    pub fn set(&self, i: usize, v: T) -> Option<Self> {
        match self {
            PVec::Nil => None,
            PVec::One(_) => (i == 0).then(|| PVec::One(v)),
            PVec::Two(l, r) => {
                let ls = l.size();
                if i < ls {
                    // Update left subtree; share right subtree as-is.
                    l.set(i, v)
                        .map(|new_l| PVec::Two(Rc::new(new_l), Rc::clone(r)))
                } else {
                    // Update right subtree; share left subtree as-is.
                    r.set(i - ls, v)
                        .map(|new_r| PVec::Two(Rc::clone(l), Rc::new(new_r)))
                }
            }
        }
    }

    /// Build a persistent vector from a slice by halving recursively.
    /// Produces a balanced tree with O(log n) depth.
    pub fn from_slice(items: &[T]) -> Self {
        match items {
            [] => PVec::Nil,
            [x] => PVec::One(x.clone()),
            _ => {
                let mid = items.len() / 2;
                PVec::Two(
                    Rc::new(Self::from_slice(&items[..mid])),
                    Rc::new(Self::from_slice(&items[mid..])),
                )
            }
        }
    }
}

/// Simple recursive version using `Box` — mirrors OCaml style directly.
/// No structural sharing: `set` deep-clones unchanged subtrees.
#[derive(Debug, Clone, PartialEq)]
pub enum PVecBox<T> {
    Nil,
    One(T),
    Two(Box<PVecBox<T>>, Box<PVecBox<T>>),
}

impl<T: Clone> PVecBox<T> {
    pub fn size(&self) -> usize {
        match self {
            PVecBox::Nil => 0,
            PVecBox::One(_) => 1,
            PVecBox::Two(l, r) => l.size() + r.size(),
        }
    }

    pub fn get(&self, i: usize) -> Option<&T> {
        match self {
            PVecBox::Nil => None,
            PVecBox::One(x) => (i == 0).then_some(x),
            PVecBox::Two(l, r) => {
                let ls = l.size();
                if i < ls {
                    l.get(i)
                } else {
                    r.get(i - ls)
                }
            }
        }
    }

    pub fn set(&self, i: usize, v: T) -> Option<Self> {
        match self {
            PVecBox::Nil => None,
            PVecBox::One(_) => (i == 0).then(|| PVecBox::One(v)),
            PVecBox::Two(l, r) => {
                let ls = l.size();
                if i < ls {
                    l.set(i, v)
                        .map(|new_l| PVecBox::Two(Box::new(new_l), r.clone()))
                } else {
                    r.set(i - ls, v)
                        .map(|new_r| PVecBox::Two(l.clone(), Box::new(new_r)))
                }
            }
        }
    }

    pub fn from_slice(items: &[T]) -> Self {
        match items {
            [] => PVecBox::Nil,
            [x] => PVecBox::One(x.clone()),
            _ => {
                let mid = items.len() / 2;
                PVecBox::Two(
                    Box::new(Self::from_slice(&items[..mid])),
                    Box::new(Self::from_slice(&items[mid..])),
                )
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_vec() {
        let v: PVec<i32> = PVec::empty();
        assert_eq!(v.size(), 0);
        assert_eq!(v.get(0), None);
        assert_eq!(v.set(0, 42), None);
    }

    #[test]
    fn test_single_element() {
        let v = PVec::from_slice(&[42_i32]);
        assert_eq!(v.size(), 1);
        assert_eq!(v.get(0), Some(&42));
        assert_eq!(v.get(1), None);
    }

    #[test]
    fn test_multiple_get() {
        let v = PVec::from_slice(&[10_i32, 20, 30, 40, 50]);
        assert_eq!(v.size(), 5);
        assert_eq!(v.get(0), Some(&10));
        assert_eq!(v.get(2), Some(&30));
        assert_eq!(v.get(4), Some(&50));
        assert_eq!(v.get(5), None);
    }

    #[test]
    fn test_set_returns_new_version_old_unchanged() {
        let v = PVec::from_slice(&[10_i32, 20, 30, 40, 50]);
        let v2 = v.set(2, 99).expect("index 2 is valid");
        // New version has updated value.
        assert_eq!(v2.get(2), Some(&99));
        // Old version unchanged — persistence guarantees.
        assert_eq!(v.get(2), Some(&30));
    }

    #[test]
    fn test_set_out_of_bounds_returns_none() {
        let v = PVec::from_slice(&[1_i32, 2, 3]);
        assert_eq!(v.set(10, 99), None);
    }

    #[test]
    fn test_multiple_independent_versions() {
        let v0 = PVec::from_slice(&[1_i32, 2, 3]);
        let v1 = v0.set(0, 10).unwrap();
        let v2 = v0.set(1, 20).unwrap();
        // v0 is unchanged.
        assert_eq!(v0.get(0), Some(&1));
        // v1 and v2 diverged from v0 independently.
        assert_eq!(v1.get(0), Some(&10));
        assert_eq!(v2.get(1), Some(&20));
        assert_eq!(v1.get(1), Some(&2));
    }

    #[test]
    fn test_box_version_matches_rc_version() {
        let data = [10_i32, 20, 30, 40, 50];
        let rc_v = PVec::from_slice(&data);
        let box_v = PVecBox::from_slice(&data);
        for i in 0..data.len() {
            assert_eq!(rc_v.get(i), box_v.get(i));
        }
    }

    #[test]
    fn test_all_indices_accessible() {
        let data: Vec<i32> = (0..8).collect();
        let v = PVec::from_slice(&data);
        assert_eq!(v.size(), 8);
        for (i, &expected) in data.iter().enumerate() {
            assert_eq!(v.get(i), Some(&expected));
        }
    }
}
