//! Cell and RefCell for Interior Mutability
//!
//! Mutating through shared references.

use std::cell::{Cell, RefCell};

/// Cell for Copy types.
pub struct Counter {
    value: Cell<i32>,
}

impl Counter {
    pub fn new(value: i32) -> Self {
        Counter {
            value: Cell::new(value),
        }
    }

    pub fn get(&self) -> i32 {
        self.value.get()
    }

    pub fn set(&self, value: i32) {
        self.value.set(value);
    }

    pub fn increment(&self) {
        self.value.set(self.value.get() + 1);
    }
}

/// RefCell for non-Copy types.
pub struct Cache {
    data: RefCell<Vec<String>>,
}

impl Cache {
    pub fn new() -> Self {
        Cache {
            data: RefCell::new(Vec::new()),
        }
    }

    pub fn add(&self, item: String) {
        self.data.borrow_mut().push(item);
    }

    pub fn get_all(&self) -> Vec<String> {
        self.data.borrow().clone()
    }

    pub fn len(&self) -> usize {
        self.data.borrow().len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.borrow().is_empty()
    }
}

impl Default for Cache {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_counter() {
        let counter = Counter::new(0);
        counter.increment();
        counter.increment();
        assert_eq!(counter.get(), 2);
    }

    #[test]
    fn test_cache() {
        let cache = Cache::new();
        cache.add("a".into());
        cache.add("b".into());
        assert_eq!(cache.len(), 2);
    }
}
