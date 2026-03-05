// 990: Semaphore — Tokio version
// tokio::sync::Semaphore — async counting semaphore with permits

use std::sync::Arc;
use tokio::sync::{Mutex, Semaphore};
use std::time::Duration;

/// Limit concurrent workers using tokio::sync::Semaphore
async fn limited_concurrency() -> usize {
    let sem = Arc::new(Semaphore::new(3));
    let active = Arc::new(Mutex::new(0usize));
    let max_active = Arc::new(Mutex::new(0usize));

    let handles: Vec<_> = (0..10)
        .map(|_| {
            let sem = Arc::clone(&sem);
            let active = Arc::clone(&active);
            let max_active = Arc::clone(&max_active);
            tokio::spawn(async move {
                let _permit = sem.acquire().await.unwrap();
                {
                    let mut a = active.lock().await;
                    *a += 1;
                    let mut m = max_active.lock().await;
                    if *a > *m { *m = *a; }
                }
                tokio::time::sleep(Duration::from_millis(5)).await;
                *active.lock().await -= 1;
                // _permit drops here — releases semaphore
            })
        })
        .collect();

    for h in handles { h.await.unwrap(); }
    *max_active.lock().await
}

/// Binary semaphore (permits=1) as mutex equivalent
async fn binary_semaphore_counter() -> u32 {
    let sem = Arc::new(Semaphore::new(1));
    let counter = Arc::new(Mutex::new(0u32));

    let handles: Vec<_> = (0..5)
        .map(|_| {
            let sem = Arc::clone(&sem);
            let counter = Arc::clone(&counter);
            tokio::spawn(async move {
                for _ in 0..100 {
                    let _permit = sem.acquire().await.unwrap();
                    *counter.lock().await += 1;
                }
            })
        })
        .collect();

    for h in handles { h.await.unwrap(); }
    *counter.lock().await
}

/// Resource pool with semaphore
async fn resource_pool_demo() -> Vec<usize> {
    const POOL_SIZE: usize = 2;
    let sem = Arc::new(Semaphore::new(POOL_SIZE));
    let usage_log = Arc::new(Mutex::new(Vec::new()));

    let handles: Vec<_> = (0..6)
        .map(|i| {
            let sem = Arc::clone(&sem);
            let log = Arc::clone(&usage_log);
            tokio::spawn(async move {
                let _permit = sem.acquire().await.unwrap();
                log.lock().await.push(i);
                tokio::time::sleep(Duration::from_millis(2)).await;
            })
        })
        .collect();

    for h in handles { h.await.unwrap(); }
    let mut log = usage_log.lock().await.clone();
    log.sort();
    log
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_limited_concurrency() {
        let max = limited_concurrency().await;
        assert!(max <= 3, "max concurrent was {}, expected ≤ 3", max);
        assert!(max >= 1);
    }

    #[tokio::test]
    async fn test_binary_semaphore_correctness() {
        assert_eq!(binary_semaphore_counter().await, 500);
    }

    #[tokio::test]
    async fn test_resource_pool() {
        let log = resource_pool_demo().await;
        assert_eq!(log.len(), 6);
        assert_eq!(log, vec![0, 1, 2, 3, 4, 5]);
    }

    #[tokio::test]
    async fn test_semaphore_permits_count() {
        let sem = Semaphore::new(3);
        assert_eq!(sem.available_permits(), 3);
        let _p = sem.acquire().await.unwrap();
        assert_eq!(sem.available_permits(), 2);
        drop(_p);
        assert_eq!(sem.available_permits(), 3);
    }

    #[tokio::test]
    async fn test_try_acquire() {
        let sem = Semaphore::new(1);
        let p1 = sem.try_acquire();
        assert!(p1.is_ok());
        let p2 = sem.try_acquire();
        assert!(p2.is_err()); // no permits available
        drop(p1);
        let p3 = sem.try_acquire();
        assert!(p3.is_ok());
    }
}
