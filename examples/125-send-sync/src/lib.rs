#![allow(clippy::all)]
// Example 125: Send and Sync Marker Traits
//
// Send: a type whose ownership can be transferred to another thread.
// Sync: a type whose references (&T) can be shared between threads.
// Both are auto-implemented by the compiler for types whose fields satisfy them.
// Violating them (e.g. sharing Rc across threads) is a *compile error*.

use std::sync::{mpsc, Arc, Mutex};
use std::thread;

// ---------------------------------------------------------------------------
// Solution 1: Idiomatic — Arc<Mutex<T>> for shared mutable state
//
// Arc<T>   is Send + Sync when T: Send (atomic ref-count is thread-safe)
// Mutex<T> is Send + Sync when T: Send (lock enforces exclusive access)
// ---------------------------------------------------------------------------
pub fn parallel_sum(numbers: Vec<i32>) -> i32 {
    let total = Arc::new(Mutex::new(0i32));
    let mid = numbers.len() / 2;
    let (left, right) = numbers.split_at(mid);
    let left = left.to_vec();
    let right = right.to_vec();

    // Clone the Arc — each thread gets its own handle to the same Mutex.
    let total_clone = Arc::clone(&total);
    let handle = thread::spawn(move || {
        // left: Vec<i32> is Send, so this closure is Send.
        let partial: i32 = left.iter().sum();
        *total_clone.lock().unwrap() += partial;
    });

    let partial: i32 = right.iter().sum();
    *total.lock().unwrap() += partial;

    handle.join().unwrap();
    let result = *total.lock().unwrap();
    result
}

// ---------------------------------------------------------------------------
// Solution 2: Functional — channel-based (mpsc) scatter/gather
//
// Sender<T> is Send when T: Send.  Values flow through the channel without
// shared mutable state, matching OCaml's typical concurrent style.
// ---------------------------------------------------------------------------
pub fn channel_sum(numbers: Vec<i32>) -> i32 {
    let (tx, rx) = mpsc::channel::<i32>();
    let mid = numbers.len() / 2;
    let (left, right) = numbers.split_at(mid);
    let left = left.to_vec();
    let right = right.to_vec();

    let tx2 = tx.clone();
    thread::spawn(move || {
        let partial: i32 = left.iter().sum();
        tx2.send(partial).unwrap();
    });

    let partial: i32 = right.iter().sum();
    tx.send(partial).unwrap();

    // Collect exactly 2 partial sums.
    rx.iter().take(2).sum()
}

// ---------------------------------------------------------------------------
// Solution 3: Demonstrate Send explicitly via thread::spawn type constraints.
//
// thread::spawn requires F: Send + 'static.  Immutable data moved into the
// closure satisfies this automatically when T: Send.
// ---------------------------------------------------------------------------
pub fn spawn_and_collect<T, F, R>(items: Vec<T>, f: F) -> R
where
    T: Send + 'static,
    F: FnOnce(Vec<T>) -> R + Send + 'static,
    R: Send + 'static,
{
    thread::spawn(move || f(items)).join().unwrap()
}

// ---------------------------------------------------------------------------
// Illustrative wrapper: show that Sync allows shared reads.
// Arc<Vec<i32>> — Vec<i32>: Sync, so &Vec<i32> can cross thread boundaries.
// ---------------------------------------------------------------------------
pub fn shared_read_sum(data: Arc<Vec<i32>>) -> i32 {
    let data2 = Arc::clone(&data);
    let handle = thread::spawn(move || data2.iter().sum::<i32>());
    handle.join().unwrap()
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    #[test]
    fn test_parallel_sum_basic() {
        assert_eq!(parallel_sum(vec![1, 2, 3, 4, 5]), 15);
    }

    #[test]
    fn test_parallel_sum_empty() {
        assert_eq!(parallel_sum(vec![]), 0);
    }

    #[test]
    fn test_parallel_sum_single() {
        assert_eq!(parallel_sum(vec![42]), 42);
    }

    #[test]
    fn test_channel_sum_basic() {
        assert_eq!(channel_sum(vec![10, 20, 30, 40]), 100);
    }

    #[test]
    fn test_channel_sum_empty() {
        assert_eq!(channel_sum(vec![]), 0);
    }

    #[test]
    fn test_spawn_and_collect() {
        let result = spawn_and_collect(vec![1, 2, 3, 4, 5], |v| v.iter().sum::<i32>());
        assert_eq!(result, 15);
    }

    #[test]
    fn test_shared_read_sum() {
        let data = Arc::new(vec![1, 2, 3, 4, 5]);
        assert_eq!(shared_read_sum(data), 15);
    }

    #[test]
    fn test_arc_mutex_counter() {
        // Verify Arc<Mutex<T>> correctly serialises increments across threads.
        let counter = Arc::new(Mutex::new(0u32));
        let handles: Vec<_> = (0..10)
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
        assert_eq!(*counter.lock().unwrap(), 10);
    }
}
