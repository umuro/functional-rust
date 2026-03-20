#![allow(clippy::all)]
// 988: Thread-Local Storage
// Rust: thread_local! macro — each thread gets its own instance

use std::cell::RefCell;
use std::sync::{Arc, Mutex};
use std::thread;

// --- Approach 1: thread_local! with Cell (simple counter) ---
thread_local! {
    static COUNTER: RefCell<i32> = const { RefCell::new(0) };
}

fn thread_local_counter() -> Vec<i32> {
    let results = Arc::new(Mutex::new(Vec::new()));

    let handles: Vec<_> = (0..5i32)
        .map(|i| {
            let results = Arc::clone(&results);
            thread::spawn(move || {
                // Each thread has its own COUNTER — no sharing
                COUNTER.with(|c| *c.borrow_mut() = i * 10);
                thread::yield_now();
                let v = COUNTER.with(|c| *c.borrow());
                results.lock().unwrap().push(v);
            })
        })
        .collect();

    for h in handles {
        h.join().unwrap();
    }
    let mut v = results.lock().unwrap().clone();
    v.sort();
    v
}

// --- Approach 2: Thread-local accumulator (no shared state needed) ---
thread_local! {
    static LOCAL_SUM: RefCell<i64> = const { RefCell::new(0) };
}

fn thread_local_sum(id: i64) -> i64 {
    LOCAL_SUM.with(|s| {
        *s.borrow_mut() = 0; // reset for this thread
        for i in 1..=10 {
            *s.borrow_mut() += i * id;
        }
        *s.borrow()
    })
}

fn parallel_sums() -> i64 {
    let results = Arc::new(Mutex::new(Vec::new()));

    let handles: Vec<_> = (0..4i64)
        .map(|id| {
            let results = Arc::clone(&results);
            thread::spawn(move || {
                let s = thread_local_sum(id);
                results.lock().unwrap().push(s);
            })
        })
        .collect();

    for h in handles {
        h.join().unwrap();
    }
    let x = results.lock().unwrap().iter().sum();
    x
}

// --- Approach 3: Thread-local cache (computed once per thread) ---
thread_local! {
    static THREAD_ID_CACHE: RefCell<Option<String>> = const { RefCell::new(None) };
}

fn get_thread_name(name: &str) -> String {
    THREAD_ID_CACHE.with(|cache| {
        let mut c = cache.borrow_mut();
        if c.is_none() {
            *c = Some(format!("thread-{}", name));
        }
        c.clone().unwrap()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_thread_local_isolation() {
        let counts = thread_local_counter();
        assert_eq!(counts, vec![0, 10, 20, 30, 40]);
    }

    #[test]
    fn test_parallel_sums() {
        // 0 + 55 + 110 + 165 = 330
        assert_eq!(parallel_sums(), 330);
    }

    #[test]
    fn test_thread_local_doesnt_leak_across_threads() {
        COUNTER.with(|c| *c.borrow_mut() = 999);
        let val_in_new_thread = thread::spawn(|| {
            COUNTER.with(|c| *c.borrow()) // should be 0, not 999
        })
        .join()
        .unwrap();
        assert_eq!(val_in_new_thread, 0);
    }

    #[test]
    fn test_thread_name_cached() {
        let n1 = get_thread_name("x");
        let n2 = get_thread_name("y"); // returns cached value, not "thread-y"
        assert_eq!(n1, n2); // same thread — cached
    }
}
