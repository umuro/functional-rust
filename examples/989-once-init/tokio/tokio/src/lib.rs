#![allow(clippy::all)]
// 989: One-Time Initialization — Tokio version
// tokio::sync::OnceCell — async one-time init

use std::sync::Arc;
use tokio::sync::{Mutex, OnceCell};

/// Static OnceCell for async one-time init
static CONFIG: OnceCell<String> = OnceCell::const_new();

async fn get_config() -> &'static String {
    CONFIG.get_or_init(|| async {
        "production-config-v42".to_string()
    }).await
}

/// OnceCell with expensive async computation
static PRIMES: OnceCell<Vec<u32>> = OnceCell::const_new();

fn sieve(limit: usize) -> Vec<u32> {
    let mut is_prime = vec![true; limit + 1];
    is_prime[0] = false;
    if limit > 0 { is_prime[1] = false; }
    for i in 2..=limit {
        if is_prime[i] {
            let mut j = i * i;
            while j <= limit { is_prime[j] = false; j += i; }
        }
    }
    (2..=limit as u32).filter(|&n| is_prime[n as usize]).collect()
}

async fn get_primes() -> &'static [u32] {
    PRIMES.get_or_init(|| async { sieve(100) }).await
}

/// Instance-level OnceCell
struct LazyConfig {
    inner: OnceCell<String>,
    prefix: String,
}

impl LazyConfig {
    fn new(prefix: &str) -> Self {
        LazyConfig {
            inner: OnceCell::const_new(),
            prefix: prefix.to_string(),
        }
    }

    async fn get(&self) -> &str {
        let prefix = self.prefix.clone();
        self.inner.get_or_init(|| async move {
            format!("{}-initialized", prefix)
        }).await
    }
}

/// Concurrent init — only one task runs the init closure
async fn concurrent_once_init() -> usize {
    let cell: Arc<OnceCell<usize>> = Arc::new(OnceCell::const_new());
    let call_count = Arc::new(Mutex::new(0usize));

    let handles: Vec<_> = (0..10)
        .map(|_| {
            let cell = Arc::clone(&cell);
            let count = Arc::clone(&call_count);
            tokio::spawn(async move {
                cell.get_or_init(|| async {
                    *count.lock().await += 1;
                    42
                }).await;
            })
        })
        .collect();

    for h in handles { h.await.unwrap(); }
    *call_count.lock().await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_config_same_value() {
        let c1 = get_config().await;
        let c2 = get_config().await;
        assert_eq!(c1, c2);
        assert!(std::ptr::eq(c1 as *const _, c2 as *const _));
    }

    #[tokio::test]
    async fn test_primes_correctness() {
        let primes = get_primes().await;
        assert_eq!(&primes[..5], &[2, 3, 5, 7, 11]);
        assert!(!primes.contains(&4));
    }

    #[tokio::test]
    async fn test_lazy_config_cached() {
        let lc = LazyConfig::new("test");
        let v1 = lc.get().await;
        let v2 = lc.get().await;
        assert_eq!(v1, "test-initialized");
        assert_eq!(v1, v2);
    }

    #[tokio::test]
    async fn test_concurrent_once_init_runs_exactly_once() {
        let count = concurrent_once_init().await;
        assert_eq!(count, 1);
    }

    #[tokio::test]
    async fn test_oncecell_get_before_init() {
        let cell: OnceCell<i32> = OnceCell::const_new();
        assert!(cell.get().is_none());
        cell.get_or_init(|| async { 42 }).await;
        assert_eq!(cell.get(), Some(&42));
    }
}
