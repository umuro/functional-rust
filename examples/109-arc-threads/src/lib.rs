#![allow(clippy::all)]
// Example 109: Arc<T> — Thread-Safe Shared Ownership
//
// Arc<T> = Atomic Reference Count. Like Rc<T> but thread-safe.
// Clone an Arc to share ownership across threads; the value is
// freed only after every thread drops its clone.

use std::sync::{Arc, Mutex};
use std::thread;

// --- Approach 1: Shared immutable data across threads ----------------------

/// Parallel sum: split a Vec into two halves, sum each half in its own thread.
/// The Vec is wrapped in Arc so both threads can read it without copying.
pub fn parallel_sum(data: Arc<Vec<i32>>) -> i32 {
    let mid = data.len() / 2;

    // Clone the Arc (bumps atomic counter) — no heap allocation of data.
    let left = Arc::clone(&data);
    let handle_left = thread::spawn(move || left[..mid].iter().sum::<i32>());

    let right = Arc::clone(&data);
    let handle_right = thread::spawn(move || right[mid..].iter().sum::<i32>());

    handle_left.join().unwrap() + handle_right.join().unwrap()
}

// --- Approach 2: Map-reduce with Arc-shared configuration ------------------

/// A processing configuration shared read-only across worker threads.
#[derive(Debug)]
pub struct Config {
    pub multiplier: i32,
    pub offset: i32,
}

/// Apply `config` to every element across `n_threads` worker threads.
/// Each thread owns a clone of the Arc — the Config is never copied.
pub fn parallel_map(data: Vec<i32>, config: Arc<Config>, n_threads: usize) -> Vec<i32> {
    let data = Arc::new(data);
    let len = data.len();
    let chunk = len.div_ceil(n_threads);

    let handles: Vec<_> = (0..n_threads)
        .map(|i| {
            let data = Arc::clone(&data);
            let cfg = Arc::clone(&config);
            thread::spawn(move || {
                let start = i * chunk;
                let end = (start + chunk).min(len);
                data[start..end]
                    .iter()
                    .map(|&x| x * cfg.multiplier + cfg.offset)
                    .collect::<Vec<i32>>()
            })
        })
        .collect();

    handles
        .into_iter()
        .flat_map(|h| h.join().unwrap())
        .collect()
}

// --- Approach 3: Arc<Mutex<T>> for shared mutable state --------------------

/// Accumulate results from multiple threads into a shared counter.
/// Arc owns the Mutex; Mutex guards the i32 inside.
pub fn concurrent_count(items: Vec<i32>) -> i32 {
    let total: Arc<Mutex<i32>> = Arc::new(Mutex::new(0));

    let handles: Vec<_> = items
        .into_iter()
        .map(|x| {
            let total = Arc::clone(&total);
            thread::spawn(move || {
                let mut guard = total.lock().unwrap();
                *guard += x;
            })
        })
        .collect();

    for h in handles {
        h.join().unwrap();
    }

    let result = *total.lock().unwrap();
    result
}

/// Demonstrate Arc reference counting: count rises with clones, falls with drops.
pub fn arc_ref_count_demo() -> (usize, usize) {
    let a: Arc<Vec<i32>> = Arc::new(vec![1, 2, 3]);
    let b = Arc::clone(&a);
    let count_two = Arc::strong_count(&a); // 2: `a` + `b`
    drop(b);
    let count_one = Arc::strong_count(&a); // 1: only `a`
    (count_two, count_one)
}

// --------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parallel_sum_correctness() {
        let data = Arc::new((1..=100).collect::<Vec<i32>>());
        assert_eq!(parallel_sum(data), 5050);
    }

    #[test]
    fn test_parallel_sum_empty() {
        let data = Arc::new(vec![]);
        assert_eq!(parallel_sum(data), 0);
    }

    #[test]
    fn test_parallel_sum_single() {
        let data = Arc::new(vec![42]);
        assert_eq!(parallel_sum(data), 42);
    }

    #[test]
    fn test_parallel_map_applies_config() {
        let cfg = Arc::new(Config {
            multiplier: 2,
            offset: 1,
        });
        let result = parallel_map(vec![1, 2, 3, 4], cfg, 2);
        // Each x → x*2 + 1
        assert_eq!(result, vec![3, 5, 7, 9]);
    }

    #[test]
    fn test_parallel_map_single_thread() {
        let cfg = Arc::new(Config {
            multiplier: 3,
            offset: 0,
        });
        let result = parallel_map(vec![1, 2, 3], cfg, 1);
        assert_eq!(result, vec![3, 6, 9]);
    }

    #[test]
    fn test_concurrent_count() {
        let items: Vec<i32> = (1..=10).collect();
        assert_eq!(concurrent_count(items), 55);
    }

    #[test]
    fn test_concurrent_count_empty() {
        assert_eq!(concurrent_count(vec![]), 0);
    }

    #[test]
    fn test_arc_ref_count() {
        let (two, one) = arc_ref_count_demo();
        assert_eq!(two, 2);
        assert_eq!(one, 1);
    }
}
