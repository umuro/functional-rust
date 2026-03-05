//! # Atomic Ordering — Memory Ordering Semantics
//!
//! Understanding when to use Relaxed, Acquire, Release, AcqRel, and SeqCst.

use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;

/// Relaxed ordering: only guarantees atomicity
///
/// Use when you only need the counter to be accurate eventually,
/// and don't care about ordering relative to other operations.
pub fn relaxed_counter(threads: usize, increments: usize) -> usize {
    let counter = Arc::new(AtomicUsize::new(0));

    let handles: Vec<_> = (0..threads)
        .map(|_| {
            let c = Arc::clone(&counter);
            thread::spawn(move || {
                for _ in 0..increments {
                    // Relaxed: cheapest, no synchronization
                    c.fetch_add(1, Ordering::Relaxed);
                }
            })
        })
        .collect();

    for h in handles {
        h.join().unwrap();
    }

    // Use SeqCst for final read to ensure we see all updates
    counter.load(Ordering::SeqCst)
}

/// Release-Acquire ordering: producer-consumer pattern
///
/// Release: all writes before this store are visible after Acquire load
/// Acquire: sees all writes that happened before the Release store
pub fn release_acquire_flag() -> bool {
    let data = Arc::new(AtomicUsize::new(0));
    let ready = Arc::new(AtomicBool::new(false));

    let data_clone = Arc::clone(&data);
    let ready_clone = Arc::clone(&ready);

    // Producer
    let producer = thread::spawn(move || {
        data_clone.store(42, Ordering::Relaxed);
        // Release: ensures data store is visible before ready
        ready_clone.store(true, Ordering::Release);
    });

    // Consumer
    let consumer = thread::spawn(move || {
        // Spin until ready (Acquire ensures we see data)
        while !ready.load(Ordering::Acquire) {
            thread::yield_now();
        }
        // Guaranteed to see 42
        data.load(Ordering::Relaxed) == 42
    });

    producer.join().unwrap();
    consumer.join().unwrap()
}

/// SeqCst ordering: total ordering across all threads
///
/// Most expensive but provides strongest guarantees.
pub fn seqcst_example() -> usize {
    let a = Arc::new(AtomicUsize::new(0));
    let b = Arc::new(AtomicUsize::new(0));

    let a1 = Arc::clone(&a);
    let b1 = Arc::clone(&b);

    let t1 = thread::spawn(move || {
        a1.store(1, Ordering::SeqCst);
        b1.load(Ordering::SeqCst)
    });

    let a2 = Arc::clone(&a);
    let b2 = Arc::clone(&b);

    let t2 = thread::spawn(move || {
        b2.store(1, Ordering::SeqCst);
        a2.load(Ordering::SeqCst)
    });

    let r1 = t1.join().unwrap();
    let r2 = t2.join().unwrap();

    // With SeqCst, at least one of them must see the other's write
    // It's impossible for both to return 0
    r1 + r2
}

/// AcqRel: for read-modify-write operations
pub fn acqrel_example() -> usize {
    let counter = Arc::new(AtomicUsize::new(0));

    let handles: Vec<_> = (0..2)
        .map(|_| {
            let c = Arc::clone(&counter);
            thread::spawn(move || {
                for _ in 0..100 {
                    // AcqRel: combines Acquire (read) and Release (write)
                    c.fetch_add(1, Ordering::AcqRel);
                }
            })
        })
        .collect();

    for h in handles {
        h.join().unwrap();
    }

    counter.load(Ordering::SeqCst)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_relaxed_counter() {
        let result = relaxed_counter(4, 1000);
        assert_eq!(result, 4000);
    }

    #[test]
    fn test_release_acquire() {
        // Run multiple times to catch potential issues
        for _ in 0..10 {
            assert!(release_acquire_flag());
        }
    }

    #[test]
    fn test_seqcst() {
        // Run multiple times
        for _ in 0..10 {
            let sum = seqcst_example();
            // With SeqCst, at least one thread sees the other's write
            assert!(sum >= 1);
        }
    }

    #[test]
    fn test_acqrel() {
        let result = acqrel_example();
        assert_eq!(result, 200);
    }

    #[test]
    fn test_ordering_hierarchy() {
        // Demonstrate that all orderings are valid for basic operations
        let a = AtomicUsize::new(0);

        a.store(1, Ordering::Relaxed);
        a.store(2, Ordering::Release);
        a.store(3, Ordering::SeqCst);

        let _ = a.load(Ordering::Relaxed);
        let _ = a.load(Ordering::Acquire);
        let _ = a.load(Ordering::SeqCst);

        a.fetch_add(1, Ordering::Relaxed);
        a.fetch_add(1, Ordering::AcqRel);
        a.fetch_add(1, Ordering::SeqCst);
    }

    #[test]
    fn test_compare_exchange_orderings() {
        let a = AtomicUsize::new(5);

        // Success ordering, failure ordering
        let _ = a.compare_exchange(5, 10, Ordering::SeqCst, Ordering::Relaxed);
        assert_eq!(a.load(Ordering::SeqCst), 10);

        // Fails because current != expected
        let result = a.compare_exchange(5, 15, Ordering::SeqCst, Ordering::Relaxed);
        assert!(result.is_err());
    }
}
