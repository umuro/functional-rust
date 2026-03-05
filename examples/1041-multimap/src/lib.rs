// 1041: Multimap — HashMap<K, Vec<V>> with Helpers
// A map where each key can have multiple values

use std::collections::HashMap;

/// A multimap: each key maps to a Vec of values
struct MultiMap<K, V> {
    inner: HashMap<K, Vec<V>>,
}

impl<K: std::hash::Hash + Eq, V> MultiMap<K, V> {
    fn new() -> Self {
        MultiMap { inner: HashMap::new() }
    }

    fn insert(&mut self, key: K, value: V) {
        self.inner.entry(key).or_default().push(value);
    }

    fn get(&self, key: &K) -> &[V] {
        self.inner.get(key).map_or(&[], |v| v.as_slice())
    }

    fn remove_key(&mut self, key: &K) -> Option<Vec<V>> {
        self.inner.remove(key)
    }

    fn count_values(&self, key: &K) -> usize {
        self.get(key).len()
    }

    fn total_values(&self) -> usize {
        self.inner.values().map(|v| v.len()).sum()
    }

    fn keys(&self) -> impl Iterator<Item = &K> {
        self.inner.keys()
    }

    fn contains_key(&self, key: &K) -> bool {
        self.inner.contains_key(key)
    }
}

impl<K: std::hash::Hash + Eq, V: PartialEq> MultiMap<K, V> {
    fn remove_value(&mut self, key: &K, value: &V) -> bool {
        if let Some(values) = self.inner.get_mut(key) {
            if let Some(pos) = values.iter().position(|v| v == value) {
                values.remove(pos);
                if values.is_empty() {
                    self.inner.remove(key);
                }
                return true;
            }
        }
        false
    }

    fn contains_value(&self, key: &K, value: &V) -> bool {
        self.get(key).contains(value)
    }
}

fn basic_ops() {
    let mut mm = MultiMap::new();
    mm.insert("fruits", "apple");
    mm.insert("fruits", "banana");
    mm.insert("fruits", "cherry");
    mm.insert("vegs", "carrot");
    mm.insert("vegs", "pea");

    assert_eq!(mm.get(&"fruits"), &["apple", "banana", "cherry"]);
    assert_eq!(mm.get(&"vegs"), &["carrot", "pea"]);
    assert_eq!(mm.count_values(&"fruits"), 3);
    assert_eq!(mm.total_values(), 5);
}

fn remove_ops() {
    let mut mm = MultiMap::new();
    mm.insert("tags", "rust");
    mm.insert("tags", "ocaml");
    mm.insert("tags", "haskell");

    assert!(mm.remove_value(&"tags", &"ocaml"));
    assert_eq!(mm.get(&"tags"), &["rust", "haskell"]);

    mm.remove_key(&"tags");
    assert_eq!(mm.get(&"tags"), &[] as &[&str]);
}

fn build_index() {
    let data = vec![
        ("lang", "Rust"), ("lang", "OCaml"), ("lang", "Haskell"),
        ("paradigm", "functional"), ("paradigm", "imperative"),
    ];

    let mut mm = MultiMap::new();
    for (key, value) in data {
        mm.insert(key, value);
    }

    assert_eq!(mm.get(&"lang"), &["Rust", "OCaml", "Haskell"]);
    assert_eq!(mm.get(&"paradigm"), &["functional", "imperative"]);
    assert!(mm.contains_value(&"lang", &"Rust"));
    assert!(!mm.contains_value(&"lang", &"Python"));
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() { basic_ops(); }

    #[test]
    fn test_remove() { remove_ops(); }

    #[test]
    fn test_index() { build_index(); }

    #[test]
    fn test_empty_get() {
        let mm: MultiMap<&str, i32> = MultiMap::new();
        assert_eq!(mm.get(&"missing"), &[] as &[i32]);
        assert_eq!(mm.count_values(&"missing"), 0);
    }
}
