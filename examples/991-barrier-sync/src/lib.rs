// 991: Barrier Synchronization
// Rust: std::sync::Barrier — wait until N threads all arrive

use std::sync::{Arc, Barrier};
use std::thread;
use std::time::Duration;

// --- Approach 1: Simple barrier — all threads synchronize at one point ---
fn barrier_demo() -> (Vec<String>, Vec<String>) {
    let n = 5;
    let barrier = Arc::new(Barrier::new(n));
    let phase1_log = Arc::new(std::sync::Mutex::new(Vec::new()));
    let phase2_log = Arc::new(std::sync::Mutex::new(Vec::new()));

    let handles: Vec<_> = (0..n).map(|i| {
        let barrier = Arc::clone(&barrier);
        let p1 = Arc::clone(&phase1_log);
        let p2 = Arc::clone(&phase2_log);
        thread::spawn(move || {
            // Phase 1: independent work
            thread::sleep(Duration::from_millis(i as u64 * 2));
            p1.lock().unwrap().push(format!("p1:{}", i));

            // BARRIER — blocks until all N threads arrive
            barrier.wait();

            // Phase 2: all start together after barrier
            p2.lock().unwrap().push(format!("p2:{}", i));
        })
    }).collect();

    for h in handles { h.join().unwrap(); }

    let p1 = phase1_log.lock().unwrap().clone();
    let p2 = phase2_log.lock().unwrap().clone();
    (p1, p2)
}

// --- Approach 2: Detect the "leader" (the last thread to arrive) ---
fn barrier_with_leader() -> Vec<bool> {
    let n = 4;
    let barrier = Arc::new(Barrier::new(n));
    let is_leader = Arc::new(std::sync::Mutex::new(Vec::new()));

    let handles: Vec<_> = (0..n).map(|_| {
        let barrier = Arc::clone(&barrier);
        let leaders = Arc::clone(&is_leader);
        thread::spawn(move || {
            let result = barrier.wait();
            // BarrierWaitResult::is_leader() is true for exactly one thread
            leaders.lock().unwrap().push(result.is_leader());
        })
    }).collect();

    for h in handles { h.join().unwrap(); }
    let x = is_leader.lock().unwrap().clone(); x
}

// --- Approach 3: Reusable barrier across multiple rounds ---
fn multi_round_barrier() -> Vec<usize> {
    let n = 3;
    let barrier = Arc::new(Barrier::new(n));
    let counts = Arc::new(std::sync::Mutex::new(vec![0usize; 2]));

    let handles: Vec<_> = (0..n).map(|_| {
        let barrier = Arc::clone(&barrier);
        let counts = Arc::clone(&counts);
        thread::spawn(move || {
            for round in 0..2 {
                counts.lock().unwrap()[round] += 1;
                barrier.wait(); // resets automatically after all arrive
            }
        })
    }).collect();

    for h in handles { h.join().unwrap(); }
    let x = counts.lock().unwrap().clone(); x
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_barrier_both_phases_complete() {
        let (p1, p2) = barrier_demo();
        assert_eq!(p1.len(), 5);
        assert_eq!(p2.len(), 5);
    }

    #[test]
    fn test_exactly_one_leader() {
        let leaders = barrier_with_leader();
        assert_eq!(leaders.len(), 4);
        assert_eq!(leaders.iter().filter(|&&b| b).count(), 1);
    }

    #[test]
    fn test_reusable_barrier() {
        let rounds = multi_round_barrier();
        assert_eq!(rounds, vec![3, 3]); // all 3 threads counted in each round
    }

    #[test]
    fn test_barrier_new() {
        // Barrier of 1 passes immediately
        let b = Barrier::new(1);
        let result = b.wait();
        assert!(result.is_leader());
    }

    #[test]
    fn test_barrier_synchronizes_ordering() {
        // Ensure no thread reaches phase2 before all finish phase1
        let n = 4;
        let barrier = Arc::new(Barrier::new(n));
        let phase1_done = Arc::new(std::sync::Mutex::new(0usize));
        let error = Arc::new(std::sync::Mutex::new(false));

        let handles: Vec<_> = (0..n).map(|_| {
            let b = Arc::clone(&barrier);
            let done = Arc::clone(&phase1_done);
            let err = Arc::clone(&error);
            thread::spawn(move || {
                *done.lock().unwrap() += 1;
                b.wait();
                // After barrier, all must have finished phase1
                if *done.lock().unwrap() != n {
                    *err.lock().unwrap() = true;
                }
            })
        }).collect();

        for h in handles { h.join().unwrap(); }
        assert!(!*error.lock().unwrap());
    }
}
