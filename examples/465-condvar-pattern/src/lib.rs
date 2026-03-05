//! # Condition Variables — Wait for State Changes
//!
//! Wait efficiently for a condition to become true.

use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::time::Duration;

/// A simple flag that threads can wait on
pub struct WaitFlag {
    state: Mutex<bool>,
    condvar: Condvar,
}

impl WaitFlag {
    pub fn new() -> Self {
        Self {
            state: Mutex::new(false),
            condvar: Condvar::new(),
        }
    }

    /// Set the flag and wake waiters
    pub fn set(&self) {
        let mut state = self.state.lock().unwrap();
        *state = true;
        self.condvar.notify_all();
    }

    /// Wait for the flag to be set
    pub fn wait(&self) {
        let mut state = self.state.lock().unwrap();
        while !*state {
            state = self.condvar.wait(state).unwrap();
        }
    }

    /// Wait with timeout
    pub fn wait_timeout(&self, timeout: Duration) -> bool {
        let state = self.state.lock().unwrap();
        let (state, result) = self
            .condvar
            .wait_timeout_while(state, timeout, |s| !*s)
            .unwrap();
        *state
    }
}

impl Default for WaitFlag {
    fn default() -> Self {
        Self::new()
    }
}

/// Bounded blocking queue using condvar
pub struct BlockingQueue<T> {
    queue: Mutex<Vec<T>>,
    not_empty: Condvar,
    not_full: Condvar,
    capacity: usize,
}

impl<T> BlockingQueue<T> {
    pub fn new(capacity: usize) -> Self {
        Self {
            queue: Mutex::new(Vec::with_capacity(capacity)),
            not_empty: Condvar::new(),
            not_full: Condvar::new(),
            capacity,
        }
    }

    /// Push, blocking if full
    pub fn push(&self, item: T) {
        let mut queue = self.queue.lock().unwrap();
        while queue.len() >= self.capacity {
            queue = self.not_full.wait(queue).unwrap();
        }
        queue.push(item);
        self.not_empty.notify_one();
    }

    /// Pop, blocking if empty
    pub fn pop(&self) -> T {
        let mut queue = self.queue.lock().unwrap();
        while queue.is_empty() {
            queue = self.not_empty.wait(queue).unwrap();
        }
        let item = queue.remove(0);
        self.not_full.notify_one();
        item
    }

    /// Try pop without blocking
    pub fn try_pop(&self) -> Option<T> {
        let mut queue = self.queue.lock().unwrap();
        if queue.is_empty() {
            None
        } else {
            let item = queue.remove(0);
            self.not_full.notify_one();
            Some(item)
        }
    }

    pub fn len(&self) -> usize {
        self.queue.lock().unwrap().len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Producer-consumer using condvar
pub fn producer_consumer_demo(num_items: usize) -> Vec<i32> {
    let queue = Arc::new(BlockingQueue::new(5));
    let results = Arc::new(Mutex::new(Vec::new()));

    // Producer
    let q = Arc::clone(&queue);
    let producer = thread::spawn(move || {
        for i in 0..num_items as i32 {
            q.push(i);
        }
    });

    // Consumer
    let q = Arc::clone(&queue);
    let r = Arc::clone(&results);
    let consumer = thread::spawn(move || {
        for _ in 0..num_items {
            let item = q.pop();
            r.lock().unwrap().push(item);
        }
    });

    producer.join().unwrap();
    consumer.join().unwrap();

    Arc::try_unwrap(results).unwrap().into_inner().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wait_flag() {
        let flag = Arc::new(WaitFlag::new());

        let f = Arc::clone(&flag);
        let waiter = thread::spawn(move || {
            f.wait();
            true
        });

        thread::sleep(Duration::from_millis(10));
        flag.set();

        assert!(waiter.join().unwrap());
    }

    #[test]
    fn test_wait_timeout() {
        let flag = WaitFlag::new();
        assert!(!flag.wait_timeout(Duration::from_millis(10)));

        flag.set();
        assert!(flag.wait_timeout(Duration::from_millis(10)));
    }

    #[test]
    fn test_blocking_queue() {
        let queue = BlockingQueue::new(3);

        queue.push(1);
        queue.push(2);
        queue.push(3);

        assert_eq!(queue.len(), 3);
        assert_eq!(queue.pop(), 1);
        assert_eq!(queue.pop(), 2);
        assert_eq!(queue.len(), 1);
    }

    #[test]
    fn test_try_pop() {
        let queue = BlockingQueue::new(2);
        assert!(queue.try_pop().is_none());

        queue.push(42);
        assert_eq!(queue.try_pop(), Some(42));
        assert!(queue.try_pop().is_none());
    }

    #[test]
    fn test_producer_consumer() {
        let results = producer_consumer_demo(10);
        assert_eq!(results, (0..10).collect::<Vec<_>>());
    }

    #[test]
    fn test_concurrent_queue() {
        let queue = Arc::new(BlockingQueue::new(10));

        let producers: Vec<_> = (0..4)
            .map(|id| {
                let q = Arc::clone(&queue);
                thread::spawn(move || {
                    for i in 0..25 {
                        q.push(id * 100 + i);
                    }
                })
            })
            .collect();

        let consumers: Vec<_> = (0..2)
            .map(|_| {
                let q = Arc::clone(&queue);
                thread::spawn(move || {
                    let mut count = 0;
                    for _ in 0..50 {
                        let _ = q.pop();
                        count += 1;
                    }
                    count
                })
            })
            .collect();

        for p in producers {
            p.join().unwrap();
        }

        let total: i32 = consumers.into_iter().map(|c| c.join().unwrap()).sum();
        assert_eq!(total, 100);
    }
}
