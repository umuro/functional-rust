// 982: Join Parallel Async — Tokio version
// tokio::join! and tokio::spawn for true concurrent async tasks

/// Join two async tasks (tokio::join! — like Lwt.both)
async fn parallel_both() -> (i32, i32) {
    let (a, b) = tokio::join!(
        async { 6 * 7 },
        async { 10 + 20 }
    );
    (a, b)
}

/// Join N tasks via tokio::spawn + JoinHandle
async fn parallel_map(tasks: Vec<i32>) -> Vec<i32> {
    let handles: Vec<_> = tasks
        .into_iter()
        .map(|v| tokio::spawn(async move { v * v }))
        .collect();

    let mut results = Vec::new();
    for h in handles {
        results.push(h.await.unwrap());
    }
    results
}

/// Parallel sum via spawned tasks
async fn parallel_sum(ns: Vec<i32>) -> i32 {
    let handles: Vec<_> = ns
        .into_iter()
        .map(|n| tokio::spawn(async move { n * n }))
        .collect();

    let mut sum = 0;
    for h in handles {
        sum += h.await.unwrap();
    }
    sum
}

/// tokio::try_join! — short-circuits on first error
async fn try_join_demo() -> Result<(i32, i32), &'static str> {
    let a = async { Ok::<i32, &str>(42) };
    let b = async { Ok::<i32, &str>(99) };
    tokio::try_join!(a, b)
}

async fn try_join_fails() -> Result<(i32, i32), &'static str> {
    let a = async { Ok::<i32, &str>(42) };
    let b = async { Err::<i32, &str>("oops") };
    tokio::try_join!(a, b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_parallel_both() {
        let (a, b) = parallel_both().await;
        assert_eq!(a, 42);
        assert_eq!(b, 30);
    }

    #[tokio::test]
    async fn test_parallel_map() {
        let results = parallel_map(vec![2, 3, 4]).await;
        assert_eq!(results, vec![4, 9, 16]);
    }

    #[tokio::test]
    async fn test_parallel_sum() {
        // 1+4+9+16 = 30
        assert_eq!(parallel_sum(vec![1, 2, 3, 4]).await, 30);
    }

    #[tokio::test]
    async fn test_empty_parallel_map() {
        let results = parallel_map(vec![]).await;
        assert!(results.is_empty());
    }

    #[tokio::test]
    async fn test_try_join_success() {
        assert_eq!(try_join_demo().await, Ok((42, 99)));
    }

    #[tokio::test]
    async fn test_try_join_failure() {
        assert_eq!(try_join_fails().await, Err("oops"));
    }

    #[tokio::test]
    async fn test_join_independent() {
        let (x, y) = tokio::join!(
            async { "hello" },
            async { 42u32 }
        );
        assert_eq!(x, "hello");
        assert_eq!(y, 42);
    }
}
