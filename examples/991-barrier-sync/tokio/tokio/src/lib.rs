// 991: Barrier Synchronization — Tokio version
// tokio::sync::Barrier — async barrier for task synchronization

use std::sync::Arc;
use tokio::sync::{Barrier, Mutex};

/// Simple barrier — all tasks synchronize at one point
async fn barrier_demo() -> (Vec<String>, Vec<String>) {
    let n = 5;
    let barrier = Arc::new(Barrier::new(n));
    let phase1_log = Arc::new(Mutex::new(Vec::new()));
    let phase2_log = Arc::new(Mutex::new(Vec::new()));

    let handles: Vec<_> = (0..n)
        .map(|i| {
            let barrier = Arc::clone(&barrier);
            let p1 = Arc::clone(&phase1_log);
            let p2 = Arc::clone(&phase2_log);
            tokio::spawn(async move {
                // Phase 1: independent work
                tokio::time::sleep(std::time::Duration::from_millis(i as u64 * 2)).await;
                p1.lock().await.push(format!("p1:{}", i));

                // BARRIER — blocks until all N tasks arrive
                barrier.wait().await;

                // Phase 2: all start together after barrier
                p2.lock().await.push(format!("p2:{}", i));
            })
        })
        .collect();

    for h in handles { h.await.unwrap(); }

    let p1 = phase1_log.lock().await.clone();
    let p2 = phase2_log.lock().await.clone();
    (p1, p2)
}

/// Detect the "leader" (the last task to arrive)
async fn barrier_with_leader() -> Vec<bool> {
    let n = 4;
    let barrier = Arc::new(Barrier::new(n));
    let is_leader = Arc::new(Mutex::new(Vec::new()));

    let handles: Vec<_> = (0..n)
        .map(|_| {
            let barrier = Arc::clone(&barrier);
            let leaders = Arc::clone(&is_leader);
            tokio::spawn(async move {
                let result = barrier.wait().await;
                leaders.lock().await.push(result.is_leader());
            })
        })
        .collect();

    for h in handles { h.await.unwrap(); }
    is_leader.lock().await.clone()
}

/// Reusable barrier across multiple rounds
async fn multi_round_barrier() -> Vec<usize> {
    let n = 3;
    let barrier = Arc::new(Barrier::new(n));
    let counts = Arc::new(Mutex::new(vec![0usize; 2]));

    let handles: Vec<_> = (0..n)
        .map(|_| {
            let barrier = Arc::clone(&barrier);
            let counts = Arc::clone(&counts);
            tokio::spawn(async move {
                for round in 0..2 {
                    counts.lock().await[round] += 1;
                    barrier.wait().await;
                }
            })
        })
        .collect();

    for h in handles { h.await.unwrap(); }
    counts.lock().await.clone()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_barrier_both_phases_complete() {
        let (p1, p2) = barrier_demo().await;
        assert_eq!(p1.len(), 5);
        assert_eq!(p2.len(), 5);
    }

    #[tokio::test]
    async fn test_exactly_one_leader() {
        let leaders = barrier_with_leader().await;
        assert_eq!(leaders.len(), 4);
        assert_eq!(leaders.iter().filter(|&&b| b).count(), 1);
    }

    #[tokio::test]
    async fn test_reusable_barrier() {
        let rounds = multi_round_barrier().await;
        assert_eq!(rounds, vec![3, 3]);
    }

    #[tokio::test]
    async fn test_barrier_of_one() {
        let b = Barrier::new(1);
        let result = b.wait().await;
        assert!(result.is_leader());
    }

    #[tokio::test]
    async fn test_barrier_synchronizes_ordering() {
        let n = 4;
        let barrier = Arc::new(Barrier::new(n));
        let phase1_done = Arc::new(Mutex::new(0usize));
        let error = Arc::new(Mutex::new(false));

        let handles: Vec<_> = (0..n)
            .map(|_| {
                let b = Arc::clone(&barrier);
                let done = Arc::clone(&phase1_done);
                let err = Arc::clone(&error);
                tokio::spawn(async move {
                    *done.lock().await += 1;
                    b.wait().await;
                    if *done.lock().await != n {
                        *err.lock().await = true;
                    }
                })
            })
            .collect();

        for h in handles { h.await.unwrap(); }
        assert!(!*error.lock().await);
    }
}
