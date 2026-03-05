use std::collections::VecDeque;

struct BatchSink<T> {
    buffer: VecDeque<T>,
    capacity: usize,
    flushed_batches: Vec<Vec<T>>,
}

impl<T: Clone> BatchSink<T> {
    fn new(capacity: usize) -> Self {
        Self { buffer: VecDeque::new(), capacity, flushed_batches: Vec::new() }
    }
    fn send(&mut self, item: T) -> Result<(), String> {
        self.buffer.push_back(item);
        if self.buffer.len() >= self.capacity { self.flush()?; }
        Ok(())
    }
    fn flush(&mut self) -> Result<(), String> {
        if !self.buffer.is_empty() {
            let b: Vec<T> = self.buffer.drain(..).collect();
            println!("Flushing {} items", b.len());
            self.flushed_batches.push(b);
        }
        Ok(())
    }
    fn into_batches(self) -> Vec<Vec<T>> { self.flushed_batches }
}

fn main() {
    let mut s: BatchSink<i32> = BatchSink::new(3);
    for i in 1..=8 { s.send(i).unwrap(); }
    s.flush().unwrap();
    println!("Batches: {}", s.into_batches().len());
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn flushes_in_batches() {
        let mut s: BatchSink<i32> = BatchSink::new(3);
        for i in 1..=9 { s.send(i).unwrap(); }
        let b = s.into_batches();
        assert_eq!(b.len(), 3);
        assert_eq!(b[0], vec![1,2,3]);
    }
    #[test] fn partial_flush() {
        let mut s: BatchSink<i32> = BatchSink::new(5);
        for i in 1..=3 { s.send(i).unwrap(); }
        s.flush().unwrap();
        let b = s.into_batches();
        assert_eq!(b[0], vec![1,2,3]);
    }
}
