//! # Timeouts with time::timeout
//!
//! Wrap any async operation with a deadline — if it doesn't complete in time,
//! get a structured error instead of waiting forever.

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

/// Error type distinguishing timeout from operation failure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TimeoutError<E> {
    /// Operation took too long
    Elapsed,
    /// Operation ran but returned an error
    TaskFailed(E),
}

impl<E: std::fmt::Display> std::fmt::Display for TimeoutError<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Elapsed => write!(f, "operation timed out"),
            Self::TaskFailed(e) => write!(f, "task failed: {}", e),
        }
    }
}

impl<E: std::fmt::Debug + std::fmt::Display> std::error::Error for TimeoutError<E> {}

/// Run a function with a timeout. Returns Err(TimeoutError::Elapsed) if it exceeds the deadline.
pub fn with_timeout<T, E>(
    timeout: Duration,
    f: impl FnOnce() -> Result<T, E> + Send + 'static,
) -> Result<T, TimeoutError<E>>
where
    T: Send + 'static,
    E: Send + 'static,
{
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let _ = tx.send(f());
    });

    match rx.recv_timeout(timeout) {
        Ok(Ok(value)) => Ok(value),
        Ok(Err(e)) => Err(TimeoutError::TaskFailed(e)),
        Err(mpsc::RecvTimeoutError::Timeout) => Err(TimeoutError::Elapsed),
        Err(mpsc::RecvTimeoutError::Disconnected) => {
            Err(TimeoutError::TaskFailed(panic!("thread disconnected")))
        }
    }
}

/// Simplified timeout for operations that can't fail (except by timeout).
pub fn with_timeout_simple<T>(timeout: Duration, f: impl FnOnce() -> T + Send + 'static) -> Option<T>
where
    T: Send + 'static,
{
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let _ = tx.send(f());
    });
    rx.recv_timeout(timeout).ok()
}

/// Run multiple operations with individual timeouts, returning the first success.
pub fn first_success_with_timeout<T, E>(
    timeout_each: Duration,
    operations: Vec<Box<dyn FnOnce() -> Result<T, E> + Send>>,
) -> Result<T, TimeoutError<E>>
where
    T: Send + 'static,
    E: Send + 'static + Clone,
{
    let mut last_error = None;

    for op in operations {
        match with_timeout(timeout_each, op) {
            Ok(v) => return Ok(v),
            Err(e) => last_error = Some(e),
        }
    }

    Err(last_error.unwrap_or(TimeoutError::Elapsed))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn slow_operation(delay_ms: u64, value: i32) -> Result<i32, String> {
        thread::sleep(Duration::from_millis(delay_ms));
        Ok(value)
    }

    #[test]
    fn test_completes_before_timeout() {
        let result = with_timeout(Duration::from_millis(200), || slow_operation(10, 42));
        assert_eq!(result.unwrap(), 42);
    }

    #[test]
    fn test_times_out() {
        let result = with_timeout(Duration::from_millis(10), || slow_operation(500, 0));
        assert!(matches!(result, Err(TimeoutError::Elapsed)));
    }

    #[test]
    fn test_task_error_propagates() {
        let result: Result<i32, TimeoutError<String>> =
            with_timeout(Duration::from_millis(100), || Err("failed".to_string()));
        assert!(matches!(result, Err(TimeoutError::TaskFailed(_))));
    }

    #[test]
    fn test_timeout_simple_success() {
        let result = with_timeout_simple(Duration::from_millis(100), || {
            thread::sleep(Duration::from_millis(5));
            42
        });
        assert_eq!(result, Some(42));
    }

    #[test]
    fn test_timeout_simple_failure() {
        let result = with_timeout_simple(Duration::from_millis(10), || {
            thread::sleep(Duration::from_millis(200));
            42
        });
        assert_eq!(result, None);
    }
}
