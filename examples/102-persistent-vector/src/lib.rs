#![allow(dead_code)]

use std::rc::Rc;

/// Persistent vector using a balanced binary tree with structural sharing via `Rc`.
///
/// `set` returns a new `PVec` sharing all unchanged subtrees — O(log n) new allocations.
/// The original vector is untouched, making this safe to use as an immutable value.
#[derive(Debug, Clone)]
pub enum PVec<T> {
    Nil,
    Leaf(T),
    Branch(Rc<PVec<T>>, Rc<PVec<T>>),
}

impl<T> PVec<T> {
    pub fn empty() -> Self {
        PVec::Nil
    }

    pub fn size(&self) -> usize {
        match self {
            PVec::Nil => 0,
            PVec::Leaf(_) => 1,
            PVec::Branch(l, r) => l.size() + r.size(),
        }
    }

    /// Returns a reference to element at index `i`, or `None` if out of bounds.
    pub fn get(&self, i: usize) -> Option<&T> {
        match self {
            PVec::Nil => None,
            PVec::Leaf(x) => (i == 0).then_some(x),
            PVec::Branch(l, r) => {
                let ls = l.size();
                if i < ls {
                    l.get(i)
                } else {
                    r.get(i - ls)
                }
            }
        }
    }
}

impl<T: Clone> PVec<T> {
    /// Returns a new `PVec` with index `i` replaced by `v`.
    ///
    /// Shares unchanged subtrees via `Rc::clone` — only the path from root to the
    /// modified leaf is newly allocated (O(log n) nodes). The original is unmodified.
    pub fn set(&self, i: usize, v: T) -> Option<Self> {
        match self {
            PVec::Nil => None,
            PVec::Leaf(_) => (i == 0).then(|| PVec::Leaf(v)),
            PVec::Branch(l, r) => {
                let ls = l.size();
                if i < ls {
                    l.set(i, v)
                        .map(|new_l| PVec::Branch(Rc::new(new_l), Rc::clone(r)))
                } else {
                    r.set(i - ls, v)
                        .map(|new_r| PVec::Branch(Rc::clone(l), Rc::new(new_r)))
                }
            }
        }
    }

    /// Build a balanced `PVec` from a slice in O(n) time.
    pub fn from_slice(items: &[T]) -> Self {
        match items {
            [] => PVec::Nil,
            [x] => PVec::Leaf(x.clone()),
            _ => {
                let mid = items.len() / 2;
                PVec::Branch(
                    Rc::new(Self::from_slice(&items[..mid])),
                    Rc::new(Self::from_slice(&items[mid..])),
                )
            }
        }
    }

    /// Flatten the tree back to a `Vec<T>` in index order.
    pub fn to_vec(&self) -> Vec<T> {
        match self {
            PVec::Nil => vec![],
            PVec::Leaf(x) => vec![x.clone()],
            PVec::Branch(l, r) => l.to_vec().into_iter().chain(r.to_vec()).collect(),
        }
    }
}

impl<T: Clone> Default for PVec<T> {
    fn default() -> Self {
        PVec::empty()
    }
}

/// Box-based persistent vector — mirrors the OCaml style more directly.
///
/// No structural sharing: each `set` allocates the entire path from root to leaf
/// and clones the unchanged subtrees. Simpler, but O(n) per update in the worst case.
#[derive(Debug, Clone)]
pub enum PVecBox<T> {
    Nil,
    Leaf(T),
    Branch(Box<PVecBox<T>>, Box<PVecBox<T>>),
}

impl<T> PVecBox<T> {
    pub fn size(&self) -> usize {
        match self {
            PVecBox::Nil => 0,
            PVecBox::Leaf(_) => 1,
            PVecBox::Branch(l, r) => l.size() + r.size(),
        }
    }

    pub fn get(&self, i: usize) -> Option<&T> {
        match self {
            PVecBox::Nil => None,
            PVecBox::Leaf(x) => (i == 0).then_some(x),
            PVecBox::Branch(l, r) => {
                let ls = l.size();
                if i < ls {
                    l.get(i)
                } else {
                    r.get(i - ls)
                }
            }
        }
    }
}

impl<T: Clone> PVecBox<T> {
    pub fn set(&self, i: usize, v: T) -> Option<Self> {
        match self {
            PVecBox::Nil => None,
            PVecBox::Leaf(_) => (i == 0).then(|| PVecBox::Leaf(v)),
            PVecBox::Branch(l, r) => {
                let ls = l.size();
                if i < ls {
                    // Right subtree cloned wholesale (no sharing)
                    l.set(i, v)
                        .map(|new_l| PVecBox::Branch(Box::new(new_l), r.clone()))
                } else {
                    r.set(i - ls, v)
                        .map(|new_r| PVecBox::Branch(l.clone(), Box::new(new_r)))
                }
            }
        }
    }

    pub fn from_slice(items: &[T]) -> Self {
        match items {
            [] => PVecBox::Nil,
            [x] => PVecBox::Leaf(x.clone()),
            _ => {
                let mid = items.len() / 2;
                PVecBox::Branch(
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

    // --- PVec (Rc-based) tests ---

    #[test]
    fn test_empty_size_and_get() {
        let v: PVec<i32> = PVec::empty();
        assert_eq!(v.size(), 0);
        assert_eq!(v.get(0), None);
    }

    #[test]
    fn test_single_element() {
        let v = PVec::from_slice(&[42]);
        assert_eq!(v.size(), 1);
        assert_eq!(v.get(0), Some(&42));
        assert_eq!(v.get(1), None);
    }

    #[test]
    fn test_from_slice_and_get() {
        let v = PVec::from_slice(&[10, 20, 30, 40, 50]);
        assert_eq!(v.size(), 5);
        assert_eq!(v.get(0), Some(&10));
        assert_eq!(v.get(2), Some(&30));
        assert_eq!(v.get(4), Some(&50));
        assert_eq!(v.get(5), None);
    }

    #[test]
    fn test_set_returns_new_version() {
        let v1 = PVec::from_slice(&[10, 20, 30, 40, 50]);
        let v2 = v1.set(2, 99).expect("valid index");

        // v2 has the update
        assert_eq!(v2.get(2), Some(&99));
        // v1 is unchanged (persistence)
        assert_eq!(v1.get(2), Some(&30));
        // Other positions preserved in v2
        assert_eq!(v2.get(0), Some(&10));
        assert_eq!(v2.get(4), Some(&50));
    }

    #[test]
    fn test_set_out_of_bounds_returns_none() {
        let v = PVec::from_slice(&[1, 2, 3]);
        assert!(v.set(3, 99).is_none());
        assert!(v.set(10, 0).is_none());
    }

    #[test]
    fn test_to_vec_round_trip() {
        let input = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let pv = PVec::from_slice(&input);
        assert_eq!(pv.to_vec(), input);
    }

    #[test]
    fn test_multiple_persistent_versions() {
        let v0 = PVec::from_slice(&[1, 2, 3]);
        let v1 = v0.set(0, 10).unwrap();
        let v2 = v1.set(1, 20).unwrap();
        let v3 = v2.set(2, 30).unwrap();

        assert_eq!(v0.to_vec(), vec![1, 2, 3]);
        assert_eq!(v1.to_vec(), vec![10, 2, 3]);
        assert_eq!(v2.to_vec(), vec![10, 20, 3]);
        assert_eq!(v3.to_vec(), vec![10, 20, 30]);
    }

    // --- PVecBox (Box-based) tests ---

    #[test]
    fn test_box_empty() {
        let v: PVecBox<i32> = PVecBox::Nil;
        assert_eq!(v.size(), 0);
        assert_eq!(v.get(0), None);
    }

    #[test]
    fn test_box_from_slice_and_get() {
        let v = PVecBox::from_slice(&[10, 20, 30, 40, 50]);
        assert_eq!(v.size(), 5);
        assert_eq!(v.get(2), Some(&30));
    }

    #[test]
    fn test_box_set_persistence() {
        let v1 = PVecBox::from_slice(&[10, 20, 30]);
        let v2 = v1.set(1, 99).expect("valid index");
        assert_eq!(v2.get(1), Some(&99));
        assert_eq!(v1.get(1), Some(&20)); // original unchanged
    }

    #[test]
    fn test_box_set_out_of_bounds() {
        let v = PVecBox::from_slice(&[1, 2]);
        assert!(v.set(5, 0).is_none());
    }
}
