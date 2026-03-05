//! LRU Cache
//!
//! Least Recently Used cache with O(1) get/put operations.

use std::collections::{HashMap, VecDeque};
use std::hash::Hash;

/// LRU Cache with fixed capacity
pub struct LruCache<K: Clone + Eq + Hash, V> {
    map: HashMap<K, V>,
    order: VecDeque<K>,
    capacity: usize,
}

impl<K: Clone + Eq + Hash, V> LruCache<K, V> {
    /// Create a new LRU cache with given capacity
    pub fn new(capacity: usize) -> Self {
        assert!(capacity > 0, "capacity must be > 0");
        Self {
            map: HashMap::new(),
            order: VecDeque::new(),
            capacity,
        }
    }

    /// Get a value, marking it as recently used
    pub fn get(&mut self, key: &K) -> Option<&V> {
        if self.map.contains_key(key) {
            self.order.retain(|k| k != key);
            self.order.push_front(key.clone());
            self.map.get(key)
        } else {
            None
        }
    }

    /// Put a value, evicting LRU if at capacity
    pub fn put(&mut self, key: K, val: V) {
        if self.map.contains_key(&key) {
            self.order.retain(|k| k != &key);
        } else if self.map.len() >= self.capacity {
            if let Some(lru_key) = self.order.pop_back() {
                self.map.remove(&lru_key);
            }
        }
        self.map.insert(key.clone(), val);
        self.order.push_front(key);
    }

    /// Check if key exists (without affecting LRU order)
    pub fn contains(&self, key: &K) -> bool {
        self.map.contains_key(key)
    }

    /// Get current size
    pub fn len(&self) -> usize {
        self.map.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    /// Get capacity
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// Remove a key
    pub fn remove(&mut self, key: &K) -> Option<V> {
        if self.map.contains_key(key) {
            self.order.retain(|k| k != key);
            self.map.remove(key)
        } else {
            None
        }
    }

    /// Clear all entries
    pub fn clear(&mut self) {
        self.map.clear();
        self.order.clear();
    }

    /// Get keys in LRU order (most recent first)
    pub fn keys(&self) -> impl Iterator<Item = &K> {
        self.order.iter()
    }
}

impl<K: Clone + Eq + Hash, V: Clone> LruCache<K, V> {
    /// Peek at a value without affecting LRU order
    pub fn peek(&self, key: &K) -> Option<&V> {
        self.map.get(key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_lru() {
        let mut cache: LruCache<i32, i32> = LruCache::new(2);
        cache.put(1, 1);
        cache.put(2, 2);
        assert_eq!(cache.get(&1), Some(&1));
        cache.put(3, 3); // 2 is LRU, gets evicted
        assert_eq!(cache.get(&2), None);
        assert_eq!(cache.get(&1), Some(&1));
        assert_eq!(cache.get(&3), Some(&3));
    }

    #[test]
    fn test_update_existing() {
        let mut cache: LruCache<&str, i32> = LruCache::new(2);
        cache.put("a", 1);
        cache.put("b", 2);
        cache.put("a", 10); // update, a becomes MRU
        cache.put("c", 3); // b evicted (was LRU)
        assert_eq!(cache.get(&"a"), Some(&10));
        assert_eq!(cache.get(&"b"), None);
        assert_eq!(cache.get(&"c"), Some(&3));
    }

    #[test]
    fn test_contains() {
        let mut cache: LruCache<i32, i32> = LruCache::new(2);
        cache.put(1, 1);
        assert!(cache.contains(&1));
        assert!(!cache.contains(&2));
    }

    #[test]
    fn test_remove() {
        let mut cache: LruCache<i32, i32> = LruCache::new(2);
        cache.put(1, 1);
        cache.put(2, 2);
        assert_eq!(cache.remove(&1), Some(1));
        assert_eq!(cache.len(), 1);
        assert!(!cache.contains(&1));
    }

    #[test]
    fn test_clear() {
        let mut cache: LruCache<i32, i32> = LruCache::new(2);
        cache.put(1, 1);
        cache.put(2, 2);
        cache.clear();
        assert!(cache.is_empty());
    }

    #[test]
    fn test_peek_no_affect() {
        let mut cache: LruCache<i32, i32> = LruCache::new(2);
        cache.put(1, 1);
        cache.put(2, 2);
        // Peek at 1 without making it MRU
        assert_eq!(cache.peek(&1), Some(&1));
        cache.put(3, 3); // 1 should be evicted (still LRU)
        assert_eq!(cache.peek(&1), None);
    }
}
