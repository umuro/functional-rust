#![allow(clippy::all)]
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

#[derive(Clone)]
struct CancellationToken {
    cancelled: Arc<AtomicBool>,
}

impl CancellationToken {
    fn new() -> Self {
        Self { cancelled: Arc::new(AtomicBool::new(false)) }
    }

    fn cancel(&self) {
        self.cancelled.store(true, Ordering::Release);
    }

    fn is_cancelled(&self) -> bool {
        self.cancelled.load(Ordering::Acquire)
    }
}

fn long_task(token: CancellationToken, steps: usize) -> Result<String, String> {
    for i in 0..steps {
        if token.is_cancelled() {
            return Err(format!("cancelled at step {i}"));
        }
        // Do work
        thread::sleep(Duration::from_millis(10));
        println!("Step {i} complete");
    }
    Ok(format!("completed all {steps} steps"))
}

fn cancellable_sum(token: CancellationToken, data: &[i64]) -> Option<i64> {
    let mut sum = 0i64;
    for (i, &x) in data.iter().enumerate() {
        if i % 1000 == 0 && token.is_cancelled() {
            return None;
        }
        sum = sum.saturating_add(x);
    }
    Some(sum)
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn token_starts_not_cancelled() {
        let t = CancellationToken::new();
        assert!(!t.is_cancelled());
    }
    #[test]
    fn cancel_sets_flag() {
        let t = CancellationToken::new();
        t.cancel();
        assert!(t.is_cancelled());
    }
    #[test]
    fn task_completes_without_cancel() {
        let t = CancellationToken::new();
        let result = long_task(t, 2);
        assert!(result.is_ok());
    }
    #[test]
    fn task_cancelled_immediately() {
        let t = CancellationToken::new();
        t.cancel();
        assert!(long_task(t, 100).is_err());
    }
}
