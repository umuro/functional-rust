// 961: LRU Cache
// OCaml: Hashtbl + Queue; Rust: HashMap + VecDeque
// Both O(1) get/put amortized (with O(n) queue cleanup in this simplified version)

use std::collections::{HashMap, VecDeque};

pub struct LruCache<K, V> {
    capacity: usize,
    table: HashMap<K, V>,
    order: VecDeque<K>,
}

impl<K: Eq + std::hash::Hash + Clone, V> LruCache<K, V> {
    pub fn new(capacity: usize) -> Self {
        assert!(capacity > 0, "capacity must be > 0");
        LruCache {
            capacity,
            table: HashMap::with_capacity(capacity),
            order: VecDeque::with_capacity(capacity),
        }
    }

    pub fn get(&mut self, key: &K) -> Option<&V> {
        if self.table.contains_key(key) {
            // Move to back (most recently used)
            self.order.retain(|k| k != key);
            self.order.push_back(key.clone());
            self.table.get(key)
        } else {
            None
        }
    }

    pub fn put(&mut self, key: K, value: V) {
        if self.table.contains_key(&key) {
            // Update: remove from order, re-add at back
            self.order.retain(|k| k != &key);
        } else if self.table.len() >= self.capacity {
            // Evict LRU (front of deque)
            if let Some(lru_key) = self.order.pop_front() {
                self.table.remove(&lru_key);
            }
        }
        self.table.insert(key.clone(), value);
        self.order.push_back(key);
    }

    pub fn size(&self) -> usize {
        self.table.len()
    }

    pub fn contains(&self, key: &K) -> bool {
        self.table.contains_key(key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_get_put() {
        let mut c: LruCache<&str, i32> = LruCache::new(3);
        c.put("a", 1);
        c.put("b", 2);
        c.put("c", 3);

        assert_eq!(c.get(&"a"), Some(&1));
        assert_eq!(c.get(&"b"), Some(&2));
        assert_eq!(c.size(), 3);
    }

    #[test]
    fn test_eviction() {
        let mut c: LruCache<&str, i32> = LruCache::new(3);
        c.put("a", 1);
        c.put("b", 2);
        c.put("c", 3);

        // Access "a" to make it recently used → order: b, c, a
        c.get(&"a");
        // Insert "d" → evicts "b" (front)
        c.put("d", 4);

        assert_eq!(c.size(), 3);
        assert_eq!(c.get(&"b"), None); // evicted
        assert_eq!(c.get(&"a"), Some(&1));
        assert_eq!(c.get(&"c"), Some(&3));
        assert_eq!(c.get(&"d"), Some(&4));
    }

    #[test]
    fn test_update_existing() {
        let mut c: LruCache<&str, i32> = LruCache::new(3);
        c.put("a", 1);
        c.put("b", 2);
        c.put("a", 99);
        assert_eq!(c.get(&"a"), Some(&99));
        assert_eq!(c.size(), 2);
    }

    #[test]
    fn test_capacity_one() {
        let mut c: LruCache<i32, &str> = LruCache::new(1);
        c.put(1, "one");
        c.put(2, "two"); // evicts 1
        assert_eq!(c.get(&1), None);
        assert_eq!(c.get(&2), Some(&"two"));
    }

    #[test]
    fn test_miss() {
        let mut c: LruCache<&str, i32> = LruCache::new(2);
        c.put("x", 10);
        assert_eq!(c.get(&"y"), None);
    }
}
