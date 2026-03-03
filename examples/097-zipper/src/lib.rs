//! # Zipper — Functional List Cursor
//!
//! A zipper provides O(1) navigation and update at the focus point.
//! OCaml's record with `{ left; focus; right }` maps directly to a Rust struct.

// ---------------------------------------------------------------------------
// Approach A: Struct with Vec (idiomatic Rust)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq)]
pub struct Zipper<T> {
    left: Vec<T>,  // reversed — top is nearest to focus
    focus: T,
    right: Vec<T>,
}

impl<T: Clone> Zipper<T> {
    pub fn from_vec(v: Vec<T>) -> Option<Self> {
        let mut iter = v.into_iter();
        let focus = iter.next()?;
        Some(Zipper {
            left: vec![],
            focus,
            right: iter.collect(),
        })
    }

    pub fn focus(&self) -> &T {
        &self.focus
    }

    pub fn go_right(&self) -> Option<Self> {
        let mut right = self.right.clone();
        if right.is_empty() {
            return None;
        }
        let new_focus = right.remove(0);
        let mut left = self.left.clone();
        left.push(self.focus.clone());
        Some(Zipper { left, focus: new_focus, right })
    }

    pub fn go_left(&self) -> Option<Self> {
        let mut left = self.left.clone();
        let new_focus = left.pop()?;
        let mut right = self.right.clone();
        right.insert(0, self.focus.clone());
        Some(Zipper { left, focus: new_focus, right })
    }

    pub fn update<F: FnOnce(&T) -> T>(&self, f: F) -> Self {
        Zipper {
            left: self.left.clone(),
            focus: f(&self.focus),
            right: self.right.clone(),
        }
    }

    pub fn to_vec(&self) -> Vec<T> {
        let mut result: Vec<T> = self.left.iter().rev().cloned().collect();
        result.push(self.focus.clone());
        result.extend(self.right.iter().cloned());
        result
    }
}

// ---------------------------------------------------------------------------
// Approach B: VecDeque-based for O(1) operations
// ---------------------------------------------------------------------------

use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct ZipperDeque<T> {
    left: Vec<T>,
    focus: T,
    right: VecDeque<T>,
}

impl<T: Clone> ZipperDeque<T> {
    pub fn from_vec(v: Vec<T>) -> Option<Self> {
        let mut dq: VecDeque<T> = v.into();
        let focus = dq.pop_front()?;
        Some(ZipperDeque { left: vec![], focus, right: dq })
    }

    pub fn go_right(&mut self) -> bool {
        if let Some(new_focus) = self.right.pop_front() {
            let old = std::mem::replace(&mut self.focus, new_focus);
            self.left.push(old);
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_navigation() {
        let z = Zipper::from_vec(vec![1, 2, 3, 4, 5]).unwrap();
        let z = z.go_right().unwrap();
        let z = z.go_right().unwrap();
        assert_eq!(*z.focus(), 3);
    }

    #[test]
    fn test_update() {
        let z = Zipper::from_vec(vec![1, 2, 3, 4, 5]).unwrap();
        let z = z.go_right().unwrap().go_right().unwrap();
        let z = z.update(|x| x * 10);
        assert_eq!(z.to_vec(), vec![1, 2, 30, 4, 5]);
    }

    #[test]
    fn test_go_left() {
        let z = Zipper::from_vec(vec![1, 2, 3]).unwrap();
        let z = z.go_right().unwrap().go_right().unwrap();
        let z = z.go_left().unwrap();
        assert_eq!(*z.focus(), 2);
    }

    #[test]
    fn test_boundary() {
        let z = Zipper::from_vec(vec![1]).unwrap();
        assert!(z.go_right().is_none());
        assert!(z.go_left().is_none());
    }

    #[test]
    fn test_empty() {
        assert!(Zipper::<i32>::from_vec(vec![]).is_none());
    }

    #[test]
    fn test_to_vec_preserves_order() {
        let z = Zipper::from_vec(vec![1, 2, 3, 4, 5]).unwrap();
        assert_eq!(z.to_vec(), vec![1, 2, 3, 4, 5]);
    }
}
