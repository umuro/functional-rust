// 999: Rate Limiter — Token Bucket
// Tokens refill over time; consume one per request. Uses std::time::Instant.

use std::sync::Mutex;
use std::thread;
use std::time::{Duration, Instant};

struct TokenBucket {
    state: Mutex<BucketState>,
    capacity: f64,
    refill_rate: f64, // tokens per second
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

    fn try_acquire(&self, cost: f64) -> bool {
        let mut state = self.state.lock().unwrap();
        Self::refill(&mut state, self.capacity, self.refill_rate);
        if state.tokens >= cost {
            state.tokens -= cost;
            true
        } else {
            false
        }
    }

    fn acquire(&self, cost: f64) {
        while !self.try_acquire(cost) {
            thread::sleep(Duration::from_millis(1));
        }
    }

    fn available_tokens(&self) -> f64 {
        let mut state = self.state.lock().unwrap();
        Self::refill(&mut state, self.capacity, self.refill_rate);
        state.tokens
    }
}

// --- Approach 1: Burst then deny ---
fn burst_then_deny() -> (usize, usize) {
    let bucket = TokenBucket::new(5.0, 1.0); // 5 capacity, 1 token/sec
    let mut allowed = 0;
    let mut denied = 0;

    for _ in 0..10 {
        if bucket.try_acquire(1.0) {
            allowed += 1;
        } else {
            denied += 1;
        }
    }
    (allowed, denied)
}

// --- Approach 2: Refill over time ---
fn refill_over_time() -> usize {
    let bucket = TokenBucket::new(3.0, 1000.0); // 1000 tokens/sec

    // Drain all 3 tokens
    for _ in 0..3 {
        assert!(bucket.try_acquire(1.0));
    }
    assert!(!bucket.try_acquire(1.0)); // empty

    // Wait 10ms → should get ~10 tokens back, capped at 3
    thread::sleep(Duration::from_millis(15));

    let mut refilled = 0;
    for _ in 0..5 {
        if bucket.try_acquire(1.0) {
            refilled += 1;
        }
    }
    refilled
}

// --- Approach 3: Rate-limited batch processing ---
fn rate_limited_processing(items: Vec<i32>, rps: f64) -> Vec<i32> {
    let bucket = TokenBucket::new(rps, rps); // allow rps/sec burst
    items
        .into_iter()
        .map(|item| {
            bucket.acquire(1.0); // wait for token
            item * 2
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_burst_allows_up_to_capacity() {
        let (allowed, denied) = burst_then_deny();
        assert_eq!(allowed, 5);
        assert_eq!(denied, 5);
    }

    #[test]
    fn test_tokens_refill() {
        let refilled = refill_over_time();
        assert!(
            refilled >= 3,
            "expected at least 3 tokens refilled, got {}",
            refilled
        );
    }

    #[test]
    fn test_try_acquire_returns_false_when_empty() {
        let bucket = TokenBucket::new(2.0, 0.001); // very slow refill
        bucket.try_acquire(1.0);
        bucket.try_acquire(1.0);
        assert!(!bucket.try_acquire(1.0));
    }

    #[test]
    fn test_available_tokens_starts_at_capacity() {
        let bucket = TokenBucket::new(10.0, 1.0);
        let tokens = bucket.available_tokens();
        assert!((tokens - 10.0).abs() < 0.1, "expected ~10, got {}", tokens);
    }

    #[test]
    fn test_cost_greater_than_one() {
        let bucket = TokenBucket::new(10.0, 1.0);
        // Acquire 5 tokens at once (one heavy request)
        assert!(bucket.try_acquire(5.0));
        // Now only 5 left — can't take 6
        assert!(!bucket.try_acquire(6.0));
        // Can take 5
        assert!(bucket.try_acquire(5.0));
    }

    #[test]
    fn test_rate_limited_processing() {
        let results = rate_limited_processing(vec![1, 2, 3], 1000.0);
        assert_eq!(results, vec![2, 4, 6]);
    }
}
