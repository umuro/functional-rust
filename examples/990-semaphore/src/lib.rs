// 990: Semaphore via Mutex<usize> + Condvar
// Counting semaphore: limit N concurrent operations

use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::time::Duration;

struct Semaphore {
    count: Mutex<usize>,
    cond: Condvar,
    max: usize,
}

impl Semaphore {
    fn new(n: usize) -> Self {
        Semaphore {
            count: Mutex::new(n),
            cond: Condvar::new(),
            max: n,
        }
    }

    fn acquire(&self) {
        let mut count = self.count.lock().unwrap();
        while *count == 0 {
            count = self.cond.wait(count).unwrap();
        }
        *count -= 1;
    }

    fn release(&self) {
        let mut count = self.count.lock().unwrap();
        if *count < self.max {
            *count += 1;
            self.cond.notify_one();
        }
    }

    fn with_permit<T, F: FnOnce() -> T>(&self, f: F) -> T {
        self.acquire();
        let result = f();
        self.release();
        result
    }
}

// --- Approach 1: Limit concurrent workers ---
fn limited_concurrency() -> usize {
    let sem = Arc::new(Semaphore::new(3));
    let active = Arc::new(Mutex::new(0usize));
    let max_active = Arc::new(Mutex::new(0usize));

    let handles: Vec<_> = (0..10)
        .map(|_| {
            let sem = Arc::clone(&sem);
            let active = Arc::clone(&active);
            let max_active = Arc::clone(&max_active);
            thread::spawn(move || {
                sem.with_permit(|| {
                    {
                        let mut a = active.lock().unwrap();
                        *a += 1;
                        let mut m = max_active.lock().unwrap();
                        if *a > *m {
                            *m = *a;
                        }
                    }
                    thread::sleep(Duration::from_millis(5));
                    *active.lock().unwrap() -= 1;
                });
            })
        })
        .collect();

    for h in handles {
        h.join().unwrap();
    }
    let x = *max_active.lock().unwrap();
    x
}

// --- Approach 2: Binary semaphore as mutex ---
fn binary_semaphore_counter() -> u32 {
    let sem = Arc::new(Semaphore::new(1));
    let counter = Arc::new(Mutex::new(0u32));

    let handles: Vec<_> = (0..5)
        .map(|_| {
            let sem = Arc::clone(&sem);
            let counter = Arc::clone(&counter);
            thread::spawn(move || {
                for _ in 0..100 {
                    sem.with_permit(|| {
                        *counter.lock().unwrap() += 1;
                    });
                }
            })
        })
        .collect();

    for h in handles {
        h.join().unwrap();
    }
    let x = *counter.lock().unwrap();
    x
}

// --- Approach 3: Drain a resource pool ---
fn resource_pool_demo() -> Vec<usize> {
    const POOL_SIZE: usize = 2;
    let sem = Arc::new(Semaphore::new(POOL_SIZE));
    let usage_log = Arc::new(Mutex::new(Vec::new()));

    let handles: Vec<_> = (0..6)
        .map(|i| {
            let sem = Arc::clone(&sem);
            let log = Arc::clone(&usage_log);
            thread::spawn(move || {
                sem.with_permit(|| {
                    log.lock().unwrap().push(i);
                    thread::sleep(Duration::from_millis(2));
                });
            })
        })
        .collect();

    for h in handles {
        h.join().unwrap();
    }
    let mut log = usage_log.lock().unwrap().clone();
    log.sort();
    log
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_limited_concurrency() {
        let max = limited_concurrency();
        assert!(max <= 3, "max concurrent was {}, expected ≤ 3", max);
        assert!(max >= 1);
    }

    #[test]
    fn test_binary_semaphore_correctness() {
        assert_eq!(binary_semaphore_counter(), 500);
    }

    #[test]
    fn test_resource_pool() {
        let log = resource_pool_demo();
        assert_eq!(log.len(), 6);
        assert_eq!(log, vec![0, 1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_semaphore_acquire_release() {
        let sem = Semaphore::new(2);
        sem.acquire();
        sem.acquire();
        // Can't acquire a third — release one
        sem.release();
        sem.acquire(); // should succeed
        sem.release();
        sem.release();
    }

    #[test]
    fn test_semaphore_permits_count() {
        let sem = Semaphore::new(3);
        assert_eq!(*sem.count.lock().unwrap(), 3);
        sem.acquire();
        assert_eq!(*sem.count.lock().unwrap(), 2);
        sem.release();
        assert_eq!(*sem.count.lock().unwrap(), 3);
    }
}
