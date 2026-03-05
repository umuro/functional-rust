//! # Concurrent Set — Thread-Safe Set Operations
//!
//! A thread-safe set implementation.

use std::collections::HashSet;
use std::hash::Hash;
use std::sync::RwLock;

/// Simple concurrent set using RwLock
pub struct ConcurrentSet<T> {
    inner: RwLock<HashSet<T>>,
}

impl<T: Hash + Eq + Clone> ConcurrentSet<T> {
    pub fn new() -> Self {
        Self {
            inner: RwLock::new(HashSet::new()),
        }
    }

    /// Insert a value, returns true if it was not present
    pub fn insert(&self, value: T) -> bool {
        self.inner.write().unwrap().insert(value)
    }

    /// Remove a value, returns true if it was present
    pub fn remove(&self, value: &T) -> bool {
        self.inner.write().unwrap().remove(value)
    }

    /// Check if value exists
    pub fn contains(&self, value: &T) -> bool {
        self.inner.read().unwrap().contains(value)
    }

    /// Get set size
    pub fn len(&self) -> usize {
        self.inner.read().unwrap().len()
    }

    pub fn is_empty(&self) -> bool {
        self.inner.read().unwrap().is_empty()
    }

    /// Clear the set
    pub fn clear(&self) {
        self.inner.write().unwrap().clear();
    }

    /// Get all values (cloned)
    pub fn to_vec(&self) -> Vec<T> {
        self.inner.read().unwrap().iter().cloned().collect()
    }
}

impl<T: Hash + Eq + Clone> Default for ConcurrentSet<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// Set with add-only semantics (simpler for concurrent use)
pub struct AddOnlySet<T> {
    inner: RwLock<HashSet<T>>,
}

impl<T: Hash + Eq + Clone> AddOnlySet<T> {
    pub fn new() -> Self {
        Self {
            inner: RwLock::new(HashSet::new()),
        }
    }

    pub fn add(&self, value: T) -> bool {
        self.inner.write().unwrap().insert(value)
    }

    pub fn contains(&self, value: &T) -> bool {
        self.inner.read().unwrap().contains(value)
    }

    pub fn len(&self) -> usize {
        self.inner.read().unwrap().len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<T: Hash + Eq + Clone> Default for AddOnlySet<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// Concurrent set union, intersection, difference
pub fn set_union<T: Hash + Eq + Clone>(a: &ConcurrentSet<T>, b: &ConcurrentSet<T>) -> Vec<T> {
    let set_a = a.inner.read().unwrap();
    let set_b = b.inner.read().unwrap();
    set_a.union(&set_b).cloned().collect()
}

pub fn set_intersection<T: Hash + Eq + Clone>(a: &ConcurrentSet<T>, b: &ConcurrentSet<T>) -> Vec<T> {
    let set_a = a.inner.read().unwrap();
    let set_b = b.inner.read().unwrap();
    set_a.intersection(&set_b).cloned().collect()
}

pub fn set_difference<T: Hash + Eq + Clone>(a: &ConcurrentSet<T>, b: &ConcurrentSet<T>) -> Vec<T> {
    let set_a = a.inner.read().unwrap();
    let set_b = b.inner.read().unwrap();
    set_a.difference(&set_b).cloned().collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_insert_contains() {
        let set = ConcurrentSet::new();
        assert!(set.insert(1));
        assert!(!set.insert(1)); // Already exists
        assert!(set.contains(&1));
        assert!(!set.contains(&2));
    }

    #[test]
    fn test_remove() {
        let set = ConcurrentSet::new();
        set.insert(1);
        assert!(set.remove(&1));
        assert!(!set.remove(&1)); // Already removed
        assert!(!set.contains(&1));
    }

    #[test]
    fn test_concurrent_insert() {
        let set = Arc::new(ConcurrentSet::new());

        let handles: Vec<_> = (0..4)
            .map(|i| {
                let s = Arc::clone(&set);
                thread::spawn(move || {
                    for j in 0..100 {
                        s.insert(i * 100 + j);
                    }
                })
            })
            .collect();

        for h in handles {
            h.join().unwrap();
        }

        assert_eq!(set.len(), 400);
    }

    #[test]
    fn test_add_only_set() {
        let set = AddOnlySet::new();
        assert!(set.add(1));
        assert!(!set.add(1));
        assert!(set.contains(&1));
    }

    #[test]
    fn test_set_union() {
        let a = ConcurrentSet::new();
        let b = ConcurrentSet::new();

        a.insert(1);
        a.insert(2);
        b.insert(2);
        b.insert(3);

        let mut union = set_union(&a, &b);
        union.sort();
        assert_eq!(union, vec![1, 2, 3]);
    }

    #[test]
    fn test_set_intersection() {
        let a = ConcurrentSet::new();
        let b = ConcurrentSet::new();

        a.insert(1);
        a.insert(2);
        a.insert(3);
        b.insert(2);
        b.insert(3);
        b.insert(4);

        let mut inter = set_intersection(&a, &b);
        inter.sort();
        assert_eq!(inter, vec![2, 3]);
    }

    #[test]
    fn test_set_difference() {
        let a = ConcurrentSet::new();
        let b = ConcurrentSet::new();

        a.insert(1);
        a.insert(2);
        a.insert(3);
        b.insert(2);

        let mut diff = set_difference(&a, &b);
        diff.sort();
        assert_eq!(diff, vec![1, 3]);
    }

    #[test]
    fn test_clear() {
        let set = ConcurrentSet::new();
        set.insert(1);
        set.insert(2);
        set.clear();
        assert!(set.is_empty());
    }
}
