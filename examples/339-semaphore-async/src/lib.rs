//! # Semaphore Async
//!
//! Limit how many tasks run concurrently — rate limiting, connection pools, throttling.

use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::time::Duration;

/// A counting semaphore that limits concurrent access.
pub struct Semaphore {
    count: Mutex<usize>,
    cond: Condvar,
}

impl Semaphore {
    pub fn new(permits: usize) -> Arc<Self> {
        Arc::new(Self {
            count: Mutex::new(permits),
            cond: Condvar::new(),
        })
    }

    /// Acquire a permit, blocking if none available.
    pub fn acquire(&self) {
        let mut count = self.count.lock().unwrap();
        while *count == 0 {
            count = self.cond.wait(count).unwrap();
        }
        *count -= 1;
    }

    /// Release a permit, potentially waking a waiting thread.
    pub fn release(&self) {
        *self.count.lock().unwrap() += 1;
        self.cond.notify_one();
    }

    /// Get current available permits.
    pub fn available(&self) -> usize {
        *self.count.lock().unwrap()
    }
}

/// RAII permit that auto-releases when dropped.
pub struct Permit<'a> {
    semaphore: &'a Semaphore,
}

impl<'a> Permit<'a> {
    pub fn new(semaphore: &'a Semaphore) -> Self {
        semaphore.acquire();
        Self { semaphore }
    }
}

impl<'a> Drop for Permit<'a> {
    fn drop(&mut self) {
        self.semaphore.release();
    }
}

impl Semaphore {
    /// Acquire a permit that auto-releases on drop.
    pub fn permit(&self) -> Permit<'_> {
        Permit::new(self)
    }
}

/// Demonstrates semaphore limiting concurrency.
pub fn limited_concurrency_demo(max_concurrent: usize, num_workers: usize) -> usize {
    use std::sync::atomic::{AtomicUsize, Ordering};

    let sem = Semaphore::new(max_concurrent);
    let active = Arc::new(AtomicUsize::new(0));
    let peak = Arc::new(AtomicUsize::new(0));

    let handles: Vec<_> = (0..num_workers)
        .map(|_| {
            let sem = Arc::clone(&sem);
            let active = Arc::clone(&active);
            let peak = Arc::clone(&peak);
            thread::spawn(move || {
                let _permit = sem.permit();
                let current = active.fetch_add(1, Ordering::SeqCst) + 1;
                peak.fetch_max(current, Ordering::SeqCst);
                thread::sleep(Duration::from_millis(5));
                active.fetch_sub(1, Ordering::SeqCst);
            })
        })
        .collect();

    for h in handles {
        h.join().unwrap();
    }

    peak.load(Ordering::SeqCst)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    #[test]
    fn test_semaphore_limits_concurrency() {
        let peak = limited_concurrency_demo(2, 6);
        assert!(peak <= 2, "Peak {} exceeded limit 2", peak);
    }

    #[test]
    fn test_permit_auto_release() {
        let sem = Semaphore::new(1);
        {
            let _permit = sem.permit();
            assert_eq!(sem.available(), 0);
        }
        assert_eq!(sem.available(), 1);
    }

    #[test]
    fn test_multiple_permits() {
        let sem = Semaphore::new(3);
        let _p1 = sem.permit();
        let _p2 = sem.permit();
        assert_eq!(sem.available(), 1);
    }

    #[test]
    fn test_acquire_release() {
        let sem = Semaphore::new(2);
        sem.acquire();
        assert_eq!(sem.available(), 1);
        sem.release();
        assert_eq!(sem.available(), 2);
    }

    #[test]
    fn test_concurrent_workers() {
        let sem = Semaphore::new(3);
        let active = Arc::new(AtomicUsize::new(0));
        let peak = Arc::new(AtomicUsize::new(0));

        let handles: Vec<_> = (0..8)
            .map(|_| {
                let sem = Arc::clone(&sem);
                let active = Arc::clone(&active);
                let peak = Arc::clone(&peak);
                thread::spawn(move || {
                    let _permit = sem.permit();
                    let cur = active.fetch_add(1, Ordering::SeqCst) + 1;
                    peak.fetch_max(cur, Ordering::SeqCst);
                    thread::sleep(Duration::from_millis(2));
                    active.fetch_sub(1, Ordering::SeqCst);
                })
            })
            .collect();

        for h in handles {
            h.join().unwrap();
        }

        assert!(peak.load(Ordering::SeqCst) <= 3);
    }
}
