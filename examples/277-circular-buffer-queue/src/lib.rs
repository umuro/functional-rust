#![allow(clippy::all)]
/// A functional queue with amortized O(1) operations using two `Vec`s.
///
/// Elements are enqueued onto `back` and dequeued from `front`.
/// When `front` is empty, `back` is reversed into `front` — this gives
/// amortized O(1) per operation, matching OCaml's classic two-list queue.
#[derive(Debug, Clone)]
pub struct Queue<T> {
    front: Vec<T>,
    back: Vec<T>,
}

impl<T> Queue<T> {
    /// Create an empty queue.
    pub fn new() -> Self {
        Queue {
            front: Vec::new(),
            back: Vec::new(),
        }
    }

    /// Check if the queue is empty.
    pub fn is_empty(&self) -> bool {
        self.front.is_empty() && self.back.is_empty()
    }

    /// Enqueue an element at the back — returns a new queue (functional style).
    pub fn enqueue(mut self, x: T) -> Self {
        self.back.push(x);
        self
    }

    /// Dequeue an element from the front — returns the element and the remaining queue.
    /// Returns `None` if the queue is empty.
    ///
    /// When `front` is empty, reverses `back` into `front` (amortized O(1)).
    pub fn dequeue(mut self) -> Option<(T, Self)> {
        if self.front.is_empty() {
            if self.back.is_empty() {
                return None;
            }
            self.back.reverse();
            std::mem::swap(&mut self.front, &mut self.back);
        }
        // front is non-empty — pop from the end (which is the oldest element
        // after reversal). This is O(1) and maintains FIFO order.
        let head = self.front.pop().unwrap();
        Some((head, self))
    }

    /// Convert the queue to a `Vec` in FIFO order.
    /// `front` is stored reversed (oldest at end), so we iterate it in reverse.
    /// `back` is stored in push order (oldest first).
    pub fn to_vec(&self) -> Vec<&T> {
        self.front.iter().rev().chain(self.back.iter()).collect()
    }

    /// Return the number of elements in the queue.
    pub fn len(&self) -> usize {
        self.front.len() + self.back.len()
    }
}

impl<T> Default for Queue<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// Recursive drain — collects all elements from the queue in FIFO order.
/// Mirrors the OCaml recursive `drain` function.
pub fn drain_recursive<T>(queue: Queue<T>) -> Vec<T> {
    match queue.dequeue() {
        None => Vec::new(),
        Some((x, rest)) => {
            let mut result = vec![x];
            result.extend(drain_recursive(rest));
            result
        }
    }
}

/// Iterative drain — more idiomatic Rust, avoids stack depth issues.
pub fn drain_iterative<T>(mut queue: Queue<T>) -> Vec<T> {
    let mut result = Vec::new();
    while let Some((x, rest)) = queue.dequeue() {
        result.push(x);
        queue = rest;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_queue() {
        let q: Queue<i32> = Queue::new();
        assert!(q.is_empty());
        assert_eq!(q.len(), 0);
        assert!(q.dequeue().is_none());
    }

    #[test]
    fn test_single_element() {
        let q = Queue::new().enqueue(42);
        assert!(!q.is_empty());
        assert_eq!(q.len(), 1);
        let (val, q2) = q.dequeue().unwrap();
        assert_eq!(val, 42);
        assert!(q2.is_empty());
    }

    #[test]
    fn test_fifo_order() {
        let q = Queue::new().enqueue(1).enqueue(2).enqueue(3);
        let drained = drain_iterative(q);
        assert_eq!(drained, vec![1, 2, 3]);
    }

    #[test]
    fn test_recursive_drain() {
        let q = Queue::new().enqueue(1).enqueue(2).enqueue(3);
        let drained = drain_recursive(q);
        assert_eq!(drained, vec![1, 2, 3]);
    }

    #[test]
    fn test_to_vec() {
        let q = Queue::new().enqueue(1).enqueue(2).enqueue(3);
        let v = q.to_vec();
        assert_eq!(v, vec![&1, &2, &3]);
    }

    #[test]
    fn test_interleaved_operations() {
        let q = Queue::new().enqueue(1).enqueue(2);
        let (val, q) = q.dequeue().unwrap();
        assert_eq!(val, 1);
        let q = q.enqueue(3);
        let drained = drain_iterative(q);
        assert_eq!(drained, vec![2, 3]);
    }

    #[test]
    fn test_many_elements() {
        let mut q = Queue::new();
        for i in 0..100 {
            q = q.enqueue(i);
        }
        let drained = drain_iterative(q);
        let expected: Vec<i32> = (0..100).collect();
        assert_eq!(drained, expected);
    }
}
