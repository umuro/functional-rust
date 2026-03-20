#![allow(clippy::all)]
//! # Producer-Consumer Pattern
//! Classic concurrent pattern with bounded buffer.

use std::collections::VecDeque;
use std::sync::{mpsc, Arc, Condvar, Mutex};
use std::thread;

pub struct BoundedBuffer<T> {
    data: Mutex<VecDeque<T>>,
    capacity: usize,
    not_empty: Condvar,
    not_full: Condvar,
}

impl<T> BoundedBuffer<T> {
    pub fn new(capacity: usize) -> Arc<Self> {
        Arc::new(Self {
            data: Mutex::new(VecDeque::new()),
            capacity,
            not_empty: Condvar::new(),
            not_full: Condvar::new(),
        })
    }

    pub fn put(&self, item: T) {
        let mut data = self.data.lock().unwrap();
        while data.len() >= self.capacity {
            data = self.not_full.wait(data).unwrap();
        }
        data.push_back(item);
        self.not_empty.notify_one();
    }

    pub fn take(&self) -> T {
        let mut data = self.data.lock().unwrap();
        while data.is_empty() {
            data = self.not_empty.wait(data).unwrap();
        }
        let item = data.pop_front().unwrap();
        self.not_full.notify_one();
        item
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn producer_consumer() {
        let buffer = BoundedBuffer::new(2);
        let b1 = Arc::clone(&buffer);
        let producer = thread::spawn(move || {
            for i in 0..5 {
                b1.put(i);
            }
        });
        let mut results = Vec::new();
        for _ in 0..5 {
            results.push(buffer.take());
        }
        producer.join().unwrap();
        assert_eq!(results, vec![0, 1, 2, 3, 4]);
    }
}
