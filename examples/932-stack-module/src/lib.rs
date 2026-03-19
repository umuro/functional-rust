#![allow(clippy::all)]
/// Stack Module with Signature
///
/// OCaml uses module types (signatures) to enforce abstraction.
/// Rust achieves the same with traits — defining an interface that
/// multiple types can implement.

/// The trait is Rust's equivalent of OCaml's `module type STACK`.
pub trait Stack: Sized {
    type Item;

    fn empty() -> Self;
    fn is_empty(&self) -> bool;
    fn push(&self, item: Self::Item) -> Self;
    fn peek(&self) -> Option<&Self::Item>;
    fn pop(&self) -> Option<Self>;
    fn size(&self) -> usize;
}

/// A persistent (immutable) stack backed by a Vec.
/// Each push/pop returns a new stack — the old one is unchanged.
#[derive(Debug, Clone, PartialEq)]
pub struct ListStack<T> {
    items: Vec<T>,
}

impl<T: Clone> Stack for ListStack<T> {
    type Item = T;

    fn empty() -> Self {
        ListStack { items: Vec::new() }
    }

    fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Push returns a new stack with the item on top.
    fn push(&self, item: T) -> Self {
        let mut new_items = self.items.clone();
        new_items.push(item);
        ListStack { items: new_items }
    }

    fn peek(&self) -> Option<&T> {
        self.items.last()
    }

    fn pop(&self) -> Option<Self> {
        if self.items.is_empty() {
            None
        } else {
            let mut new_items = self.items.clone();
            new_items.pop();
            Some(ListStack { items: new_items })
        }
    }

    fn size(&self) -> usize {
        self.items.len()
    }
}

/// A more efficient mutable stack (idiomatic Rust — ownership-based).
/// This is what you'd actually use in Rust: take ownership, mutate, return.
#[derive(Debug, Clone, PartialEq)]
pub struct MutStack<T> {
    items: Vec<T>,
}

impl<T> MutStack<T> {
    pub fn new() -> Self {
        MutStack { items: Vec::new() }
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn push(&mut self, item: T) {
        self.items.push(item);
    }

    pub fn peek(&self) -> Option<&T> {
        self.items.last()
    }

    pub fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }

    pub fn size(&self) -> usize {
        self.items.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_persistent_stack() {
        let s = ListStack::empty();
        let s = s.push(1);
        let s = s.push(2);
        let s = s.push(3);
        assert_eq!(s.size(), 3);
        assert_eq!(s.peek(), Some(&3));
        let s2 = s.pop().unwrap();
        assert_eq!(s2.peek(), Some(&2));
        // Original unchanged (persistent)
        assert_eq!(s.peek(), Some(&3));
    }

    #[test]
    fn test_persistent_empty() {
        let s: ListStack<i32> = ListStack::empty();
        assert!(s.is_empty());
        assert_eq!(s.peek(), None);
        assert_eq!(s.pop(), None);
    }

    #[test]
    fn test_mut_stack() {
        let mut s = MutStack::new();
        s.push(1);
        s.push(2);
        s.push(3);
        assert_eq!(s.size(), 3);
        assert_eq!(s.pop(), Some(3));
        assert_eq!(s.peek(), Some(&2));
    }

    #[test]
    fn test_mut_stack_empty() {
        let mut s = MutStack::<i32>::new();
        assert!(s.is_empty());
        assert_eq!(s.pop(), None);
    }

    #[test]
    fn test_single_element() {
        let s = ListStack::empty().push(42);
        assert_eq!(s.size(), 1);
        assert_eq!(s.peek(), Some(&42));
        let s2 = s.pop().unwrap();
        assert!(s2.is_empty());
    }
}
