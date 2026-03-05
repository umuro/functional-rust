//! # Arc<RwLock<T>> — Multiple Readers, One Writer
//!
//! Allow many threads to read shared data simultaneously, while guaranteeing
//! exclusive access for writes.

use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;

/// Approach 1: Shared configuration map
pub struct SharedConfig {
    data: Arc<RwLock<HashMap<String, String>>>,
}

impl SharedConfig {
    pub fn new() -> Self {
        Self {
            data: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn get(&self, key: &str) -> Option<String> {
        self.data.read().unwrap().get(key).cloned()
    }

    pub fn set(&self, key: String, value: String) {
        self.data.write().unwrap().insert(key, value);
    }

    pub fn clone_handle(&self) -> Arc<RwLock<HashMap<String, String>>> {
        Arc::clone(&self.data)
    }
}

impl Default for SharedConfig {
    fn default() -> Self {
        Self::new()
    }
}

/// Approach 2: Read-heavy workload simulation
pub fn simulate_read_heavy(
    num_readers: usize,
    reads_per_thread: usize,
    write_delay_ms: u64,
) -> (usize, String) {
    let data = Arc::new(RwLock::new(String::from("initial")));
    let read_count = Arc::new(std::sync::atomic::AtomicUsize::new(0));

    let readers: Vec<_> = (0..num_readers)
        .map(|_| {
            let d = Arc::clone(&data);
            let count = Arc::clone(&read_count);
            thread::spawn(move || {
                for _ in 0..reads_per_thread {
                    let guard = d.read().unwrap();
                    let _ = guard.len();
                    count.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                    drop(guard);
                    thread::sleep(Duration::from_micros(10));
                }
            })
        })
        .collect();

    let writer = {
        let d = Arc::clone(&data);
        thread::spawn(move || {
            thread::sleep(Duration::from_millis(write_delay_ms));
            *d.write().unwrap() = String::from("updated");
        })
    };

    for r in readers {
        r.join().unwrap();
    }
    writer.join().unwrap();

    let final_reads = read_count.load(std::sync::atomic::Ordering::Relaxed);
    let final_value = data.read().unwrap().clone();
    (final_reads, final_value)
}

/// Approach 3: Cache with concurrent reads
pub struct ReadCache<K, V> {
    cache: Arc<RwLock<HashMap<K, V>>>,
}

impl<K, V> ReadCache<K, V>
where
    K: std::hash::Hash + Eq + Clone,
    V: Clone,
{
    pub fn new() -> Self {
        Self {
            cache: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn get(&self, key: &K) -> Option<V> {
        self.cache.read().unwrap().get(key).cloned()
    }

    pub fn insert(&self, key: K, value: V) {
        self.cache.write().unwrap().insert(key, value);
    }

    pub fn len(&self) -> usize {
        self.cache.read().unwrap().len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn clone_inner(&self) -> Arc<RwLock<HashMap<K, V>>> {
        Arc::clone(&self.cache)
    }
}

impl<K, V> Default for ReadCache<K, V>
where
    K: std::hash::Hash + Eq + Clone,
    V: Clone,
{
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_concurrent_reads() {
        let d = Arc::new(RwLock::new(vec![1, 2, 3]));
        let handles: Vec<_> = (0..4)
            .map(|_| {
                let d = Arc::clone(&d);
                thread::spawn(move || d.read().unwrap().iter().sum::<i32>())
            })
            .collect();

        for h in handles {
            assert_eq!(h.join().unwrap(), 6);
        }
    }

    #[test]
    fn test_write_then_read() {
        let d = RwLock::new(0u32);
        *d.write().unwrap() = 42;
        assert_eq!(*d.read().unwrap(), 42);
    }

    #[test]
    fn test_shared_config() {
        let config = SharedConfig::new();
        config.set("host".into(), "localhost".into());
        config.set("port".into(), "8080".into());

        assert_eq!(config.get("host"), Some("localhost".into()));
        assert_eq!(config.get("port"), Some("8080".into()));
        assert_eq!(config.get("missing"), None);
    }

    #[test]
    fn test_shared_config_concurrent() {
        let config = SharedConfig::new();
        config.set("key".into(), "value".into());
        let handle = config.clone_handle();

        thread::scope(|s| {
            for _ in 0..4 {
                let h = Arc::clone(&handle);
                s.spawn(move || {
                    let guard = h.read().unwrap();
                    assert_eq!(guard.get("key"), Some(&String::from("value")));
                });
            }
        });
    }

    #[test]
    fn test_read_heavy_simulation() {
        let (reads, final_value) = simulate_read_heavy(4, 10, 5);
        assert_eq!(reads, 40);
        assert_eq!(final_value, "updated");
    }

    #[test]
    fn test_read_cache() {
        let cache: ReadCache<String, i32> = ReadCache::new();
        cache.insert("one".into(), 1);
        cache.insert("two".into(), 2);

        assert_eq!(cache.get(&"one".into()), Some(1));
        assert_eq!(cache.get(&"three".into()), None);
        assert_eq!(cache.len(), 2);
    }

    #[test]
    fn test_try_read_write() {
        let lock = RwLock::new(0);

        // Can get multiple read guards
        let r1 = lock.read().unwrap();
        let r2 = lock.try_read();
        assert!(r2.is_ok());
        drop(r1);
        drop(r2);

        // Write blocks reads
        let _w = lock.write().unwrap();
        assert!(lock.try_read().is_err());
    }
}
