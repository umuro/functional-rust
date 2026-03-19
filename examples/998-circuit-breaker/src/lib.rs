// 998: Circuit Breaker
// Open/Half-Open/Closed state machine for fault tolerance

use std::sync::Mutex;
use std::time::{Duration, Instant};

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

    fn maybe_transition_to_half_open(&self) {
        let mut state = self.state.lock().unwrap();
        if let BreakerState::Open { opened_at } = *state {
            if opened_at.elapsed() >= self.recovery_timeout {
                *state = BreakerState::HalfOpen;
            }
        }
    }

    pub fn call<T, E, F>(&self, f: F) -> CallResult<T, E>
    where
        F: FnOnce() -> Result<T, E>,
    {
        self.maybe_transition_to_half_open();

        let current_state = self.state.lock().unwrap().clone();
        match current_state {
            BreakerState::Open { .. } => CallResult::CircuitOpen,
            BreakerState::Closed | BreakerState::HalfOpen => {
                match f() {
                    Ok(v) => {
                        // Success: reset failures, close circuit
                        *self.failures.lock().unwrap() = 0;
                        *self.state.lock().unwrap() = BreakerState::Closed;
                        CallResult::Success(v)
                    }
                    Err(e) => {
                        let mut failures = self.failures.lock().unwrap();
                        *failures += 1;
                        if *failures >= self.failure_threshold {
                            *self.state.lock().unwrap() = BreakerState::Open {
                                opened_at: Instant::now(),
                            };
                        }
                        CallResult::Failure(e)
                    }
                }
            }
        }
    }

    pub fn state_name(&self) -> &'static str {
        match *self.state.lock().unwrap() {
            BreakerState::Closed => "Closed",
            BreakerState::Open { .. } => "Open",
            BreakerState::HalfOpen => "HalfOpen",
        }
    }

    pub fn reset(&self) {
        *self.state.lock().unwrap() = BreakerState::Closed;
        *self.failures.lock().unwrap() = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initial_state_closed() {
        let cb = CircuitBreaker::new(3, Duration::from_secs(1));
        assert_eq!(cb.state_name(), "Closed");
    }

    #[test]
    fn test_opens_after_threshold() {
        let cb = CircuitBreaker::new(3, Duration::from_secs(10));
        for _ in 0..3 {
            cb.call(|| Err::<i32, &str>("err"));
        }
        assert_eq!(cb.state_name(), "Open");
    }

    #[test]
    fn test_rejects_when_open() {
        let cb = CircuitBreaker::new(2, Duration::from_secs(10));
        cb.call(|| Err::<i32, &str>("e"));
        cb.call(|| Err::<i32, &str>("e")); // trip breaker
        let r = cb.call(|| Ok::<i32, &str>(42));
        assert_eq!(r, CallResult::CircuitOpen);
    }

    #[test]
    fn test_recovers_after_timeout() {
        let cb = CircuitBreaker::new(2, Duration::from_millis(20));
        cb.call(|| Err::<i32, &str>("e"));
        cb.call(|| Err::<i32, &str>("e")); // open
        assert_eq!(cb.state_name(), "Open");

        std::thread::sleep(Duration::from_millis(30));

        let r = cb.call(|| Ok::<i32, &str>(99));
        assert_eq!(r, CallResult::Success(99));
        assert_eq!(cb.state_name(), "Closed");
    }

    #[test]
    fn test_success_resets_failures() {
        let cb = CircuitBreaker::new(3, Duration::from_secs(1));
        cb.call(|| Err::<i32, &str>("e"));
        cb.call(|| Err::<i32, &str>("e")); // 2 failures
        cb.call(|| Ok::<i32, &str>(1)); // success — reset
        cb.call(|| Err::<i32, &str>("e")); // 1 failure — not open yet
        assert_eq!(cb.state_name(), "Closed");
    }
}
