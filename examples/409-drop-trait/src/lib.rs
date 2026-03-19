#![allow(clippy::all)]
//! Drop Trait and RAII
//!
//! Automatic resource cleanup when values go out of scope.

use std::cell::Cell;

/// A simulated file handle demonstrating Drop.
#[derive(Debug)]
pub struct FileHandle {
    name: String,
    is_open: Cell<bool>,
}

impl FileHandle {
    /// Opens a file handle.
    pub fn open(name: &str) -> Self {
        FileHandle {
            name: name.to_string(),
            is_open: Cell::new(true),
        }
    }

    /// Returns the file name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns whether the file is open.
    pub fn is_open(&self) -> bool {
        self.is_open.get()
    }

    /// Simulates reading from the file.
    pub fn read(&self) -> Option<String> {
        if self.is_open.get() {
            Some(format!("Contents of {}", self.name))
        } else {
            None
        }
    }
}

impl Drop for FileHandle {
    fn drop(&mut self) {
        if self.is_open.get() {
            self.is_open.set(false);
            // In real code: close file descriptor, flush buffers, etc.
        }
    }
}

/// A lock guard demonstrating RAII pattern.
pub struct LockGuard<'a> {
    resource_name: &'a str,
    lock_id: u32,
    released: Cell<bool>,
}

impl<'a> LockGuard<'a> {
    /// Acquires a lock on a resource.
    pub fn acquire(resource: &'a str) -> Self {
        LockGuard {
            resource_name: resource,
            lock_id: 42, // Simulated
            released: Cell::new(false),
        }
    }

    /// Returns the resource name.
    pub fn resource(&self) -> &str {
        self.resource_name
    }

    /// Returns whether the lock is still held.
    pub fn is_held(&self) -> bool {
        !self.released.get()
    }
}

impl Drop for LockGuard<'_> {
    fn drop(&mut self) {
        if !self.released.get() {
            self.released.set(true);
            // In real code: release mutex, semaphore, etc.
        }
    }
}

/// A transaction guard that commits on success or rolls back on drop.
pub struct Transaction {
    name: String,
    committed: Cell<bool>,
}

impl Transaction {
    /// Begins a new transaction.
    pub fn begin(name: &str) -> Self {
        Transaction {
            name: name.to_string(),
            committed: Cell::new(false),
        }
    }

    /// Commits the transaction.
    pub fn commit(self) {
        self.committed.set(true);
        // Don't call drop's rollback
    }

    /// Returns the transaction name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Checks if committed.
    pub fn is_committed(&self) -> bool {
        self.committed.get()
    }
}

impl Drop for Transaction {
    fn drop(&mut self) {
        if !self.committed.get() {
            // Rollback
        }
    }
}

/// Counter that tracks active instances.
pub struct TrackedResource {
    id: u32,
    counter: *mut u32,
}

impl TrackedResource {
    /// Creates a new tracked resource with external counter.
    ///
    /// # Safety
    /// The counter pointer must remain valid for the lifetime of the resource.
    pub fn new(id: u32, counter: &mut u32) -> Self {
        *counter += 1;
        TrackedResource {
            id,
            counter: counter as *mut u32,
        }
    }

    pub fn id(&self) -> u32 {
        self.id
    }
}

impl Drop for TrackedResource {
    fn drop(&mut self) {
        unsafe {
            *self.counter -= 1;
        }
    }
}

/// Demonstrates drop order (reverse of creation).
pub fn demonstrate_drop_order() -> Vec<String> {
    struct OrderTracker {
        name: String,
        log: *mut Vec<String>,
    }

    impl Drop for OrderTracker {
        fn drop(&mut self) {
            unsafe {
                (*self.log).push(format!("Dropped: {}", self.name));
            }
        }
    }

    let mut log = Vec::new();
    {
        let _a = OrderTracker {
            name: "A".to_string(),
            log: &mut log,
        };
        let _b = OrderTracker {
            name: "B".to_string(),
            log: &mut log,
        };
        let _c = OrderTracker {
            name: "C".to_string(),
            log: &mut log,
        };
        // Drops in reverse order: C, B, A
    }
    log
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_handle_open_close() {
        let handle = FileHandle::open("test.txt");
        assert!(handle.is_open());
        assert_eq!(handle.name(), "test.txt");
        assert!(handle.read().is_some());
    }

    #[test]
    fn test_file_handle_drop() {
        let handle = FileHandle::open("test.txt");
        assert!(handle.is_open());
        drop(handle);
        // handle is no longer accessible
    }

    #[test]
    fn test_lock_guard_raii() {
        let guard = LockGuard::acquire("database");
        assert!(guard.is_held());
        assert_eq!(guard.resource(), "database");
        drop(guard);
        // Lock automatically released
    }

    #[test]
    fn test_transaction_commit() {
        let tx = Transaction::begin("update_user");
        assert!(!tx.is_committed());
        tx.commit();
        // Committed, no rollback in drop
    }

    #[test]
    fn test_transaction_rollback_on_drop() {
        let tx = Transaction::begin("update_user");
        assert!(!tx.is_committed());
        drop(tx);
        // Rollback happened in drop
    }

    #[test]
    fn test_tracked_resource_counter() {
        let mut counter = 0u32;
        {
            let _r1 = TrackedResource::new(1, &mut counter);
            assert_eq!(counter, 1);
            {
                let _r2 = TrackedResource::new(2, &mut counter);
                assert_eq!(counter, 2);
            }
            assert_eq!(counter, 1); // r2 dropped
        }
        assert_eq!(counter, 0); // r1 dropped
    }

    #[test]
    fn test_drop_order() {
        let log = demonstrate_drop_order();
        assert_eq!(log, vec!["Dropped: C", "Dropped: B", "Dropped: A"]);
    }

    #[test]
    fn test_explicit_drop() {
        let handle = FileHandle::open("explicit.txt");
        let name = handle.name().to_string();
        std::mem::drop(handle);
        assert_eq!(name, "explicit.txt");
        // Cannot use handle after drop
    }
}
