// 997: Retry with Exponential Backoff
// Pure functional combinator — wraps any FnMut() -> Result<T,E>

use std::thread;
use std::time::Duration;

// --- Core retry with exponential backoff ---
fn retry<T, E, F>(max_attempts: usize, base_delay_ms: u64, mut f: F) -> Result<T, E>
where
    F: FnMut() -> Result<T, E>,
{
    let mut last_err = None;
    for attempt in 0..max_attempts {
        match f() {
            Ok(v) => return Ok(v),
            Err(e) => {
                last_err = Some(e);
                if attempt + 1 < max_attempts {
                    let delay = base_delay_ms * (1 << attempt);
                    thread::sleep(Duration::from_millis(delay));
                }
            }
        }
    }
    Err(last_err.unwrap())
}

// --- Retry with jitter to avoid thundering herd ---
fn retry_with_jitter<T, E, F>(max_attempts: usize, base_delay_ms: u64, mut f: F) -> Result<T, E>
where
    F: FnMut() -> Result<T, E>,
{
    let mut last_err = None;
    for attempt in 0..max_attempts {
        match f() {
            Ok(v) => return Ok(v),
            Err(e) => {
                last_err = Some(e);
                if attempt + 1 < max_attempts {
                    let base = base_delay_ms * (1 << attempt);
                    // Simple deterministic "jitter" using attempt number
                    let jitter = base / 3 * (attempt as u64 % 3);
                    thread::sleep(Duration::from_millis(base + jitter));
                }
            }
        }
    }
    Err(last_err.unwrap())
}

// --- Retry only for retryable errors ---
#[derive(Debug, PartialEq)]
enum MyError {
    Transient(String),
    Permanent(String),
}

fn retry_if<T, F, P>(
    max_attempts: usize,
    base_delay_ms: u64,
    is_retryable: P,
    mut f: F,
) -> Result<T, MyError>
where
    F: FnMut() -> Result<T, MyError>,
    P: Fn(&MyError) -> bool,
{
    let mut last_err = None;
    for attempt in 0..max_attempts {
        match f() {
            Ok(v) => return Ok(v),
            Err(e) if !is_retryable(&e) => return Err(e),
            Err(e) => {
                last_err = Some(e);
                if attempt + 1 < max_attempts {
                    let delay = base_delay_ms * (1 << attempt);
                    thread::sleep(Duration::from_millis(delay));
                }
            }
        }
    }
    Err(last_err.unwrap())
}

// --- Builder pattern for ergonomic retry configuration ---
struct RetryConfig {
    max_attempts: usize,
    base_delay_ms: u64,
}

impl RetryConfig {
    fn new() -> Self { RetryConfig { max_attempts: 3, base_delay_ms: 100 } }
    fn attempts(mut self, n: usize) -> Self { self.max_attempts = n; self }
    fn base_delay(mut self, ms: u64) -> Self { self.base_delay_ms = ms; self }

    fn run<T, E, F: FnMut() -> Result<T, E>>(&self, f: F) -> Result<T, E> {
        retry(self.max_attempts, self.base_delay_ms, f)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_retry_succeeds_on_third_attempt() {
        let mut count = 0;
        let result = retry(5, 1, || {
            count += 1;
            if count < 3 { Err("fail") } else { Ok(count) }
        });
        assert_eq!(result, Ok(3));
        assert_eq!(count, 3);
    }

    #[test]
    fn test_retry_exhausts_attempts() {
        let mut count = 0;
        let result = retry::<i32, &str, _>(3, 1, || {
            count += 1;
            Err("always fails")
        });
        assert!(result.is_err());
        assert_eq!(count, 3);
    }

    #[test]
    fn test_retry_succeeds_first_try() {
        let result = retry::<i32, &str, _>(3, 1, || Ok(42));
        assert_eq!(result, Ok(42));
    }

    #[test]
    fn test_retry_if_stops_on_permanent() {
        let mut count = 0;
        let result = retry_if(
            5, 1,
            |e| matches!(e, MyError::Transient(_)),
            || {
                count += 1;
                Err(MyError::Permanent("unrecoverable".to_string()))
            },
        );
        assert!(result.is_err());
        assert_eq!(count, 1); // stopped immediately on Permanent error
    }

    #[test]
    fn test_retry_if_retries_transient() {
        let mut count = 0;
        let result = retry_if(
            5, 1,
            |e| matches!(e, MyError::Transient(_)),
            || {
                count += 1;
                if count < 3 { Err(MyError::Transient("temp".to_string())) } else { Ok(42) }
            },
        );
        assert_eq!(result, Ok(42));
        assert_eq!(count, 3);
    }

    #[test]
    fn test_builder_config() {
        let mut n = 0;
        let r = RetryConfig::new().attempts(4).base_delay(1).run(|| {
            n += 1;
            if n < 2 { Err("x") } else { Ok(n) }
        });
        assert_eq!(r, Ok(2));
    }
}
