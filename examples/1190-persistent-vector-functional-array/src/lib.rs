#![allow(dead_code)]

use std::rc::Rc;

/// Persistent vector backed by a balanced binary tree.
///
/// `set` shares all unchanged subtrees via `Rc`, so old versions remain fully
/// accessible after an update — purely functional update semantics.
#[derive(Debug, Clone, PartialEq)]
pub enum PVec<T> {
    Nil,
    One(T),
    Two(Rc<PVec<T>>, Rc<PVec<T>>),
}

impl<T: Clone> PVec<T> {
    /// Construct an empty vector.
    pub fn empty() -> Self {
        PVec::Nil
    }

    /// Number of elements (O(n) — traverses the tree).
    pub fn size(&self) -> usize {
        match self {
            PVec::Nil => 0,
            PVec::One(_) => 1,
            PVec::Two(l, r) => l.size() + r.size(),
        }
    }

    /// Get element at index `i`. Returns `None` if out of bounds.
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

    /// Return a new vector with element at index `i` replaced by `v`.
    /// Unchanged subtrees are shared with the original via `Rc` (structural sharing).
    /// Returns `None` if `i` is out of bounds.
    pub fn set(&self, i: usize, v: T) -> Option<Self> {
        match self {
            PVec::Nil => None,
            PVec::One(_) => (i == 0).then(|| PVec::One(v)),
            PVec::Two(l, r) => {
                let ls = l.size();
                if i < ls {
                    l.set(i, v)
                        .map(|new_l| PVec::Two(Rc::new(new_l), Rc::clone(r)))
                } else {
                    r.set(i - ls, v)
                        .map(|new_r| PVec::Two(Rc::clone(l), Rc::new(new_r)))
                }
            }
        }
    }

    /// Build from a slice by halving recursively — produces a balanced tree.
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

/// Recursive implementation using `Box` — mirrors OCaml style directly.
/// No structural sharing: `set` deep-clones unchanged subtrees.
/// Simpler ownership model; illustrates the OCaml→Rust pattern without `Rc`.
#[derive(Debug, Clone, PartialEq)]
pub enum PVecRec<T> {
    Nil,
    One(T),
    Two(Box<PVecRec<T>>, Box<PVecRec<T>>),
}

impl<T: Clone> PVecRec<T> {
    pub fn size(&self) -> usize {
        match self {
            PVecRec::Nil => 0,
            PVecRec::One(_) => 1,
            PVecRec::Two(l, r) => l.size() + r.size(),
        }
    }

    pub fn get(&self, i: usize) -> Option<&T> {
        match self {
            PVecRec::Nil => None,
            PVecRec::One(x) => (i == 0).then_some(x),
            PVecRec::Two(l, r) => {
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
            PVecRec::Nil => None,
            PVecRec::One(_) => (i == 0).then(|| PVecRec::One(v)),
            PVecRec::Two(l, r) => {
                let ls = l.size();
                if i < ls {
                    l.set(i, v)
                        .map(|new_l| PVecRec::Two(Box::new(new_l), r.clone()))
                } else {
                    r.set(i - ls, v)
                        .map(|new_r| PVecRec::Two(l.clone(), Box::new(new_r)))
                }
            }
        }
    }

    pub fn from_slice(items: &[T]) -> Self {
        match items {
            [] => PVecRec::Nil,
            [x] => PVecRec::One(x.clone()),
            _ => {
                let mid = items.len() / 2;
                PVecRec::Two(
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
    fn test_empty() {
        let v: PVec<i32> = PVec::empty();
        assert_eq!(v.size(), 0);
        assert_eq!(v.get(0), None);
        assert_eq!(v.set(0, 42), None);
    }

    #[test]
    fn test_single() {
        let v = PVec::from_slice(&[42]);
        assert_eq!(v.size(), 1);
        assert_eq!(v.get(0), Some(&42));
        assert_eq!(v.get(1), None);
    }

    #[test]
    fn test_multiple_get() {
        let v = PVec::from_slice(&[10, 20, 30, 40, 50]);
        assert_eq!(v.size(), 5);
        assert_eq!(v.get(0), Some(&10));
        assert_eq!(v.get(2), Some(&30));
        assert_eq!(v.get(4), Some(&50));
        assert_eq!(v.get(5), None);
    }

    #[test]
    fn test_set_returns_new_version() {
        let v = PVec::from_slice(&[10, 20, 30, 40, 50]);
        let v2 = v.set(2, 99).expect("index 2 is valid");
        // New version has updated value
        assert_eq!(v2.get(2), Some(&99));
        // Old version unchanged — persistence!
        assert_eq!(v.get(2), Some(&30));
    }

    #[test]
    fn test_set_out_of_bounds() {
        let v = PVec::from_slice(&[1, 2, 3]);
        assert_eq!(v.set(10, 99), None);
    }

    #[test]
    fn test_multiple_sets_are_independent() {
        let v0 = PVec::from_slice(&[1, 2, 3]);
        let v1 = v0.set(0, 10).unwrap();
        let v2 = v0.set(1, 20).unwrap();
        // v0 unchanged
        assert_eq!(v0.get(0), Some(&1));
        assert_eq!(v0.get(1), Some(&2));
        // v1 and v2 diverged independently from v0
        assert_eq!(v1.get(0), Some(&10));
        assert_eq!(v1.get(1), Some(&2));
        assert_eq!(v2.get(0), Some(&1));
        assert_eq!(v2.get(1), Some(&20));
    }

    #[test]
    fn test_recursive_version_matches_behavior() {
        let v = PVecRec::from_slice(&[10, 20, 30, 40, 50]);
        assert_eq!(v.get(2), Some(&30));
        let v2 = v.set(2, 99).unwrap();
        assert_eq!(v2.get(2), Some(&99));
        assert_eq!(v.get(2), Some(&30));
    }

    #[test]
    fn test_from_slice_all_indices() {
        let data = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let v = PVec::from_slice(&data);
        assert_eq!(v.size(), 8);
        for (i, &expected) in data.iter().enumerate() {
            assert_eq!(v.get(i), Some(&expected));
        }
    }
}
