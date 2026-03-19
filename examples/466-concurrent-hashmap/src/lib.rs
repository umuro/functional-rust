//! # Concurrent HashMap — Thread-Safe Key-Value Store
//!
//! A sharded hashmap for concurrent access with reduced lock contention.

use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::sync::{Arc, RwLock};

const NUM_SHARDS: usize = 16;

/// Sharded concurrent hashmap
pub struct ConcurrentHashMap<K, V> {
    shards: Vec<RwLock<HashMap<K, V>>>,
}

impl<K, V> ConcurrentHashMap<K, V>
where
    K: Hash + Eq + Clone,
    V: Clone,
{
    pub fn new() -> Self {
        let shards = (0..NUM_SHARDS)
            .map(|_| RwLock::new(HashMap::new()))
            .collect();
        Self { shards }
    }

    fn shard_index(&self, key: &K) -> usize {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        key.hash(&mut hasher);
        (hasher.finish() as usize) % NUM_SHARDS
    }

    /// Insert a key-value pair
    pub fn insert(&self, key: K, value: V) -> Option<V> {
        let idx = self.shard_index(&key);
        self.shards[idx].write().unwrap().insert(key, value)
    }

    /// Get a value by key
    pub fn get(&self, key: &K) -> Option<V> {
        let idx = self.shard_index(key);
        self.shards[idx].read().unwrap().get(key).cloned()
    }

    /// Remove a key
    pub fn remove(&self, key: &K) -> Option<V> {
        let idx = self.shard_index(key);
        self.shards[idx].write().unwrap().remove(key)
    }

    /// Check if key exists
    pub fn contains_key(&self, key: &K) -> bool {
        let idx = self.shard_index(key);
        self.shards[idx].read().unwrap().contains_key(key)
    }

    /// Get total length across all shards
    pub fn len(&self) -> usize {
        self.shards.iter().map(|s| s.read().unwrap().len()).sum()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Update a value atomically
    pub fn update<F>(&self, key: &K, f: F) -> Option<V>
    where
        F: FnOnce(&V) -> V,
    {
        let idx = self.shard_index(key);
        let mut shard = self.shards[idx].write().unwrap();
        if let Some(v) = shard.get(key) {
            let new_value = f(v);
            shard.insert(key.clone(), new_value.clone());
            Some(new_value)
        } else {
            None
        }
    }

    /// Get or insert default
    pub fn get_or_insert(&self, key: K, default: V) -> V {
        let idx = self.shard_index(&key);
        let mut shard = self.shards[idx].write().unwrap();
        shard.entry(key).or_insert(default).clone()
    }
}

impl<K, V> Default for ConcurrentHashMap<K, V>
where
    K: Hash + Eq + Clone,
    V: Clone,
{
    fn default() -> Self {
        Self::new()
    }
}

/// Simple concurrent counter map
pub struct CounterMap {
    map: ConcurrentHashMap<String, i64>,
}

impl CounterMap {
    pub fn new() -> Self {
        Self {
            map: ConcurrentHashMap::new(),
        }
    }

    pub fn increment(&self, key: &str) -> i64 {
        let idx = {
            let mut hasher = std::collections::hash_map::DefaultHasher::new();
            key.hash(&mut hasher);
            (hasher.finish() as usize) % NUM_SHARDS
        };

        let mut shard = self.map.shards[idx].write().unwrap();
        let entry = shard.entry(key.to_string()).or_insert(0);
        *entry += 1;
        *entry
    }

    pub fn get(&self, key: &str) -> i64 {
        self.map.get(&key.to_string()).unwrap_or(0)
    }
}

impl Default for CounterMap {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_insert_get() {
        let map = ConcurrentHashMap::new();
        map.insert("key1", 42);
        map.insert("key2", 100);

        assert_eq!(map.get(&"key1"), Some(42));
        assert_eq!(map.get(&"key2"), Some(100));
        assert_eq!(map.get(&"key3"), None);
    }

    #[test]
    fn test_remove() {
        let map = ConcurrentHashMap::new();
        map.insert("key", "value");

        assert_eq!(map.remove(&"key"), Some("value"));
        assert_eq!(map.get(&"key"), None);
    }

    #[test]
    fn test_concurrent_insert() {
        let map = Arc::new(ConcurrentHashMap::new());

        let handles: Vec<_> = (0..8)
            .map(|i| {
                let m = Arc::clone(&map);
                thread::spawn(move || {
                    for j in 0..100 {
                        m.insert(format!("key-{}-{}", i, j), i * 100 + j);
                    }
                })
            })
            .collect();

        for h in handles {
            h.join().unwrap();
        }

        assert_eq!(map.len(), 800);
    }

    #[test]
    fn test_update() {
        let map = ConcurrentHashMap::new();
        map.insert("counter", 0);

        for _ in 0..10 {
            map.update(&"counter", |v| v + 1);
        }

        assert_eq!(map.get(&"counter"), Some(10));
    }

    #[test]
    fn test_counter_map() {
        let counter = Arc::new(CounterMap::new());

        let handles: Vec<_> = (0..4)
            .map(|_| {
                let c = Arc::clone(&counter);
                thread::spawn(move || {
                    for _ in 0..100 {
                        c.increment("shared");
                    }
                })
            })
            .collect();

        for h in handles {
            h.join().unwrap();
        }

        assert_eq!(counter.get("shared"), 400);
    }

    #[test]
    fn test_get_or_insert() {
        let map = ConcurrentHashMap::new();

        let v1 = map.get_or_insert("key", 42);
        assert_eq!(v1, 42);

        let v2 = map.get_or_insert("key", 100);
        assert_eq!(v2, 42); // Original value
    }
}
