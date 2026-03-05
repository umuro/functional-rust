//! # Mutex Poisoning — Handling Panics in Critical Sections
//!
//! When a thread panics while holding a mutex, the mutex becomes "poisoned".

use std::sync::{Arc, Mutex, PoisonError};
use std::thread;

/// A counter that demonstrates poisoning
pub struct Counter {
    value: Mutex<i32>,
}

impl Counter {
    pub fn new(initial: i32) -> Self {
        Self {
            value: Mutex::new(initial),
        }
    }

    /// Increment, propagating poison
    pub fn increment(&self) -> Result<i32, PoisonError<std::sync::MutexGuard<'_, i32>>> {
        let mut guard = self.value.lock()?;
        *guard += 1;
        Ok(*guard)
    }

    /// Increment, recovering from poison
    pub fn increment_recover(&self) -> i32 {
        let mut guard = self.value.lock().unwrap_or_else(|e| {
            eprintln!("Mutex was poisoned, recovering...");
            e.into_inner()
        });
        *guard += 1;
        *guard
    }

    /// Get value, propagating poison
    pub fn get(&self) -> Result<i32, PoisonError<std::sync::MutexGuard<'_, i32>>> {
        let guard = self.value.lock()?;
        Ok(*guard)
    }

    /// Get value, recovering from poison
    pub fn get_recover(&self) -> i32 {
        let guard = self.value.lock().unwrap_or_else(|e| e.into_inner());
        *guard
    }

    /// Check if poisoned
    pub fn is_poisoned(&self) -> bool {
        self.value.is_poisoned()
    }

    /// Clear poison by replacing value
    pub fn clear_poison(&self, new_value: i32) {
        self.value.clear_poison();
        *self.value.lock().unwrap() = new_value;
    }
}

/// Demonstrate poisoning
pub fn demonstrate_poisoning() -> bool {
    let counter = Arc::new(Counter::new(0));
    let counter_clone = Arc::clone(&counter);

    // Thread that will panic while holding the lock
    let handle = thread::spawn(move || {
        let _guard = counter_clone.value.lock().unwrap();
        panic!("Intentional panic while holding mutex");
    });

    // Wait for thread to panic
    let _ = handle.join();

    counter.is_poisoned()
}

/// Demonstrate recovery
pub fn demonstrate_recovery() -> i32 {
    let counter = Arc::new(Counter::new(10));
    let counter_clone = Arc::clone(&counter);

    let handle = thread::spawn(move || {
        let mut guard = counter_clone.value.lock().unwrap();
        *guard = 42; // Set value before panicking
        panic!("Panic!");
    });

    let _ = handle.join();

    // Recover the value
    counter.get_recover()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_operation() {
        let counter = Counter::new(0);

        counter.increment().unwrap();
        counter.increment().unwrap();

        assert_eq!(counter.get().unwrap(), 2);
        assert!(!counter.is_poisoned());
    }

    #[test]
    fn test_poisoning_detection() {
        assert!(demonstrate_poisoning());
    }

    #[test]
    fn test_recovery() {
        let value = demonstrate_recovery();
        assert_eq!(value, 42); // Recovered the value set before panic
    }

    #[test]
    fn test_increment_recover() {
        let counter = Arc::new(Counter::new(0));
        let counter_clone = Arc::clone(&counter);

        let _ = thread::spawn(move || {
            let _guard = counter_clone.value.lock().unwrap();
            panic!("Panic!");
        })
        .join();

        // Should still work despite poisoning
        let value = counter.increment_recover();
        assert_eq!(value, 1);
    }

    #[test]
    fn test_clear_poison() {
        let counter = Arc::new(Counter::new(0));
        let counter_clone = Arc::clone(&counter);

        let _ = thread::spawn(move || {
            let _guard = counter_clone.value.lock().unwrap();
            panic!("Panic!");
        })
        .join();

        assert!(counter.is_poisoned());

        counter.clear_poison(100);

        assert!(!counter.is_poisoned());
        assert_eq!(counter.get().unwrap(), 100);
    }

    #[test]
    fn test_unwrap_or_else_pattern() {
        let mutex = Mutex::new(42);

        // Simulate poison
        let result = std::panic::catch_unwind(|| {
            let _guard = mutex.lock().unwrap();
            panic!("test");
        });
        assert!(result.is_err());

        // Recover
        let guard = mutex.lock().unwrap_or_else(|poisoned| {
            println!("Recovered from poison");
            poisoned.into_inner()
        });

        assert_eq!(*guard, 42);
    }
}
