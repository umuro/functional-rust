struct CircularBuffer<T> {
    data: Vec<Option<T>>,
    head: usize,
    tail: usize,
    size: usize,
    capacity: usize,
}

impl<T> CircularBuffer<T> {
    fn new(capacity: usize) -> Self {
        assert!(capacity > 0);
        let mut data = Vec::with_capacity(capacity);
        for _ in 0..capacity { data.push(None); }
        Self { data, head: 0, tail: 0, size: 0, capacity }
    }

    fn push(&mut self, val: T) -> Result<(), &'static str> {
        if self.is_full() { return Err("buffer full"); }
        self.data[self.tail] = Some(val);
        self.tail = (self.tail + 1) % self.capacity;
        self.size += 1;
        Ok(())
    }

    fn pop(&mut self) -> Option<T> {
        if self.is_empty() { return None; }
        let val = self.data[self.head].take();
        self.head = (self.head + 1) % self.capacity;
        self.size -= 1;
        val
    }

    fn peek(&self) -> Option<&T> {
        self.data[self.head].as_ref()
    }

    fn push_overwrite(&mut self, val: T) {
        if self.is_full() { self.pop(); } // drop oldest
        self.push(val).unwrap();
    }

    fn is_empty(&self) -> bool { self.size == 0 }
    fn is_full(&self) -> bool { self.size == self.capacity }
    fn len(&self) -> usize { self.size }
}

fn main() {
    let mut buf: CircularBuffer<i32> = CircularBuffer::new(4);
    buf.push(1).unwrap(); buf.push(2).unwrap(); buf.push(3).unwrap();
    println!("Pop: {:?}", buf.pop());
    buf.push(4).unwrap(); buf.push(5).unwrap();
    while let Some(v) = buf.pop() { print!("{v} "); }
    println!();

    // Overwrite mode (ring buffer of fixed size)
    let mut ring: CircularBuffer<i32> = CircularBuffer::new(3);
    for i in 0..6 { ring.push_overwrite(i); }
    println!("Ring (size 3, filled with 0..5):");
    while let Some(v) = ring.pop() { print!("{v} "); }
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn fifo_order() {
        let mut b: CircularBuffer<i32> = CircularBuffer::new(4);
        b.push(1).unwrap(); b.push(2).unwrap(); b.push(3).unwrap();
        assert_eq!(b.pop(), Some(1)); assert_eq!(b.pop(), Some(2));
    }
    #[test] fn full_err() {
        let mut b: CircularBuffer<i32> = CircularBuffer::new(2);
        b.push(1).unwrap(); b.push(2).unwrap();
        assert!(b.push(3).is_err());
    }
    #[test] fn overwrite_oldest() {
        let mut b: CircularBuffer<i32> = CircularBuffer::new(3);
        for i in 0..5 { b.push_overwrite(i); }
        assert_eq!(b.pop(), Some(2));
    }
}
