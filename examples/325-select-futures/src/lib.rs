//! # Racing Futures with select!
//!
//! Demonstrates racing multiple tasks where the first one to complete wins
//! and others are discarded. Includes timeout patterns.

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

/// Race multiple labeled tasks. Returns the label and result of the first to complete.
pub fn race<T>(tasks: Vec<(&'static str, Box<dyn FnOnce() -> T + Send>)>) -> (&'static str, T)
where
    T: Send + 'static,
{
    let (tx, rx) = mpsc::channel();

    for (label, f) in tasks {
        let tx = tx.clone();
        thread::spawn(move || {
            let _ = tx.send((label, f()));
        });
    }

    rx.recv().expect("all senders dropped")
}

/// Race tasks without labels.
pub fn race_anonymous<T>(tasks: Vec<Box<dyn FnOnce() -> T + Send>>) -> T
where
    T: Send + 'static,
{
    let (tx, rx) = mpsc::channel();

    for f in tasks {
        let tx = tx.clone();
        thread::spawn(move || {
            let _ = tx.send(f());
        });
    }

    rx.recv().expect("all senders dropped")
}

/// Run a task with a timeout. Returns None if the timeout fires first.
pub fn with_timeout<T>(f: Box<dyn FnOnce() -> T + Send>, timeout_ms: u64) -> Option<T>
where
    T: Send + 'static,
{
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let _ = tx.send(f());
    });

    rx.recv_timeout(Duration::from_millis(timeout_ms)).ok()
}

/// Run a task with a timeout, returning a Result with a descriptive error.
pub fn with_timeout_result<T>(
    f: Box<dyn FnOnce() -> T + Send>,
    timeout_ms: u64,
) -> Result<T, TimeoutError>
where
    T: Send + 'static,
{
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let _ = tx.send(f());
    });

    rx.recv_timeout(Duration::from_millis(timeout_ms))
        .map_err(|_| TimeoutError { timeout_ms })
}

/// Error type for timeout operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TimeoutError {
    pub timeout_ms: u64,
}

impl std::fmt::Display for TimeoutError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "operation timed out after {}ms", self.timeout_ms)
    }
}

impl std::error::Error for TimeoutError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_race_fastest_wins() {
        let tasks: Vec<(&'static str, Box<dyn FnOnce() -> i32 + Send>)> = vec![
            (
                "slow",
                Box::new(|| {
                    thread::sleep(Duration::from_millis(100));
                    1
                }),
            ),
            (
                "fast",
                Box::new(|| {
                    thread::sleep(Duration::from_millis(10));
                    2
                }),
            ),
        ];
        let (label, value) = race(tasks);
        assert_eq!(label, "fast");
        assert_eq!(value, 2);
    }

    #[test]
    fn test_race_anonymous_returns_first() {
        let tasks: Vec<Box<dyn FnOnce() -> i32 + Send>> = vec![
            Box::new(|| {
                thread::sleep(Duration::from_millis(50));
                100
            }),
            Box::new(|| {
                thread::sleep(Duration::from_millis(5));
                42
            }),
        ];
        let result = race_anonymous(tasks);
        assert_eq!(result, 42);
    }

    #[test]
    fn test_with_timeout_succeeds() {
        let result = with_timeout(
            Box::new(|| {
                thread::sleep(Duration::from_millis(5));
                99
            }),
            200,
        );
        assert_eq!(result, Some(99));
    }

    #[test]
    fn test_with_timeout_fails() {
        let result = with_timeout(
            Box::new(|| {
                thread::sleep(Duration::from_millis(200));
                0
            }),
            50,
        );
        assert_eq!(result, None);
    }

    #[test]
    fn test_with_timeout_result_error() {
        let result = with_timeout_result(
            Box::new(|| {
                thread::sleep(Duration::from_millis(200));
                0
            }),
            50,
        );
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().timeout_ms, 50);
    }

    #[test]
    fn test_timeout_error_display() {
        let err = TimeoutError { timeout_ms: 100 };
        assert_eq!(err.to_string(), "operation timed out after 100ms");
    }
}
