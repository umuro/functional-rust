#![allow(clippy::all)]
//! PhantomData for Lifetime Markers
//!
//! Using PhantomData to carry lifetime information.

use std::marker::PhantomData;

/// Struct that conceptually borrows from 'a.
pub struct Handle<'a, T> {
    id: usize,
    _marker: PhantomData<&'a T>,
}

impl<'a, T> Handle<'a, T> {
    pub fn new(id: usize) -> Self {
        Handle {
            id,
            _marker: PhantomData,
        }
    }

    pub fn id(&self) -> usize {
        self.id
    }
}

/// Typed index into a collection.
pub struct Index<T> {
    idx: usize,
    _marker: PhantomData<T>,
}

impl<T> Index<T> {
    pub fn new(idx: usize) -> Self {
        Index {
            idx,
            _marker: PhantomData,
        }
    }

    pub fn get(self) -> usize {
        self.idx
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handle() {
        let h: Handle<i32> = Handle::new(42);
        assert_eq!(h.id(), 42);
    }

    #[test]
    fn test_index() {
        let idx: Index<String> = Index::new(5);
        assert_eq!(idx.get(), 5);
    }
}
