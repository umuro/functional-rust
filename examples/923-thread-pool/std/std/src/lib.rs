// 923: Thread Pool — std version
// Fixed N worker threads consuming tasks from a shared channel

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
        let receiver = Arc::new(Mutex::new(receiver));

        let workers = (0..size)
            .map(|_| {
                let rx = Arc::clone(&receiver);
                thread::spawn(move || loop {
                    let task = {
                        let lock = rx.lock().unwrap();
                        lock.recv()
                    };
                    match task {
                        Ok(f) => f(),
                        Err(_) => break,
                    }
                })
            })
            .collect();

        ThreadPool { sender, workers }
    }

    fn execute<F: FnOnce() + Send + 'static>(&self, f: F) {
        self.sender.send(Box::new(f)).unwrap();
    }

    fn shutdown(self) {
        drop(self.sender);
        for w in self.workers {
            w.join().unwrap();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pool_computes() {
        let pool = ThreadPool::new(4);
        let results = Arc::new(Mutex::new(Vec::new()));

        for i in 1..=10 {
            let r = Arc::clone(&results);
            pool.execute(move || {
                r.lock().unwrap().push(i * i);
            });
        }

        pool.shutdown();
        let mut v = results.lock().unwrap().clone();
        v.sort();
        let sum: i32 = v.iter().sum();
        assert_eq!(sum, 385); // sum of squares 1..10
    }

    #[test]
    fn test_pool_empty() {
        let pool = ThreadPool::new(2);
        pool.shutdown(); // no tasks — should not hang
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
    fn test_pool_many_tasks() {
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
