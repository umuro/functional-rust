//! # Spawning Concurrent Tasks
//!
//! Demonstrates spawning tasks to run in the background independently.
//! Fire and forget, or collect results later via handles.

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

/// Spawns a worker that completes after a delay and returns a message.
pub fn spawn_worker(id: usize, delay_ms: u64) -> thread::JoinHandle<String> {
    thread::spawn(move || {
        thread::sleep(Duration::from_millis(delay_ms));
        format!("worker-{} done after {}ms", id, delay_ms)
    })
}

/// Spawns multiple workers and returns their handles.
pub fn spawn_workers(count: usize) -> Vec<thread::JoinHandle<String>> {
    (0..count)
        .map(|i| spawn_worker(i, ((count - i) * 10) as u64))
        .collect()
}

/// Spawns workers that send results through a channel.
/// Returns results in completion order (not spawn order).
pub fn spawn_with_channel(n: usize) -> Vec<String> {
    let (tx, rx) = mpsc::channel();

    for i in 0..n {
        let tx = tx.clone();
        thread::spawn(move || {
            thread::sleep(Duration::from_millis((n - i) as u64 * 5));
            tx.send(format!("task-{}", i)).unwrap();
        });
    }

    drop(tx); // Important: drop original sender so receiver knows when to stop
    rx.into_iter().collect()
}

/// Spawns a task that returns a computed value.
pub fn spawn_compute<T, F>(f: F) -> thread::JoinHandle<T>
where
    T: Send + 'static,
    F: FnOnce() -> T + Send + 'static,
{
    thread::spawn(f)
}

/// Spawns N parallel computations and collects results.
pub fn parallel_map<T, U, F>(items: Vec<T>, f: F) -> Vec<U>
where
    T: Send + 'static,
    U: Send + 'static,
    F: Fn(T) -> U + Send + Sync + 'static + Clone,
{
    let handles: Vec<_> = items
        .into_iter()
        .map(|item| {
            let f = f.clone();
            thread::spawn(move || f(item))
        })
        .collect();

    handles
        .into_iter()
        .map(|h| h.join().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spawn_worker_returns_message() {
        let handle = spawn_worker(42, 1);
        let result = handle.join().unwrap();
        assert!(result.contains("worker-42"));
    }

    #[test]
    fn test_spawn_workers_all_complete() {
        let handles = spawn_workers(3);
        let results: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();
        assert_eq!(results.len(), 3);
    }

    #[test]
    fn test_spawn_with_channel_collects_all() {
        let results = spawn_with_channel(5);
        assert_eq!(results.len(), 5);
        // Results may be in any order due to concurrency
    }

    #[test]
    fn test_spawn_compute() {
        let handle = spawn_compute(|| 2 + 2);
        assert_eq!(handle.join().unwrap(), 4);
    }

    #[test]
    fn test_parallel_map() {
        let items = vec![1, 2, 3, 4, 5];
        let results = parallel_map(items, |x| x * x);
        assert_eq!(results, vec![1, 4, 9, 16, 25]);
    }

    #[test]
    fn test_parallel_map_preserves_order() {
        let items = vec!["a", "bb", "ccc"];
        let results = parallel_map(items, |s| s.len());
        assert_eq!(results, vec![1, 2, 3]);
    }
}
