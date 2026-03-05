/// 747: Test Fixtures — RAII teardown, shared state, per-test isolation

use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

// ── Code under test ────────────────────────────────────────────────────────────

pub struct Database {
    store: HashMap<String, String>,
}

impl Database {
    pub fn new() -> Self {
        Database { store: HashMap::new() }
    }

    pub fn insert(&mut self, key: &str, value: &str) {
        self.store.insert(key.to_owned(), value.to_owned());
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        self.store.get(key).map(String::as_str)
    }

    pub fn delete(&mut self, key: &str) -> bool {
        self.store.remove(key).is_some()
    }

    pub fn count(&self) -> usize {
        self.store.len()
    }
}

fn main() {
    let mut db = Database::new();
    db.insert("key1", "value1");
    println!("Get key1: {:?}", db.get("key1"));
    println!("Count: {}", db.count());
}

// ── Test infrastructure ────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // ── RAII fixture: auto-teardown via Drop ──────────────────────────────────

    struct DatabaseFixture {
        pub db: Database,
        name: &'static str,
    }

    impl DatabaseFixture {
        /// Setup: creates a pre-populated database.
        fn new(name: &'static str) -> Self {
            let mut db = Database::new();
            db.insert("user:1", "Alice");
            db.insert("user:2", "Bob");
            db.insert("user:3", "Carol");
            println!("[fixture:{}] Set up ({} entries)", name, db.count());
            DatabaseFixture { db, name }
        }
    }

    impl Drop for DatabaseFixture {
        /// Teardown: runs even if the test panics!
        fn drop(&mut self) {
            println!("[fixture:{}] Torn down", self.name);
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

    // ── Shared read-only fixture via OnceLock ─────────────────────────────────

    static SHARED_DATA: OnceLock<Vec<i32>> = OnceLock::new();

    fn shared_data() -> &'static [i32] {
        SHARED_DATA.get_or_init(|| {
            println!("[shared] Initializing shared data (runs once)");
            (1..=100).collect()
        })
    }

    #[test]
    fn test_shared_data_sum() {
        let data = shared_data();
        let sum: i32 = data.iter().sum();
        assert_eq!(sum, 5050);  // sum(1..=100)
    }

    #[test]
    fn test_shared_data_length() {
        assert_eq!(shared_data().len(), 100);
    }

    // ── Mutex for shared mutable state across tests ────────────────────────────
    // Note: Usually prefer per-test isolation over shared mutable state.

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
}
