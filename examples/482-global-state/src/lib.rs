//! # Global State — Static Variables
//!
//! Managing global state safely in Rust.

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Mutex, OnceLock, RwLock};

// Atomic global counter
static COUNTER: AtomicUsize = AtomicUsize::new(0);

pub fn increment_global() -> usize {
    COUNTER.fetch_add(1, Ordering::SeqCst)
}

pub fn get_global_count() -> usize {
    COUNTER.load(Ordering::SeqCst)
}

// Lazy initialized global
static CONFIG: OnceLock<String> = OnceLock::new();

pub fn get_config() -> &'static str {
    CONFIG.get_or_init(|| String::from("default_config"))
}

pub fn set_config(value: String) -> bool {
    CONFIG.set(value).is_ok()
}

// Mutex-protected global
static GLOBAL_LIST: Mutex<Vec<i32>> = Mutex::new(Vec::new());

pub fn push_global(value: i32) {
    GLOBAL_LIST.lock().unwrap().push(value);
}

pub fn get_global_list() -> Vec<i32> {
    GLOBAL_LIST.lock().unwrap().clone()
}

// RwLock for read-heavy globals
static CACHE: RwLock<Vec<String>> = RwLock::new(Vec::new());

pub fn add_to_cache(value: String) {
    CACHE.write().unwrap().push(value);
}

pub fn read_cache() -> Vec<String> {
    CACHE.read().unwrap().clone()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_atomic_counter() {
        let start = get_global_count();
        increment_global();
        assert_eq!(get_global_count(), start + 1);
    }

    #[test]
    fn test_once_lock() {
        let config = get_config();
        assert!(!config.is_empty());
    }

    #[test]
    fn test_global_list() {
        push_global(42);
        let list = get_global_list();
        assert!(list.contains(&42));
    }
}
