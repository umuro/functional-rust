// 920: Buffered Stream — Tokio version
// Bounded concurrency map using tokio::sync::Semaphore

use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{Mutex, Semaphore};

/// Buffered map: process items with bounded concurrency
async fn buffered_map<T, U, F, Fut>(items: Vec<T>, concurrency: usize, f: F) -> Vec<U>
where
    T: Send + 'static,
    U: Send + 'static,
    F: Fn(T) -> Fut + Send + Sync + 'static,
    Fut: std::future::Future<Output = U> + Send + 'static,
{
    let sem = Arc::new(Semaphore::new(concurrency));
    let f = Arc::new(f);
    let results: Arc<Mutex<Vec<(usize, U)>>> = Arc::new(Mutex::new(Vec::new()));

    let handles: Vec<_> = items
        .into_iter()
        .enumerate()
        .map(|(i, item)| {
            let sem = Arc::clone(&sem);
            let f = Arc::clone(&f);
            let results = Arc::clone(&results);
            tokio::spawn(async move {
                let _permit = sem.acquire().await.unwrap();
                let result = f(item).await;
                results.lock().await.push((i, result));
            })
        })
        .collect();

    for h in handles { h.await.unwrap(); }

    let mut res = results.lock().await.drain(..).collect::<Vec<_>>();
    res.sort_by_key(|(i, _)| *i);
    res.into_iter().map(|(_, v)| v).collect()
}

/// Simpler version using tokio::spawn with semaphore
async fn buffered_map_simple<T, U, F>(items: Vec<T>, concurrency: usize, f: F) -> Vec<U>
where
    T: Send + 'static,
    U: Send + 'static,
    F: Fn(T) -> U + Send + Sync + 'static,
{
    let sem = Arc::new(Semaphore::new(concurrency));
    let f = Arc::new(f);

    let handles: Vec<_> = items
        .into_iter()
        .map(|item| {
            let sem = Arc::clone(&sem);
            let f = Arc::clone(&f);
            tokio::spawn(async move {
                let _permit = sem.acquire().await.unwrap();
                f(item)
            })
        })
        .collect();

    let mut results = Vec::new();
    for h in handles {
        results.push(h.await.unwrap());
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_buffered_map_all_results() {
        let r = buffered_map(
            vec![1u64, 2, 3, 4, 5],
            2,
            |x| async move { x * 2 },
        ).await;
        assert_eq!(r, vec![2, 4, 6, 8, 10]);
    }

    #[tokio::test]
    async fn test_concurrency_1_sequential() {
        let r = buffered_map_simple(vec![1, 2, 3], 1, |x: i32| x + 10);
        let r = r.await;
        assert_eq!(r, vec![11, 12, 13]);
    }

    #[tokio::test]
    async fn test_order_preserved() {
        let r = buffered_map(
            vec![5u64, 4, 3, 2, 1],
            3,
            |x| async move {
                tokio::time::sleep(Duration::from_millis((6 - x) * 2)).await;
                x * x
            },
        ).await;
        assert_eq!(r, vec![25, 16, 9, 4, 1]);
    }

    #[tokio::test]
    async fn test_empty_input() {
        let r: Vec<i32> = buffered_map(vec![], 3, |x: i32| async move { x }).await;
        assert!(r.is_empty());
    }

    #[tokio::test]
    async fn test_bounded_concurrency() {
        let active = Arc::new(Mutex::new(0usize));
        let max_active = Arc::new(Mutex::new(0usize));

        let items: Vec<usize> = (0..10).collect();
        let active2 = Arc::clone(&active);
        let max2 = Arc::clone(&max_active);

        buffered_map(items, 3, move |_x| {
            let active = Arc::clone(&active2);
            let max = Arc::clone(&max2);
            async move {
                let mut a = active.lock().await;
                *a += 1;
                let mut m = max.lock().await;
                if *a > *m { *m = *a; }
                drop(m);
                drop(a);
                tokio::time::sleep(Duration::from_millis(5)).await;
                *active.lock().await -= 1;
            }
        }).await;

        assert!(*max_active.lock().await <= 3);
    }
}
