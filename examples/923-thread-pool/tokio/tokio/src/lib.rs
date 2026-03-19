#![allow(clippy::all)]
// 923: Thread Pool — Tokio version
// Tokio IS a thread pool — multi-threaded runtime with work stealing

use std::sync::Arc;
use tokio::sync::Mutex;

/// Tokio's runtime IS the thread pool — just spawn tasks
async fn pool_compute(items: Vec<i32>) -> Vec<i32> {
    let handles: Vec<_> = items
        .into_iter()
        .map(|x| tokio::spawn(async move { x * x }))
        .collect();

    let mut results = Vec::new();
    for h in handles {
        results.push(h.await.unwrap());
    }
    results
}

/// Bounded pool via semaphore
async fn bounded_pool(items: Vec<i32>, max_concurrent: usize) -> Vec<i32> {
    let sem = Arc::new(tokio::sync::Semaphore::new(max_concurrent));

    let handles: Vec<_> = items
        .into_iter()
        .map(|x| {
            let sem = Arc::clone(&sem);
            tokio::spawn(async move {
                let _permit = sem.acquire().await.unwrap();
                x * x
            })
        })
        .collect();

    let mut results = Vec::new();
    for h in handles {
        results.push(h.await.unwrap());
    }
    results
}

/// CPU-bound work via tokio::task::spawn_blocking
async fn blocking_compute(items: Vec<i32>) -> Vec<i32> {
    let handles: Vec<_> = items
        .into_iter()
        .map(|x| {
            tokio::task::spawn_blocking(move || {
                // Simulate CPU-intensive work
                (0..100).fold(x, |acc, _| acc.wrapping_mul(acc).wrapping_add(1))
            })
        })
        .collect();

    let mut results = Vec::new();
    for h in handles {
        results.push(h.await.unwrap());
    }
    results
}

/// Demonstrate multi-threaded runtime
async fn multi_thread_demo() -> Vec<String> {
    let results = Arc::new(Mutex::new(Vec::new()));

    let handles: Vec<_> = (0..5)
        .map(|i| {
            let results = Arc::clone(&results);
            tokio::spawn(async move {
                let thread_id = format!("{:?}", std::thread::current().id());
                results.lock().await.push(format!("task-{}: {}", i, thread_id));
            })
        })
        .collect();

    for h in handles { h.await.unwrap(); }
    results.lock().await.clone()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_pool_compute() {
        let results = pool_compute(vec![1, 2, 3, 4, 5]).await;
        assert_eq!(results, vec![1, 4, 9, 16, 25]);
    }

    #[tokio::test]
    async fn test_bounded_pool() {
        let results = bounded_pool(vec![1, 2, 3, 4, 5], 2).await;
        assert_eq!(results, vec![1, 4, 9, 16, 25]);
    }

    #[tokio::test]
    async fn test_blocking_compute() {
        let results = blocking_compute(vec![1, 2, 3]).await;
        assert_eq!(results.len(), 3);
    }

    #[tokio::test]
    async fn test_empty_pool() {
        let results = pool_compute(vec![]).await;
        assert!(results.is_empty());
    }

    #[tokio::test]
    async fn test_large_batch() {
        let items: Vec<i32> = (1..=100).collect();
        let results = pool_compute(items).await;
        assert_eq!(results.len(), 100);
        assert_eq!(results[0], 1);
        assert_eq!(results[99], 10000);
    }

    #[tokio::test]
    async fn test_multi_thread() {
        let results = multi_thread_demo().await;
        assert_eq!(results.len(), 5);
    }
}
