#![allow(clippy::all)]
//! # Work Stealing — Load Balancing Across Threads
//!
//! A pattern where idle workers "steal" tasks from busy workers'
//! queues to balance load dynamically.

use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::thread;

/// A work-stealing deque for a single worker
pub type WorkQueue<T> = Arc<Mutex<VecDeque<T>>>;

/// Create a new work queue
pub fn new_queue<T>() -> WorkQueue<T> {
    Arc::new(Mutex::new(VecDeque::new()))
}

/// Approach 1: Simple work stealing with shared deques
pub fn work_stealing_demo(num_jobs: usize, num_workers: usize) -> usize {
    let queues: Vec<WorkQueue<u32>> = (0..num_workers).map(|_| new_queue()).collect();

    // Load all jobs into first worker's queue
    {
        let mut q = queues[0].lock().unwrap();
        for j in 0..num_jobs as u32 {
            q.push_back(j);
        }
    }

    let completed = Arc::new(std::sync::atomic::AtomicUsize::new(0));

    let handles: Vec<_> = (0..num_workers)
        .map(|i| {
            let own_queue = Arc::clone(&queues[i]);
            let other_queues: Vec<_> = queues
                .iter()
                .enumerate()
                .filter(|&(j, _)| j != i)
                .map(|(_, q)| Arc::clone(q))
                .collect();
            let completed = Arc::clone(&completed);

            thread::spawn(move || {
                loop {
                    // Try own queue first (pop from front)
                    if let Some(_job) = own_queue.lock().unwrap().pop_front() {
                        completed.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                        continue;
                    }

                    // Try stealing from others (pop from back)
                    let mut stole = false;
                    for other in &other_queues {
                        if let Ok(mut guard) = other.try_lock() {
                            if let Some(_job) = guard.pop_back() {
                                drop(guard);
                                completed.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                                stole = true;
                                break;
                            }
                        }
                    }

                    if !stole {
                        break;
                    }
                }
            })
        })
        .collect();

    for h in handles {
        h.join().unwrap();
    }

    completed.load(std::sync::atomic::Ordering::SeqCst)
}

/// Approach 2: Work stealing with local processing
pub struct WorkStealingPool {
    queues: Vec<WorkQueue<Box<dyn FnOnce() + Send>>>,
    num_workers: usize,
}

impl WorkStealingPool {
    pub fn new(num_workers: usize) -> Self {
        let queues = (0..num_workers).map(|_| new_queue()).collect();
        Self {
            queues,
            num_workers,
        }
    }

    /// Push work to a specific worker's queue
    pub fn push(&self, worker_id: usize, job: Box<dyn FnOnce() + Send>) {
        let id = worker_id % self.num_workers;
        self.queues[id].lock().unwrap().push_back(job);
    }

    /// Push work round-robin
    pub fn push_round_robin(&self, jobs: Vec<Box<dyn FnOnce() + Send>>) {
        for (i, job) in jobs.into_iter().enumerate() {
            self.push(i, job);
        }
    }

    /// Run all jobs using work stealing
    pub fn run(self) {
        let handles: Vec<_> = (0..self.num_workers)
            .map(|i| {
                let own = Arc::clone(&self.queues[i]);
                let others: Vec<_> = self
                    .queues
                    .iter()
                    .enumerate()
                    .filter(|&(j, _)| j != i)
                    .map(|(_, q)| Arc::clone(q))
                    .collect();

                thread::spawn(move || {
                    loop {
                        // Own queue first
                        if let Some(job) = own.lock().unwrap().pop_front() {
                            job();
                            continue;
                        }

                        // Steal
                        let mut done = true;
                        for other in &others {
                            if let Ok(mut g) = other.try_lock() {
                                if let Some(job) = g.pop_back() {
                                    drop(g);
                                    job();
                                    done = false;
                                    break;
                                }
                            }
                        }

                        if done {
                            break;
                        }
                    }
                })
            })
            .collect();

        for h in handles {
            h.join().unwrap();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    #[test]
    fn test_steal_from_back() {
        let q: WorkQueue<u32> = new_queue();
        {
            let mut g = q.lock().unwrap();
            g.push_back(1);
            g.push_back(2);
            g.push_back(3);
        }

        // Steal from back
        assert_eq!(q.lock().unwrap().pop_back(), Some(3));
        // Own work from front
        assert_eq!(q.lock().unwrap().pop_front(), Some(1));
    }

    #[test]
    fn test_all_jobs_complete() {
        let completed = work_stealing_demo(20, 4);
        assert_eq!(completed, 20);
    }

    #[test]
    fn test_single_worker() {
        let completed = work_stealing_demo(10, 1);
        assert_eq!(completed, 10);
    }

    #[test]
    fn test_work_stealing_pool() {
        let count = Arc::new(AtomicUsize::new(0));
        let pool = WorkStealingPool::new(4);

        for _ in 0..20 {
            let c = Arc::clone(&count);
            pool.push(
                0,
                Box::new(move || {
                    c.fetch_add(1, Ordering::Relaxed);
                }),
            );
        }

        pool.run();
        assert_eq!(count.load(Ordering::SeqCst), 20);
    }

    #[test]
    fn test_round_robin_distribution() {
        let counts: Vec<_> = (0..4).map(|_| Arc::new(AtomicUsize::new(0))).collect();
        let pool = WorkStealingPool::new(4);

        let jobs: Vec<_> = (0..8)
            .map(|i| {
                let c = Arc::clone(&counts[i % 4]);
                let job: Box<dyn FnOnce() + Send> = Box::new(move || {
                    c.fetch_add(1, Ordering::Relaxed);
                });
                job
            })
            .collect();

        pool.push_round_robin(jobs);
        pool.run();

        // Each counter should have been incremented twice
        for c in &counts {
            assert_eq!(c.load(Ordering::SeqCst), 2);
        }
    }
}
