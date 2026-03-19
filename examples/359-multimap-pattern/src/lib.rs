//! # Multimap Pattern
//! Map with multiple values per key.

use std::collections::HashMap;

pub struct MultiMap<K, V> {
    inner: HashMap<K, Vec<V>>,
}

impl<K: Eq + std::hash::Hash, V> MultiMap<K, V> {
    pub fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }

    pub fn insert(&mut self, key: K, value: V) {
        self.inner.entry(key).or_default().push(value);
    }

    pub fn get(&self, key: &K) -> Option<&Vec<V>> {
        self.inner.get(key)
    }

    pub fn get_all(&self, key: &K) -> Vec<&V> {
        self.inner
            .get(key)
            .map(|v| v.iter().collect())
            .unwrap_or_default()
    }

    pub fn remove_one(&mut self, key: &K) -> Option<V> {
        self.inner.get_mut(key).and_then(|v| v.pop())
    }

    pub fn count(&self, key: &K) -> usize {
        self.inner.get(key).map(|v| v.len()).unwrap_or(0)
    }
}

impl<K: Eq + std::hash::Hash, V> Default for MultiMap<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn multiple_values() {
        let mut mm = MultiMap::new();
        mm.insert("tags", "rust");
        mm.insert("tags", "async");
        assert_eq!(mm.count(&"tags"), 2);
    }
    #[test]
    fn get_all_values() {
        let mut mm = MultiMap::new();
        mm.insert(1, "a");
        mm.insert(1, "b");
        let vals: Vec<_> = mm.get_all(&1).into_iter().cloned().collect();
        assert_eq!(vals, vec!["a", "b"]);
    }
    #[test]
    fn remove_one() {
        let mut mm = MultiMap::new();
        mm.insert("k", 1);
        mm.insert("k", 2);
        assert_eq!(mm.remove_one(&"k"), Some(2));
        assert_eq!(mm.count(&"k"), 1);
    }
}
