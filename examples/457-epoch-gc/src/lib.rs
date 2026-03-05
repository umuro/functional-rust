//! # Epoch-Based Garbage Collection — Memory Reclamation
//!
//! A technique for safe memory reclamation in lock-free data structures.
//! Threads track which epoch they're in; old epochs can be reclaimed.

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

/// Global epoch counter
pub struct EpochTracker {
    global_epoch: AtomicUsize,
    thread_epochs: Vec<AtomicUsize>,
    garbage: [Mutex<Vec<Box<dyn Send>>>; 3],
}

impl EpochTracker {
    pub fn new(num_threads: usize) -> Self {
        Self {
            global_epoch: AtomicUsize::new(0),
            thread_epochs: (0..num_threads).map(|_| AtomicUsize::new(usize::MAX)).collect(),
            garbage: [
                Mutex::new(Vec::new()),
                Mutex::new(Vec::new()),
                Mutex::new(Vec::new()),
            ],
        }
    }

    /// Enter a critical section (announce thread is active)
    pub fn pin(&self, thread_id: usize) -> Guard {
        let epoch = self.global_epoch.load(Ordering::Relaxed);
        self.thread_epochs[thread_id].store(epoch, Ordering::Release);
        std::sync::atomic::fence(Ordering::SeqCst);
        Guard {
            tracker: self,
            thread_id,
        }
    }

    /// Get current epoch
    pub fn current_epoch(&self) -> usize {
        self.global_epoch.load(Ordering::Relaxed)
    }

    /// Try to advance the global epoch
    pub fn try_advance(&self) -> bool {
        let current = self.global_epoch.load(Ordering::Relaxed);

        // Check if all threads have left the old epoch
        let can_advance = self.thread_epochs.iter().all(|te| {
            let t_epoch = te.load(Ordering::Acquire);
            t_epoch == usize::MAX || t_epoch >= current
        });

        if can_advance {
            self.global_epoch.fetch_add(1, Ordering::Release);

            // Reclaim garbage from two epochs ago
            let old_epoch = (current + 1) % 3;
            let mut old_garbage = self.garbage[old_epoch].lock().unwrap();
            old_garbage.clear();

            true
        } else {
            false
        }
    }

    /// Defer destruction of a value
    pub fn defer<T: Send + 'static>(&self, value: T) {
        let epoch = self.global_epoch.load(Ordering::Relaxed) % 3;
        self.garbage[epoch].lock().unwrap().push(Box::new(value));
    }

    fn unpin(&self, thread_id: usize) {
        self.thread_epochs[thread_id].store(usize::MAX, Ordering::Release);
    }
}

/// Guard that automatically unpins on drop
pub struct Guard<'a> {
    tracker: &'a EpochTracker,
    thread_id: usize,
}

impl<'a> Drop for Guard<'a> {
    fn drop(&mut self) {
        self.tracker.unpin(self.thread_id);
    }
}

/// Simple epoch-based counter for demonstration
pub struct EpochCounter {
    value: AtomicUsize,
    epoch_tracker: Arc<EpochTracker>,
}

impl EpochCounter {
    pub fn new(tracker: Arc<EpochTracker>) -> Self {
        Self {
            value: AtomicUsize::new(0),
            epoch_tracker: tracker,
        }
    }

    pub fn increment(&self, thread_id: usize) -> usize {
        let _guard = self.epoch_tracker.pin(thread_id);
        self.value.fetch_add(1, Ordering::Relaxed)
    }

    pub fn get(&self, thread_id: usize) -> usize {
        let _guard = self.epoch_tracker.pin(thread_id);
        self.value.load(Ordering::Relaxed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_epoch_tracker_basic() {
        let tracker = EpochTracker::new(4);

        assert_eq!(tracker.current_epoch(), 0);

        {
            let _guard = tracker.pin(0);
            // While pinned, epoch should stay stable
        }

        // After unpin, epoch can advance
        tracker.try_advance();
    }

    #[test]
    fn test_pin_unpin() {
        let tracker = EpochTracker::new(2);

        // Thread 0 pins
        let guard = tracker.pin(0);

        // Check thread is pinned
        assert_ne!(
            tracker.thread_epochs[0].load(Ordering::Acquire),
            usize::MAX
        );

        // Drop guard
        drop(guard);

        // Check thread is unpinned
        assert_eq!(
            tracker.thread_epochs[0].load(Ordering::Acquire),
            usize::MAX
        );
    }

    #[test]
    fn test_defer() {
        let tracker = EpochTracker::new(2);

        tracker.defer(vec![1, 2, 3]);
        tracker.defer(String::from("hello"));

        // Garbage exists in current epoch
        let epoch = tracker.current_epoch() % 3;
        let garbage = tracker.garbage[epoch].lock().unwrap();
        assert_eq!(garbage.len(), 2);
    }

    #[test]
    fn test_epoch_counter() {
        let tracker = Arc::new(EpochTracker::new(4));
        let counter = Arc::new(EpochCounter::new(Arc::clone(&tracker)));

        let handles: Vec<_> = (0..4)
            .map(|id| {
                let c = Arc::clone(&counter);
                thread::spawn(move || {
                    for _ in 0..100 {
                        c.increment(id);
                    }
                })
            })
            .collect();

        for h in handles {
            h.join().unwrap();
        }

        assert_eq!(counter.get(0), 400);
    }
}
