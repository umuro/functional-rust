#![allow(clippy::all)]
//! 702 — Unsafe Traits
//! unsafe trait, unsafe impl, and Send/Sync marker traits.

use std::sync::{Arc, Mutex};
use std::thread;

// ── Custom unsafe trait ───────────────────────────────────────────────────

/// Marker trait: implementors guarantee this type can cross thread boundaries.
///
/// # Safety
/// The type must not contain non-thread-safe interior mutability or raw
/// pointers that could alias across threads.
pub unsafe trait ThreadSafe: Send + Sync {
    fn describe(&self) -> String;
}

// ── A type safe to implement ThreadSafe on ────────────────────────────────

pub struct AtomicCounter {
    value: std::sync::atomic::AtomicI64,
}

impl AtomicCounter {
    pub fn new(v: i64) -> Self {
        Self {
            value: std::sync::atomic::AtomicI64::new(v),
        }
    }
    pub fn get(&self) -> i64 {
        self.value.load(std::sync::atomic::Ordering::SeqCst)
    }
    pub fn increment(&self) {
        self.value.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    }
}

// SAFETY: AtomicCounter uses AtomicI64 — all operations are thread-safe.
// No raw pointers, no non-Sync interior mutability.
unsafe impl ThreadSafe for AtomicCounter {
    fn describe(&self) -> String {
        format!("AtomicCounter({})", self.get())
    }
}

// ── Type that is NOT Send (contains *mut T) ───────────────────────────────

pub struct NotSend {
    _ptr: *mut i32, // *mut T is !Send by default
}

// fn use_not_send_in_thread(x: NotSend) {
//     thread::spawn(move || drop(x));  // compile error: *mut i32 is not Send
// }

// ── Demonstrate Send/Sync in practice ────────────────────────────────────

fn run_in_thread<T: ThreadSafe + 'static>(val: Arc<T>) {
    let v = Arc::clone(&val);
    thread::spawn(move || println!("Thread: {}", v.describe()))
        .join()
        .unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_atomic_counter_send_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<AtomicCounter>(); // won't compile if wrong
    }

    #[test]
    fn test_threadsafe_describe() {
        let c = AtomicCounter::new(42);
        assert_eq!(c.describe(), "AtomicCounter(42)");
    }

    #[test]
    fn test_concurrent_increment() {
        let c = Arc::new(AtomicCounter::new(0));
        let handles: Vec<_> = (0..10)
            .map(|_| {
                let cc = Arc::clone(&c);
                thread::spawn(move || cc.increment())
            })
            .collect();
        for h in handles {
            h.join().unwrap();
        }
        assert_eq!(c.get(), 10);
    }
}
