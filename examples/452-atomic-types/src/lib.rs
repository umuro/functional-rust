#![allow(clippy::all)]
//! # Atomic Types — Lock-Free Concurrent Primitives
//!
//! Use atomic types for lock-free operations on shared data.
//! Atomics provide thread-safe operations without locks.

use std::sync::atomic::{AtomicBool, AtomicI64, AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;

/// Approach 1: Atomic counter
pub struct AtomicCounter {
    value: AtomicUsize,
}

impl AtomicCounter {
    pub fn new(initial: usize) -> Self {
        Self {
            value: AtomicUsize::new(initial),
        }
    }

    pub fn increment(&self) -> usize {
        self.value.fetch_add(1, Ordering::SeqCst)
    }

    pub fn decrement(&self) -> usize {
        self.value.fetch_sub(1, Ordering::SeqCst)
    }

    pub fn get(&self) -> usize {
        self.value.load(Ordering::SeqCst)
    }

    pub fn set(&self, value: usize) {
        self.value.store(value, Ordering::SeqCst);
    }
}

/// Approach 2: Atomic flag for signaling
pub struct AtomicFlag {
    flag: AtomicBool,
}

impl AtomicFlag {
    pub fn new() -> Self {
        Self {
            flag: AtomicBool::new(false),
        }
    }

    pub fn set(&self) {
        self.flag.store(true, Ordering::Release);
    }

    pub fn clear(&self) {
        self.flag.store(false, Ordering::Release);
    }

    pub fn is_set(&self) -> bool {
        self.flag.load(Ordering::Acquire)
    }

    /// Test and set: returns previous value
    pub fn test_and_set(&self) -> bool {
        self.flag.swap(true, Ordering::SeqCst)
    }
}

impl Default for AtomicFlag {
    fn default() -> Self {
        Self::new()
    }
}

/// Approach 3: Atomic max tracker
pub struct AtomicMax {
    value: AtomicI64,
}

impl AtomicMax {
    pub fn new(initial: i64) -> Self {
        Self {
            value: AtomicI64::new(initial),
        }
    }

    pub fn update(&self, new: i64) -> i64 {
        let mut current = self.value.load(Ordering::Relaxed);
        loop {
            if new <= current {
                return current;
            }
            match self.value.compare_exchange_weak(
                current,
                new,
                Ordering::SeqCst,
                Ordering::Relaxed,
            ) {
                Ok(_) => return new,
                Err(actual) => current = actual,
            }
        }
    }

    pub fn get(&self) -> i64 {
        self.value.load(Ordering::SeqCst)
    }
}

/// Parallel increment test
pub fn parallel_increment(threads: usize, increments: usize) -> usize {
    let counter = Arc::new(AtomicUsize::new(0));

    let handles: Vec<_> = (0..threads)
        .map(|_| {
            let c = Arc::clone(&counter);
            thread::spawn(move || {
                for _ in 0..increments {
                    c.fetch_add(1, Ordering::Relaxed);
                }
            })
        })
        .collect();

    for h in handles {
        h.join().unwrap();
    }

    counter.load(Ordering::SeqCst)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_atomic_counter_basic() {
        let counter = AtomicCounter::new(0);
        assert_eq!(counter.increment(), 0);
        assert_eq!(counter.increment(), 1);
        assert_eq!(counter.get(), 2);
    }

    #[test]
    fn test_atomic_counter_concurrent() {
        let counter = Arc::new(AtomicCounter::new(0));

        let handles: Vec<_> = (0..4)
            .map(|_| {
                let c = Arc::clone(&counter);
                thread::spawn(move || {
                    for _ in 0..100 {
                        c.increment();
                    }
                })
            })
            .collect();

        for h in handles {
            h.join().unwrap();
        }

        assert_eq!(counter.get(), 400);
    }

    #[test]
    fn test_atomic_flag() {
        let flag = AtomicFlag::new();
        assert!(!flag.is_set());

        flag.set();
        assert!(flag.is_set());

        flag.clear();
        assert!(!flag.is_set());
    }

    #[test]
    fn test_atomic_flag_test_and_set() {
        let flag = AtomicFlag::new();

        let prev = flag.test_and_set();
        assert!(!prev); // was false
        assert!(flag.is_set());

        let prev = flag.test_and_set();
        assert!(prev); // was true
    }

    #[test]
    fn test_atomic_max() {
        let max = AtomicMax::new(0);

        max.update(5);
        assert_eq!(max.get(), 5);

        max.update(3); // Less than current
        assert_eq!(max.get(), 5);

        max.update(10);
        assert_eq!(max.get(), 10);
    }

    #[test]
    fn test_parallel_increment() {
        let result = parallel_increment(4, 1000);
        assert_eq!(result, 4000);
    }

    #[test]
    fn test_fetch_operations() {
        let a = AtomicUsize::new(10);

        let old = a.fetch_add(5, Ordering::SeqCst);
        assert_eq!(old, 10);
        assert_eq!(a.load(Ordering::SeqCst), 15);

        let old = a.fetch_sub(3, Ordering::SeqCst);
        assert_eq!(old, 15);
        assert_eq!(a.load(Ordering::SeqCst), 12);
    }

    #[test]
    fn test_swap() {
        let flag = AtomicBool::new(false);
        let prev = flag.swap(true, Ordering::SeqCst);
        assert!(!prev);
        assert!(flag.load(Ordering::SeqCst));
    }
}
