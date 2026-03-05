//! # Async RwLock
//!
//! Multiple concurrent readers, one exclusive writer — the right lock for read-heavy shared state.

use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::thread;

/// A shared database with read-write lock semantics.
pub struct SharedDb {
    data: RwLock<HashMap<String, i32>>,
}

impl SharedDb {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            data: RwLock::new(HashMap::new()),
        })
    }

    /// Read a value (multiple readers can run simultaneously).
    pub fn read(&self, key: &str) -> Option<i32> {
        self.data.read().unwrap().get(key).copied()
    }

    /// Write a value (exclusive access).
    pub fn write(&self, key: &str, value: i32) {
        self.data.write().unwrap().insert(key.to_string(), value);
    }

    /// Update a value with a function.
    pub fn update(&self, key: &str, f: impl Fn(i32) -> i32) {
        if let Some(v) = self.data.write().unwrap().get_mut(key) {
            *v = f(*v);
        }
    }

    /// Get all keys.
    pub fn keys(&self) -> Vec<String> {
        self.data.read().unwrap().keys().cloned().collect()
    }

    /// Get the number of entries.
    pub fn len(&self) -> usize {
        self.data.read().unwrap().len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl Default for SharedDb {
    fn default() -> Self {
        Self {
            data: RwLock::new(HashMap::new()),
        }
    }
}

/// Demonstrates concurrent reads don't block each other.
pub fn concurrent_reads(db: &Arc<SharedDb>) -> Vec<Option<i32>> {
    let handles: Vec<_> = (0..5)
        .map(|_| {
            let db = Arc::clone(db);
            thread::spawn(move || db.read("x"))
        })
        .collect();

    handles.into_iter().map(|h| h.join().unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_write() {
        let db = SharedDb::new();
        db.write("key", 99);
        assert_eq!(db.read("key"), Some(99));
    }

    #[test]
    fn test_missing_key_returns_none() {
        let db = SharedDb::new();
        assert_eq!(db.read("nonexistent"), None);
    }

    #[test]
    fn test_update() {
        let db = SharedDb::new();
        db.write("x", 10);
        db.update("x", |v| v * 2);
        assert_eq!(db.read("x"), Some(20));
    }

    #[test]
    fn test_concurrent_reads_all_succeed() {
        let db = SharedDb::new();
        db.write("k", 7);

        let handles: Vec<_> = (0..10)
            .map(|_| {
                let db = Arc::clone(&db);
                thread::spawn(move || db.read("k"))
            })
            .collect();

        assert!(handles.into_iter().all(|h| h.join().unwrap() == Some(7)));
    }

    #[test]
    fn test_keys_and_len() {
        let db = SharedDb::new();
        db.write("a", 1);
        db.write("b", 2);
        assert_eq!(db.len(), 2);
        let mut keys = db.keys();
        keys.sort();
        assert_eq!(keys, vec!["a", "b"]);
    }
}
