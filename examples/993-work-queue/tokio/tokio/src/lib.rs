// 993: Work Queue — Tokio version
// Async task pool using tokio::spawn + Semaphore for bounded concurrency

use std::sync::Arc;
use tokio::sync::{mpsc, Mutex, Semaphore};

/// Spawn tasks via tokio runtime — runtime IS the thread pool
async fn pool_squares() -> Vec<i64> {
    let results = Arc::new(Mutex::new(Vec::new()));

    let handles: Vec<_> = (1i64..=20)
        .map(|i| {
            let results = Arc::clone(&results);
            tokio::spawn(async move {
                results.lock().await.push(i * i);
            })
        })
        .collect();

    for h in handles { h.await.unwrap(); }

    let mut v = results.lock().await.clone();
    v.sort();
    v
}

/// Work queue with bounded concurrency via Semaphore
async fn bounded_pool(inputs: Vec<i32>, max_concurrent: usize) -> Vec<i32> {
    let sem = Arc::new(Semaphore::new(max_concurrent));
    let (tx, mut rx) = mpsc::channel::<i32>(inputs.len().max(1));

    let n = inputs.len();
    for x in inputs {
        let sem = Arc::clone(&sem);
        let tx = tx.clone();
        tokio::spawn(async move {
            let _permit = sem.acquire().await.unwrap();
            tx.send(x * x).await.unwrap();
        });
    }
    drop(tx);

    let mut results = Vec::new();
    while let Some(v) = rx.recv().await {
        results.push(v);
    }
    assert_eq!(results.len(), n);
    results.sort();
    results
}

/// Work queue with return values via oneshot channels
async fn pool_with_results(inputs: Vec<i32>) -> Vec<i32> {
    let mut receivers = Vec::new();

    for x in inputs {
        let (tx, rx) = tokio::sync::oneshot::channel();
        tokio::spawn(async move {
            tx.send(x * x).ok();
        });
        receivers.push(rx);
    }

    let mut results = Vec::new();
    for rx in receivers {
        results.push(rx.await.unwrap());
    }
    results.sort();
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_pool_squares_all_computed() {
        let squares = pool_squares().await;
        assert_eq!(squares.len(), 20);
        let sum: i64 = squares.iter().sum();
        assert_eq!(sum, 2870);
    }

    #[tokio::test]
    async fn test_bounded_pool() {
        let results = bounded_pool(vec![1, 2, 3, 4, 5], 2).await;
        assert_eq!(results, vec![1, 4, 9, 16, 25]);
    }

    #[tokio::test]
    async fn test_pool_empty() {
        let results = bounded_pool(vec![], 2).await;
        assert!(results.is_empty());
    }

    #[tokio::test]
    async fn test_pool_single_worker() {
        let results = bounded_pool(vec![1, 2, 3, 4, 5], 1).await;
        assert_eq!(results, vec![1, 4, 9, 16, 25]);
    }

    #[tokio::test]
    async fn test_pool_with_results() {
        let results = pool_with_results(vec![1, 2, 3, 4, 5]).await;
        assert_eq!(results, vec![1, 4, 9, 16, 25]);
    }

    #[tokio::test]
    async fn test_pool_more_tasks_than_workers() {
        let counter = Arc::new(Mutex::new(0u32));
        let handles: Vec<_> = (0..100)
            .map(|_| {
                let c = Arc::clone(&counter);
                tokio::spawn(async move { *c.lock().await += 1; })
            })
            .collect();
        for h in handles { h.await.unwrap(); }
        assert_eq!(*counter.lock().await, 100);
    }
}
