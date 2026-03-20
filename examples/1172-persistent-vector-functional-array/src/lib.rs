use std::rc::Rc;

/// Persistent vector using a balanced binary tree.
///
/// `Rc` enables structural sharing: `set` creates only O(log n) new nodes,
/// sharing the unchanged subtrees with the original version. The original
/// is never mutated — both old and new versions coexist.
#[derive(Debug, Clone)]
pub enum PVec<T> {
    Nil,
    One(T),
    /// Internal node: left subtree and right subtree, shared via Rc.
    Two(Rc<PVec<T>>, Rc<PVec<T>>),
}

// --- Idiomatic Rust: method-based API ---

impl<T> PVec<T> {
    /// Number of elements in the persistent vector.
    pub fn size(&self) -> usize {
        match self {
            PVec::Nil => 0,
            PVec::One(_) => 1,
            PVec::Two(l, r) => l.size() + r.size(),
        }
    }

    /// Get element at index i, or None if out of bounds.
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

    /// Return a new persistent vector with element at index i replaced by v.
    ///
    /// The original remains unmodified. Only the O(log n) path from root to
    /// the modified leaf is newly allocated; all other subtrees are shared
    /// via `Rc::clone` (reference-count increment only, no deep copy).
    pub fn set(&self, i: usize, v: T) -> Option<Self> {
        match self {
            PVec::Nil => None,
            PVec::One(_) => (i == 0).then(|| PVec::One(v)),
            PVec::Two(l, r) => {
                let ls = l.size();
                if i < ls {
                    let new_l = l.set(i, v)?;
                    // Rc::clone shares r — O(1), no deep copy
                    Some(PVec::Two(Rc::new(new_l), Rc::clone(r)))
                } else {
                    let new_r = r.set(i - ls, v)?;
                    Some(PVec::Two(Rc::clone(l), Rc::new(new_r)))
                }
            }
        }
    }
}

impl<T: Clone> PVec<T> {
    /// Build a balanced persistent vector from a slice. O(n) time and space.
    pub fn of_list(lst: &[T]) -> Self {
        match lst {
            [] => PVec::Nil,
            [x] => PVec::One(x.clone()),
            lst => {
                let mid = lst.len() / 2;
                PVec::Two(
                    Rc::new(Self::of_list(&lst[..mid])),
                    Rc::new(Self::of_list(&lst[mid..])),
                )
            }
        }
    }
}

// --- Functional style: free functions mirroring OCaml argument order ---
// OCaml convention: index before collection (get i v, set i val v)

/// Size as a free function — mirrors OCaml's `size v`.
pub fn pvec_size<T>(v: &PVec<T>) -> usize {
    match v {
        PVec::Nil => 0,
        PVec::One(_) => 1,
        PVec::Two(l, r) => pvec_size(l) + pvec_size(r),
    }
}

/// Get as a free function — `pvec_get(i, v)` mirrors OCaml's `get i v`.
pub fn pvec_get<T>(i: usize, v: &PVec<T>) -> Option<&T> {
    match v {
        PVec::Nil => None,
        PVec::One(x) => (i == 0).then_some(x),
        PVec::Two(l, r) => {
            let ls = pvec_size(l);
            if i < ls {
                pvec_get(i, l)
            } else {
                pvec_get(i - ls, r)
            }
        }
    }
}

/// Set as a free function — `pvec_set(i, val, v)` mirrors OCaml's `set i v pvec`.
/// Returns a new persistent vector; the original is unchanged.
pub fn pvec_set<T>(i: usize, val: T, v: &PVec<T>) -> Option<PVec<T>> {
    match v {
        PVec::Nil => None,
        PVec::One(_) => (i == 0).then(|| PVec::One(val)),
        PVec::Two(l, r) => {
            let ls = pvec_size(l);
            if i < ls {
                let new_l = pvec_set(i, val, l)?;
                Some(PVec::Two(Rc::new(new_l), Rc::clone(r)))
            } else {
                let new_r = pvec_set(i - ls, val, r)?;
                Some(PVec::Two(Rc::clone(l), Rc::new(new_r)))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        let v: PVec<i32> = PVec::Nil;
        assert_eq!(v.size(), 0);
        assert_eq!(v.get(0), None);
        assert!(v.set(0, 1).is_none());
    }

    #[test]
    fn test_single_element() {
        let v = PVec::of_list(&[42]);
        assert_eq!(v.size(), 1);
        assert_eq!(v.get(0), Some(&42));
        assert_eq!(v.get(1), None);
        let v2 = v.set(0, 99).unwrap();
        assert_eq!(v2.get(0), Some(&99));
        assert_eq!(v.get(0), Some(&42)); // original unchanged
    }

    #[test]
    fn test_get_multiple_elements() {
        let v = PVec::of_list(&[10, 20, 30, 40, 50]);
        assert_eq!(v.size(), 5);
        assert_eq!(v.get(0), Some(&10));
        assert_eq!(v.get(2), Some(&30));
        assert_eq!(v.get(4), Some(&50));
        assert_eq!(v.get(5), None);
    }

    #[test]
    fn test_set_is_persistent() {
        let v1 = PVec::of_list(&[10, 20, 30, 40, 50]);
        let v2 = v1.set(2, 99).unwrap();
        // v1 is unchanged — structural sharing preserved original
        assert_eq!(v1.get(2), Some(&30));
        // v2 reflects the update
        assert_eq!(v2.get(2), Some(&99));
        // shared elements are accessible in v2
        assert_eq!(v2.get(0), Some(&10));
        assert_eq!(v2.get(4), Some(&50));
    }

    #[test]
    fn test_set_out_of_bounds_returns_none() {
        let v = PVec::of_list(&[1, 2, 3]);
        assert!(v.set(5, 99).is_none());
        assert!(PVec::<i32>::Nil.set(0, 99).is_none());
    }

    #[test]
    fn test_set_first_and_last() {
        let v = PVec::of_list(&[1, 2, 3, 4, 5]);
        let v_first = v.set(0, 100).unwrap();
        let v_last = v.set(4, 500).unwrap();
        assert_eq!(v_first.get(0), Some(&100));
        assert_eq!(v_first.get(4), Some(&5));
        assert_eq!(v_last.get(0), Some(&1));
        assert_eq!(v_last.get(4), Some(&500));
        assert_eq!(v.get(0), Some(&1)); // original still intact
    }

    #[test]
    fn test_functional_free_functions() {
        let v = PVec::of_list(&[10, 20, 30, 40, 50]);
        assert_eq!(pvec_size(&v), 5);
        assert_eq!(pvec_get(2, &v), Some(&30));
        let v2 = pvec_set(2, 99, &v).unwrap();
        assert_eq!(pvec_get(2, &v), Some(&30)); // original unchanged
        assert_eq!(pvec_get(2, &v2), Some(&99));
    }

    #[test]
    fn test_chained_updates() {
        let v0 = PVec::of_list(&[1, 2, 3, 4, 5]);
        let v1 = v0.set(1, 20).unwrap();
        let v2 = v1.set(3, 40).unwrap();
        // v0 untouched
        assert_eq!(v0.get(1), Some(&2));
        assert_eq!(v0.get(3), Some(&4));
        // v1 has first update
        assert_eq!(v1.get(1), Some(&20));
        assert_eq!(v1.get(3), Some(&4));
        // v2 has both updates
        assert_eq!(v2.get(1), Some(&20));
        assert_eq!(v2.get(3), Some(&40));
    }
}
