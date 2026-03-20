#![allow(clippy::all)]
// 968: Double-Ended Queue (Deque)
// Approach 1: std VecDeque (built-in, O(1) amortized all operations)
// Approach 2: Functional two-stack deque (mirrors OCaml approach)

use std::collections::VecDeque;

// --- Approach 1: VecDeque (idiomatic Rust, ring buffer internally) ---
pub struct Deque<T> {
    inner: VecDeque<T>,
}

impl<T> Deque<T> {
    pub fn new() -> Self {
        Deque {
            inner: VecDeque::new(),
        }
    }

    pub fn push_front(&mut self, x: T) {
        self.inner.push_front(x);
    }
    pub fn push_back(&mut self, x: T) {
        self.inner.push_back(x);
    }
    pub fn pop_front(&mut self) -> Option<T> {
        self.inner.pop_front()
    }
    pub fn pop_back(&mut self) -> Option<T> {
        self.inner.pop_back()
    }
    pub fn peek_front(&self) -> Option<&T> {
        self.inner.front()
    }
    pub fn peek_back(&self) -> Option<&T> {
        self.inner.back()
    }
    pub fn size(&self) -> usize {
        self.inner.len()
    }
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
}

impl<T> Default for Deque<T> {
    fn default() -> Self {
        Self::new()
    }
}

// --- Approach 2: Functional two-stack deque (mirrors OCaml) ---
#[derive(Clone, Debug)]
pub struct FunctionalDeque<T: Clone> {
    front: Vec<T>, // front stack (top = front of deque)
    back: Vec<T>,  // back stack (top = back of deque)
}

impl<T: Clone + PartialEq> FunctionalDeque<T> {
    pub fn new() -> Self {
        FunctionalDeque {
            front: vec![],
            back: vec![],
        }
    }

    pub fn is_empty(&self) -> bool {
        self.front.is_empty() && self.back.is_empty()
    }

    pub fn size(&self) -> usize {
        self.front.len() + self.back.len()
    }

    fn balance(mut self) -> Self {
        if self.front.is_empty() {
            self.front = self.back.clone();
            self.front.reverse();
            self.back.clear();
        }
        self
    }

    pub fn push_front(mut self, x: T) -> Self {
        self.front.push(x);
        self.balance()
    }

    pub fn push_back(mut self, x: T) -> Self {
        self.back.push(x);
        self.balance()
    }

    pub fn pop_front(self) -> Option<(T, Self)> {
        let mut d = self.balance();
        if d.front.is_empty() {
            None
        } else {
            let x = d.front.pop().unwrap();
            Some((x, d.balance()))
        }
    }

    pub fn peek_front(&self) -> Option<&T> {
        self.front.last()
    }
}

impl<T: Clone + PartialEq> Default for FunctionalDeque<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vecdeque_operations() {
        let mut d: Deque<i32> = Deque::new();
        assert!(d.is_empty());

        d.push_back(1);
        d.push_back(2);
        d.push_back(3);
        d.push_front(0);

        assert_eq!(d.size(), 4);
        assert_eq!(d.peek_front(), Some(&0));
        assert_eq!(d.peek_back(), Some(&3));

        assert_eq!(d.pop_front(), Some(0));
        assert_eq!(d.pop_back(), Some(3));
        assert_eq!(d.size(), 2);
    }

    #[test]
    fn test_vecdeque_empty() {
        let mut d: Deque<i32> = Deque::new();
        assert_eq!(d.pop_front(), None);
        assert_eq!(d.pop_back(), None);
        assert_eq!(d.peek_front(), None);
    }

    #[test]
    fn test_functional_deque() {
        let d: FunctionalDeque<i32> = FunctionalDeque::new();
        assert!(d.is_empty());

        let d = d.push_back(1).push_back(2).push_back(3).push_front(0);
        assert_eq!(d.size(), 4);
        assert_eq!(d.peek_front(), Some(&0));

        let (v, d) = d.pop_front().unwrap();
        assert_eq!(v, 0);
        assert_eq!(d.peek_front(), Some(&1));
    }

    #[test]
    fn test_fifo_order() {
        let mut d: Deque<i32> = Deque::new();
        for i in 1..=5 {
            d.push_back(i);
        }
        let mut out = vec![];
        while let Some(v) = d.pop_front() {
            out.push(v);
        }
        assert_eq!(out, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_lifo_order() {
        let mut d: Deque<i32> = Deque::new();
        for i in 1..=5 {
            d.push_back(i);
        }
        let mut out = vec![];
        while let Some(v) = d.pop_back() {
            out.push(v);
        }
        assert_eq!(out, vec![5, 4, 3, 2, 1]);
    }
}
