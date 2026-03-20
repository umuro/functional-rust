#![allow(clippy::all)]
//! Circular Buffer (Ring Buffer)
//!
//! Fixed-size FIFO queue with O(1) operations.

/// A circular buffer with fixed capacity
pub struct CircularBuffer<T> {
    data: Vec<Option<T>>,
    head: usize,
    tail: usize,
    size: usize,
    capacity: usize,
}

impl<T> CircularBuffer<T> {
    /// Create a new circular buffer with given capacity
    pub fn new(capacity: usize) -> Self {
        assert!(capacity > 0, "capacity must be > 0");
        let mut data = Vec::with_capacity(capacity);
        for _ in 0..capacity {
            data.push(None);
        }
        Self {
            data,
            head: 0,
            tail: 0,
            size: 0,
            capacity,
        }
    }

    /// Push an element to the back (returns error if full)
    pub fn push(&mut self, val: T) -> Result<(), &'static str> {
        if self.is_full() {
            return Err("buffer full");
        }
        self.data[self.tail] = Some(val);
        self.tail = (self.tail + 1) % self.capacity;
        self.size += 1;
        Ok(())
    }

    /// Pop element from front
    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        let val = self.data[self.head].take();
        self.head = (self.head + 1) % self.capacity;
        self.size -= 1;
        val
    }

    /// Peek at front element without removing
    pub fn peek(&self) -> Option<&T> {
        if self.is_empty() {
            None
        } else {
            self.data[self.head].as_ref()
        }
    }

    /// Push with overwrite - drops oldest if full
    pub fn push_overwrite(&mut self, val: T) {
        if self.is_full() {
            self.pop();
        }
        self.push(val).unwrap();
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    /// Check if full
    pub fn is_full(&self) -> bool {
        self.size == self.capacity
    }

    /// Get current size
    pub fn len(&self) -> usize {
        self.size
    }

    /// Get capacity
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// Clear all elements
    pub fn clear(&mut self) {
        while self.pop().is_some() {}
    }

    /// Iterate over elements (front to back)
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        let mut items = Vec::with_capacity(self.size);
        let mut i = self.head;
        for _ in 0..self.size {
            if let Some(ref val) = self.data[i] {
                items.push(val);
            }
            i = (i + 1) % self.capacity;
        }
        items.into_iter()
    }
}

impl<T: Clone> CircularBuffer<T> {
    /// Convert to Vec
    pub fn to_vec(&self) -> Vec<T> {
        self.iter().cloned().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fifo_order() {
        let mut buf: CircularBuffer<i32> = CircularBuffer::new(4);
        buf.push(1).unwrap();
        buf.push(2).unwrap();
        buf.push(3).unwrap();
        assert_eq!(buf.pop(), Some(1));
        assert_eq!(buf.pop(), Some(2));
        assert_eq!(buf.pop(), Some(3));
    }

    #[test]
    fn test_full_error() {
        let mut buf: CircularBuffer<i32> = CircularBuffer::new(2);
        buf.push(1).unwrap();
        buf.push(2).unwrap();
        assert!(buf.push(3).is_err());
    }

    #[test]
    fn test_overwrite_oldest() {
        let mut buf: CircularBuffer<i32> = CircularBuffer::new(3);
        for i in 0..5 {
            buf.push_overwrite(i);
        }
        assert_eq!(buf.pop(), Some(2));
        assert_eq!(buf.pop(), Some(3));
        assert_eq!(buf.pop(), Some(4));
    }

    #[test]
    fn test_peek() {
        let mut buf: CircularBuffer<i32> = CircularBuffer::new(3);
        buf.push(42).unwrap();
        assert_eq!(buf.peek(), Some(&42));
        assert_eq!(buf.len(), 1); // peek doesn't remove
    }

    #[test]
    fn test_wrap_around() {
        let mut buf: CircularBuffer<i32> = CircularBuffer::new(3);
        buf.push(1).unwrap();
        buf.push(2).unwrap();
        buf.pop();
        buf.push(3).unwrap();
        buf.push(4).unwrap();
        assert_eq!(buf.to_vec(), vec![2, 3, 4]);
    }

    #[test]
    fn test_clear() {
        let mut buf: CircularBuffer<i32> = CircularBuffer::new(3);
        buf.push(1).unwrap();
        buf.push(2).unwrap();
        buf.clear();
        assert!(buf.is_empty());
    }
}
