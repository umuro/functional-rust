//! # Thread Pool Pattern — Reusable Worker Threads
//!
//! A pool of worker threads that process jobs from a shared queue,
//! avoiding the overhead of spawning threads per task.

use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};

/// A job is a boxed closure that runs once
type Job = Box<dyn FnOnce() + Send + 'static>;

/// Approach 1: Basic thread pool with channel-based job queue
pub struct ThreadPool {
    workers: Vec<JoinHandle<()>>,
    sender: Option<Sender<Job>>,
}

impl ThreadPool {
    /// Create a new thread pool with `size` workers
    pub fn new(size: usize) -> Self {
        assert!(size > 0, "Thread pool must have at least one worker");

        let (sender, receiver) = mpsc::channel::<Job>();
        let receiver = Arc::new(Mutex::new(receiver));

        let workers = (0..size)
            .map(|_id| {
                let rx = Arc::clone(&receiver);
                thread::spawn(move || loop {
                    let job = rx.lock().unwrap().recv();
                    match job {
                        Ok(job) => job(),
                        Err(_) => break, // Channel closed
                    }
                })
            })
            .collect();

        ThreadPool {
            workers,
            sender: Some(sender),
        }
    }

    /// Submit a job to be executed by a worker
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.as_ref().unwrap().send(job).unwrap();
    }

    /// Get the number of workers
    pub fn size(&self) -> usize {
        self.workers.len()
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        // Drop sender to close channel
        drop(self.sender.take());

        // Wait for all workers to finish
        for worker in self.workers.drain(..) {
            worker.join().unwrap();
        }
    }
}

/// Approach 2: Scoped thread pool for borrowed data
pub fn scoped_pool<T, R, F>(data: &[T], num_threads: usize, f: F) -> Vec<R>
where
    T: Sync,
    R: Send + Default + Clone,
    F: Fn(&T) -> R + Sync,
{
    let chunk_size = (data.len() + num_threads - 1) / num_threads;
    let mut results = vec![R::default(); data.len()];

    thread::scope(|s| {
        for (chunk_data, chunk_results) in
            data.chunks(chunk_size).zip(results.chunks_mut(chunk_size))
        {
            s.spawn(|| {
                for (input, output) in chunk_data.iter().zip(chunk_results.iter_mut()) {
                    *output = f(input);
                }
            });
        }
    });

    results
}

/// Approach 3: Simple parallel map using thread pool
pub fn parallel_map<T, U, F>(pool: &ThreadPool, data: Vec<T>, f: F) -> Vec<U>
where
    T: Send + 'static,
    U: Send + std::fmt::Debug + 'static,
    F: Fn(T) -> U + Send + Sync + Clone + 'static,
{
    let results: Arc<Mutex<Vec<Option<(usize, U)>>>> = Arc::new(Mutex::new(Vec::new()));

    for (i, item) in data.into_iter().enumerate() {
        let f = f.clone();
        let results = Arc::clone(&results);
        pool.execute(move || {
            let result = f(item);
            results.lock().unwrap().push(Some((i, result)));
        });
    }

    // Wait for results (this is a simplified approach)
    drop(pool);

    let mut collected: Vec<_> = Arc::try_unwrap(results)
        .unwrap()
        .into_inner()
        .unwrap()
        .into_iter()
        .flatten()
        .collect();

    collected.sort_by_key(|(i, _)| *i);
    collected.into_iter().map(|(_, v)| v).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    #[test]
    fn test_pool_executes_all_jobs() {
        let count = Arc::new(AtomicUsize::new(0));

        {
            let pool = ThreadPool::new(4);

            for _ in 0..10 {
                let count = Arc::clone(&count);
                pool.execute(move || {
                    count.fetch_add(1, Ordering::SeqCst);
                });
            }
        } // Pool dropped, all jobs complete

        assert_eq!(count.load(Ordering::SeqCst), 10);
    }

    #[test]
    fn test_pool_size() {
        let pool = ThreadPool::new(4);
        assert_eq!(pool.size(), 4);
    }

    #[test]
    fn test_multiple_pools() {
        let count = Arc::new(AtomicUsize::new(0));

        {
            let pool1 = ThreadPool::new(2);
            let pool2 = ThreadPool::new(2);

            for _ in 0..5 {
                let c = Arc::clone(&count);
                pool1.execute(move || {
                    c.fetch_add(1, Ordering::SeqCst);
                });
            }
            for _ in 0..5 {
                let c = Arc::clone(&count);
                pool2.execute(move || {
                    c.fetch_add(1, Ordering::SeqCst);
                });
            }
        }

        assert_eq!(count.load(Ordering::SeqCst), 10);
    }

    #[test]
    fn test_scoped_pool() {
        let data: Vec<i32> = (1..=10).collect();
        let results = scoped_pool(&data, 4, |x| x * x);
        assert_eq!(results, vec![1, 4, 9, 16, 25, 36, 49, 64, 81, 100]);
    }

    #[test]
    fn test_results_collected() {
        let results = Arc::new(Mutex::new(Vec::new()));

        {
            let pool = ThreadPool::new(2);
            for i in 0..5 {
                let r = Arc::clone(&results);
                pool.execute(move || {
                    r.lock().unwrap().push(i * i);
                });
            }
        }

        let mut collected = results.lock().unwrap().clone();
        collected.sort();
        assert_eq!(collected, vec![0, 1, 4, 9, 16]);
    }

    #[test]
    #[should_panic]
    fn test_zero_workers_panics() {
        let _ = ThreadPool::new(0);
    }
}
