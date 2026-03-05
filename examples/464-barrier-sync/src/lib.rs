//! # Barrier Synchronization — Wait for All Threads
//!
//! A barrier blocks threads until all have reached it.

use std::sync::{Arc, Barrier};
use std::thread;

/// Demonstrate basic barrier usage
pub fn barrier_demo(num_threads: usize) -> Vec<(usize, bool)> {
    let barrier = Arc::new(Barrier::new(num_threads));
    let results = Arc::new(std::sync::Mutex::new(Vec::new()));

    let handles: Vec<_> = (0..num_threads)
        .map(|id| {
            let barrier = Arc::clone(&barrier);
            let results = Arc::clone(&results);
            thread::spawn(move || {
                // Phase 1: each thread does its work
                let work = id * 10;

                // Wait for all threads
                let wait_result = barrier.wait();

                // Record if we're the leader
                results.lock().unwrap().push((work, wait_result.is_leader()));
            })
        })
        .collect();

    for h in handles {
        h.join().unwrap();
    }

    Arc::try_unwrap(results).unwrap().into_inner().unwrap()
}

/// Multi-phase computation with barriers
pub fn multi_phase_computation(num_threads: usize, phases: usize) -> Vec<Vec<i32>> {
    let barrier = Arc::new(Barrier::new(num_threads));
    let phase_results: Arc<std::sync::Mutex<Vec<Vec<i32>>>> =
        Arc::new(std::sync::Mutex::new(vec![Vec::new(); phases]));

    let handles: Vec<_> = (0..num_threads)
        .map(|id| {
            let barrier = Arc::clone(&barrier);
            let phase_results = Arc::clone(&phase_results);
            thread::spawn(move || {
                for phase in 0..phases {
                    // Compute something for this phase
                    let result = (id as i32 + 1) * (phase as i32 + 1);

                    // Record result
                    phase_results.lock().unwrap()[phase].push(result);

                    // Wait for all threads before next phase
                    barrier.wait();
                }
            })
        })
        .collect();

    for h in handles {
        h.join().unwrap();
    }

    let mut results = Arc::try_unwrap(phase_results).unwrap().into_inner().unwrap();
    for phase in &mut results {
        phase.sort();
    }
    results
}

/// Parallel initialization pattern
pub fn parallel_init<T, F>(num_threads: usize, init_fn: F) -> Vec<T>
where
    T: Send + 'static,
    F: Fn(usize) -> T + Send + Sync + 'static,
{
    let barrier = Arc::new(Barrier::new(num_threads + 1)); // +1 for main
    let results = Arc::new(std::sync::Mutex::new(Vec::with_capacity(num_threads)));
    let init_fn = Arc::new(init_fn);

    let handles: Vec<_> = (0..num_threads)
        .map(|id| {
            let barrier = Arc::clone(&barrier);
            let results = Arc::clone(&results);
            let init_fn = Arc::clone(&init_fn);
            thread::spawn(move || {
                let value = init_fn(id);
                results.lock().unwrap().push((id, value));
                barrier.wait(); // Signal done
            })
        })
        .collect();

    // Wait for all initializations
    barrier.wait();

    for h in handles {
        h.join().unwrap();
    }

    let mut pairs = Arc::try_unwrap(results).unwrap().into_inner().unwrap();
    pairs.sort_by_key(|(id, _)| *id);
    pairs.into_iter().map(|(_, v)| v).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_barrier_basic() {
        let results = barrier_demo(4);

        // All 4 threads completed
        assert_eq!(results.len(), 4);

        // Exactly one is the leader
        let leaders: usize = results.iter().filter(|(_, is_leader)| *is_leader).count();
        assert_eq!(leaders, 1);
    }

    #[test]
    fn test_multi_phase() {
        let results = multi_phase_computation(3, 2);

        // Phase 0: threads 1,2,3 * phase 1 = [1, 2, 3]
        assert_eq!(results[0], vec![1, 2, 3]);

        // Phase 1: threads 1,2,3 * phase 2 = [2, 4, 6]
        assert_eq!(results[1], vec![2, 4, 6]);
    }

    #[test]
    fn test_parallel_init() {
        let results = parallel_init(4, |id| format!("thread-{}", id));

        assert_eq!(results.len(), 4);
        assert_eq!(results[0], "thread-0");
        assert_eq!(results[3], "thread-3");
    }

    #[test]
    fn test_barrier_reuse() {
        let barrier = Arc::new(Barrier::new(2));
        let count = Arc::new(std::sync::atomic::AtomicUsize::new(0));

        let b = Arc::clone(&barrier);
        let c = Arc::clone(&count);
        let t1 = thread::spawn(move || {
            for _ in 0..3 {
                c.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                b.wait();
            }
        });

        let t2 = thread::spawn(move || {
            for _ in 0..3 {
                count.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                barrier.wait();
            }
        });

        t1.join().unwrap();
        t2.join().unwrap();
    }
}
