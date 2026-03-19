#![allow(clippy::all)]
//! # Test Fixtures
//!
//! RAII teardown, shared state, and per-test isolation patterns.

use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

/// A simple key-value database for testing
#[derive(Debug)]
pub struct Database {
    store: HashMap<String, String>,
}

impl Database {
    /// Create a new empty database
    pub fn new() -> Self {
        Database {
            store: HashMap::new(),
        }
    }

    /// Insert a key-value pair
    pub fn insert(&mut self, key: &str, value: &str) {
        self.store.insert(key.to_owned(), value.to_owned());
    }

    /// Get a value by key
    pub fn get(&self, key: &str) -> Option<&str> {
        self.store.get(key).map(String::as_str)
    }

    /// Delete a key, returns true if it existed
    pub fn delete(&mut self, key: &str) -> bool {
        self.store.remove(key).is_some()
    }

    /// Count of entries
    pub fn count(&self) -> usize {
        self.store.len()
    }
}

impl Default for Database {
    fn default() -> Self {
        Self::new()
    }
}

/// A fixture builder for testing
pub struct DatabaseBuilder {
    db: Database,
}

impl DatabaseBuilder {
    /// Create a new builder
    pub fn new() -> Self {
        DatabaseBuilder {
            db: Database::new(),
        }
    }

    /// Add a user entry
    pub fn with_user(mut self, id: u32, name: &str) -> Self {
        self.db.insert(&format!("user:{}", id), name);
        self
    }

    /// Add arbitrary key-value
    pub fn with_entry(mut self, key: &str, value: &str) -> Self {
        self.db.insert(key, value);
        self
    }

    /// Build the database
    pub fn build(self) -> Database {
        self.db
    }
}

impl Default for DatabaseBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // RAII fixture: auto-teardown via Drop
    struct DatabaseFixture {
        pub db: Database,
        #[allow(dead_code)]
        name: &'static str,
    }

    impl DatabaseFixture {
        fn new(name: &'static str) -> Self {
            let db = DatabaseBuilder::new()
                .with_user(1, "Alice")
                .with_user(2, "Bob")
                .with_user(3, "Carol")
                .build();
            DatabaseFixture { db, name }
        }
    }

    impl Drop for DatabaseFixture {
        fn drop(&mut self) {
            // Teardown runs even if test panics
        }
    }

    #[test]
    fn test_lookup_existing_user() {
        let f = DatabaseFixture::new("lookup_existing");
        assert_eq!(f.db.get("user:1"), Some("Alice"));
    }

    #[test]
    fn test_lookup_missing_returns_none() {
        let f = DatabaseFixture::new("lookup_missing");
        assert_eq!(f.db.get("user:99"), None);
    }

    #[test]
    fn test_insert_and_retrieve() {
        let mut f = DatabaseFixture::new("insert_retrieve");
        f.db.insert("user:4", "Dave");
        assert_eq!(f.db.get("user:4"), Some("Dave"));
    }

    #[test]
    fn test_delete_reduces_count() {
        let mut f = DatabaseFixture::new("delete");
        let before = f.db.count();
        assert!(f.db.delete("user:1"));
        assert_eq!(f.db.count(), before - 1);
    }

    #[test]
    fn test_delete_nonexistent_returns_false() {
        let mut f = DatabaseFixture::new("delete_nonexistent");
        assert!(!f.db.delete("ghost:999"));
    }

    // Shared read-only fixture via OnceLock
    static SHARED_DATA: OnceLock<Vec<i32>> = OnceLock::new();

    fn shared_data() -> &'static [i32] {
        SHARED_DATA.get_or_init(|| (1..=100).collect())
    }

    #[test]
    fn test_shared_data_sum() {
        let data = shared_data();
        let sum: i32 = data.iter().sum();
        assert_eq!(sum, 5050);
    }

    #[test]
    fn test_shared_data_length() {
        assert_eq!(shared_data().len(), 100);
    }

    // Mutex for shared mutable state (prefer isolation over this)
    static COUNTER: OnceLock<Mutex<u32>> = OnceLock::new();

    fn get_counter() -> &'static Mutex<u32> {
        COUNTER.get_or_init(|| Mutex::new(0))
    }

    #[test]
    fn test_counter_increment() {
        let mut guard = get_counter().lock().unwrap();
        let before = *guard;
        *guard += 1;
        assert_eq!(*guard, before + 1);
    }

    #[test]
    fn test_builder_pattern() {
        let db = DatabaseBuilder::new()
            .with_user(1, "Test")
            .with_entry("config:timeout", "30")
            .build();
        assert_eq!(db.get("user:1"), Some("Test"));
        assert_eq!(db.get("config:timeout"), Some("30"));
    }
}
