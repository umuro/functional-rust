#![allow(clippy::all)]
// 987: Read-Write Lock Pattern — Tokio version
// tokio::sync::RwLock — async-aware: many readers OR one writer

use std::sync::Arc;
use tokio::sync::RwLock;

/// Multiple readers in parallel
async fn concurrent_readers() -> Vec<i32> {
    let data = Arc::new(RwLock::new(42i32));

    let handles: Vec<_> = (0..5)
        .map(|_| {
            let data = Arc::clone(&data);
            tokio::spawn(async move {
                let guard = data.read().await; // shared read lock
                *guard
            })
        })
        .collect();

    let mut results = Vec::new();
    for h in handles {
        results.push(h.await.unwrap());
    }
    results
}

/// Writer excludes readers
async fn write_then_read() -> i32 {
    let data = Arc::new(RwLock::new(0i32));

    {
        let mut guard = data.write().await;
        *guard = 100;
    }

    let guard = data.read().await;
    *guard
}

/// Shared config pattern (read-heavy)
#[derive(Clone, Debug)]
struct Config {
    threshold: i32,
    name: String,
}

async fn config_pattern() -> (String, i32) {
    let config = Arc::new(RwLock::new(Config {
        threshold: 10,
        name: "default".to_string(),
    }));

    // Many readers
    let readers: Vec<_> = (0..4)
        .map(|_| {
            let config = Arc::clone(&config);
            tokio::spawn(async move {
                let c = config.read().await;
                (c.name.clone(), c.threshold)
            })
        })
        .collect();

    // One writer updates the config
    {
        let cfg = Arc::clone(&config);
        let writer = tokio::spawn(async move {
            let mut c = cfg.write().await;
            c.threshold = 99;
            c.name = "updated".to_string();
        });
        writer.await.unwrap();
    }

    for h in readers { h.await.unwrap(); }

    let c = config.read().await;
    (c.name.clone(), c.threshold)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_concurrent_readers_all_see_same() {
        let reads = concurrent_readers().await;
        assert_eq!(reads.len(), 5);
        assert!(reads.iter().all(|&v| v == 42));
    }

    #[tokio::test]
    async fn test_write_then_read() {
        assert_eq!(write_then_read().await, 100);
    }

    #[tokio::test]
    async fn test_config_pattern() {
        let (name, threshold) = config_pattern().await;
        assert_eq!(name, "updated");
        assert_eq!(threshold, 99);
    }

    #[tokio::test]
    async fn test_rwlock_write_exclusive() {
        let rw = Arc::new(RwLock::new(vec![1, 2, 3]));
        {
            let mut w = rw.write().await;
            w.push(4);
        }
        assert_eq!(*rw.read().await, vec![1, 2, 3, 4]);
    }
}
