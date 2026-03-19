//! # Thread Basics — Spawn and Join
//!
//! Launch OS threads with `std::thread::spawn`, collect results with
//! `JoinHandle::join`, and catch panics without crashing the process.

use std::thread::{self, JoinHandle};
use std::time::Duration;

/// Approach 1: Spawn multiple threads and collect their results
///
/// Maps work items to threads and joins them to collect results.
pub fn parallel_compute<T, R, F>(items: Vec<T>, f: F) -> Vec<R>
where
    T: Send + 'static,
    R: Send + 'static,
    F: Fn(T) -> R + Send + Sync + 'static + Clone,
{
    let handles: Vec<JoinHandle<R>> = items
        .into_iter()
        .map(|item| {
            let f = f.clone();
            thread::spawn(move || f(item))
        })
        .collect();

    handles
        .into_iter()
        .map(|h| h.join().expect("thread panicked"))
        .collect()
}

/// Approach 2: Simple spawn and join pattern
///
/// Spawn a single thread with a computation and wait for the result.
pub fn spawn_and_join<T, F>(f: F) -> Result<T, Box<dyn std::any::Any + Send>>
where
    T: Send + 'static,
    F: FnOnce() -> T + Send + 'static,
{
    thread::spawn(f).join()
}

/// Approach 3: Spawn with delay (simulating work)
///
/// Spawns threads with configurable delays to simulate varying workloads.
pub fn spawn_with_delays(count: usize, delay_ms: u64) -> Vec<usize> {
    let handles: Vec<_> = (0..count)
        .map(|i| {
            thread::spawn(move || {
                thread::sleep(Duration::from_millis(delay_ms * i as u64));
                i * i
            })
        })
        .collect();

    handles.into_iter().map(|h| h.join().unwrap()).collect()
}

/// Check if a thread panic is safely contained
pub fn panic_contained() -> bool {
    let handle = thread::spawn(|| -> i32 { panic!("intentional panic") });
    handle.join().is_err()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_spawn_and_join_success() {
        let result = spawn_and_join(|| 42u32);
        assert_eq!(result.unwrap(), 42);
    }

    #[test]
    fn test_spawn_and_join_computation() {
        let result = spawn_and_join(|| {
            let mut sum = 0u64;
            for i in 1..=100 {
                sum += i;
            }
            sum
        });
        assert_eq!(result.unwrap(), 5050);
    }

    #[test]
    fn test_multiple_threads() {
        let handles: Vec<_> = (0..8u32).map(|i| thread::spawn(move || i * 2)).collect();

        let results: Vec<u32> = handles.into_iter().map(|h| h.join().unwrap()).collect();

        assert_eq!(results, vec![0, 2, 4, 6, 8, 10, 12, 14]);
    }

    #[test]
    fn test_panic_is_caught() {
        let handle = thread::spawn(|| panic!("boom"));
        assert!(handle.join().is_err());
    }

    #[test]
    fn test_panic_contained_helper() {
        assert!(panic_contained());
    }

    #[test]
    fn test_spawn_with_delays() {
        let results = spawn_with_delays(4, 1);
        assert_eq!(results, vec![0, 1, 4, 9]);
    }

    #[test]
    fn test_thread_returns_string() {
        let result = spawn_and_join(|| String::from("hello from thread"));
        assert_eq!(result.unwrap(), "hello from thread");
    }

    #[test]
    fn test_thread_captures_value() {
        let value = 100;
        let result = thread::spawn(move || value * 2).join().unwrap();
        assert_eq!(result, 200);
    }
}
