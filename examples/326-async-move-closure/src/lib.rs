#![allow(clippy::all)]
//! # Capturing with async move
//!
//! Demonstrates how `move` closures capture their environment by value,
//! enabling them to outlive their creating scope - essential for async tasks.

use std::sync::{Arc, Mutex};
use std::thread;

/// Creates a greeter closure that captures the name by value.
/// The returned closure owns `name` and can be called from anywhere.
pub fn make_greeter(name: String) -> impl Fn() -> String {
    move || format!("Hello, {}!", name)
}

/// Creates a counter closure that maintains mutable state.
/// Each call increments and returns the previous value.
pub fn make_counter(start: i32) -> impl FnMut() -> i32 {
    let mut count = start;
    move || {
        let current = count;
        count += 1;
        current
    }
}

/// Creates a stateful accumulator that can be reset.
pub fn make_accumulator() -> impl FnMut(i32) -> i32 {
    let mut total = 0;
    move |delta| {
        total += delta;
        total
    }
}

/// Demonstrates shared state across threads using Arc<Mutex<T>>.
/// Each thread increments a shared counter - the pattern for async move blocks.
pub fn shared_counter_demo(num_threads: usize) -> i32 {
    let shared = Arc::new(Mutex::new(0));

    let handles: Vec<_> = (0..num_threads)
        .map(|_| {
            let shared = Arc::clone(&shared); // Clone Arc before moving
            thread::spawn(move || {
                // Each thread owns its Arc handle
                let mut guard = shared.lock().unwrap();
                *guard += 1;
            })
        })
        .collect();

    for h in handles {
        h.join().unwrap();
    }

    let result = *shared.lock().unwrap();
    result
}

/// Demonstrates capturing multiple values in a move closure.
pub fn compute_with_context(base: i32, multiplier: i32, offset: i32) -> impl FnOnce(i32) -> i32 {
    move |x| (base + x) * multiplier + offset
}

/// Factory that creates worker closures with captured configuration.
pub fn make_workers(prefix: String, count: usize) -> Vec<Box<dyn Fn(i32) -> String + Send>> {
    (0..count)
        .map(|id| {
            let prefix = prefix.clone(); // Clone for each closure
            Box::new(move |value: i32| format!("{}-worker-{}: {}", prefix, id, value))
                as Box<dyn Fn(i32) -> String + Send>
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greeter_captures_name() {
        let greet = make_greeter("Alice".to_string());
        assert_eq!(greet(), "Hello, Alice!");
    }

    #[test]
    fn test_counter_increments() {
        let mut counter = make_counter(10);
        assert_eq!(counter(), 10);
        assert_eq!(counter(), 11);
        assert_eq!(counter(), 12);
    }

    #[test]
    fn test_accumulator() {
        let mut acc = make_accumulator();
        assert_eq!(acc(5), 5);
        assert_eq!(acc(3), 8);
        assert_eq!(acc(-2), 6);
    }

    #[test]
    fn test_shared_counter_counts_all_threads() {
        let result = shared_counter_demo(5);
        assert_eq!(result, 5);
    }

    #[test]
    fn test_compute_with_context() {
        let compute = compute_with_context(10, 2, 5);
        // (10 + 3) * 2 + 5 = 31
        assert_eq!(compute(3), 31);
    }

    #[test]
    fn test_make_workers() {
        let workers = make_workers("test".to_string(), 3);
        assert_eq!(workers.len(), 3);
        assert_eq!(workers[0](42), "test-worker-0: 42");
        assert_eq!(workers[1](100), "test-worker-1: 100");
    }
}
