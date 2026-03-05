// 996: Timeout Pattern — Tokio version
// tokio::time::timeout — idiomatic async timeout

use std::time::Duration;
use tokio::time::timeout;

/// Timeout on an async operation
async fn with_timeout_demo(delay_ms: u64, timeout_ms: u64) -> Result<i32, &'static str> {
    let result = timeout(
        Duration::from_millis(timeout_ms),
        async {
            tokio::time::sleep(Duration::from_millis(delay_ms)).await;
            42
        },
    ).await;

    match result {
        Ok(v) => Ok(v),
        Err(_) => Err("timeout"),
    }
}

/// Run any async function with a timeout
async fn with_timeout<T, F>(timeout_dur: Duration, f: F) -> Option<T>
where
    F: std::future::Future<Output = T>,
{
    timeout(timeout_dur, f).await.ok()
}

/// Race: first future to complete wins (tokio::select!)
async fn race_fastest() -> &'static str {
    tokio::select! {
        _ = tokio::time::sleep(Duration::from_millis(50)) => "slow",
        _ = tokio::time::sleep(Duration::from_millis(5)) => "fast",
        _ = tokio::time::sleep(Duration::from_millis(30)) => "medium",
    }
}

/// Retry with per-attempt timeout
async fn retry_with_timeout<T, E, F, Fut>(
    max_attempts: usize,
    timeout_per_attempt: Duration,
    mut f: F,
) -> Result<T, &'static str>
where
    F: FnMut() -> Fut,
    Fut: std::future::Future<Output = Result<T, E>>,
{
    for attempt in 0..max_attempts {
        match timeout(timeout_per_attempt, f()).await {
            Ok(Ok(v)) => return Ok(v),
            Ok(Err(_)) | Err(_) => {
                if attempt + 1 < max_attempts {
                    tokio::time::sleep(Duration::from_millis(1 << attempt)).await;
                }
            }
        }
    }
    Err("max attempts exceeded")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_fast_op_succeeds() {
        let result = with_timeout_demo(10, 500).await;
        assert_eq!(result, Ok(42));
    }

    #[tokio::test]
    async fn test_slow_op_times_out() {
        let result = with_timeout_demo(200, 20).await;
        assert_eq!(result, Err("timeout"));
    }

    #[tokio::test]
    async fn test_with_timeout_succeeds() {
        let result = with_timeout(Duration::from_millis(500), async {
            tokio::time::sleep(Duration::from_millis(5)).await;
            99i32
        }).await;
        assert_eq!(result, Some(99));
    }

    #[tokio::test]
    async fn test_with_timeout_expires() {
        let result = with_timeout(Duration::from_millis(5), async {
            tokio::time::sleep(Duration::from_millis(100)).await;
            99i32
        }).await;
        assert_eq!(result, None);
    }

    #[tokio::test]
    async fn test_race_fastest_wins() {
        assert_eq!(race_fastest().await, "fast");
    }

    #[tokio::test]
    async fn test_retry_with_timeout() {
        let mut count = 0;
        let result = retry_with_timeout(
            5,
            Duration::from_millis(100),
            || {
                count += 1;
                let c = count;
                async move {
                    if c < 3 { Err::<i32, &str>("fail") } else { Ok(c) }
                }
            },
        ).await;
        assert_eq!(result, Ok(3));
    }
}
