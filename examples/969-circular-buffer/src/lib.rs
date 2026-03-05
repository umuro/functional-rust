// 969: Circular / Ring Buffer
// Fixed-capacity FIFO: push overwrites oldest when full
// OCaml: array + head/tail/count refs; Rust: Vec + indices

pub struct RingBuffer<T> {
    data: Vec<T>,
    capacity: usize,
    head: usize, // next read index
    tail: usize, // next write index
    count: usize,
}

impl<T: Default + Clone> RingBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        assert!(capacity > 0, "capacity must be > 0");
        RingBuffer {
            data: vec![T::default(); capacity],
            capacity,
            head: 0,
            tail: 0,
            count: 0,
        }
    }
}

impl<T: Clone> RingBuffer<T> {
    pub fn is_full(&self) -> bool { self.count == self.capacity }
    pub fn is_empty(&self) -> bool { self.count == 0 }
    pub fn size(&self) -> usize { self.count }

    /// Push: if full, overwrites oldest element
    pub fn push(&mut self, x: T) {
        self.data[self.tail] = x;
        self.tail = (self.tail + 1) % self.capacity;
        if self.is_full() {
            // Overwrite: advance head (drop oldest)
            self.head = (self.head + 1) % self.capacity;
        } else {
            self.count += 1;
        }
    }

    /// Pop: removes and returns oldest element
    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            let x = self.data[self.head].clone();
            self.head = (self.head + 1) % self.capacity;
            self.count -= 1;
            Some(x)
        }
    }

    /// Peek: view oldest without removing
    pub fn peek(&self) -> Option<&T> {
        if self.is_empty() {
            None
        } else {
            Some(&self.data[self.head])
        }
    }

    /// Collect all elements in order (oldest first)
    pub fn to_vec(&self) -> Vec<T> {
        (0..self.count)
            .map(|i| self.data[(self.head + i) % self.capacity].clone())
            .collect()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_push_pop() {
        let mut r: RingBuffer<i32> = RingBuffer::new(4);
        assert!(r.is_empty());
        r.push(1);
        r.push(2);
        r.push(3);
        r.push(4);
        assert!(r.is_full());
        assert_eq!(r.size(), 4);
        assert_eq!(r.peek(), Some(&1));
    }

    #[test]
    fn test_overwrite_on_full() {
        let mut r: RingBuffer<i32> = RingBuffer::new(4);
        r.push(1);
        r.push(2);
        r.push(3);
        r.push(4);
        r.push(5); // overwrites 1
        assert_eq!(r.size(), 4);
        assert_eq!(r.peek(), Some(&2));
        assert_eq!(r.to_vec(), vec![2, 3, 4, 5]);
    }

    #[test]
    fn test_pop_order() {
        let mut r: RingBuffer<i32> = RingBuffer::new(4);
        for i in 1..=4 { r.push(i); }
        assert_eq!(r.pop(), Some(1));
        assert_eq!(r.pop(), Some(2));
        assert_eq!(r.size(), 2);
    }

    #[test]
    fn test_wrap_around() {
        let mut r: RingBuffer<i32> = RingBuffer::new(4);
        for i in 1..=4 { r.push(i); }
        r.pop();
        r.pop();
        r.push(5);
        r.push(6);
        r.push(7); // overwrite
        assert_eq!(r.to_vec(), vec![4, 5, 6, 7]);
    }

    #[test]
    fn test_empty_pop() {
        let mut r: RingBuffer<i32> = RingBuffer::new(2);
        assert_eq!(r.pop(), None);
        assert_eq!(r.peek(), None);
    }
}
