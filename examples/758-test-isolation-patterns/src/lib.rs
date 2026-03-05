//! # Test Isolation Patterns
//!
//! Ensuring tests don't interfere with each other.

use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::{Arc, Mutex, OnceLock};

/// A global service (anti-pattern without isolation)
static GLOBAL_COUNTER: OnceLock<Mutex<u64>> = OnceLock::new();

fn global_counter() -> &'static Mutex<u64> {
    GLOBAL_COUNTER.get_or_init(|| Mutex::new(0))
}

/// Increment the global counter (test pollution risk!)
pub fn increment_global() -> u64 {
    let mut guard = global_counter().lock().unwrap();
    *guard += 1;
    *guard
}

// ═══════════════════════════════════════════════════════════════════════════════
// BETTER: Dependency Injection for Isolation
// ═══════════════════════════════════════════════════════════════════════════════

/// A counter service that can be injected
pub trait Counter {
    fn increment(&self) -> u64;
    fn get(&self) -> u64;
    fn reset(&self);
}

/// Thread-safe counter implementation
pub struct AtomicCounter {
    value: Mutex<u64>,
}

impl AtomicCounter {
    pub fn new() -> Self {
        AtomicCounter {
            value: Mutex::new(0),
        }
    }
}

impl Default for AtomicCounter {
    fn default() -> Self {
        Self::new()
    }
}

impl Counter for AtomicCounter {
    fn increment(&self) -> u64 {
        let mut guard = self.value.lock().unwrap();
        *guard += 1;
        *guard
    }

    fn get(&self) -> u64 {
        *self.value.lock().unwrap()
    }

    fn reset(&self) {
        *self.value.lock().unwrap() = 0;
    }
}

/// Service that uses an injected counter
pub struct Service<C: Counter> {
    counter: C,
    name: String,
}

impl<C: Counter> Service<C> {
    pub fn new(name: &str, counter: C) -> Self {
        Service {
            counter,
            name: name.to_string(),
        }
    }

    pub fn process(&self) -> String {
        let count = self.counter.increment();
        format!("[{}] Processed item #{}", self.name, count)
    }

    pub fn status(&self) -> String {
        format!("[{}] Count: {}", self.name, self.counter.get())
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Test-specific implementations
// ═══════════════════════════════════════════════════════════════════════════════

/// A per-test isolated counter
pub struct IsolatedCounter {
    value: RefCell<u64>,
}

impl IsolatedCounter {
    pub fn new() -> Self {
        IsolatedCounter {
            value: RefCell::new(0),
        }
    }
}

impl Default for IsolatedCounter {
    fn default() -> Self {
        Self::new()
    }
}

impl Counter for IsolatedCounter {
    fn increment(&self) -> u64 {
        let mut v = self.value.borrow_mut();
        *v += 1;
        *v
    }

    fn get(&self) -> u64 {
        *self.value.borrow()
    }

    fn reset(&self) {
        *self.value.borrow_mut() = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_isolated_counter_1() {
        // Each test gets its own counter - no pollution
        let counter = IsolatedCounter::new();
        let service = Service::new("test1", counter);
        assert_eq!(service.process(), "[test1] Processed item #1");
        assert_eq!(service.process(), "[test1] Processed item #2");
    }

    #[test]
    fn test_isolated_counter_2() {
        // This test is independent of test_isolated_counter_1
        let counter = IsolatedCounter::new();
        let service = Service::new("test2", counter);
        assert_eq!(service.process(), "[test2] Processed item #1");
    }

    #[test]
    fn test_atomic_counter() {
        let counter = AtomicCounter::new();
        assert_eq!(counter.increment(), 1);
        assert_eq!(counter.increment(), 2);
        assert_eq!(counter.get(), 2);
        counter.reset();
        assert_eq!(counter.get(), 0);
    }

    #[test]
    fn test_service_status() {
        let counter = IsolatedCounter::new();
        let service = Service::new("status", counter);
        service.process();
        service.process();
        assert_eq!(service.status(), "[status] Count: 2");
    }

    #[test]
    fn test_shared_counter_with_arc() {
        let counter = Arc::new(AtomicCounter::new());
        let c1 = counter.clone();
        let c2 = counter.clone();

        c1.increment();
        c2.increment();

        assert_eq!(counter.get(), 2);
    }
}
