#![allow(clippy::all)]
// 924: Work Stealing — std version
// Simple work-stealing deque using std threads

use std::sync::{Arc, Mutex};
use std::thread;
use std::collections::VecDeque;
use std::time::Duration;

/// A simple work-stealing queue: each worker has a local deque,
/// idle workers steal from others
struct WorkStealingPool {
    queues: Vec<Arc<Mutex<VecDeque<Box<dyn FnOnce() + Send>>>>>,
    workers: Vec<thread::JoinHandle<()>>,
}

impl WorkStealingPool {
    fn new(num_workers: usize) -> Self {
        let queues: Vec<_> = (0..num_workers)
            .map(|_| Arc::new(Mutex::new(VecDeque::<Box<dyn FnOnce() + Send>>::new())))
            .collect();

        let done = Arc::new(std::sync::atomic::AtomicBool::new(false));

        let workers: Vec<_> = (0..num_workers)
            .map(|i| {
                let queues = queues.clone();
                let done = Arc::clone(&done);
                thread::spawn(move || {
                    loop {
                        // Try own queue first
                        let task = queues[i].lock().unwrap().pop_front();
                        if let Some(f) = task {
                            f();
                            continue;
                        }

                        // Try stealing from others
                        let mut stolen = false;
                        for j in 0..queues.len() {
                            if j == i { continue; }
                            let task = queues[j].lock().unwrap().pop_back(); // steal from back
                            if let Some(f) = task {
                                f();
                                stolen = true;
                                break;
                            }
                        }

                        if !stolen {
                            if done.load(std::sync::atomic::Ordering::Acquire) {
                                // Check all queues empty
                                let all_empty = queues.iter()
                                    .all(|q| q.lock().unwrap().is_empty());
                                if all_empty { break; }
                            }
                            thread::sleep(Duration::from_millis(1));
                        }
                    }
                })
            })
            .collect();

        WorkStealingPool { queues, workers }
    }

    fn submit(&self, worker_id: usize, f: impl FnOnce() + Send + 'static) {
        self.queues[worker_id % self.queues.len()]
            .lock()
            .unwrap()
            .push_back(Box::new(f));
    }

    fn shutdown(self) {
        // Signal done — workers exit when all queues empty
        // (In production, use a proper shutdown signal)
        for w in self.workers {
            w.join().unwrap();
        }
    }
}

/// Simple demonstration: unbalanced load, work stealing redistributes
fn unbalanced_demo() -> Vec<i64> {
    let results = Arc::new(Mutex::new(Vec::new()));
    let done = Arc::new(std::sync::atomic::AtomicBool::new(false));
    let queues: Vec<Arc<Mutex<VecDeque<Box<dyn FnOnce() + Send>>>>> = (0..3)
        .map(|_| Arc::new(Mutex::new(VecDeque::new())))
        .collect();

    // All tasks go to worker 0's queue
    for i in 0..6i64 {
        let r = Arc::clone(&results);
        queues[0].lock().unwrap().push_back(Box::new(move || {
            r.lock().unwrap().push(i * i);
        }));
    }

    let handles: Vec<_> = (0..3)
        .map(|i| {
            let queues = queues.clone();
            let done = Arc::clone(&done);
            thread::spawn(move || {
                for _ in 0..20 {
                    let task = queues[i].lock().unwrap().pop_front()
                        .or_else(|| {
                            // Steal from other queues
                            for j in 0..queues.len() {
                                if j == i { continue; }
                                if let Some(t) = queues[j].lock().unwrap().pop_back() {
                                    return Some(t);
                                }
                            }
                            None
                        });
                    match task {
                        Some(f) => f(),
                        None => {
                            if done.load(std::sync::atomic::Ordering::Acquire) { break; }
                            thread::sleep(Duration::from_millis(1));
                        }
                    }
                }
            })
        })
        .collect();

    // Wait a bit then signal done
    thread::sleep(Duration::from_millis(20));
    done.store(true, std::sync::atomic::Ordering::Release);
    for h in handles { h.join().unwrap(); }

    let mut r = results.lock().unwrap().clone();
    r.sort();
    r
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unbalanced_demo() {
        let results = unbalanced_demo();
        assert_eq!(results.len(), 6);
        assert_eq!(results, vec![0, 1, 4, 9, 16, 25]);
    }

    #[test]
    fn test_work_stealing_concept() {
        // Verify the basic idea: tasks complete even when
        // all submitted to one queue
        let results = Arc::new(Mutex::new(Vec::new()));
        let q1 = Arc::new(Mutex::new(VecDeque::<Box<dyn FnOnce() + Send>>::new()));
        let q2 = Arc::new(Mutex::new(VecDeque::<Box<dyn FnOnce() + Send>>::new()));

        // Submit to q1 only
        for i in 0..4 {
            let r = Arc::clone(&results);
            q1.lock().unwrap().push_back(Box::new(move || {
                r.lock().unwrap().push(i);
            }));
        }

        // Worker 2 steals from q1
        let q1c = Arc::clone(&q1);
        while let Some(f) = q1c.lock().unwrap().pop_back() {
            f();
        }

        let mut r = results.lock().unwrap().clone();
        r.sort();
        assert_eq!(r, vec![0, 1, 2, 3]);
    }
}
