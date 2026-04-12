#![allow(dead_code)]
#![allow(clippy::all)]
// 1042: Bidirectional Map — Two HashMaps for Key↔Value
// Both key and value must be unique — inserting overwrites both directions

use std::collections::HashMap;
use std::hash::Hash;

/// Bidirectional map: key ↔ value, both unique
struct BiMap<K, V> {
    forward: HashMap<K, V>,
    backward: HashMap<V, K>,
}

impl<K, V> BiMap<K, V>
where
    K: Hash + Eq + Clone,
    V: Hash + Eq + Clone,
{
    fn new() -> Self {
        BiMap {
            forward: HashMap::new(),
            backward: HashMap::new(),
        }
    }

    fn insert(&mut self, key: K, value: V) {
        // Remove old mappings if key or value already exists
        if let Some(old_value) = self.forward.remove(&key) {
            self.backward.remove(&old_value);
        }
        if let Some(old_key) = self.backward.remove(&value) {
            self.forward.remove(&old_key);
        }

        self.backward.insert(value.clone(), key.clone());
        self.forward.insert(key, value);
    }

    fn get_by_key(&self, key: &K) -> Option<&V> {
        self.forward.get(key)
    }

    fn get_by_value(&self, value: &V) -> Option<&K> {
        self.backward.get(value)
    }

    fn remove_by_key(&mut self, key: &K) -> Option<V> {
        if let Some(value) = self.forward.remove(key) {
            self.backward.remove(&value);
            Some(value)
        } else {
            None
        }
    }

    fn remove_by_value(&mut self, value: &V) -> Option<K> {
        if let Some(key) = self.backward.remove(value) {
            self.forward.remove(&key);
            Some(key)
        } else {
            None
        }
    }

    fn len(&self) -> usize {
        self.forward.len()
    }

    fn contains_key(&self, key: &K) -> bool {
        self.forward.contains_key(key)
    }

    fn contains_value(&self, value: &V) -> bool {
        self.backward.contains_key(value)
    }
}

fn basic_ops() {
    let mut bm = BiMap::new();
    bm.insert("one", 1);
    bm.insert("two", 2);
    bm.insert("three", 3);

    assert_eq!(bm.get_by_key(&"two"), Some(&2));
    assert_eq!(bm.get_by_value(&2), Some(&"two"));
    assert_eq!(bm.get_by_key(&"four"), None);
    assert_eq!(bm.len(), 3);
}

fn overwrite_test() {
    let mut bm = BiMap::new();
    bm.insert("a", 1);
    bm.insert("b", 2);
    bm.insert("a", 3); // overwrites "a"->1, removes 1->"a"

    assert_eq!(bm.get_by_key(&"a"), Some(&3));
    assert_eq!(bm.get_by_value(&1), None); // old mapping gone
    assert_eq!(bm.get_by_value(&3), Some(&"a"));
}

fn codec_test() {
    let mut bm = BiMap::new();
    bm.insert("red", 0xFF0000u32);
    bm.insert("green", 0x00FF00);
    bm.insert("blue", 0x0000FF);

    assert_eq!(bm.get_by_key(&"red"), Some(&0xFF0000));
    assert_eq!(bm.get_by_value(&0x00FF00), Some(&"green"));

    bm.remove_by_key(&"red");
    assert_eq!(bm.get_by_key(&"red"), None);
    assert_eq!(bm.get_by_value(&0xFF0000u32), None);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        basic_ops();
    }

    #[test]
    fn test_overwrite() {
        overwrite_test();
    }

    #[test]
    fn test_codec() {
        codec_test();
    }

    #[test]
    fn test_remove_by_value() {
        let mut bm = BiMap::new();
        bm.insert("x", 10);
        bm.insert("y", 20);
        assert_eq!(bm.remove_by_value(&10), Some("x"));
        assert!(!bm.contains_key(&"x"));
        assert!(!bm.contains_value(&10));
    }
}
