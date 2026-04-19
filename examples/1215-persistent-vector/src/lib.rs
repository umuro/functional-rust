#![allow(dead_code)]
//! Simplified persistent vector — an immutable sequence built from the
//! `Nil | One | Two` tree shape defined in the OCaml source.
//!
//! Every mutating operation (`push`, `pop`) returns a *new* `PVec` that
//! structurally shares the unchanged subtrees with its predecessor via
//! [`Rc`], so repeated versions coexist cheaply and the old value keeps
//! pointing at its own history.

use std::rc::Rc;

/// A persistent vector: empty, a single element, or the concatenation of
/// two subtrees.  `Two(l, r)` represents `l` followed by `r`, so indices
/// run left-to-right.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum PVec<T> {
    #[default]
    Nil,
    One(T),
    Two(Rc<PVec<T>>, Rc<PVec<T>>),
}

impl<T: Clone> PVec<T> {
    /// Empty vector — O(1).
    pub fn new() -> Self {
        PVec::Nil
    }

    /// Number of elements.  O(n) in the worst case because the tree is
    /// not self-balancing; each node is walked once.
    pub fn len(&self) -> usize {
        match self {
            PVec::Nil => 0,
            PVec::One(_) => 1,
            PVec::Two(l, r) => l.len() + r.len(),
        }
    }

    pub fn is_empty(&self) -> bool {
        matches!(self, PVec::Nil)
    }

    /// Borrow the element at position `i`, or `None` if out of range.
    /// Descends left when the index falls inside the left subtree.
    pub fn get(&self, i: usize) -> Option<&T> {
        match self {
            PVec::Nil => None,
            PVec::One(v) => (i == 0).then_some(v),
            PVec::Two(l, r) => {
                let left_len = l.len();
                if i < left_len {
                    l.get(i)
                } else {
                    r.get(i - left_len)
                }
            }
        }
    }

    /// Append `x` to the end, returning a new vector.  The old subtree is
    /// shared — only a thin `Two` node is allocated — so this is O(1).
    pub fn push(&self, x: T) -> Self {
        match self {
            PVec::Nil => PVec::One(x),
            _ => PVec::Two(Rc::new(self.clone()), Rc::new(PVec::One(x))),
        }
    }

    /// Remove the last element.  Returns `(value, new_vec)` or `None`
    /// when empty.  Collapses `Two(l, Nil)` back to `l` to keep the tree
    /// from accumulating dead spine nodes.
    pub fn pop(&self) -> Option<(T, Self)> {
        match self {
            PVec::Nil => None,
            PVec::One(v) => Some((v.clone(), PVec::Nil)),
            PVec::Two(l, r) => {
                let (v, new_r) = r.pop()?;
                let rest = match new_r {
                    PVec::Nil => (**l).clone(),
                    other => PVec::Two(l.clone(), Rc::new(other)),
                };
                Some((v, rest))
            }
        }
    }

    /// Build a vector from a slice, pushing left-to-right.
    pub fn from_slice(items: &[T]) -> Self {
        items.iter().fold(PVec::Nil, |acc, x| acc.push(x.clone()))
    }

    /// Flatten into a `Vec<T>` in index order.
    pub fn to_vec(&self) -> Vec<T> {
        let mut out = Vec::with_capacity(self.len());
        self.extend_into(&mut out);
        out
    }

    fn extend_into(&self, out: &mut Vec<T>) {
        match self {
            PVec::Nil => {}
            PVec::One(v) => out.push(v.clone()),
            PVec::Two(l, r) => {
                l.extend_into(out);
                r.extend_into(out);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_vec_is_empty() {
        let v: PVec<i32> = PVec::new();
        assert!(v.is_empty());
        assert_eq!(v.len(), 0);
        assert_eq!(v.get(0), None);
    }

    #[test]
    fn push_grows_and_preserves_order() {
        let v = PVec::new().push(1).push(2).push(3);
        assert_eq!(v.len(), 3);
        assert_eq!(v.get(0), Some(&1));
        assert_eq!(v.get(1), Some(&2));
        assert_eq!(v.get(2), Some(&3));
        assert_eq!(v.get(3), None);
    }

    #[test]
    fn pop_returns_last_and_shrinks() {
        let v = PVec::from_slice(&[1, 2, 3, 4, 5]);
        let (x, rest) = v.pop().expect("non-empty");
        assert_eq!(x, 5);
        assert_eq!(rest.to_vec(), vec![1, 2, 3, 4]);
        assert!(PVec::<i32>::new().pop().is_none());
    }

    #[test]
    fn drain_in_lifo_order_matches_ocaml_driver() {
        // The OCaml `let () = List.iter (push s) [1;2;3;4;5]` then `drain`
        // prints 5 4 3 2 1.  Mirror that here.
        let mut v = PVec::from_slice(&[1, 2, 3, 4, 5]);
        let mut drained = Vec::new();
        while let Some((x, rest)) = v.pop() {
            drained.push(x);
            v = rest;
        }
        assert_eq!(drained, vec![5, 4, 3, 2, 1]);
    }

    #[test]
    fn push_preserves_old_versions() {
        // Structural sharing: a previous version is still usable and
        // unchanged after a push produces a new one.
        let v1 = PVec::from_slice(&[10, 20, 30]);
        let v2 = v1.push(40);
        assert_eq!(v1.to_vec(), vec![10, 20, 30]);
        assert_eq!(v2.to_vec(), vec![10, 20, 30, 40]);
    }

    #[test]
    fn round_trip_to_vec_and_from_slice() {
        let src = vec!["a", "b", "c", "d"];
        let pv = PVec::from_slice(&src);
        assert_eq!(pv.to_vec(), src);
    }
}
