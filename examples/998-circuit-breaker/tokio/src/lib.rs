// 998: Circuit Breaker — Tokio version
// Async circuit breaker using tokio::sync::Mutex and tokio::time

use std::time::{Duration, Instant};
use tokio::sync::Mutex;

#[derive(Debug, PartialEq, Clone)]
enum BreakerState {
    Closed,
    Open { opened_at: Instant },
    HalfOpen,
}

pub struct CircuitBreaker {
    state: Mutex<BreakerState>,
    failures: Mutex<u32>,
    failure_threshold: u32,
    recovery_timeout: Duration,
}

#[derive(Debug, PartialEq)]
pub enum CallResult<T, E> {
    Success(T),
    Failure(E),
    CircuitOpen,
}

impl CircuitBreaker {
    pub fn new(failure_threshold: u32, recovery_timeout: Duration) -> Self {
        CircuitBreaker {
            state: Mutex::new(BreakerState::Closed),
            failures: Mutex::new(0),
            failure_threshold,
            recovery_timeout,
        }
    }

    async fn maybe_transition_to_half_open(&self) {
        let mut state = self.state.lock().await;
        if let BreakerState::Open { opened_at } = *state {
            if opened_at.elapsed() >= self.recovery_timeout {
                *state = BreakerState::HalfOpen;
            }
        }
    }

    /// Call through the circuit breaker with an async function
    pub async fn call<T, E, F, Fut>(&self, f: F) -> CallResult<T, E>
    where
        F: FnOnce() -> Fut,
        Fut: std::future::Future<Output = Result<T, E>>,
    {
        self.maybe_transition_to_half_open().await;

        let current_state = self.state.lock().await.clone();
        match current_state {
            BreakerState::Open { .. } => CallResult::CircuitOpen,
            BreakerState::Closed | BreakerState::HalfOpen => {
                match f().await {
                    Ok(v) => {
                        *self.failures.lock().await = 0;
                        *self.state.lock().await = BreakerState::Closed;
                        CallResult::Success(v)
                    }
                    Err(e) => {
                        let mut failures = self.failures.lock().await;
                        *failures += 1;
                        if *failures >= self.failure_threshold {
                            *self.state.lock().await = BreakerState::Open {
                                opened_at: Instant::now(),
                            };
                        }
                        CallResult::Failure(e)
                    }
                }
            }
        }
    }

    pub async fn state_name(&self) -> &'static str {
        match *self.state.lock().await {
            BreakerState::Closed => "Closed",
            BreakerState::Open { .. } => "Open",
            BreakerState::HalfOpen => "HalfOpen",
        }
    }

    pub async fn reset(&self) {
        *self.state.lock().await = BreakerState::Closed;
        *self.failures.lock().await = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_initial_state_closed() {
        let cb = CircuitBreaker::new(3, Duration::from_secs(1));
        assert_eq!(cb.state_name().await, "Closed");
    }

    #[tokio::test]
    async fn test_opens_after_threshold() {
        let cb = CircuitBreaker::new(3, Duration::from_secs(10));
        for _ in 0..3 {
            cb.call(|| async { Err::<i32, &str>("err") }).await;
        }
        assert_eq!(cb.state_name().await, "Open");
    }

    #[tokio::test]
    async fn test_rejects_when_open() {
        let cb = CircuitBreaker::new(2, Duration::from_secs(10));
        cb.call(|| async { Err::<i32, &str>("e") }).await;
        cb.call(|| async { Err::<i32, &str>("e") }).await;
        let r = cb.call(|| async { Ok::<i32, &str>(42) }).await;
        assert_eq!(r, CallResult::CircuitOpen);
    }

    #[tokio::test]
    async fn test_recovers_after_timeout() {
        let cb = CircuitBreaker::new(2, Duration::from_millis(20));
        cb.call(|| async { Err::<i32, &str>("e") }).await;
        cb.call(|| async { Err::<i32, &str>("e") }).await;
        assert_eq!(cb.state_name().await, "Open");

        tokio::time::sleep(Duration::from_millis(30)).await;

        let r = cb.call(|| async { Ok::<i32, &str>(99) }).await;
        assert_eq!(r, CallResult::Success(99));
        assert_eq!(cb.state_name().await, "Closed");
    }

    #[tokio::test]
    async fn test_success_resets_failures() {
        let cb = CircuitBreaker::new(3, Duration::from_secs(1));
        cb.call(|| async { Err::<i32, &str>("e") }).await;
        cb.call(|| async { Err::<i32, &str>("e") }).await;
        cb.call(|| async { Ok::<i32, &str>(1) }).await;
        cb.call(|| async { Err::<i32, &str>("e") }).await;
        assert_eq!(cb.state_name().await, "Closed");
    }
}
