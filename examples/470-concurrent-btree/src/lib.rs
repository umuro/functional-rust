#![allow(clippy::all)]
//! # Concurrent BTree — Ordered Thread-Safe Map
//!
//! A thread-safe ordered map using BTreeMap with RwLock.

use std::collections::BTreeMap;
use std::ops::RangeBounds;
use std::sync::RwLock;

/// Concurrent BTreeMap wrapper
pub struct ConcurrentBTree<K, V> {
    inner: RwLock<BTreeMap<K, V>>,
}

impl<K: Ord + Clone, V: Clone> ConcurrentBTree<K, V> {
    pub fn new() -> Self {
        Self {
            inner: RwLock::new(BTreeMap::new()),
        }
    }

    /// Insert a key-value pair
    pub fn insert(&self, key: K, value: V) -> Option<V> {
        self.inner.write().unwrap().insert(key, value)
    }

    /// Get a value by key
    pub fn get(&self, key: &K) -> Option<V> {
        self.inner.read().unwrap().get(key).cloned()
    }

    /// Remove a key
    pub fn remove(&self, key: &K) -> Option<V> {
        self.inner.write().unwrap().remove(key)
    }

    /// Check if key exists
    pub fn contains_key(&self, key: &K) -> bool {
        self.inner.read().unwrap().contains_key(key)
    }

    /// Get map length
    pub fn len(&self) -> usize {
        self.inner.read().unwrap().len()
    }

    pub fn is_empty(&self) -> bool {
        self.inner.read().unwrap().is_empty()
    }

    /// Get the smallest key
    pub fn first_key(&self) -> Option<K> {
        self.inner.read().unwrap().keys().next().cloned()
    }

    /// Get the largest key
    pub fn last_key(&self) -> Option<K> {
        self.inner.read().unwrap().keys().last().cloned()
    }

    /// Get all keys in range
    pub fn range<R: RangeBounds<K>>(&self, range: R) -> Vec<(K, V)> {
        self.inner
            .read()
            .unwrap()
            .range(range)
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect()
    }

    /// Get all keys
    pub fn keys(&self) -> Vec<K> {
        self.inner.read().unwrap().keys().cloned().collect()
    }

    /// Clear the map
    pub fn clear(&self) {
        self.inner.write().unwrap().clear();
    }
}

impl<K: Ord + Clone, V: Clone> Default for ConcurrentBTree<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

/// Concurrent sorted set
pub struct ConcurrentSortedSet<T> {
    inner: ConcurrentBTree<T, ()>,
}

impl<T: Ord + Clone> ConcurrentSortedSet<T> {
    pub fn new() -> Self {
        Self {
            inner: ConcurrentBTree::new(),
        }
    }

    pub fn insert(&self, value: T) -> bool {
        self.inner.insert(value, ()).is_none()
    }

    pub fn remove(&self, value: &T) -> bool {
        self.inner.remove(value).is_some()
    }

    pub fn contains(&self, value: &T) -> bool {
        self.inner.contains_key(value)
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    pub fn first(&self) -> Option<T> {
        self.inner.first_key()
    }

    pub fn last(&self) -> Option<T> {
        self.inner.last_key()
    }

    pub fn to_vec(&self) -> Vec<T> {
        self.inner.keys()
    }
}

impl<T: Ord + Clone> Default for ConcurrentSortedSet<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_insert_get() {
        let tree = ConcurrentBTree::new();
        tree.insert(5, "five");
        tree.insert(3, "three");
        tree.insert(7, "seven");

        assert_eq!(tree.get(&5), Some("five"));
        assert_eq!(tree.get(&3), Some("three"));
        assert_eq!(tree.get(&1), None);
    }

    #[test]
    fn test_ordering() {
        let tree = ConcurrentBTree::new();
        tree.insert(5, 'e');
        tree.insert(3, 'c');
        tree.insert(7, 'g');
        tree.insert(1, 'a');

        assert_eq!(tree.first_key(), Some(1));
        assert_eq!(tree.last_key(), Some(7));

        let keys = tree.keys();
        assert_eq!(keys, vec![1, 3, 5, 7]);
    }

    #[test]
    fn test_range() {
        let tree = ConcurrentBTree::new();
        for i in 0..10 {
            tree.insert(i, i * 10);
        }

        let range: Vec<_> = tree.range(3..7);
        assert_eq!(range, vec![(3, 30), (4, 40), (5, 50), (6, 60)]);
    }

    #[test]
    fn test_concurrent_insert() {
        let tree = Arc::new(ConcurrentBTree::new());

        let handles: Vec<_> = (0..4)
            .map(|i| {
                let t = Arc::clone(&tree);
                thread::spawn(move || {
                    for j in 0..100 {
                        t.insert(i * 100 + j, j);
                    }
                })
            })
            .collect();

        for h in handles {
            h.join().unwrap();
        }

        assert_eq!(tree.len(), 400);
    }

    #[test]
    fn test_sorted_set() {
        let set = ConcurrentSortedSet::new();
        set.insert(5);
        set.insert(3);
        set.insert(7);
        set.insert(3); // Duplicate

        assert_eq!(set.len(), 3);
        assert_eq!(set.first(), Some(3));
        assert_eq!(set.last(), Some(7));
        assert_eq!(set.to_vec(), vec![3, 5, 7]);
    }

    #[test]
    fn test_remove() {
        let tree = ConcurrentBTree::new();
        tree.insert(1, "one");
        tree.insert(2, "two");

        assert_eq!(tree.remove(&1), Some("one"));
        assert!(!tree.contains_key(&1));
        assert_eq!(tree.len(), 1);
    }

    #[test]
    fn test_clear() {
        let tree = ConcurrentBTree::new();
        tree.insert(1, 1);
        tree.insert(2, 2);
        tree.clear();
        assert!(tree.is_empty());
    }
}
