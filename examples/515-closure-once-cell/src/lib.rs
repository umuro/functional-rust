#![allow(clippy::all)]
//! Lazy Evaluation with OnceLock
//!
//! Deferred computation using std::sync::OnceLock.

use std::sync::OnceLock;

/// Global lazy value — initialized once on first access.
static EXPENSIVE_VALUE: OnceLock<i64> = OnceLock::new();

pub fn get_expensive_value() -> i64 {
    *EXPENSIVE_VALUE.get_or_init(|| (1..=1_000i64).sum())
}

/// Lazy struct: computes fields only when accessed.
pub struct LazyConfig {
    raw: String,
    parsed_items: OnceLock<Vec<String>>,
    item_count: OnceLock<usize>,
}

impl LazyConfig {
    pub fn new(raw: &str) -> Self {
        LazyConfig {
            raw: raw.to_string(),
            parsed_items: OnceLock::new(),
            item_count: OnceLock::new(),
        }
    }

    pub fn items(&self) -> &[String] {
        self.parsed_items.get_or_init(|| {
            self.raw
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect()
        })
    }

    pub fn count(&self) -> usize {
        *self.item_count.get_or_init(|| self.items().len())
    }

    pub fn raw(&self) -> &str {
        &self.raw
    }
}

/// Lazy computation with custom initializer.
pub struct Lazy<T, F = fn() -> T> {
    cell: OnceLock<T>,
    init: F,
}

impl<T, F: Fn() -> T> Lazy<T, F> {
    pub const fn new(init: F) -> Self {
        Lazy {
            cell: OnceLock::new(),
            init,
        }
    }

    pub fn get(&self) -> &T {
        self.cell.get_or_init(&self.init)
    }
}

/// Memoized single-value computation.
pub struct Memo<T> {
    value: OnceLock<T>,
}

impl<T> Memo<T> {
    pub const fn new() -> Self {
        Memo {
            value: OnceLock::new(),
        }
    }

    pub fn get_or_compute(&self, compute: impl FnOnce() -> T) -> &T {
        self.value.get_or_init(compute)
    }
}

impl<T> Default for Memo<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    #[test]
    fn test_expensive_value() {
        let v1 = get_expensive_value();
        let v2 = get_expensive_value();
        assert_eq!(v1, v2);
        assert_eq!(v1, 500500); // sum of 1..=1000
    }

    #[test]
    fn test_lazy_config_items() {
        let cfg = LazyConfig::new("a, b, c, d");
        assert_eq!(cfg.items(), &["a", "b", "c", "d"]);
    }

    #[test]
    fn test_lazy_config_count() {
        let cfg = LazyConfig::new("x, y, z");
        assert_eq!(cfg.count(), 3);
    }

    #[test]
    fn test_lazy_config_caches() {
        let cfg = LazyConfig::new("one, two");
        let items1 = cfg.items();
        let items2 = cfg.items();
        // Same reference (cached)
        assert!(std::ptr::eq(items1, items2));
    }

    #[test]
    fn test_memo_computes_once() {
        static CALL_COUNT: AtomicUsize = AtomicUsize::new(0);
        let memo: Memo<i32> = Memo::new();

        let v1 = memo.get_or_compute(|| {
            CALL_COUNT.fetch_add(1, Ordering::SeqCst);
            42
        });

        let v2 = memo.get_or_compute(|| {
            CALL_COUNT.fetch_add(1, Ordering::SeqCst);
            99
        });

        assert_eq!(*v1, 42);
        assert_eq!(*v2, 42);
        assert_eq!(CALL_COUNT.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn test_lazy_struct() {
        static INIT_COUNT: AtomicUsize = AtomicUsize::new(0);

        let lazy = Lazy::new(|| {
            INIT_COUNT.fetch_add(1, Ordering::SeqCst);
            vec![1, 2, 3]
        });

        assert_eq!(INIT_COUNT.load(Ordering::SeqCst), 0);
        assert_eq!(lazy.get(), &vec![1, 2, 3]);
        assert_eq!(INIT_COUNT.load(Ordering::SeqCst), 1);
        assert_eq!(lazy.get(), &vec![1, 2, 3]);
        assert_eq!(INIT_COUNT.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn test_empty_config() {
        let cfg = LazyConfig::new("");
        assert!(cfg.items().is_empty());
        assert_eq!(cfg.count(), 0);
    }
}
