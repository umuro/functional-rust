#![allow(clippy::all)]
//! # IndexMap Ordered
//! HashMap that preserves insertion order (simulated without external crate).

use std::collections::HashMap;

pub struct OrderedMap<K, V> {
    map: HashMap<K, V>,
    order: Vec<K>,
}

impl<K: Clone + Eq + std::hash::Hash, V> OrderedMap<K, V> {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
            order: Vec::new(),
        }
    }

    pub fn insert(&mut self, key: K, value: V) {
        if self.map.insert(key.clone(), value).is_none() {
            self.order.push(key);
        }
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        self.map.get(key)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&K, &V)> {
        self.order
            .iter()
            .filter_map(|k| self.map.get(k).map(|v| (k, v)))
    }

    pub fn len(&self) -> usize {
        self.map.len()
    }
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }
}

impl<K: Clone + Eq + std::hash::Hash, V> Default for OrderedMap<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn preserves_order() {
        let mut m = OrderedMap::new();
        m.insert("b", 2);
        m.insert("a", 1);
        m.insert("c", 3);
        let keys: Vec<_> = m.iter().map(|(k, _)| *k).collect();
        assert_eq!(keys, vec!["b", "a", "c"]);
    }
    #[test]
    fn get_works() {
        let mut m = OrderedMap::new();
        m.insert("k", 42);
        assert_eq!(m.get(&"k"), Some(&42));
    }
}
