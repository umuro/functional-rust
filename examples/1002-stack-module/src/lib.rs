#![allow(clippy::all)]
//! Stack Module with Signature
//! See example.ml for OCaml reference
//!
//! Mirrors OCaml's `ListStack` module: push/pop consume and return a new stack,
//! giving a persistent, functional interface over a `Vec` backbone.

#[derive(Debug, Clone, PartialEq)]
pub struct Stack<T> {
    items: Vec<T>,
}

#[derive(Debug, PartialEq)]
pub struct Empty;

impl<T> Stack<T> {
    /// The empty stack.
    pub fn empty() -> Self {
        Stack { items: Vec::new() }
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn size(&self) -> usize {
        self.items.len()
    }

    /// Push an element, returning the new stack (consuming `self`).
    pub fn push(mut self, x: T) -> Self {
        self.items.push(x);
        self
    }

    /// Return a reference to the top element without removing it.
    pub fn peek(&self) -> Result<&T, Empty> {
        self.items.last().ok_or(Empty)
    }

    /// Remove the top element, returning the remainder (consuming `self`).
    pub fn pop(mut self) -> Result<Self, Empty> {
        if self.items.is_empty() {
            Err(Empty)
        } else {
            self.items.pop();
            Ok(self)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        let s: Stack<i32> = Stack::empty();
        assert!(s.is_empty());
        assert_eq!(s.size(), 0);
    }

    #[test]
    fn test_push_peek_size() {
        let s = Stack::empty().push(1).push(2).push(3);
        assert_eq!(s.size(), 3);
        assert_eq!(s.peek(), Ok(&3));
        assert!(!s.is_empty());
    }

    #[test]
    fn test_pop() {
        let s = Stack::empty().push(1).push(2).push(3);
        let s = s.pop().unwrap();
        assert_eq!(s.peek(), Ok(&2));
        let s = s.pop().unwrap();
        assert_eq!(s.peek(), Ok(&1));
        let s = s.pop().unwrap();
        assert!(s.is_empty());
    }

    #[test]
    fn test_peek_empty_err() {
        let s: Stack<i32> = Stack::empty();
        assert_eq!(s.peek(), Err(Empty));
    }

    #[test]
    fn test_pop_empty_err() {
        let s: Stack<i32> = Stack::empty();
        assert!(s.pop().is_err());
    }

    #[test]
    fn test_pipeline_style() {
        // Mirror OCaml: empty |> push 1 |> push 2 |> push 3
        let s = Stack::empty().push(1).push(2).push(3);
        assert_eq!(s.peek(), Ok(&3));
        let s = s.pop().unwrap();
        assert_eq!(s.peek(), Ok(&2));
    }
}
