#![allow(clippy::all)]
// 989: One-Time Initialization
// Rust: OnceLock<T> — set once, read many times (thread-safe)

use std::sync::{Arc, Mutex, OnceLock};
use std::thread;

// --- Approach 1: OnceLock<T> for global one-time init ---
static CONFIG: OnceLock<String> = OnceLock::new();

fn get_config() -> &'static String {
    CONFIG.get_or_init(|| {
        // Only runs once, even with concurrent calls
        "production-config-v42".to_string()
    })
}

// --- Approach 2: OnceLock with expensive computation ---
static PRIMES: OnceLock<Vec<u32>> = OnceLock::new();

fn sieve(limit: usize) -> Vec<u32> {
    let mut is_prime = vec![true; limit + 1];
    is_prime[0] = false;
    if limit > 0 {
        is_prime[1] = false;
    }
    for i in 2..=limit {
        if is_prime[i] {
            let mut j = i * i;
            while j <= limit {
                is_prime[j] = false;
                j += i;
            }
        }
    }
    (2..=limit as u32)
        .filter(|&n| is_prime[n as usize])
        .collect()
}

fn get_primes() -> &'static [u32] {
    PRIMES.get_or_init(|| sieve(100))
}

// --- Approach 3: Instance-level OnceLock (not just global) ---
struct LazyConfig {
    inner: OnceLock<String>,
    prefix: String,
}

impl LazyConfig {
    fn new(prefix: &str) -> Self {
        LazyConfig {
            inner: OnceLock::new(),
            prefix: prefix.to_string(),
        }
    }

    fn get(&self) -> &str {
        self.inner
            .get_or_init(|| format!("{}-initialized", self.prefix))
    }
}

// --- Approach 4: Thread-safe once across multiple threads ---
fn concurrent_once_init() -> usize {
    static INIT_COUNT: OnceLock<usize> = OnceLock::new();
    let call_count = Arc::new(Mutex::new(0usize));

    let handles: Vec<_> = (0..10)
        .map(|_| {
            let count = Arc::clone(&call_count);
            thread::spawn(move || {
                INIT_COUNT.get_or_init(|| {
                    *count.lock().unwrap() += 1;
                    42
                });
            })
        })
        .collect();

    for h in handles {
        h.join().unwrap();
    }
    let x = *call_count.lock().unwrap();
    x // should be 1 — init ran only once
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_config_same_value() {
        let c1 = get_config();
        let c2 = get_config();
        assert_eq!(c1, c2);
        assert!(std::ptr::eq(c1 as *const _, c2 as *const _)); // same allocation
    }

    #[test]
    fn test_primes_correctness() {
        let primes = get_primes();
        assert_eq!(&primes[..5], &[2, 3, 5, 7, 11]);
        assert!(!primes.contains(&4));
        assert!(!primes.contains(&100));
    }

    #[test]
    fn test_lazy_config_cached() {
        let lc = LazyConfig::new("test");
        let v1 = lc.get();
        let v2 = lc.get();
        assert_eq!(v1, "test-initialized");
        assert_eq!(v1, v2);
    }

    #[test]
    fn test_concurrent_once_init_runs_exactly_once() {
        // OnceLock guarantees init closure runs at most once
        // even with 10 concurrent threads
        let count = concurrent_once_init();
        assert_eq!(count, 1, "init should run exactly once");
    }

    #[test]
    fn test_oncelock_get_before_init() {
        let lock: OnceLock<i32> = OnceLock::new();
        assert!(lock.get().is_none());
        lock.get_or_init(|| 42);
        assert_eq!(lock.get(), Some(&42));
    }
}
