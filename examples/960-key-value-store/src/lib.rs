// 960: Key-Value Store
// OCaml: Hashtbl (mutable) or association list (functional)
// Rust: HashMap<String, String> (mutable) or BTreeMap for sorted iteration

use std::collections::HashMap;

// Approach 1: HashMap-based mutable store
pub struct KvStore {
    data: HashMap<String, String>,
}

impl KvStore {
    pub fn new() -> Self {
        KvStore {
            data: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.data.insert(key.into(), value.into());
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        self.data.get(key).map(|s| s.as_str())
    }

    pub fn delete(&mut self, key: &str) -> bool {
        self.data.remove(key).is_some()
    }

    pub fn contains(&self, key: &str) -> bool {
        self.data.contains_key(key)
    }

    pub fn keys(&self) -> Vec<&str> {
        let mut keys: Vec<&str> = self.data.keys().map(|s| s.as_str()).collect();
        keys.sort();
        keys
    }

    pub fn values(&self) -> Vec<&str> {
        self.data.values().map(|s| s.as_str()).collect()
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

impl Default for KvStore {
    fn default() -> Self {
        Self::new()
    }
}

// Approach 2: Functional (immutable) KV using Vec of pairs
#[derive(Clone, Debug)]
pub struct FunctionalStore<V: Clone> {
    data: Vec<(String, V)>,
}

impl<V: Clone> FunctionalStore<V> {
    pub fn new() -> Self {
        FunctionalStore { data: vec![] }
    }

    pub fn set(&self, key: impl Into<String>, value: V) -> Self {
        let key = key.into();
        let mut data: Vec<(String, V)> = self
            .data
            .iter()
            .filter(|(k, _)| k != &key)
            .cloned()
            .collect();
        data.insert(0, (key, value));
        FunctionalStore { data }
    }

    pub fn get(&self, key: &str) -> Option<&V> {
        self.data.iter().find(|(k, _)| k == key).map(|(_, v)| v)
    }

    pub fn delete(&self, key: &str) -> Self {
        let data = self
            .data
            .iter()
            .filter(|(k, _)| k != key)
            .cloned()
            .collect();
        FunctionalStore { data }
    }

    pub fn keys(&self) -> Vec<&str> {
        self.data.iter().map(|(k, _)| k.as_str()).collect()
    }
}

impl<V: Clone> Default for FunctionalStore<V> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mutable_store() {
        let mut s = KvStore::new();
        assert_eq!(s.size(), 0);
        assert!(s.is_empty());

        s.set("name", "Alice");
        s.set("age", "30");
        s.set("city", "Amsterdam");

        assert_eq!(s.get("name"), Some("Alice"));
        assert_eq!(s.get("age"), Some("30"));
        assert_eq!(s.get("missing"), None);
        assert!(s.contains("city"));
        assert!(!s.contains("missing"));
        assert_eq!(s.size(), 3);
    }

    #[test]
    fn test_update() {
        let mut s = KvStore::new();
        s.set("name", "Alice");
        s.set("name", "Bob");
        assert_eq!(s.get("name"), Some("Bob"));
        assert_eq!(s.size(), 1);
    }

    #[test]
    fn test_delete() {
        let mut s = KvStore::new();
        s.set("name", "Alice");
        s.set("age", "30");
        let removed = s.delete("age");
        assert!(removed);
        assert_eq!(s.get("age"), None);
        assert_eq!(s.size(), 1);
        assert!(!s.delete("missing"));
    }

    #[test]
    fn test_keys_sorted() {
        let mut s = KvStore::new();
        s.set("city", "Amsterdam");
        s.set("name", "Alice");
        assert_eq!(s.keys(), vec!["city", "name"]);
    }

    #[test]
    fn test_functional_store() {
        let fs: FunctionalStore<i32> = FunctionalStore::new();
        let fs = fs.set("x", 1);
        let fs = fs.set("y", 2);
        let fs = fs.set("x", 10); // update
        assert_eq!(fs.get("x"), Some(&10));
        assert_eq!(fs.get("y"), Some(&2));
        assert_eq!(fs.get("z"), None);

        let fs2 = fs.delete("y");
        assert_eq!(fs2.get("y"), None);
        assert_eq!(fs.get("y"), Some(&2)); // original unchanged
    }
}
