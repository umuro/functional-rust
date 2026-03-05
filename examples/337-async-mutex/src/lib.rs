//! # Async Mutex
//!
//! Lock shared state safely across async tasks — demonstrates correct patterns
//! for using `std::sync::Mutex` and avoiding deadlocks across await points.

use std::sync::{Arc, Mutex};
use std::thread;

/// Demonstrates concurrent increments with a mutex.
pub fn concurrent_increment(num_threads: usize) -> i32 {
    let counter = Arc::new(Mutex::new(0));

    let handles: Vec<_> = (0..num_threads)
        .map(|_| {
            let c = Arc::clone(&counter);
            thread::spawn(move || {
                *c.lock().unwrap() += 1;
            })
        })
        .collect();

    for h in handles {
        h.join().unwrap();
    }

    let result = *counter.lock().unwrap();
    result
}

/// Demonstrates the correct pattern: release lock before doing other work.
pub fn correct_lock_pattern(data: Vec<i32>) -> i32 {
    let shared = Arc::new(Mutex::new(data));

    // CORRECT: compute value inside a scope, guard drops at scope end
    let sum = {
        let guard = shared.lock().unwrap();
        guard.iter().sum::<i32>()
    }; // guard drops here, lock released BEFORE any other work

    sum
}

/// Demonstrates safe read-modify-write pattern.
pub fn safe_update<F>(mutex: &Mutex<i32>, f: F) -> i32
where
    F: FnOnce(i32) -> i32,
{
    let mut guard = mutex.lock().unwrap();
    *guard = f(*guard);
    *guard
}

/// Demonstrates poison recovery after a panic.
pub fn with_poison_recovery(mutex: &Mutex<i32>) -> Result<i32, i32> {
    match mutex.lock() {
        Ok(guard) => Ok(*guard),
        Err(poisoned) => {
            // Recover by accessing the data anyway
            let recovered = poisoned.into_inner();
            Err(*recovered)
        }
    }
}

/// A thread-safe counter using Mutex.
pub struct Counter {
    value: Mutex<i32>,
}

impl Counter {
    pub fn new(initial: i32) -> Self {
        Self {
            value: Mutex::new(initial),
        }
    }

    pub fn increment(&self) -> i32 {
        let mut guard = self.value.lock().unwrap();
        *guard += 1;
        *guard
    }

    pub fn get(&self) -> i32 {
        *self.value.lock().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_concurrent_increment() {
        assert_eq!(concurrent_increment(10), 10);
    }

    #[test]
    fn test_high_contention() {
        let counter = Arc::new(Mutex::new(0));
        let handles: Vec<_> = (0..100)
            .map(|_| {
                let c = Arc::clone(&counter);
                thread::spawn(move || {
                    *c.lock().unwrap() += 1;
                })
            })
            .collect();

        for h in handles {
            h.join().unwrap();
        }

        assert_eq!(*counter.lock().unwrap(), 100);
    }

    #[test]
    fn test_correct_lock_pattern() {
        let sum = correct_lock_pattern(vec![1, 2, 3, 4, 5]);
        assert_eq!(sum, 15);
    }

    #[test]
    fn test_safe_update() {
        let m = Mutex::new(10);
        let result = safe_update(&m, |x| x * 2);
        assert_eq!(result, 20);
    }

    #[test]
    fn test_counter() {
        let counter = Counter::new(0);
        assert_eq!(counter.increment(), 1);
        assert_eq!(counter.increment(), 2);
        assert_eq!(counter.get(), 2);
    }
}
