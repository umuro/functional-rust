//! # Memory Ordering — Fences and Barriers
//!
//! Understanding memory fences and their relationship to atomic ordering.

use std::sync::atomic::{fence, AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;

/// Demonstrate fence usage for synchronization
pub fn fence_synchronization() -> bool {
    let data = Arc::new(AtomicUsize::new(0));
    let ready = Arc::new(AtomicBool::new(false));

    let data_clone = Arc::clone(&data);
    let ready_clone = Arc::clone(&ready);

    // Writer thread
    let writer = thread::spawn(move || {
        data_clone.store(42, Ordering::Relaxed);
        fence(Ordering::Release); // All previous writes visible after fence
        ready_clone.store(true, Ordering::Relaxed);
    });

    // Reader thread
    let reader = thread::spawn(move || {
        while !ready.load(Ordering::Relaxed) {
            thread::yield_now();
        }
        fence(Ordering::Acquire); // See all writes before the Release fence
        data.load(Ordering::Relaxed) == 42
    });

    writer.join().unwrap();
    reader.join().unwrap()
}

/// Store buffer example - why ordering matters
pub fn store_buffer_example() -> (usize, usize) {
    let x = Arc::new(AtomicUsize::new(0));
    let y = Arc::new(AtomicUsize::new(0));

    let x1 = Arc::clone(&x);
    let y1 = Arc::clone(&y);

    let t1 = thread::spawn(move || {
        x1.store(1, Ordering::SeqCst);
        y1.load(Ordering::SeqCst)
    });

    let x2 = Arc::clone(&x);
    let y2 = Arc::clone(&y);

    let t2 = thread::spawn(move || {
        y2.store(1, Ordering::SeqCst);
        x2.load(Ordering::SeqCst)
    });

    let r1 = t1.join().unwrap();
    let r2 = t2.join().unwrap();

    // With SeqCst, it's impossible for both to return 0
    // At least one must see the other's store
    (r1, r2)
}

/// Acquire-Release pair without fences (using atomic ordering)
pub fn acquire_release_pair() -> i32 {
    let data = Arc::new(AtomicUsize::new(0));
    let flag = Arc::new(AtomicBool::new(false));

    let d = Arc::clone(&data);
    let f = Arc::clone(&flag);

    let producer = thread::spawn(move || {
        d.store(100, Ordering::Relaxed);
        f.store(true, Ordering::Release); // Release pairs with Acquire
    });

    let consumer = thread::spawn(move || {
        while !flag.load(Ordering::Acquire) {
            thread::yield_now();
        }
        data.load(Ordering::Relaxed) as i32
    });

    producer.join().unwrap();
    consumer.join().unwrap()
}

/// Memory barrier types demonstration
pub mod barriers {
    use super::*;

    /// Compiler fence only (no CPU fence)
    pub fn compiler_fence_example() {
        let x = AtomicUsize::new(0);

        x.store(1, Ordering::Relaxed);
        std::sync::atomic::compiler_fence(Ordering::SeqCst);
        // Compiler won't reorder across this point
        let _ = x.load(Ordering::Relaxed);
    }

    /// Full memory fence
    pub fn full_fence_example() {
        let x = AtomicUsize::new(0);

        x.store(1, Ordering::Relaxed);
        fence(Ordering::SeqCst); // Full CPU memory barrier
        let _ = x.load(Ordering::Relaxed);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fence_sync() {
        for _ in 0..10 {
            assert!(fence_synchronization());
        }
    }

    #[test]
    fn test_store_buffer() {
        for _ in 0..10 {
            let (r1, r2) = store_buffer_example();
            // With SeqCst, at least one sees the other's write
            assert!(r1 > 0 || r2 > 0);
        }
    }

    #[test]
    fn test_acquire_release() {
        for _ in 0..10 {
            assert_eq!(acquire_release_pair(), 100);
        }
    }

    #[test]
    fn test_fence_types() {
        // Verify fence doesn't panic
        fence(Ordering::Acquire);
        fence(Ordering::Release);
        fence(Ordering::AcqRel);
        fence(Ordering::SeqCst);
    }

    #[test]
    fn test_barrier_modules() {
        barriers::compiler_fence_example();
        barriers::full_fence_example();
    }

    #[test]
    fn test_ordering_strength() {
        // Demonstrates valid ordering combinations
        let a = AtomicUsize::new(0);

        // Store: Relaxed, Release, or SeqCst
        a.store(1, Ordering::Relaxed);
        a.store(2, Ordering::Release);
        a.store(3, Ordering::SeqCst);

        // Load: Relaxed, Acquire, or SeqCst
        let _ = a.load(Ordering::Relaxed);
        let _ = a.load(Ordering::Acquire);
        let _ = a.load(Ordering::SeqCst);

        // RMW: Relaxed, Acquire, Release, AcqRel, or SeqCst
        a.fetch_add(1, Ordering::Relaxed);
        a.fetch_add(1, Ordering::Acquire);
        a.fetch_add(1, Ordering::Release);
        a.fetch_add(1, Ordering::AcqRel);
        a.fetch_add(1, Ordering::SeqCst);
    }
}
