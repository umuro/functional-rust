// 922: Cancellation Token — Tokio version
// tokio_util::sync::CancellationToken — idiomatic async cancellation
// Note: Uses tokio's built-in select! for cancellation instead of external crate

use std::time::Duration;
use tokio::sync::watch;

/// CancellationToken using tokio::sync::watch channel
#[derive(Clone)]
struct CancellationToken {
    rx: watch::Receiver<bool>,
    tx: watch::Sender<bool>,
}

impl CancellationToken {
    fn new() -> Self {
        let (tx, rx) = watch::channel(false);
        Self { rx, tx }
    }

    fn cancel(&self) {
        self.tx.send(true).ok();
    }

    fn is_cancelled(&self) -> bool {
        *self.rx.borrow()
    }

    async fn cancelled(&mut self) {
        while !*self.rx.borrow_and_update() {
            if self.rx.changed().await.is_err() { break; }
        }
    }
}

/// Long-running async task with cancellation
async fn long_task(mut token: CancellationToken, steps: usize) -> Result<String, String> {
    for i in 0..steps {
        tokio::select! {
            _ = token.cancelled() => {
                return Err(format!("cancelled at step {i}"));
            }
            _ = tokio::time::sleep(Duration::from_millis(10)) => {
                // Step complete
            }
        }
    }
    Ok(format!("completed all {steps} steps"))
}

/// Cancellable async sum
async fn cancellable_sum(token: &CancellationToken, data: &[i64]) -> Option<i64> {
    let mut sum = 0i64;
    for (i, &x) in data.iter().enumerate() {
        if i % 1000 == 0 && token.is_cancelled() {
            return None;
        }
        sum = sum.saturating_add(x);
        // Yield to allow cancellation check
        if i % 10000 == 0 {
            tokio::task::yield_now().await;
        }
    }
    Some(sum)
}

/// Cancellation via tokio::select! — idiomatic approach
async fn select_cancellation() -> &'static str {
    let (tx, mut rx) = watch::channel(false);

    // Spawn "cancel after 20ms"
    tokio::spawn(async move {
        tokio::time::sleep(Duration::from_millis(20)).await;
        tx.send(true).ok();
    });

    tokio::select! {
        _ = async {
            // Long-running work
            tokio::time::sleep(Duration::from_millis(100)).await;
        } => "completed",
        _ = async {
            while !*rx.borrow_and_update() {
                if rx.changed().await.is_err() { break; }
            }
        } => "cancelled",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_token_starts_not_cancelled() {
        let t = CancellationToken::new();
        assert!(!t.is_cancelled());
    }

    #[tokio::test]
    async fn test_cancel_sets_flag() {
        let t = CancellationToken::new();
        t.cancel();
        assert!(t.is_cancelled());
    }

    #[tokio::test]
    async fn test_task_completes_without_cancel() {
        let t = CancellationToken::new();
        let result = long_task(t, 2).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_task_cancelled_via_select() {
        let token = CancellationToken::new();
        let task_token = token.clone();

        // Cancel after 15ms
        tokio::spawn(async move {
            tokio::time::sleep(Duration::from_millis(15)).await;
            token.cancel();
        });

        let result = long_task(task_token, 100).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_select_cancellation() {
        assert_eq!(select_cancellation().await, "cancelled");
    }

    #[tokio::test]
    async fn test_cancellable_sum_completes() {
        let t = CancellationToken::new();
        let data: Vec<i64> = (1..=100).collect();
        let result = cancellable_sum(&t, &data).await;
        assert_eq!(result, Some(5050));
    }

    #[tokio::test]
    async fn test_cancellable_sum_cancelled() {
        let t = CancellationToken::new();
        t.cancel();
        let data: Vec<i64> = (1..=10000).collect();
        let result = cancellable_sum(&t, &data).await;
        assert_eq!(result, None);
    }
}
