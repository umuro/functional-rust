//! # Thread-Local Storage — Per-Thread State
//!
//! Each thread gets its own copy of thread-local data.

use std::cell::{Cell, RefCell};
use std::sync::Arc;
use std::thread;

// Thread-local counter
thread_local! {
    static COUNTER: Cell<usize> = const { Cell::new(0) };
}

// Thread-local string buffer
thread_local! {
    static BUFFER: RefCell<String> = RefCell::new(String::new());
}

/// Increment the thread-local counter
pub fn increment_counter() -> usize {
    COUNTER.with(|c| {
        let val = c.get() + 1;
        c.set(val);
        val
    })
}

/// Get the thread-local counter value
pub fn get_counter() -> usize {
    COUNTER.with(|c| c.get())
}

/// Append to the thread-local buffer
pub fn append_buffer(s: &str) {
    BUFFER.with(|b| {
        b.borrow_mut().push_str(s);
    });
}

/// Get the thread-local buffer contents
pub fn get_buffer() -> String {
    BUFFER.with(|b| b.borrow().clone())
}

/// Clear the thread-local buffer
pub fn clear_buffer() {
    BUFFER.with(|b| {
        b.borrow_mut().clear();
    });
}

/// Demonstrate thread-local isolation
pub fn thread_local_isolation(num_threads: usize, increments: usize) -> Vec<usize> {
    let results = Arc::new(std::sync::Mutex::new(Vec::new()));

    let handles: Vec<_> = (0..num_threads)
        .map(|_| {
            let results = Arc::clone(&results);
            thread::spawn(move || {
                // Each thread starts with counter = 0
                for _ in 0..increments {
                    increment_counter();
                }
                let final_count = get_counter();
                results.lock().unwrap().push(final_count);
            })
        })
        .collect();

    for h in handles {
        h.join().unwrap();
    }

    Arc::try_unwrap(results).unwrap().into_inner().unwrap()
}

/// Thread-local random number generator pattern
pub mod rng {
    use std::cell::Cell;

    thread_local! {
        static SEED: Cell<u64> = Cell::new(12345);
    }

    pub fn next_u64() -> u64 {
        SEED.with(|s| {
            // Simple LCG
            let x = s.get().wrapping_mul(6364136223846793005).wrapping_add(1);
            s.set(x);
            x
        })
    }

    pub fn seed(value: u64) {
        SEED.with(|s| s.set(value));
    }
}

/// Thread-local allocation tracking
pub mod alloc_tracking {
    use std::cell::Cell;

    thread_local! {
        static ALLOCATIONS: Cell<usize> = const { Cell::new(0) };
        static BYTES: Cell<usize> = const { Cell::new(0) };
    }

    pub fn record_allocation(bytes: usize) {
        ALLOCATIONS.with(|a| a.set(a.get() + 1));
        BYTES.with(|b| b.set(b.get() + bytes));
    }

    pub fn get_stats() -> (usize, usize) {
        (
            ALLOCATIONS.with(|a| a.get()),
            BYTES.with(|b| b.get()),
        )
    }

    pub fn reset() {
        ALLOCATIONS.with(|a| a.set(0));
        BYTES.with(|b| b.set(0));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_counter_basic() {
        // Reset for this test
        COUNTER.with(|c| c.set(0));

        assert_eq!(get_counter(), 0);
        increment_counter();
        assert_eq!(get_counter(), 1);
        increment_counter();
        assert_eq!(get_counter(), 2);
    }

    #[test]
    fn test_buffer() {
        clear_buffer();

        append_buffer("hello");
        append_buffer(" world");
        assert_eq!(get_buffer(), "hello world");

        clear_buffer();
        assert_eq!(get_buffer(), "");
    }

    #[test]
    fn test_thread_isolation() {
        let results = thread_local_isolation(4, 100);

        // Each thread should have counted to 100 independently
        for r in results {
            assert_eq!(r, 100);
        }
    }

    #[test]
    fn test_rng() {
        rng::seed(42);
        let a = rng::next_u64();
        let b = rng::next_u64();
        assert_ne!(a, b);

        // Reseeding gives same sequence
        rng::seed(42);
        assert_eq!(rng::next_u64(), a);
    }

    #[test]
    fn test_alloc_tracking() {
        alloc_tracking::reset();

        alloc_tracking::record_allocation(100);
        alloc_tracking::record_allocation(200);

        let (count, bytes) = alloc_tracking::get_stats();
        assert_eq!(count, 2);
        assert_eq!(bytes, 300);

        alloc_tracking::reset();
        let (count, bytes) = alloc_tracking::get_stats();
        assert_eq!(count, 0);
        assert_eq!(bytes, 0);
    }
}
