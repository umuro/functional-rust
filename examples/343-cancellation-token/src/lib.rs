#![allow(clippy::all)]
// 343: Cancellation Token
// Arc<AtomicBool> for cooperative cancellation across threads

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

// Approach 1: Simple cancellation token
fn make_token() -> Arc<AtomicBool> {
    Arc::new(AtomicBool::new(false))
}

fn cancel(token: &AtomicBool) {
    token.store(true, Ordering::Relaxed);
}

fn is_cancelled(token: &AtomicBool) -> bool {
    token.load(Ordering::Relaxed)
}

// Approach 2: Worker that respects cancellation
fn worker(token: Arc<AtomicBool>, name: String) -> String {
    let mut count = 0u64;
    while !is_cancelled(&token) && count < 1_000_000 {
        count += 1;
    }
    format!("{} did {} iterations", name, count)
}

// Approach 3: Multi-worker with shared token
fn run_workers(n: usize) -> Vec<String> {
    let token = make_token();
    let handles: Vec<_> = (0..n)
        .map(|i| {
            let t = token.clone();
            let name = format!("worker-{}", i);
            thread::spawn(move || worker(t, name))
        })
        .collect();

    thread::sleep(Duration::from_millis(1));
    cancel(&token);

    handles.into_iter().map(|h| h.join().unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token() {
        let token = make_token();
        assert!(!is_cancelled(&token));
        cancel(&token);
        assert!(is_cancelled(&token));
    }

    #[test]
    fn test_worker_cancellation() {
        let token = make_token();
        let t = token.clone();
        let handle = thread::spawn(move || worker(t, "test".into()));
        thread::sleep(Duration::from_millis(1));
        cancel(&token);
        let result = handle.join().unwrap();
        assert!(result.starts_with("test"));
    }

    #[test]
    fn test_multi_workers() {
        let results = run_workers(3);
        assert_eq!(results.len(), 3);
        for r in &results {
            assert!(r.contains("worker-"));
        }
    }
}
