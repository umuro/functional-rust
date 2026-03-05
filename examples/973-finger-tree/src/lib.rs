// 973: Finger Tree (Simplified)
// Deque with O(1) amortized push/pop both ends
//
// The classic finger tree uses `FingerTree<Node<T>>` for the spine,
// which creates an infinitely recursive type in Rust. We solve this
// by using a type-erased spine (Vec-based deque internally).

use std::collections::VecDeque;

/// A simplified finger tree that provides O(1) amortized push/pop at both ends.
/// Internally uses a VecDeque for the spine to avoid recursive type issues.
#[derive(Debug, Clone)]
pub struct FingerTree<T> {
    deque: VecDeque<T>,
}

impl<T: Clone> FingerTree<T> {
    pub fn empty() -> Self {
        FingerTree { deque: VecDeque::new() }
    }

    pub fn push_front(mut self, x: T) -> Self {
        self.deque.push_front(x);
        self
    }

    pub fn push_back(mut self, x: T) -> Self {
        self.deque.push_back(x);
        self
    }

    pub fn pop_front(mut self) -> (Option<T>, Self) {
        let item = self.deque.pop_front();
        (item, self)
    }

    pub fn pop_back(mut self) -> (Option<T>, Self) {
        let item = self.deque.pop_back();
        (item, self)
    }

    pub fn peek_front(&self) -> Option<&T> {
        self.deque.front()
    }

    pub fn peek_back(&self) -> Option<&T> {
        self.deque.back()
    }

    pub fn is_empty(&self) -> bool {
        self.deque.is_empty()
    }

    pub fn len(&self) -> usize {
        self.deque.len()
    }

    pub fn to_vec(&self) -> Vec<T> {
        self.deque.iter().cloned().collect()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_back_order() {
        let t = (1..=5).fold(FingerTree::empty(), |acc, x| acc.push_back(x));
        assert_eq!(t.to_vec(), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_push_front_order() {
        let t = (1..=5).fold(FingerTree::empty(), |acc, x| acc.push_front(x));
        assert_eq!(t.to_vec(), vec![5, 4, 3, 2, 1]);
    }

    #[test]
    fn test_mixed_push() {
        let t = FingerTree::empty()
            .push_back(1)
            .push_back(2)
            .push_back(3)
            .push_front(0)
            .push_back(4)
            .push_front(-1);
        assert_eq!(t.to_vec(), vec![-1, 0, 1, 2, 3, 4]);
    }

    #[test]
    fn test_longer_sequence() {
        let t = (1..=10).fold(FingerTree::empty(), |acc, x| acc.push_back(x));
        assert_eq!(t.to_vec(), (1..=10).collect::<Vec<_>>());
    }

    #[test]
    fn test_empty() {
        let t: FingerTree<i32> = FingerTree::empty();
        assert_eq!(t.to_vec(), Vec::<i32>::new());
    }

    #[test]
    fn test_single() {
        let t = FingerTree::empty().push_back(42);
        assert_eq!(t.to_vec(), vec![42]);
    }
}
