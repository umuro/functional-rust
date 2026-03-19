//! # Async Basics: Sequential vs Concurrent Execution
//!
//! Demonstrates the fundamental difference between sequential and concurrent
//! execution patterns that form the basis of async programming.

use std::thread;
use std::time::Duration;

/// Simulates fetching a user by ID with some latency.
pub fn fetch_user(id: u32) -> String {
    thread::sleep(Duration::from_millis(10));
    format!("User({})", id)
}

/// Simulates fetching posts for a user with some latency.
pub fn fetch_posts(user_id: u32) -> Vec<String> {
    thread::sleep(Duration::from_millis(8));
    vec![
        format!("Post1 by {}", user_id),
        format!("Post2 by {}", user_id),
    ]
}

/// Approach 1: Sequential fetch - each operation blocks until complete.
/// Like: `let user = fetch_user(id).await; let posts = fetch_posts(id).await;`
pub fn sequential_fetch(id: u32) -> (String, Vec<String>) {
    let user = fetch_user(id);
    let posts = fetch_posts(id);
    (user, posts)
}

/// Approach 2: Concurrent fetch using threads.
/// Like: `join!(fetch_user(id), fetch_posts(id))`
pub fn concurrent_fetch(id: u32) -> (String, Vec<String>) {
    let handle_user = thread::spawn(move || fetch_user(id));
    let handle_posts = thread::spawn(move || fetch_posts(id));

    let user = handle_user.join().expect("user thread panicked");
    let posts = handle_posts.join().expect("posts thread panicked");
    (user, posts)
}

/// Approach 3: Generic concurrent executor for multiple tasks.
/// Returns results in the same order as input tasks.
pub fn run_concurrent<T, F>(tasks: Vec<F>) -> Vec<T>
where
    T: Send + 'static,
    F: FnOnce() -> T + Send + 'static,
{
    let handles: Vec<_> = tasks.into_iter().map(|task| thread::spawn(task)).collect();

    handles
        .into_iter()
        .map(|h| h.join().expect("task panicked"))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    #[test]
    fn test_sequential_fetch_returns_correct_user() {
        let (user, _) = sequential_fetch(42);
        assert_eq!(user, "User(42)");
    }

    #[test]
    fn test_sequential_fetch_returns_correct_posts() {
        let (_, posts) = sequential_fetch(7);
        assert_eq!(posts.len(), 2);
        assert!(posts[0].contains("7"));
    }

    #[test]
    fn test_concurrent_fetch_same_results_as_sequential() {
        let (user1, posts1) = sequential_fetch(99);
        let (user2, posts2) = concurrent_fetch(99);
        assert_eq!(user1, user2);
        assert_eq!(posts1, posts2);
    }

    #[test]
    fn test_concurrent_is_faster_than_sequential() {
        let start_seq = Instant::now();
        let _ = sequential_fetch(1);
        let seq_time = start_seq.elapsed();

        let start_conc = Instant::now();
        let _ = concurrent_fetch(1);
        let conc_time = start_conc.elapsed();

        // Concurrent should be faster (both operations overlap)
        assert!(
            conc_time < seq_time,
            "Concurrent ({:?}) should be faster than sequential ({:?})",
            conc_time,
            seq_time
        );
    }

    #[test]
    fn test_run_concurrent_preserves_order() {
        let tasks: Vec<Box<dyn FnOnce() -> i32 + Send>> = vec![
            Box::new(|| {
                thread::sleep(Duration::from_millis(20));
                1
            }),
            Box::new(|| {
                thread::sleep(Duration::from_millis(5));
                2
            }),
            Box::new(|| {
                thread::sleep(Duration::from_millis(10));
                3
            }),
        ];

        let results = run_concurrent(tasks);
        assert_eq!(results, vec![1, 2, 3]);
    }

    #[test]
    fn test_run_concurrent_empty_list() {
        let tasks: Vec<Box<dyn FnOnce() -> i32 + Send>> = vec![];
        let results = run_concurrent(tasks);
        assert!(results.is_empty());
    }
}
