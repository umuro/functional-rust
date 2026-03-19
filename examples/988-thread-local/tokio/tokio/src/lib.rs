#![allow(clippy::all)]
// 988: Task-Local Storage — Tokio version
// tokio::task_local! — each spawned task gets its own value

use std::sync::Arc;
use tokio::sync::Mutex;

tokio::task_local! {
    static TASK_VALUE: i32;
}

/// Task-local values are isolated per task
async fn task_local_demo() -> Vec<i32> {
    let results = Arc::new(Mutex::new(Vec::new()));

    let handles: Vec<_> = (0..5i32)
        .map(|i| {
            let results = Arc::clone(&results);
            tokio::spawn(TASK_VALUE.scope(i * 10, async move {
                // Each task sees its own TASK_VALUE
                tokio::task::yield_now().await;
                let v = TASK_VALUE.with(|val| *val);
                results.lock().await.push(v);
            }))
        })
        .collect();

    for h in handles {
        h.await.unwrap();
    }

    let mut v = results.lock().await.clone();
    v.sort();
    v
}

/// Task-local accumulator pattern
tokio::task_local! {
    static LOCAL_SUM: std::cell::RefCell<i64>;
}

async fn task_local_sum(id: i64) -> i64 {
    LOCAL_SUM.scope(std::cell::RefCell::new(0), async move {
        for i in 1..=10 {
            LOCAL_SUM.with(|s| {
                *s.borrow_mut() += i * id;
            });
        }
        LOCAL_SUM.with(|s| *s.borrow())
    }).await
}

async fn parallel_sums() -> i64 {
    let results = Arc::new(Mutex::new(Vec::new()));

    let handles: Vec<_> = (0..4i64)
        .map(|id| {
            let results = Arc::clone(&results);
            tokio::spawn(async move {
                let s = task_local_sum(id).await;
                results.lock().await.push(s);
            })
        })
        .collect();

    for h in handles { h.await.unwrap(); }
    results.lock().await.iter().sum()
}

/// Task-local cache (computed once per task scope)
tokio::task_local! {
    static TASK_NAME: String;
}

async fn get_task_name() -> String {
    TASK_NAME.with(|name| name.clone())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_task_local_isolation() {
        let counts = task_local_demo().await;
        assert_eq!(counts, vec![0, 10, 20, 30, 40]);
    }

    #[tokio::test]
    async fn test_parallel_sums() {
        // sum of (i * id) for i=1..10 = 55 * id
        // id=0: 0, id=1: 55, id=2: 110, id=3: 165 → total=330
        assert_eq!(parallel_sums().await, 330);
    }

    #[tokio::test]
    async fn test_task_local_doesnt_leak() {
        let v1 = TASK_VALUE.scope(999, async {
            TASK_VALUE.with(|v| *v)
        }).await;
        assert_eq!(v1, 999);

        // Different scope, different value
        let v2 = TASK_VALUE.scope(111, async {
            TASK_VALUE.with(|v| *v)
        }).await;
        assert_eq!(v2, 111);
    }

    #[tokio::test]
    async fn test_task_name_cached() {
        let name = TASK_NAME.scope("worker-1".to_string(), async {
            let n1 = get_task_name().await;
            let n2 = get_task_name().await;
            assert_eq!(n1, n2);
            n1
        }).await;
        assert_eq!(name, "worker-1");
    }
}
