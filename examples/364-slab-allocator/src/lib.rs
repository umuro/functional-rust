#![allow(clippy::all)]
//! Slab Pattern for Indexed Storage
//!
//! Pre-allocated pool with stable integer indices.

/// A slab allocator that returns stable integer keys
#[derive(Debug)]
pub struct Slab<T> {
    entries: Vec<Option<T>>,
    free: Vec<usize>,
}

impl<T> Slab<T> {
    // === Approach 1: Basic API ===

    /// Create a new empty slab
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            free: Vec::new(),
        }
    }

    /// Create a slab with pre-allocated capacity
    pub fn with_capacity(cap: usize) -> Self {
        Self {
            entries: Vec::with_capacity(cap),
            free: Vec::new(),
        }
    }

    /// Insert a value and return its stable key
    pub fn insert(&mut self, val: T) -> usize {
        if let Some(key) = self.free.pop() {
            self.entries[key] = Some(val);
            key
        } else {
            let key = self.entries.len();
            self.entries.push(Some(val));
            key
        }
    }

    /// Get a reference to a value by key
    pub fn get(&self, key: usize) -> Option<&T> {
        self.entries.get(key)?.as_ref()
    }

    /// Get a mutable reference to a value by key
    pub fn get_mut(&mut self, key: usize) -> Option<&mut T> {
        self.entries.get_mut(key)?.as_mut()
    }

    /// Remove a value by key, returning it if it existed
    pub fn remove(&mut self, key: usize) -> Option<T> {
        let slot = self.entries.get_mut(key)?;
        let val = slot.take()?;
        self.free.push(key);
        Some(val)
    }

    // === Approach 2: Query methods ===

    /// Get the number of occupied slots
    pub fn len(&self) -> usize {
        self.entries.iter().filter(|e| e.is_some()).count()
    }

    /// Check if the slab is empty
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Check if a key is currently occupied
    pub fn contains(&self, key: usize) -> bool {
        self.get(key).is_some()
    }

    /// Get the total capacity (occupied + free slots)
    pub fn capacity(&self) -> usize {
        self.entries.len()
    }

    // === Approach 3: Iteration ===

    /// Iterate over (key, value) pairs
    pub fn iter(&self) -> impl Iterator<Item = (usize, &T)> {
        self.entries
            .iter()
            .enumerate()
            .filter_map(|(i, e)| e.as_ref().map(|v| (i, v)))
    }

    /// Iterate over (key, mutable value) pairs
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (usize, &mut T)> {
        self.entries
            .iter_mut()
            .enumerate()
            .filter_map(|(i, e)| e.as_mut().map(|v| (i, v)))
    }

    /// Iterate over keys only
    pub fn keys(&self) -> impl Iterator<Item = usize> + '_ {
        self.entries
            .iter()
            .enumerate()
            .filter_map(|(i, e)| if e.is_some() { Some(i) } else { None })
    }

    /// Iterate over values only
    pub fn values(&self) -> impl Iterator<Item = &T> {
        self.entries.iter().filter_map(|e| e.as_ref())
    }

    /// Clear all entries
    pub fn clear(&mut self) {
        self.entries.clear();
        self.free.clear();
    }

    /// Retain only entries matching a predicate
    pub fn retain<F>(&mut self, mut f: F)
    where
        F: FnMut(usize, &T) -> bool,
    {
        for (i, slot) in self.entries.iter_mut().enumerate() {
            if let Some(ref val) = slot {
                if !f(i, val) {
                    *slot = None;
                    self.free.push(i);
                }
            }
        }
    }
}

impl<T> Default for Slab<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> std::ops::Index<usize> for Slab<T> {
    type Output = T;

    fn index(&self, key: usize) -> &Self::Output {
        self.get(key).expect("no entry for key")
    }
}

impl<T> std::ops::IndexMut<usize> for Slab<T> {
    fn index_mut(&mut self, key: usize) -> &mut Self::Output {
        self.get_mut(key).expect("no entry for key")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_get() {
        let mut slab: Slab<i32> = Slab::new();
        let key = slab.insert(42);
        assert_eq!(slab.get(key), Some(&42));
    }

    #[test]
    fn test_remove_and_reuse() {
        let mut slab: Slab<i32> = Slab::new();
        let k1 = slab.insert(1);
        let _k2 = slab.insert(2);
        slab.remove(k1);
        let k3 = slab.insert(3);
        assert_eq!(k3, k1); // slot reused
    }

    #[test]
    fn test_stable_keys() {
        let mut slab: Slab<String> = Slab::new();
        let key = slab.insert("stable".to_string());
        for i in 0..100 {
            slab.insert(format!("filler-{}", i));
        }
        assert_eq!(slab.get(key).unwrap(), "stable");
    }

    #[test]
    fn test_get_mut() {
        let mut slab: Slab<i32> = Slab::new();
        let key = slab.insert(10);
        *slab.get_mut(key).unwrap() = 20;
        assert_eq!(slab.get(key), Some(&20));
    }

    #[test]
    fn test_len_and_contains() {
        let mut slab: Slab<&str> = Slab::new();
        assert!(slab.is_empty());

        let k1 = slab.insert("a");
        let k2 = slab.insert("b");
        assert_eq!(slab.len(), 2);
        assert!(slab.contains(k1));

        slab.remove(k1);
        assert_eq!(slab.len(), 1);
        assert!(!slab.contains(k1));
        assert!(slab.contains(k2));
    }

    #[test]
    fn test_iteration() {
        let mut slab: Slab<i32> = Slab::new();
        slab.insert(10);
        slab.insert(20);
        slab.insert(30);

        let sum: i32 = slab.values().sum();
        assert_eq!(sum, 60);

        let keys: Vec<usize> = slab.keys().collect();
        assert_eq!(keys, vec![0, 1, 2]);
    }

    #[test]
    fn test_index_operator() {
        let mut slab: Slab<&str> = Slab::new();
        let key = slab.insert("test");
        assert_eq!(slab[key], "test");
    }

    #[test]
    fn test_retain() {
        let mut slab: Slab<i32> = Slab::new();
        slab.insert(1);
        slab.insert(2);
        slab.insert(3);
        slab.insert(4);

        slab.retain(|_, v| *v % 2 == 0);
        let values: Vec<i32> = slab.values().copied().collect();
        assert_eq!(values, vec![2, 4]);
    }

    #[test]
    fn test_clear() {
        let mut slab: Slab<i32> = Slab::new();
        slab.insert(1);
        slab.insert(2);
        slab.clear();
        assert!(slab.is_empty());
    }

    #[test]
    fn test_with_capacity() {
        let slab: Slab<i32> = Slab::with_capacity(100);
        assert!(slab.is_empty());
    }
}
