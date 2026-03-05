// 997: Retry with Exponential Backoff — Tokio version
// Async retry with tokio::time::sleep for backoff delays

use std::time::Duration;

/// Core async retry with exponential backoff
async fn retry<T, E, F, Fut>(max_attempts: usize, base_delay_ms: u64, mut f: F) -> Result<T, E>
where
    F: FnMut() -> Fut,
    Fut: std::future::Future<Output = Result<T, E>>,
{
    let mut last_err = None;
    for attempt in 0..max_attempts {
        match f().await {
            Ok(v) => return Ok(v),
            Err(e) => {
                last_err = Some(e);
                if attempt + 1 < max_attempts {
                    let delay = base_delay_ms * (1 << attempt);
                    tokio::time::sleep(Duration::from_millis(delay)).await;
                }
            }
        }
    }
    Err(last_err.unwrap())
}

/// Retry with jitter
async fn retry_with_jitter<T, E, F, Fut>(max_attempts: usize, base_delay_ms: u64, mut f: F) -> Result<T, E>
where
    F: FnMut() -> Fut,
    Fut: std::future::Future<Output = Result<T, E>>,
{
    let mut last_err = None;
    for attempt in 0..max_attempts {
        match f().await {
            Ok(v) => return Ok(v),
            Err(e) => {
                last_err = Some(e);
                if attempt + 1 < max_attempts {
                    let base = base_delay_ms * (1 << attempt);
                    let jitter = base / 3 * (attempt as u64 % 3);
                    tokio::time::sleep(Duration::from_millis(base + jitter)).await;
                }
            }
        }
    }
    Err(last_err.unwrap())
}

/// Retry only for retryable errors
#[derive(Debug, PartialEq)]
enum MyError {
    Transient(String),
    Permanent(String),
}

async fn retry_if<T, F, Fut, P>(
    max_attempts: usize,
    base_delay_ms: u64,
    is_retryable: P,
    mut f: F,
) -> Result<T, MyError>
where
    F: FnMut() -> Fut,
    Fut: std::future::Future<Output = Result<T, MyError>>,
    P: Fn(&MyError) -> bool,
{
    let mut last_err = None;
    for attempt in 0..max_attempts {
        match f().await {
            Ok(v) => return Ok(v),
            Err(e) if !is_retryable(&e) => return Err(e),
            Err(e) => {
                last_err = Some(e);
                if attempt + 1 < max_attempts {
                    let delay = base_delay_ms * (1 << attempt);
                    tokio::time::sleep(Duration::from_millis(delay)).await;
                }
            }
        }
    }
    Err(last_err.unwrap())
}

/// Builder pattern
struct RetryConfig {
    max_attempts: usize,
    base_delay_ms: u64,
}

impl RetryConfig {
    fn new() -> Self { RetryConfig { max_attempts: 3, base_delay_ms: 100 } }
    fn attempts(mut self, n: usize) -> Self { self.max_attempts = n; self }
    fn base_delay(mut self, ms: u64) -> Self { self.base_delay_ms = ms; self }

    async fn run<T, E, F, Fut>(&self, f: F) -> Result<T, E>
    where
        F: FnMut() -> Fut,
        Fut: std::future::Future<Output = Result<T, E>>,
    {
        retry(self.max_attempts, self.base_delay_ms, f).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_retry_succeeds_on_third_attempt() {
        let mut count = 0;
        let result = retry(5, 1, || {
            count += 1;
            let c = count;
            async move {
                if c < 3 { Err("fail") } else { Ok(c) }
            }
        }).await;
        assert_eq!(result, Ok(3));
    }

    #[tokio::test]
    async fn test_retry_exhausts_attempts() {
        let mut count = 0;
        let result: Result<i32, &str> = retry(3, 1, || {
            count += 1;
            async { Err("always fails") }
        }).await;
        assert!(result.is_err());
        assert_eq!(count, 3);
    }

    #[tokio::test]
    async fn test_retry_succeeds_first_try() {
        let result: Result<i32, &str> = retry(3, 1, || async { Ok(42) }).await;
        assert_eq!(result, Ok(42));
    }

    #[tokio::test]
    async fn test_retry_if_stops_on_permanent() {
        let mut count = 0;
        let result = retry_if(
            5, 1,
            |e| matches!(e, MyError::Transient(_)),
            || {
                count += 1;
                async { Err(MyError::Permanent("unrecoverable".to_string())) }
            },
        ).await;
        assert!(result.is_err());
        assert_eq!(count, 1);
    }

    #[tokio::test]
    async fn test_retry_if_retries_transient() {
        let mut count = 0;
        let result = retry_if(
            5, 1,
            |e| matches!(e, MyError::Transient(_)),
            || {
                count += 1;
                let c = count;
                async move {
                    if c < 3 { Err(MyError::Transient("temp".to_string())) } else { Ok(42) }
                }
            },
        ).await;
        assert_eq!(result, Ok(42));
    }

    #[tokio::test]
    async fn test_builder_config() {
        let mut n = 0;
        let r = RetryConfig::new().attempts(4).base_delay(1).run(|| {
            n += 1;
            let c = n;
            async move {
                if c < 2 { Err("x") } else { Ok(c) }
            }
        }).await;
        assert_eq!(r, Ok(2));
    }
}
