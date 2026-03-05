//! # Retry Async
//!
//! Retry failed operations with exponential backoff —
//! the foundation of resilient async services.

use std::thread;
use std::time::Duration;

/// Error type distinguishing transient from permanent failures.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RetryError<E> {
    /// Worth retrying — network blip, timeout, overload
    Transient(E),
    /// Don't retry — bad input, auth failure, 404
    Permanent(E),
}

/// Configuration for retry behavior.
#[derive(Debug, Clone)]
pub struct RetryConfig {
    pub max_attempts: usize,
    pub base_delay: Duration,
    pub multiplier: f64,
    pub max_delay: Duration,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_attempts: 3,
            base_delay: Duration::from_millis(100),
            multiplier: 2.0,
            max_delay: Duration::from_secs(30),
        }
    }
}

impl RetryConfig {
    pub fn with_attempts(mut self, n: usize) -> Self {
        self.max_attempts = n;
        self
    }

    pub fn with_base_delay(mut self, d: Duration) -> Self {
        self.base_delay = d;
        self
    }
}

/// Retry an operation with exponential backoff.
pub fn retry<T, E: Clone>(
    config: &RetryConfig,
    mut operation: impl FnMut() -> Result<T, RetryError<E>>,
) -> Result<T, E> {
    let mut delay = config.base_delay;
    let mut last_error = None;

    for attempt in 1..=config.max_attempts {
        match operation() {
            Ok(value) => return Ok(value),
            Err(RetryError::Permanent(e)) => return Err(e),
            Err(RetryError::Transient(e)) => {
                last_error = Some(e);
                if attempt < config.max_attempts {
                    thread::sleep(delay);
                    delay = delay.mul_f64(config.multiplier).min(config.max_delay);
                }
            }
        }
    }

    Err(last_error.unwrap())
}

/// Retry with a simple predicate to determine if error is transient.
pub fn retry_if<T, E: Clone>(
    config: &RetryConfig,
    is_transient: impl Fn(&E) -> bool,
    mut operation: impl FnMut() -> Result<T, E>,
) -> Result<T, E> {
    retry(config, || match operation() {
        Ok(v) => Ok(v),
        Err(e) => {
            if is_transient(&e) {
                Err(RetryError::Transient(e))
            } else {
                Err(RetryError::Permanent(e))
            }
        }
    })
}

/// Simple retry with fixed delay (no exponential backoff).
pub fn retry_fixed<T, E: Clone>(
    max_attempts: usize,
    delay: Duration,
    mut operation: impl FnMut() -> Result<T, RetryError<E>>,
) -> Result<T, E> {
    let mut last_error = None;

    for attempt in 1..=max_attempts {
        match operation() {
            Ok(value) => return Ok(value),
            Err(RetryError::Permanent(e)) => return Err(e),
            Err(RetryError::Transient(e)) => {
                last_error = Some(e);
                if attempt < max_attempts {
                    thread::sleep(delay);
                }
            }
        }
    }

    Err(last_error.unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;

    #[test]
    fn test_succeeds_after_retries() {
        let counter = Arc::new(AtomicUsize::new(0));
        let c = Arc::clone(&counter);

        let config = RetryConfig::default()
            .with_attempts(5)
            .with_base_delay(Duration::from_millis(1));

        let result: Result<i32, String> = retry(&config, move || {
            let n = c.fetch_add(1, Ordering::SeqCst);
            if n < 2 {
                Err(RetryError::Transient(format!("attempt {}", n + 1)))
            } else {
                Ok(42)
            }
        });

        assert_eq!(result.unwrap(), 42);
        assert_eq!(counter.load(Ordering::SeqCst), 3);
    }

    #[test]
    fn test_permanent_error_no_retry() {
        let counter = Arc::new(AtomicUsize::new(0));
        let c = Arc::clone(&counter);

        let config = RetryConfig::default();

        let result: Result<i32, String> = retry(&config, move || {
            c.fetch_add(1, Ordering::SeqCst);
            Err(RetryError::Permanent("fatal".to_string()))
        });

        assert!(result.is_err());
        assert_eq!(counter.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn test_exhausts_all_attempts() {
        let counter = Arc::new(AtomicUsize::new(0));
        let c = Arc::clone(&counter);

        let config = RetryConfig::default()
            .with_attempts(3)
            .with_base_delay(Duration::from_millis(1));

        let result: Result<i32, String> = retry(&config, move || {
            c.fetch_add(1, Ordering::SeqCst);
            Err(RetryError::Transient("still failing".to_string()))
        });

        assert!(result.is_err());
        assert_eq!(counter.load(Ordering::SeqCst), 3);
    }

    #[test]
    fn test_retry_if() {
        let counter = Arc::new(AtomicUsize::new(0));
        let c = Arc::clone(&counter);

        let config = RetryConfig::default()
            .with_attempts(5)
            .with_base_delay(Duration::from_millis(1));

        let result: Result<i32, i32> = retry_if(
            &config,
            |e| *e == 500, // only retry 500 errors
            move || {
                let n = c.fetch_add(1, Ordering::SeqCst);
                if n < 2 {
                    Err(500)
                } else {
                    Ok(42)
                }
            },
        );

        assert_eq!(result.unwrap(), 42);
    }

    #[test]
    fn test_retry_fixed() {
        let counter = Arc::new(AtomicUsize::new(0));
        let c = Arc::clone(&counter);

        let result: Result<i32, String> = retry_fixed(3, Duration::from_millis(1), move || {
            let n = c.fetch_add(1, Ordering::SeqCst);
            if n < 1 {
                Err(RetryError::Transient("not yet".to_string()))
            } else {
                Ok(99)
            }
        });

        assert_eq!(result.unwrap(), 99);
    }
}
