// 993: Thread Pool / Work Queue
// Fixed N workers consuming tasks from a shared mpsc channel

use std::sync::{mpsc, Arc, Mutex};
use std::thread;

type Task = Box<dyn FnOnce() + Send + 'static>;

struct ThreadPool {
    sender: mpsc::Sender<Task>,
    workers: Vec<thread::JoinHandle<()>>,
}

impl ThreadPool {
    fn new(size: usize) -> Self {
        assert!(size > 0);
        let (sender, receiver) = mpsc::channel::<Task>();
        // Wrap receiver in Arc<Mutex> so all workers can share it
        let receiver = Arc::new(Mutex::new(receiver));

        let workers = (0..size).map(|_| {
            let rx = Arc::clone(&receiver);
            thread::spawn(move || {
                // Each worker loops: lock, get task, unlock, run task
                loop {
                    let task = {
                        let lock = rx.lock().unwrap();
                        lock.recv() // blocks until task arrives or channel closes
                    };
                    match task {
                        Ok(f) => f(),
                        Err(_) => break, // channel closed → exit
                    }
                }
            })
        }).collect();

        ThreadPool { sender, workers }
    }

    fn execute<F: FnOnce() + Send + 'static>(&self, f: F) {
        self.sender.send(Box::new(f)).unwrap();
    }

    fn shutdown(self) {
        drop(self.sender); // close channel → workers see Err and break
        for w in self.workers { w.join().unwrap(); }
    }
}

// --- Approach 1: Submit tasks that collect results ---
fn pool_squares() -> Vec<i64> {
    let pool = ThreadPool::new(4);
    let results = Arc::new(Mutex::new(Vec::new()));

    for i in 1i64..=20 {
        let results = Arc::clone(&results);
        pool.execute(move || {
            results.lock().unwrap().push(i * i);
        });
    }

    pool.shutdown();
    let mut v = results.lock().unwrap().clone();
    v.sort();
    v
}

// --- Approach 2: Work queue with return values via channel ---
fn pool_with_results(inputs: Vec<i32>) -> Vec<i32> {
    let pool = ThreadPool::new(3);
    let (tx, rx) = mpsc::channel::<i32>();

    let n = inputs.len();
    for x in inputs {
        let tx = tx.clone();
        pool.execute(move || {
            tx.send(x * x).unwrap();
        });
    }
    drop(tx); // close sender side
    pool.shutdown();

    let mut results: Vec<i32> = rx.iter().collect();
    // Ensure we got all results (pool shutdown closed the channel)
    assert_eq!(results.len(), n);
    results.sort();
    results
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pool_squares_all_computed() {
        let squares = pool_squares();
        assert_eq!(squares.len(), 20);
        // Sum of i^2 for i=1..20 = 2870
        let sum: i64 = squares.iter().sum();
        assert_eq!(sum, 2870);
    }

    #[test]
    fn test_pool_with_results() {
        let results = pool_with_results(vec![1, 2, 3, 4, 5]);
        assert_eq!(results, vec![1, 4, 9, 16, 25]);
    }

    #[test]
    fn test_pool_empty_tasks() {
        let pool = ThreadPool::new(2);
        pool.shutdown(); // should not hang
    }

    #[test]
    fn test_pool_single_worker() {
        let pool = ThreadPool::new(1);
        let results = Arc::new(Mutex::new(Vec::new()));
        for i in 0..5 {
            let r = Arc::clone(&results);
            pool.execute(move || r.lock().unwrap().push(i));
        }
        pool.shutdown();
        let mut v = results.lock().unwrap().clone();
        v.sort();
        assert_eq!(v, vec![0, 1, 2, 3, 4]);
    }

    #[test]
    fn test_pool_more_tasks_than_workers() {
        let pool = ThreadPool::new(2);
        let counter = Arc::new(Mutex::new(0u32));
        for _ in 0..100 {
            let c = Arc::clone(&counter);
            pool.execute(move || *c.lock().unwrap() += 1);
        }
        pool.shutdown();
        assert_eq!(*counter.lock().unwrap(), 100);
    }
}
