#![allow(clippy::all)]
// 987: Read-Write Lock Pattern
// Rust: RwLock<T> — many readers OR one writer, never both

use std::sync::{Arc, RwLock};
use std::thread;

// --- Approach 1: Multiple readers in parallel ---
fn concurrent_readers() -> Vec<i32> {
    let data = Arc::new(RwLock::new(42i32));

    let handles: Vec<_> = (0..5)
        .map(|_| {
            let data = Arc::clone(&data);
            thread::spawn(move || {
                let guard = data.read().unwrap(); // shared read lock
                *guard // all 5 can hold read lock simultaneously
            })
        })
        .collect();

    handles.into_iter().map(|h| h.join().unwrap()).collect()
}

// --- Approach 2: Writer excludes readers ---
fn write_then_read() -> i32 {
    let data = Arc::new(RwLock::new(0i32));

    {
        let mut guard = data.write().unwrap(); // exclusive write lock
        *guard = 100;
        // guard drops here — write lock released
    }

    let guard = data.read().unwrap();
    *guard
}

// --- Approach 3: Shared config pattern (read-heavy) ---
#[derive(Clone, Debug)]
struct Config {
    threshold: i32,
    name: String,
}

fn config_pattern() -> (String, i32) {
    let config = Arc::new(RwLock::new(Config {
        threshold: 10,
        name: "default".to_string(),
    }));

    // Many readers
    let readers: Vec<_> = (0..4)
        .map(|_| {
            let config = Arc::clone(&config);
            thread::spawn(move || {
                let c = config.read().unwrap();
                (c.name.clone(), c.threshold)
            })
        })
        .collect();

    // One writer updates the config
    {
        let cfg = Arc::clone(&config);
        let writer = thread::spawn(move || {
            let mut c = cfg.write().unwrap();
            c.threshold = 99;
            c.name = "updated".to_string();
        });
        writer.join().unwrap();
    }

    for h in readers {
        h.join().unwrap();
    } // let readers finish

    let c = config.read().unwrap();
    (c.name.clone(), c.threshold)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_concurrent_readers_all_see_same() {
        let reads = concurrent_readers();
        assert_eq!(reads.len(), 5);
        assert!(reads.iter().all(|&v| v == 42));
    }

    #[test]
    fn test_write_then_read() {
        assert_eq!(write_then_read(), 100);
    }

    #[test]
    fn test_config_pattern() {
        let (name, threshold) = config_pattern();
        assert_eq!(name, "updated");
        assert_eq!(threshold, 99);
    }

    #[test]
    fn test_try_read_write() {
        let rw = RwLock::new(0i32);
        let _r1 = rw.read().unwrap();
        let _r2 = rw.read().unwrap(); // multiple reads OK
                                      // rw.try_write() would fail here (readers active)
        assert!(rw.try_write().is_err());
    }

    #[test]
    fn test_rwlock_write_exclusive() {
        let rw = Arc::new(RwLock::new(vec![1, 2, 3]));
        {
            let mut w = rw.write().unwrap();
            w.push(4);
        }
        assert_eq!(*rw.read().unwrap(), vec![1, 2, 3, 4]);
    }
}
