#![allow(clippy::all)]
//! # Arc<Mutex<T>> — Shared Mutable State Across Threads
//!
//! Share a single mutable value across multiple threads using `Arc` for
//! ownership and `Mutex` for exclusive access.

use std::sync::{Arc, Mutex};
use std::thread;

/// Approach 1: Shared counter with multiple threads
pub fn parallel_increment(num_threads: usize, increments_per_thread: usize) -> u64 {
    let counter = Arc::new(Mutex::new(0u64));

    let handles: Vec<_> = (0..num_threads)
        .map(|_| {
            let c = Arc::clone(&counter);
            thread::spawn(move || {
                for _ in 0..increments_per_thread {
                    *c.lock().unwrap() += 1;
                }
            })
        })
        .collect();

    for h in handles {
        h.join().unwrap();
    }

    let result = *counter.lock().unwrap();
    result
}

/// Approach 2: Shared collection (Vec)
pub fn parallel_collect<T, F>(num_threads: usize, producer: F) -> Vec<T>
where
    T: Send + std::fmt::Debug + 'static,
    F: Fn(usize) -> T + Send + Sync + 'static + Clone,
{
    let results: Arc<Mutex<Vec<T>>> = Arc::new(Mutex::new(Vec::new()));

    let handles: Vec<_> = (0..num_threads)
        .map(|i| {
            let results = Arc::clone(&results);
            let producer = producer.clone();
            thread::spawn(move || {
                let value = producer(i);
                results.lock().unwrap().push(value);
            })
        })
        .collect();

    for h in handles {
        h.join().unwrap();
    }

    Arc::try_unwrap(results)
        .expect("all threads joined")
        .into_inner()
        .unwrap()
}

/// Approach 3: try_lock for non-blocking access
pub fn try_lock_demo() -> Option<u64> {
    let data = Arc::new(Mutex::new(42u64));
    let data_clone = Arc::clone(&data);

    // Hold the lock in main thread
    let _guard = data.lock().unwrap();

    // Another thread tries to get it
    let handle = thread::spawn(move || {
        // try_lock returns Err if lock is held
        match data_clone.try_lock() {
            Ok(mut guard) => {
                *guard += 1;
                Some(*guard)
            }
            Err(_) => None, // Lock was held
        }
    });

    handle.join().unwrap()
}

/// Thread-safe accumulator struct
pub struct SharedAccumulator {
    value: Arc<Mutex<i64>>,
}

impl SharedAccumulator {
    pub fn new(initial: i64) -> Self {
        Self {
            value: Arc::new(Mutex::new(initial)),
        }
    }

    pub fn add(&self, amount: i64) {
        *self.value.lock().unwrap() += amount;
    }

    pub fn get(&self) -> i64 {
        *self.value.lock().unwrap()
    }

    pub fn clone_handle(&self) -> Arc<Mutex<i64>> {
        Arc::clone(&self.value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parallel_increment_10_threads() {
        let result = parallel_increment(10, 100);
        assert_eq!(result, 1000);
    }

    #[test]
    fn test_parallel_increment_single_thread() {
        let result = parallel_increment(1, 500);
        assert_eq!(result, 500);
    }

    #[test]
    fn test_parallel_collect() {
        let mut results = parallel_collect(4, |i| format!("thread-{}", i));
        results.sort();
        assert_eq!(results.len(), 4);
        assert!(results.contains(&String::from("thread-0")));
        assert!(results.contains(&String::from("thread-3")));
    }

    #[test]
    fn test_try_lock_fails_when_held() {
        let result = try_lock_demo();
        assert_eq!(result, None);
    }

    #[test]
    fn test_try_lock_succeeds_when_free() {
        let m = Mutex::new(0);
        {
            let guard = m.try_lock();
            assert!(guard.is_ok());
        }
        // Lock released, try again
        assert!(m.try_lock().is_ok());
    }

    #[test]
    fn test_shared_accumulator() {
        let acc = SharedAccumulator::new(0);
        let handle = acc.clone_handle();

        thread::scope(|s| {
            s.spawn(|| {
                for _ in 0..100 {
                    *handle.lock().unwrap() += 1;
                }
            });
            s.spawn(|| {
                for _ in 0..100 {
                    acc.add(1);
                }
            });
        });

        assert_eq!(acc.get(), 200);
    }

    #[test]
    fn test_mutex_guard_drops_on_scope_exit() {
        let m = Mutex::new(vec![1, 2, 3]);
        {
            let mut guard = m.lock().unwrap();
            guard.push(4);
        } // guard drops here
        assert_eq!(m.lock().unwrap().len(), 4);
    }

    #[test]
    fn test_arc_clone_count() {
        let data = Arc::new(Mutex::new(0));
        assert_eq!(Arc::strong_count(&data), 1);

        let clone1 = Arc::clone(&data);
        assert_eq!(Arc::strong_count(&data), 2);

        drop(clone1);
        assert_eq!(Arc::strong_count(&data), 1);
    }
}
