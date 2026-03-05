//! # Send and Sync Bounds — Thread Safety Markers
//!
//! Understanding Send and Sync traits for thread safety.
//!
//! - `Send`: Type can be transferred to another thread
//! - `Sync`: Type can be shared between threads via &T

use std::cell::{Cell, RefCell};
use std::marker::PhantomData;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

/// A type that is both Send and Sync
/// Most types composed of Send+Sync are automatically Send+Sync
pub struct ThreadSafe {
    data: i32,
}

// ThreadSafe is automatically Send + Sync because i32 is

/// A type that is Send but NOT Sync
/// Example: Mutex<T> - can be moved, but &Mutex can't be accessed without lock
pub struct SendNotSync {
    /// Cell is not Sync (interior mutability without synchronization)
    data: Cell<i32>,
}

// Cell<i32> is Send but not Sync
// Wrapping it doesn't change that

/// A type that is neither Send nor Sync
/// Example: Rc<T> - reference count is not atomic
pub struct NotSendNotSync {
    data: Rc<i32>,
}

// Rc is neither Send nor Sync

/// A type that is Sync but NOT Send (rare)
/// Example: MutexGuard - borrowed from the Mutex, can't be moved
pub struct SyncNotSend {
    // Use PhantomData to make it !Send
    _marker: PhantomData<*const ()>,
    data: i32,
}

// Raw pointers are neither Send nor Sync by default

unsafe impl Sync for SyncNotSend {}

/// Demonstrate Send with Arc
pub fn demonstrate_send() {
    let data = Arc::new(42);

    // Arc<T> is Send if T is Send
    let handle = std::thread::spawn({
        let data = Arc::clone(&data);
        move || *data
    });

    assert_eq!(handle.join().unwrap(), 42);
}

/// Demonstrate Sync with shared reference
pub fn demonstrate_sync() {
    let data = Arc::new(Mutex::new(0));

    // Mutex<T> is Sync if T is Send
    std::thread::scope(|s| {
        for _ in 0..4 {
            let data = &data;
            s.spawn(move || {
                *data.lock().unwrap() += 1;
            });
        }
    });

    assert_eq!(*data.lock().unwrap(), 4);
}

/// Wrapper that makes a type !Send
pub struct NoSend<T> {
    inner: T,
    _marker: PhantomData<*const ()>,
}

impl<T> NoSend<T> {
    pub fn new(inner: T) -> Self {
        Self {
            inner,
            _marker: PhantomData,
        }
    }

    pub fn get(&self) -> &T {
        &self.inner
    }

    pub fn into_inner(self) -> T {
        self.inner
    }
}

/// Verify Send/Sync implementations at compile time
fn assert_send<T: Send>() {}
fn assert_sync<T: Sync>() {}
fn assert_send_sync<T: Send + Sync>() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_thread_safe_is_send_sync() {
        assert_send::<ThreadSafe>();
        assert_sync::<ThreadSafe>();
        assert_send_sync::<ThreadSafe>();
    }

    #[test]
    fn test_send_not_sync() {
        assert_send::<SendNotSync>();
        // assert_sync::<SendNotSync>(); // Would fail: Cell is not Sync
    }

    #[test]
    fn test_common_types() {
        // i32 is Send + Sync
        assert_send_sync::<i32>();

        // String is Send + Sync
        assert_send_sync::<String>();

        // Vec<T> is Send + Sync if T is
        assert_send_sync::<Vec<i32>>();

        // Arc<T> is Send + Sync if T is Send + Sync
        assert_send_sync::<Arc<i32>>();

        // Mutex<T> is Send + Sync if T is Send
        assert_send_sync::<Mutex<i32>>();

        // RefCell is Send but not Sync
        assert_send::<RefCell<i32>>();
        // assert_sync::<RefCell<i32>>(); // Would fail
    }

    #[test]
    fn test_demonstrate_send() {
        demonstrate_send();
    }

    #[test]
    fn test_demonstrate_sync() {
        demonstrate_sync();
    }

    #[test]
    fn test_no_send_wrapper() {
        let wrapped = NoSend::new(42);
        assert_eq!(*wrapped.get(), 42);

        let inner = wrapped.into_inner();
        assert_eq!(inner, 42);
    }

    // This test verifies the compile-time checks
    #[test]
    fn test_arc_mutex_pattern() {
        let shared = Arc::new(Mutex::new(Vec::<i32>::new()));

        let handles: Vec<_> = (0..4)
            .map(|i| {
                let shared = Arc::clone(&shared);
                std::thread::spawn(move || {
                    shared.lock().unwrap().push(i);
                })
            })
            .collect();

        for h in handles {
            h.join().unwrap();
        }

        let result = shared.lock().unwrap();
        assert_eq!(result.len(), 4);
    }
}
