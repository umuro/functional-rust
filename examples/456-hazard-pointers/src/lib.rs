//! # Hazard Pointers — Safe Memory Reclamation
//!
//! A technique for safely reclaiming memory in lock-free data structures.
//! Threads announce which pointers they're using; reclaimers check these.

use std::ptr;
use std::sync::atomic::{AtomicPtr, AtomicUsize, Ordering};
use std::sync::Arc;

/// A simple hazard pointer implementation concept
pub struct HazardPointer<T> {
    /// The pointer being protected
    protected: AtomicPtr<T>,
}

impl<T> HazardPointer<T> {
    pub fn new() -> Self {
        Self {
            protected: AtomicPtr::new(ptr::null_mut()),
        }
    }

    /// Protect a pointer (announce we're using it)
    pub fn protect(&self, ptr: *mut T) {
        self.protected.store(ptr, Ordering::Release);
    }

    /// Clear protection
    pub fn clear(&self) {
        self.protected.store(ptr::null_mut(), Ordering::Release);
    }

    /// Check if a pointer is protected
    pub fn is_protected(&self, ptr: *mut T) -> bool {
        self.protected.load(Ordering::Acquire) == ptr
    }

    /// Get the protected pointer
    pub fn get(&self) -> *mut T {
        self.protected.load(Ordering::Acquire)
    }
}

impl<T> Default for HazardPointer<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// Retired pointer waiting for safe reclamation
struct RetiredPointer<T> {
    ptr: *mut T,
}

/// Simple hazard pointer domain
pub struct HazardDomain<T> {
    hazard_pointers: Vec<Arc<HazardPointer<T>>>,
    retired: std::sync::Mutex<Vec<RetiredPointer<T>>>,
    retire_threshold: usize,
}

impl<T> HazardDomain<T> {
    pub fn new(num_threads: usize) -> Self {
        let hazard_pointers = (0..num_threads)
            .map(|_| Arc::new(HazardPointer::new()))
            .collect();

        Self {
            hazard_pointers,
            retired: std::sync::Mutex::new(Vec::new()),
            retire_threshold: num_threads * 2,
        }
    }

    /// Get a hazard pointer for a thread
    pub fn get_hazard(&self, thread_id: usize) -> Arc<HazardPointer<T>> {
        Arc::clone(&self.hazard_pointers[thread_id % self.hazard_pointers.len()])
    }

    /// Retire a pointer for later reclamation
    pub fn retire(&self, ptr: *mut T) {
        let mut retired = self.retired.lock().unwrap();
        retired.push(RetiredPointer { ptr });

        if retired.len() >= self.retire_threshold {
            self.try_reclaim(&mut retired);
        }
    }

    /// Try to reclaim retired pointers
    fn try_reclaim(&self, retired: &mut Vec<RetiredPointer<T>>) {
        // Collect all protected pointers
        let protected: Vec<*mut T> = self
            .hazard_pointers
            .iter()
            .map(|hp| hp.get())
            .filter(|p| !p.is_null())
            .collect();

        // Remove and free unprotected pointers
        retired.retain(|r| {
            if protected.contains(&r.ptr) {
                true // Keep, still protected
            } else {
                unsafe {
                    let _ = Box::from_raw(r.ptr);
                }
                false
            }
        });
    }
}

/// Counter tracking allocations for testing
pub struct AllocationCounter {
    allocated: AtomicUsize,
    freed: AtomicUsize,
}

impl AllocationCounter {
    pub fn new() -> Self {
        Self {
            allocated: AtomicUsize::new(0),
            freed: AtomicUsize::new(0),
        }
    }

    pub fn allocate(&self) {
        self.allocated.fetch_add(1, Ordering::SeqCst);
    }

    pub fn free(&self) {
        self.freed.fetch_add(1, Ordering::SeqCst);
    }

    pub fn allocated(&self) -> usize {
        self.allocated.load(Ordering::SeqCst)
    }

    pub fn freed(&self) -> usize {
        self.freed.load(Ordering::SeqCst)
    }

    pub fn live(&self) -> usize {
        self.allocated() - self.freed()
    }
}

impl Default for AllocationCounter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hazard_pointer_basic() {
        let hp: HazardPointer<i32> = HazardPointer::new();

        assert!(hp.get().is_null());

        let data = Box::into_raw(Box::new(42));
        hp.protect(data);

        assert_eq!(hp.get(), data);
        assert!(hp.is_protected(data));

        hp.clear();
        assert!(hp.get().is_null());

        // Cleanup
        unsafe {
            let _ = Box::from_raw(data);
        }
    }

    #[test]
    fn test_hazard_domain() {
        let domain: HazardDomain<i32> = HazardDomain::new(4);

        let hp = domain.get_hazard(0);

        let data = Box::into_raw(Box::new(42));
        hp.protect(data);

        // Retire shouldn't free while protected
        domain.retire(data);

        // Clear and retire again would allow reclamation
        hp.clear();
    }

    #[test]
    fn test_allocation_counter() {
        let counter = AllocationCounter::new();

        counter.allocate();
        counter.allocate();
        assert_eq!(counter.allocated(), 2);
        assert_eq!(counter.live(), 2);

        counter.free();
        assert_eq!(counter.freed(), 1);
        assert_eq!(counter.live(), 1);
    }
}
