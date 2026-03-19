#![allow(clippy::all)]
//! # Async Trait Pattern
//!
//! Async methods in traits require boxing — `async fn` in traits isn't directly
//! supported in stable Rust without the `async-trait` crate.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Mutex;
use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};

/// Type alias for boxed async results.
pub type AsyncResult<T, E> = Pin<Box<dyn Future<Output = Result<T, E>> + Send>>;

/// Async storage trait with boxed futures for object safety.
pub trait AsyncStore: Send + Sync {
    fn get(&self, key: &str) -> AsyncResult<Option<String>, String>;
    fn set(&self, key: String, value: String) -> AsyncResult<(), String>;
    fn delete(&self, key: &str) -> AsyncResult<bool, String>;
}

/// In-memory implementation of AsyncStore.
pub struct MemStore {
    data: Mutex<HashMap<String, String>>,
}

impl MemStore {
    pub fn new() -> Self {
        Self {
            data: Mutex::new(HashMap::new()),
        }
    }
}

impl Default for MemStore {
    fn default() -> Self {
        Self::new()
    }
}

impl AsyncStore for MemStore {
    fn get(&self, key: &str) -> AsyncResult<Option<String>, String> {
        let result = self.data.lock().unwrap().get(key).cloned();
        Box::pin(async move { Ok(result) })
    }

    fn set(&self, key: String, value: String) -> AsyncResult<(), String> {
        self.data.lock().unwrap().insert(key, value);
        Box::pin(async { Ok(()) })
    }

    fn delete(&self, key: &str) -> AsyncResult<bool, String> {
        let removed = self.data.lock().unwrap().remove(key).is_some();
        Box::pin(async move { Ok(removed) })
    }
}

/// A failing store for testing error handling.
pub struct FailStore;

impl AsyncStore for FailStore {
    fn get(&self, _key: &str) -> AsyncResult<Option<String>, String> {
        Box::pin(async { Err("connection refused".to_string()) })
    }

    fn set(&self, _key: String, _value: String) -> AsyncResult<(), String> {
        Box::pin(async { Err("read-only store".to_string()) })
    }

    fn delete(&self, _key: &str) -> AsyncResult<bool, String> {
        Box::pin(async { Err("operation not permitted".to_string()) })
    }
}

/// A minimal executor for testing.
pub fn block_on<F: Future>(fut: F) -> F::Output {
    unsafe fn clone_waker(ptr: *const ()) -> RawWaker {
        RawWaker::new(ptr, &VTABLE)
    }
    unsafe fn noop(_: *const ()) {}

    static VTABLE: RawWakerVTable = RawWakerVTable::new(clone_waker, noop, noop, noop);

    let waker = unsafe { Waker::from_raw(RawWaker::new(std::ptr::null(), &VTABLE)) };
    let mut cx = Context::from_waker(&waker);
    let mut fut = Box::pin(fut);

    loop {
        if let Poll::Ready(value) = fut.as_mut().poll(&mut cx) {
            return value;
        }
    }
}

/// Demonstrates using the store through the trait interface.
pub fn use_store(store: &dyn AsyncStore, key: &str, value: &str) -> Result<Option<String>, String> {
    block_on(store.set(key.to_string(), value.to_string()))?;
    block_on(store.get(key))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mem_store_set_get() {
        let store = MemStore::new();
        block_on(store.set("key".to_string(), "value".to_string())).unwrap();
        assert_eq!(
            block_on(store.get("key")).unwrap(),
            Some("value".to_string())
        );
    }

    #[test]
    fn test_mem_store_missing_key() {
        let store = MemStore::new();
        assert_eq!(block_on(store.get("missing")).unwrap(), None);
    }

    #[test]
    fn test_mem_store_delete() {
        let store = MemStore::new();
        block_on(store.set("k".to_string(), "v".to_string())).unwrap();
        assert!(block_on(store.delete("k")).unwrap());
        assert!(!block_on(store.delete("k")).unwrap());
    }

    #[test]
    fn test_fail_store_returns_errors() {
        let store = FailStore;
        assert!(block_on(store.get("any")).is_err());
        assert!(block_on(store.set("k".into(), "v".into())).is_err());
    }

    #[test]
    fn test_trait_object_dispatch() {
        let stores: Vec<Box<dyn AsyncStore>> = vec![Box::new(MemStore::new()), Box::new(FailStore)];

        // First store works
        assert!(block_on(stores[0].set("k".into(), "v".into())).is_ok());

        // Second store fails
        assert!(block_on(stores[1].get("k")).is_err());
    }
}
