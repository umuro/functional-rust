//! # Concurrent Counter — Thread-Safe Counting
//!
//! Various approaches to thread-safe counters.

use std::sync::atomic::{AtomicI64, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;

/// Basic atomic counter
pub struct AtomicCounter {
    value: AtomicUsize,
}

impl AtomicCounter {
    pub fn new(initial: usize) -> Self {
        Self {
            value: AtomicUsize::new(initial),
        }
    }

    pub fn increment(&self) -> usize {
        self.value.fetch_add(1, Ordering::SeqCst)
    }

    pub fn decrement(&self) -> usize {
        self.value.fetch_sub(1, Ordering::SeqCst)
    }

    pub fn add(&self, n: usize) -> usize {
        self.value.fetch_add(n, Ordering::SeqCst)
    }

    pub fn get(&self) -> usize {
        self.value.load(Ordering::SeqCst)
    }

    pub fn set(&self, value: usize) {
        self.value.store(value, Ordering::SeqCst);
    }

    pub fn reset(&self) -> usize {
        self.value.swap(0, Ordering::SeqCst)
    }
}

impl Default for AtomicCounter {
    fn default() -> Self {
        Self::new(0)
    }
}

/// Sharded counter for reduced contention
pub struct ShardedCounter {
    shards: Vec<AtomicI64>,
}

impl ShardedCounter {
    pub fn new(num_shards: usize) -> Self {
        let shards = (0..num_shards).map(|_| AtomicI64::new(0)).collect();
        Self { shards }
    }

    /// Increment using thread ID for shard selection
    pub fn increment(&self) {
        let shard = thread_id_hash() % self.shards.len();
        self.shards[shard].fetch_add(1, Ordering::Relaxed);
    }

    /// Get total across all shards
    pub fn get(&self) -> i64 {
        self.shards
            .iter()
            .map(|s| s.load(Ordering::Relaxed))
            .sum()
    }

    /// Reset all shards
    pub fn reset(&self) {
        for shard in &self.shards {
            shard.store(0, Ordering::Relaxed);
        }
    }
}

fn thread_id_hash() -> usize {
    use std::hash::{Hash, Hasher};
    let id = std::thread::current().id();
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    id.hash(&mut hasher);
    hasher.finish() as usize
}

/// Statistics counter tracking min, max, sum, count
pub struct StatsCounter {
    count: AtomicUsize,
    sum: AtomicI64,
    min: AtomicI64,
    max: AtomicI64,
}

impl StatsCounter {
    pub fn new() -> Self {
        Self {
            count: AtomicUsize::new(0),
            sum: AtomicI64::new(0),
            min: AtomicI64::new(i64::MAX),
            max: AtomicI64::new(i64::MIN),
        }
    }

    pub fn record(&self, value: i64) {
        self.count.fetch_add(1, Ordering::Relaxed);
        self.sum.fetch_add(value, Ordering::Relaxed);

        // Update min
        let mut current_min = self.min.load(Ordering::Relaxed);
        while value < current_min {
            match self.min.compare_exchange_weak(
                current_min,
                value,
                Ordering::Relaxed,
                Ordering::Relaxed,
            ) {
                Ok(_) => break,
                Err(c) => current_min = c,
            }
        }

        // Update max
        let mut current_max = self.max.load(Ordering::Relaxed);
        while value > current_max {
            match self.max.compare_exchange_weak(
                current_max,
                value,
                Ordering::Relaxed,
                Ordering::Relaxed,
            ) {
                Ok(_) => break,
                Err(c) => current_max = c,
            }
        }
    }

    pub fn count(&self) -> usize {
        self.count.load(Ordering::Relaxed)
    }

    pub fn sum(&self) -> i64 {
        self.sum.load(Ordering::Relaxed)
    }

    pub fn min(&self) -> Option<i64> {
        let v = self.min.load(Ordering::Relaxed);
        if v == i64::MAX {
            None
        } else {
            Some(v)
        }
    }

    pub fn max(&self) -> Option<i64> {
        let v = self.max.load(Ordering::Relaxed);
        if v == i64::MIN {
            None
        } else {
            Some(v)
        }
    }

    pub fn average(&self) -> Option<f64> {
        let c = self.count();
        if c == 0 {
            None
        } else {
            Some(self.sum() as f64 / c as f64)
        }
    }
}

impl Default for StatsCounter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_atomic_counter() {
        let counter = AtomicCounter::new(0);

        counter.increment();
        counter.increment();
        assert_eq!(counter.get(), 2);

        counter.decrement();
        assert_eq!(counter.get(), 1);

        counter.add(10);
        assert_eq!(counter.get(), 11);
    }

    #[test]
    fn test_atomic_counter_concurrent() {
        let counter = Arc::new(AtomicCounter::new(0));

        let handles: Vec<_> = (0..4)
            .map(|_| {
                let c = Arc::clone(&counter);
                thread::spawn(move || {
                    for _ in 0..1000 {
                        c.increment();
                    }
                })
            })
            .collect();

        for h in handles {
            h.join().unwrap();
        }

        assert_eq!(counter.get(), 4000);
    }

    #[test]
    fn test_sharded_counter() {
        let counter = Arc::new(ShardedCounter::new(8));

        let handles: Vec<_> = (0..4)
            .map(|_| {
                let c = Arc::clone(&counter);
                thread::spawn(move || {
                    for _ in 0..1000 {
                        c.increment();
                    }
                })
            })
            .collect();

        for h in handles {
            h.join().unwrap();
        }

        assert_eq!(counter.get(), 4000);
    }

    #[test]
    fn test_stats_counter() {
        let stats = StatsCounter::new();

        stats.record(10);
        stats.record(20);
        stats.record(5);
        stats.record(15);

        assert_eq!(stats.count(), 4);
        assert_eq!(stats.sum(), 50);
        assert_eq!(stats.min(), Some(5));
        assert_eq!(stats.max(), Some(20));
        assert_eq!(stats.average(), Some(12.5));
    }

    #[test]
    fn test_stats_counter_empty() {
        let stats = StatsCounter::new();

        assert_eq!(stats.count(), 0);
        assert_eq!(stats.min(), None);
        assert_eq!(stats.max(), None);
        assert_eq!(stats.average(), None);
    }

    #[test]
    fn test_reset() {
        let counter = AtomicCounter::new(10);
        let old = counter.reset();
        assert_eq!(old, 10);
        assert_eq!(counter.get(), 0);
    }
}
