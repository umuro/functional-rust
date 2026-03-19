#![allow(clippy::all)]
//! # Running Futures Concurrently with join!
//!
//! Demonstrates concurrent execution where all tasks run simultaneously
//! and we wait for ALL of them to complete. Total time is max(individual), not sum.

use std::thread;
use std::time::Duration;

/// A slow addition that simulates I/O latency.
pub fn slow_add(a: i32, b: i32, delay_ms: u64) -> i32 {
    thread::sleep(Duration::from_millis(delay_ms));
    a + b
}

/// Approach 1: Join all tasks using threads.
/// Spawns all tasks first, then waits for all to complete.
/// Time is max(tasks), not sum(tasks).
pub fn join_all<T, F>(tasks: Vec<F>) -> Vec<T>
where
    T: Send + 'static,
    F: FnOnce() -> T + Send + 'static,
{
    // Phase 1: spawn everything (all start running now)
    let handles: Vec<_> = tasks.into_iter().map(|f| thread::spawn(f)).collect();

    // Phase 2: collect results (wait for each to finish)
    handles
        .into_iter()
        .map(|h| h.join().expect("task panicked"))
        .collect()
}

/// Approach 2: Join with labels for debugging.
pub fn join_all_labeled<T, F>(tasks: Vec<(&'static str, F)>) -> Vec<(&'static str, T)>
where
    T: Send + 'static,
    F: FnOnce() -> T + Send + 'static,
{
    let handles: Vec<_> = tasks
        .into_iter()
        .map(|(label, f)| {
            let handle = thread::spawn(f);
            (label, handle)
        })
        .collect();

    handles
        .into_iter()
        .map(|(label, h)| (label, h.join().expect("task panicked")))
        .collect()
}

/// Approach 3: Join exactly two tasks and return a tuple.
/// More ergonomic for common two-task patterns.
pub fn join_pair<A, B, FA, FB>(task_a: FA, task_b: FB) -> (A, B)
where
    A: Send + 'static,
    B: Send + 'static,
    FA: FnOnce() -> A + Send + 'static,
    FB: FnOnce() -> B + Send + 'static,
{
    let handle_a = thread::spawn(task_a);
    let handle_b = thread::spawn(task_b);
    (
        handle_a.join().expect("task A panicked"),
        handle_b.join().expect("task B panicked"),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    #[test]
    fn test_join_all_returns_all_results() {
        let tasks: Vec<Box<dyn FnOnce() -> i32 + Send>> =
            vec![Box::new(|| 1 + 1), Box::new(|| 2 + 2), Box::new(|| 3 + 3)];
        let results = join_all(tasks);
        assert_eq!(results, vec![2, 4, 6]);
    }

    #[test]
    fn test_join_all_concurrent_faster_than_sequential() {
        let start = Instant::now();

        let tasks: Vec<Box<dyn FnOnce() -> i32 + Send>> = vec![
            Box::new(|| {
                thread::sleep(Duration::from_millis(30));
                1
            }),
            Box::new(|| {
                thread::sleep(Duration::from_millis(30));
                2
            }),
        ];
        let _ = join_all(tasks);

        // If sequential, would take ~60ms. Concurrent should be ~30ms.
        assert!(
            start.elapsed() < Duration::from_millis(55),
            "Should be concurrent, not sequential"
        );
    }

    #[test]
    fn test_join_all_preserves_order() {
        let tasks: Vec<Box<dyn FnOnce() -> i32 + Send>> = vec![
            Box::new(|| {
                thread::sleep(Duration::from_millis(30));
                1
            }),
            Box::new(|| {
                thread::sleep(Duration::from_millis(10));
                2
            }),
            Box::new(|| {
                thread::sleep(Duration::from_millis(20));
                3
            }),
        ];
        let results = join_all(tasks);
        // Order should match input order, not completion order
        assert_eq!(results, vec![1, 2, 3]);
    }

    #[test]
    fn test_join_pair_different_types() {
        let (s, n) = join_pair(|| "hello".to_string(), || 42);
        assert_eq!(s, "hello");
        assert_eq!(n, 42);
    }

    #[test]
    fn test_join_all_labeled() {
        let tasks: Vec<(&'static str, Box<dyn FnOnce() -> i32 + Send>)> =
            vec![("first", Box::new(|| 10)), ("second", Box::new(|| 20))];
        let results = join_all_labeled(tasks);
        assert_eq!(results, vec![("first", 10), ("second", 20)]);
    }

    #[test]
    fn test_join_all_empty() {
        let tasks: Vec<Box<dyn FnOnce() -> i32 + Send>> = vec![];
        let results = join_all(tasks);
        assert!(results.is_empty());
    }
}
