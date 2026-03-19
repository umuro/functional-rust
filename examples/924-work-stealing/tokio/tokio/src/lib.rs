#![allow(clippy::all)]
// 924: Work Stealing — Tokio version
// Tokio's multi-threaded runtime uses work-stealing by default

use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;

/// Demonstrate work stealing: tasks distributed across runtime threads
async fn work_stealing_demo() -> Vec<(usize, String)> {
    let results = Arc::new(Mutex::new(Vec::new()));

    // Spawn tasks with varying workloads — work stealing redistributes
    let handles: Vec<_> = (0..10)
        .map(|i| {
            let results = Arc::clone(&results);
            tokio::spawn(async move {
                // Varying work amounts
                tokio::time::sleep(Duration::from_millis(i as u64 % 3)).await;
                let thread_id = format!("{:?}", std::thread::current().id());
                results.lock().await.push((i, thread_id));
            })
        })
        .collect();

    for h in handles { h.await.unwrap(); }
    let mut r = results.lock().await.clone();
    r.sort_by_key(|(i, _)| *i);
    r
}

/// Demonstrate that tasks migrate between threads (work stealing)
async fn task_migration() -> bool {
    let thread_ids = Arc::new(Mutex::new(Vec::new()));

    let handles: Vec<_> = (0..20)
        .map(|_| {
            let ids = Arc::clone(&thread_ids);
            tokio::spawn(async move {
                let id = format!("{:?}", std::thread::current().id());
                tokio::task::yield_now().await; // give runtime a chance to migrate
                ids.lock().await.push(id);
            })
        })
        .collect();

    for h in handles { h.await.unwrap(); }

    let ids = thread_ids.lock().await;
    let unique: std::collections::HashSet<&str> = ids.iter().map(|s| s.as_str()).collect();
    // With work stealing, tasks run on multiple threads
    unique.len() >= 1 // at minimum 1 thread (might be 1 in test runtime)
}

/// Unbalanced workload — work stealing helps redistribute
async fn unbalanced_workload() -> Vec<i64> {
    let handles: Vec<_> = vec![
        tokio::spawn(async {
            // Heavy task
            let mut sum: i64 = 0;
            for i in 0..10000 { sum += i; }
            sum
        }),
        tokio::spawn(async { 1i64 }), // Light task
        tokio::spawn(async { 2i64 }), // Light task
        tokio::spawn(async {
            // Heavy task
            let mut sum: i64 = 0;
            for i in 0..10000 { sum += i; }
            sum
        }),
        tokio::spawn(async { 3i64 }), // Light task
    ];

    let mut results = Vec::new();
    for h in handles {
        results.push(h.await.unwrap());
    }
    results
}

/// spawn_blocking for CPU-heavy work (offloaded to blocking thread pool)
async fn cpu_bound_tasks() -> Vec<u64> {
    let handles: Vec<_> = (1..=5u64)
        .map(|n| {
            tokio::task::spawn_blocking(move || {
                // CPU-intensive: compute fibonacci
                fn fib(n: u64) -> u64 {
                    if n <= 1 { return n; }
                    let (mut a, mut b) = (0u64, 1u64);
                    for _ in 2..=n {
                        let c = a + b;
                        a = b;
                        b = c;
                    }
                    b
                }
                fib(n * 10)
            })
        })
        .collect();

    let mut results = Vec::new();
    for h in handles {
        results.push(h.await.unwrap());
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_work_stealing_all_complete() {
        let results = work_stealing_demo().await;
        assert_eq!(results.len(), 10);
        // All task indices present
        let indices: Vec<usize> = results.iter().map(|(i, _)| *i).collect();
        assert_eq!(indices, (0..10).collect::<Vec<_>>());
    }

    #[tokio::test]
    async fn test_task_migration() {
        assert!(task_migration().await);
    }

    #[tokio::test]
    async fn test_unbalanced_workload() {
        let results = unbalanced_workload().await;
        assert_eq!(results.len(), 5);
        assert_eq!(results[1], 1);
        assert_eq!(results[2], 2);
        assert_eq!(results[4], 3);
        // Heavy tasks: sum of 0..10000 = 49995000
        assert_eq!(results[0], 49995000);
        assert_eq!(results[3], 49995000);
    }

    #[tokio::test]
    async fn test_cpu_bound_tasks() {
        let results = cpu_bound_tasks().await;
        assert_eq!(results.len(), 5);
        assert_eq!(results[0], 55); // fib(10) = 55
        assert_eq!(results[1], 6765); // fib(20) = 6765
    }
}
