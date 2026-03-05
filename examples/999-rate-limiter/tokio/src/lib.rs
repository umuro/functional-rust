// 999: Rate Limiter — Tokio version
// Token bucket using tokio::sync::Mutex and tokio::time::sleep

use std::time::{Duration, Instant};
use tokio::sync::Mutex;

struct TokenBucket {
    state: Mutex<BucketState>,
    capacity: f64,
    refill_rate: f64,
}

struct BucketState {
    tokens: f64,
    last_refill: Instant,
}

impl TokenBucket {
    fn new(capacity: f64, refill_rate: f64) -> Self {
        TokenBucket {
            state: Mutex::new(BucketState {
                tokens: capacity,
                last_refill: Instant::now(),
            }),
            capacity,
            refill_rate,
        }
    }

    fn refill(state: &mut BucketState, capacity: f64, refill_rate: f64) {
        let elapsed = state.last_refill.elapsed().as_secs_f64();
        let new_tokens = elapsed * refill_rate;
        state.tokens = (state.tokens + new_tokens).min(capacity);
        state.last_refill = Instant::now();
    }

    async fn try_acquire(&self, cost: f64) -> bool {
        let mut state = self.state.lock().await;
        Self::refill(&mut state, self.capacity, self.refill_rate);
        if state.tokens >= cost {
            state.tokens -= cost;
            true
        } else {
            false
        }
    }

    /// Async acquire — yields to tokio runtime while waiting
    async fn acquire(&self, cost: f64) {
        while !self.try_acquire(cost).await {
            tokio::time::sleep(Duration::from_millis(1)).await;
        }
    }

    async fn available_tokens(&self) -> f64 {
        let mut state = self.state.lock().await;
        Self::refill(&mut state, self.capacity, self.refill_rate);
        state.tokens
    }
}

/// Burst then deny
async fn burst_then_deny() -> (usize, usize) {
    let bucket = TokenBucket::new(5.0, 1.0);
    let mut allowed = 0;
    let mut denied = 0;

    for _ in 0..10 {
        if bucket.try_acquire(1.0).await { allowed += 1; } else { denied += 1; }
    }
    (allowed, denied)
}

/// Refill over time
async fn refill_over_time() -> usize {
    let bucket = TokenBucket::new(3.0, 1000.0);

    for _ in 0..3 { assert!(bucket.try_acquire(1.0).await); }
    assert!(!bucket.try_acquire(1.0).await);

    tokio::time::sleep(Duration::from_millis(15)).await;

    let mut refilled = 0;
    for _ in 0..5 {
        if bucket.try_acquire(1.0).await { refilled += 1; }
    }
    refilled
}

/// Rate-limited async batch processing
async fn rate_limited_processing(items: Vec<i32>, rps: f64) -> Vec<i32> {
    let bucket = TokenBucket::new(rps, rps);
    let mut results = Vec::new();
    for item in items {
        bucket.acquire(1.0).await;
        results.push(item * 2);
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_burst_allows_up_to_capacity() {
        let (allowed, denied) = burst_then_deny().await;
        assert_eq!(allowed, 5);
        assert_eq!(denied, 5);
    }

    #[tokio::test]
    async fn test_tokens_refill() {
        let refilled = refill_over_time().await;
        assert!(refilled >= 3, "expected at least 3, got {}", refilled);
    }

    #[tokio::test]
    async fn test_try_acquire_returns_false_when_empty() {
        let bucket = TokenBucket::new(2.0, 0.001);
        bucket.try_acquire(1.0).await;
        bucket.try_acquire(1.0).await;
        assert!(!bucket.try_acquire(1.0).await);
    }

    #[tokio::test]
    async fn test_available_tokens_starts_at_capacity() {
        let bucket = TokenBucket::new(10.0, 1.0);
        let tokens = bucket.available_tokens().await;
        assert!((tokens - 10.0).abs() < 0.1);
    }

    #[tokio::test]
    async fn test_cost_greater_than_one() {
        let bucket = TokenBucket::new(10.0, 1.0);
        assert!(bucket.try_acquire(5.0).await);
        assert!(!bucket.try_acquire(6.0).await);
        assert!(bucket.try_acquire(5.0).await);
    }

    #[tokio::test]
    async fn test_rate_limited_processing() {
        let results = rate_limited_processing(vec![1, 2, 3], 1000.0).await;
        assert_eq!(results, vec![2, 4, 6]);
    }
}
